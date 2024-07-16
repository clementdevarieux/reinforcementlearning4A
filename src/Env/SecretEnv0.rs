use rand::Rng;
use std::collections::HashMap;
use rand::distributions::{Distribution, Uniform};
use std::ffi::c_void;
use crate::secret_env::lib_secret_env::LIB;

pub struct SecretEnv0 {
    pub env: *mut c_void,
}

impl SecretEnv0 {

    pub fn new() -> Self {
        let secret_env_0_new: libloading::Symbol<unsafe extern fn() -> *mut c_void> =
            unsafe{LIB.get(b"secret_env_0_new").expect("Failed to load function `secret_env_0_new`")};
        unsafe {
            let env = secret_env_0_new();
            SecretEnv0 { env }
        }
    }

    pub fn p(&self, s: usize, a: usize, s_p: usize, r: usize) -> f32 {
        let secret_env_0_transition_probability: libloading::Symbol<unsafe extern fn(usize, usize, usize, usize) -> f32> =
            unsafe {
                LIB.get(b"secret_env_0_transition_probability").expect("Failed to load function `secret_env_0_transition_probability`")
            };
        unsafe {
            let p = secret_env_0_transition_probability(s, a, s_p, r);
            p
        }
    }

    pub fn num_states(&self) -> i32 {
        let secret_env_0_num_states: libloading::Symbol<unsafe extern fn() -> usize> =
            unsafe {LIB.get(b"secret_env_0_num_states").expect("Failed to load function `secret_env_0_num_states`")};
        unsafe {
            let num = secret_env_0_num_states();
            num as i32
        }
    }

    pub fn num_actions(&self) -> i32 {
        let secret_env_0_num_actions: libloading::Symbol<unsafe extern fn() -> usize> =
            unsafe{LIB.get(b"secret_env_0_num_actions").expect("Failed to load function `secret_env_0_num_actions`")};
        unsafe {
            let num = secret_env_0_num_actions();
            num as i32
        }
    }

    pub fn agent_pos(&self) -> i32{
        let secret_env_0_state_id: libloading::Symbol<unsafe extern fn(*const c_void) -> usize> =
            unsafe{LIB.get(b"secret_env_0_state_id").expect("Failed to load function `secret_env_0_state_id`")};
        unsafe {
            let s = secret_env_0_state_id(self.env);
            s as i32
        }
    }

    pub fn A(&self) -> Vec<usize> {
        let secret_env_0_available_actions: libloading::Symbol<unsafe extern fn(*const c_void) -> *const usize> =
            unsafe{LIB.get(b"secret_env_0_available_actions").expect("Failed to load function `secret_env_0_available_actions`")};
        unsafe {
            let actions_ptr = secret_env_0_available_actions(self.env);
            let num_actions = self.num_actions() as usize;
            Vec::from_raw_parts(actions_ptr as *mut usize, num_actions, num_actions)
        }
    }

    pub fn num_rewards(&self) -> i32 {
        let secret_env_0_num_rewards: libloading::Symbol<unsafe extern fn() -> usize> =
            unsafe{LIB.get(b"secret_env_0_num_rewards").expect("Failed to load function `secret_env_0_num_rewards`")};
        unsafe {
            let num = secret_env_0_num_rewards();
            num as i32
        }
    }

    pub fn R(&self, pos: usize) -> f32 {
        let secret_env_0_reward: libloading::Symbol<unsafe extern fn(usize) -> f32> =
            unsafe{LIB.get(b"secret_env_0_reward").expect("Failed to load function `secret_env_0_reward`")};
        unsafe {
            let reward = secret_env_0_reward(pos);
            reward
        }
    }

    pub fn is_game_over(&self) -> bool {
        let secret_env_0_is_game_over: libloading::Symbol<unsafe extern fn(*const c_void) -> bool> =
            unsafe{LIB.get(b"secret_env_0_is_game_over").expect("Failed to load function `secret_env_0_is_game_over`")};
        unsafe {
            let result = secret_env_0_is_game_over(self.env);
            result
        }
    }

    pub fn display(&self) {
        let secret_env_0_display: libloading::Symbol<unsafe extern fn(*const c_void)> =
            unsafe{LIB.get(b"secret_env_0_display").expect("Failed to load function `secret_env_0_display`")};
        unsafe {
            secret_env_0_display(self.env)
        }
    }


