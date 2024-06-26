from typing import List
import numpy as np
from tqdm import tqdm
import tensorflow as tf


class LineWorld:
    def __init__(self):
        self.agent_pos = 2

    def state_id(self) -> int:
        return self.agent_pos

    def available_actions(self) -> np.ndarray:
        if self.is_game_over():
            return np.array([])
        return np.array([0, 1])

    def state_desc(self) -> np.ndarray:
        return tf.keras.utils.to_categorical(self.agent_pos, 5)

    def num_states(self) -> int:
        return 5

    def num_actions(self) -> int:
        return 2

    def is_game_over(self) -> bool:
        return True if self.agent_pos == 0 or self.agent_pos == 4 else False

    def available_actions(self) -> List[int]:
        if self.is_game_over():
            return []
        else:
            return [0, 1]  # Left, Right

    def score(self) -> float:
        if self.agent_pos == 0:
            return -1.0
        elif self.agent_pos == 4:
            return 1.0
        return 0.0

    def step(self, action: int):
        assert (action in [0, 1])
        assert (not self.is_game_over())

        if action == 0:  # on veut aller à gauche
            self.agent_pos -= 1
        else:  # on veut aller à droite
            self.agent_pos += 1

    def reset(self):
        self.agent_pos = 2

    def display(self):
        for i in range(5):
            if i == self.agent_pos:
                print('X', end='')
            else:
                print('_', end='')


@tf.function
def predict(q, input_tensor):
    return q(input_tensor)


@tf.function
def gradient_step_terminal(q, input_tensor, alpha, r):
    with tf.GradientTape() as tape:
        q_s_a = predict(q, input_tensor)[0]

    grads = tape.gradient(q_s_a, q.trainable_variables)

    for (w, grad) in zip(q.trainable_variables, grads):
        w.assign(w + alpha * (r - q_s_a) * grad)


@tf.function
def gradient_step_non_terminal(q, input_tensor, alpha, r, gamma, q_s_p_a_p):
    with tf.GradientTape() as tape:
        q_s_a = predict(q, input_tensor)[0]

    grads = tape.gradient(q_s_a, q.trainable_variables)

    for (w, grad) in zip(q.trainable_variables, grads):
        w.assign(w + alpha * (r + gamma * q_s_p_a_p - q_s_a) * grad)


def episodic_semi_gradient_sarsa(env: LineWorld,
                                 num_episodes: int = 1000,
                                 alpha: float = 0.05,
                                 epsilon: float = 0.2,
                                 gamma: float = 0.999,
                                 ):
    q = tf.keras.models.Sequential()
    q.add(tf.keras.layers.Dense(16, activation=tf.keras.activations.tanh))
    q.add(tf.keras.layers.Dense(16, activation=tf.keras.activations.tanh))
    q.add(tf.keras.layers.Dense(1, activation=tf.keras.activations.linear))

    for _ep_id in tqdm(range(num_episodes)):
        env.reset()
        s = env.state_desc()

        if np.random.random() < epsilon:
            a = np.random.choice(env.available_actions())
        else:
            best_a = None
            best_q_s_a = None
            for a in env.available_actions():
                a_vector = tf.keras.utils.to_categorical(a, env.num_actions())
                input_tensor = np.expand_dims(np.concatenate([s, a_vector]), 0)
                q_s_a = predict(q, input_tensor)[0]
                if best_a is None or best_q_s_a < q_s_a:
                    best_a = a
                    best_q_s_a = q_s_a
            a = best_a

        while not env.is_game_over():
            prev_score = env.score()
            env.step(a)
            new_score = env.score()

            r = new_score - prev_score
            s_p = env.state_desc()

            if env.is_game_over():
                a_vector = tf.keras.utils.to_categorical(a, env.num_actions())
                input_tensor = np.expand_dims(np.concatenate([s, a_vector]), 0)

                gradient_step_terminal(q, input_tensor, alpha, r)
            else:
                if np.random.random() < epsilon:
                    a_p = np.random.choice(env.available_actions())
                else:
                    best_a_p = None
                    best_q_s_p_a_p = None
                    for a_p in env.available_actions():
                        a_p_vector = tf.keras.utils.to_categorical(a_p, env.num_actions())
                        input_tensor_p = np.expand_dims(np.concatenate([s_p, a_p_vector]), 0)
                        q_s_p_a_p = predict(q, input_tensor_p)[0]
                        if best_a_p is None or best_q_s_p_a_p < q_s_p_a_p:
                            best_a_p = a_p
                            best_q_s_p_a_p = q_s_p_a_p
                    a_p = best_a_p

                a_p_vector = tf.keras.utils.to_categorical(a_p, env.num_actions())
                input_tensor_p = np.expand_dims(np.concatenate([s_p, a_p_vector]), 0)
                q_s_p_a_p = predict(q, input_tensor_p)[0]

                a_vector = tf.keras.utils.to_categorical(a, env.num_actions())

                input_tensor = np.expand_dims(np.concatenate([s, a_vector]), 0)
                gradient_step_non_terminal(q, input_tensor, alpha, r, gamma, q_s_p_a_p)

                s = s_p
                a = a_p
    return q


def run_experiment():
    env = LineWorld()
    q_line_world = episodic_semi_gradient_sarsa(LineWorld(), 20000)
    for s in range(env.num_states()):
        for a in range(env.num_actions()):
            input_tensor = np.expand_dims(
                np.concatenate([
                    tf.keras.utils.to_categorical(s, env.num_states()),
                    tf.keras.utils.to_categorical(a, env.num_actions())
                ]), 0
            )
            print(f"s: {s}, a: {a}, q_s_a: {q_line_world(input_tensor)[0]}")


if __name__ == "__main__":
    run_experiment()