    pub fn policy_iteration(&mut self,
                            theta: f32,
                            gamma: f32) -> Vec<usize> {

        let len_S: usize = self.num_states() as usize;
        let mut rng = rand::thread_rng();
        let mut V: Vec<f32> = Vec::with_capacity(len_S);

        for _ in 0..len_S {
            V.push(rng.gen_range(0f32..1f32));
        }

        let mut Pi= Vec::with_capacity(len_S);

        for _ in 0..len_S {
            let random_index = rng.gen_range(0..self.num_actions()) as usize;
            Pi.push(self.A()[random_index]); // mettre des valeurs aléatoires de A
        }

        //self.update_p();
        loop {
            // policy evaluation
            loop {
                let mut delta: f32 = 0.0;
                for s in 0..len_S {
                    // self.agent_pos = s as i32;
                    let mut v = V[s];
                    let mut total: f32 = 0f32;
                    for s_p in 0..len_S {
                        for r in 0..self.num_rewards() {
                            total = total + self.p(s,Pi[s],s_p,r as usize) * (self.R(r as usize) + gamma * V[s_p]);
                        }
                    }
                    V[s] = total;
                    delta = delta.max((v - V[s]).abs());
                }
                if delta < theta {
                    break;
                }
            }

            let mut policy_stable = true;

            for s in 0..self.num_states() {
                if self.is_game_over() {
                    continue;
                }

                let mut old_action = Pi[s as usize];

                let mut argmax_a: i32 = -9999999;
                let mut max_a: f32 = -9999999.0;

                for a in 0..self.num_actions() {
                    let mut total: f32 = 0.0;
                    // let mut p = 0.0f32;
                    // self.step(a);
                    for s_p in 0..self.num_states() {
                        for r_index in 0..self.num_rewards() {
                            total += self.p(s as usize,a as usize,s_p as usize, r_index as usize) * (self.R(r_index as usize) + gamma * V[s_p as usize])
                        }
                    }

                    if argmax_a == -9999999 || total >= max_a {
                        argmax_a = a as i32;
                        max_a = total;
                    }
                }

                Pi[s as usize] = argmax_a as usize;

                if old_action != Pi[s as usize] {
                    policy_stable = false;
                }
            }

            if policy_stable {
                break
            }
        }
        return Pi
    }

    pub fn value_iteration(&mut self,
                           theta: f32,
                           gamma: f32) -> Vec<i32> {
        // self.update_p();

        let len_S= self.num_states() as usize;
        let mut rng = rand::thread_rng();
        let mut V: Vec<f32> = Vec::with_capacity(len_S);

        // for i in 0..len_S {
        //     if self.T.contains(&(i as i32)) {
        //         V.push(0f32);
        //     } else {
        //         V.push(rng.gen_range(0f32..1f32));
        //     }
        // }
        for _ in 0..len_S {
            V.push(rng.gen_range(0f32..1f32));
        }

        loop {
            let mut delta = 0f32;
            for s in 0..len_S {
                if self.is_game_over() {
                    continue;
                }

                let v = V[s];
                let mut max_value: f32 = -9999f32;
                for a in 0..self.num_actions() {
                    let mut total: f32 = 0.0;
                    for s_p in 0..len_S {
                        for r in 0..self.num_rewards() {
                            total += self.p(s as usize,a as usize,s_p as usize,r as usize) * (self.R(r as usize) as f32 + gamma * V[s_p as usize]);
                        }
                    }
                    if total > max_value {
                        max_value = total;
                    }
                }

                V[s] = max_value;
                delta = delta.max((v - V[s]).abs());
            }
            if delta < theta {
                break;
            }
        }

        let mut Pi: Vec<i32> = vec![0; len_S];
        for s in 0..self.num_states() {
            if self.is_game_over() {
                continue;
            }

            let mut argmax_a: i32 = -1;
            let mut max_value: f32 = -99999f32;

            for a in 0..self.num_actions() {
                let mut total: f32 = 0.0;
                for s_p in 0..self.num_states() {
                    for r in 0..self.num_rewards() {
                        total += self.p(s as usize,a as usize,s_p as usize,r as usize) * (self.R(r as usize) as f32 + gamma * V[s_p as usize]);
                    }
                }

                if total > max_value {
                    max_value = total;
                    argmax_a = a as i32;
                }
            }

            Pi[s as usize] = argmax_a;
        }

        Pi
    }


}
