use rand::Rng;
use std::collections::HashMap;
use rand::distributions::{Distribution, Uniform};
use std::ffi::c_void;
use crate::secret_env::lib_secret_env::LIB;
use rand::prelude::SliceRandom;


pub struct SecretEnv2 {
    pub env: *mut c_void,
}

impl SecretEnv2 {

    pub fn new() -> Self {
        let secret_env_2_new: libloading::Symbol<unsafe extern fn() -> *mut c_void> =
            unsafe{LIB.get(b"secret_env_2_new").expect("Failed to load function `secret_env_2_new`")};
        unsafe {
            let env = secret_env_2_new();
            SecretEnv2 { env }
        }
    }

    pub fn p(&self, s: usize, a: usize, s_p: usize, r: usize) -> f32 {
        let secret_env_2_transition_probability: libloading::Symbol<unsafe extern fn(usize, usize, usize, usize) -> f32> =
            unsafe {
                LIB.get(b"secret_env_2_transition_probability").expect("Failed to load function `secret_env_2_transition_probability`")
            };
        unsafe {
            secret_env_2_transition_probability(s, a, s_p, r)
        }
    }

    pub fn num_states(&self) -> i32 {
        let secret_env_2_num_states: libloading::Symbol<unsafe extern fn() -> usize> =
            unsafe {LIB.get(b"secret_env_2_num_states").expect("Failed to load function `secret_env_2_num_states`")};
        unsafe {
            secret_env_2_num_states() as i32
        }
    }

    pub fn agent_pos(&self) -> i32{
        let secret_env_2_state_id: libloading::Symbol<unsafe extern fn(*const c_void) -> usize> =
            unsafe{LIB.get(b"secret_env_2_state_id").expect("Failed to load function `secret_env_2_state_id`")};
        unsafe {
            secret_env_2_state_id(self.env) as i32
        }
    }

    pub fn num_actions(&self) -> i32 {
        let secret_env_2_num_actions: libloading::Symbol<unsafe extern fn() -> usize> =
            unsafe{LIB.get(b"secret_env_2_num_actions").expect("Failed to load function `secret_env_2_num_actions`")};
        unsafe {
            secret_env_2_num_actions() as i32
        }
    }

    pub fn A(&self) -> Vec<usize> {
        let secret_env_2_available_actions: libloading::Symbol<unsafe extern fn(*const c_void) -> *const usize> =
            unsafe { LIB.get(b"secret_env_2_available_actions").expect("Failed to load function `secret_env_2_available_actions`") };
        unsafe {
            let actions_ptr = secret_env_2_available_actions(self.env);
            let num_available_actions = self.available_actions_len() as usize;
            Vec::from_raw_parts(actions_ptr as *mut usize, num_available_actions, num_available_actions)
        }
    }

    pub fn available_actions_len(&self) -> i32 {
        let secret_env_2_available_actions_len: libloading::Symbol<unsafe extern fn(*const c_void) -> usize> =
            unsafe{LIB.get(b"secret_env_2_available_actions_len").expect("Failed to load function `secret_env_2_available_actions_len`")};
        unsafe {
            secret_env_2_available_actions_len(self.env) as i32
        }
    }

    pub fn is_forbidden(&self, action: i32) -> bool {
        let secret_env_0_is_forbidden: libloading::Symbol<unsafe extern fn(*const c_void, usize) -> bool> =
            unsafe{LIB.get(b"secret_env_0_is_forbidden").expect("Failed to load function `secret_env_0_is_forbidden`")};
        unsafe {
            secret_env_0_is_forbidden(self.env, action as usize)
        }
    }

    pub fn autorized_actions(&self) -> Vec<i32> {
        let mut actions : Vec<i32> = Vec::new();
        for action in self.A(){
            if !self.is_forbidden(action as i32){
                actions.push(action as i32);
            }
        }
        actions
    }

    pub fn num_rewards(&self) -> i32 {
        let secret_env_0_num_rewards: libloading::Symbol<unsafe extern fn() -> usize> =
            unsafe{LIB.get(b"secret_env_0_num_rewards").expect("Failed to load function `secret_env_0_num_rewards`")};
        unsafe {
            secret_env_0_num_rewards() as i32
        }
    }

    pub fn R(&self, pos: usize) -> f32 {
        let secret_env_0_reward: libloading::Symbol<unsafe extern fn(usize) -> f32> =
            unsafe{LIB.get(b"secret_env_0_reward").expect("Failed to load function `secret_env_0_reward`")};
        unsafe {
            secret_env_0_reward(pos)
        }
    }

    pub fn is_game_over(&self) -> bool {
        let secret_env_0_is_game_over: libloading::Symbol<unsafe extern fn(*const c_void) -> bool> =
            unsafe{LIB.get(b"secret_env_0_is_game_over").expect("Failed to load function `secret_env_0_is_game_over`")};
        unsafe {
            secret_env_0_is_game_over(self.env)
        }
    }

    pub fn display(&self) {
        let secret_env_0_display: libloading::Symbol<unsafe extern fn(*const c_void)> =
            unsafe{LIB.get(b"secret_env_0_display").expect("Failed to load function `secret_env_0_display`")};
        unsafe {
            secret_env_0_display(self.env)
        }
    }

    pub fn from_random_state(&mut self) -> *mut c_void {
        let secret_env_0_from_random_state: libloading::Symbol<unsafe extern fn() -> *mut c_void> =
            unsafe{LIB.get(b"secret_env_0_from_random_state").expect("Failed to load function `secret_env_0_from_random_state`")};
        unsafe {
            secret_env_0_from_random_state()
        }
    }

    pub fn score(&self) -> f32 {
        let secret_env_0_score: libloading::Symbol<unsafe extern fn(*const c_void) -> f32> =
            unsafe{LIB.get(b"secret_env_0_score").expect("Failed to load function `secret_env_0_score`")};
        unsafe {
            secret_env_0_score(self.env)
        }
    }

    pub fn step(&mut self, action: i32) {
        let secret_env_0_step: libloading::Symbol<unsafe extern fn(*mut c_void, usize)> =
            unsafe{LIB.get(b"secret_env_0_step").expect("Failed to load function `secret_env_0_step`")};
        unsafe{
            secret_env_0_step(self.env, action as usize);
        }
    }
    pub fn reset(&self) {
        let secret_env_0_reset: libloading::Symbol<unsafe extern fn(*const c_void) -> usize> =
            unsafe{LIB.get(b"secret_env_0_reset").expect("Failed to load function `secret_env_0_reset`")};
        unsafe {
            secret_env_0_reset(self.env);
        }
    }

    pub fn state_id(&self) -> usize{
        let secret_env_0_state_id: libloading::Symbol<unsafe extern fn(*const c_void) -> usize> =
            unsafe{LIB.get(b"secret_env_0_state_id").expect("Failed to load function `secret_env_0_state_id`")};
        unsafe {
            secret_env_0_state_id(self.env)
        }
    }

    fn generate_random_probabilities(&self) -> Vec<f32> {
        let mut rng = rand::thread_rng();
        let between = Uniform::from(0.0..1.0);
        let mut probabilities: Vec<f32> = (0..self.autorized_actions().len()).map(|_| between.sample(&mut rng)).collect();
        let sum: f32 = probabilities.iter().sum();

        for prob in probabilities.iter_mut() {
            *prob /= sum;
        }

        probabilities
    }

    fn select_action(&self, state: &HashMap<i32, f32>) -> i32 {
        let mut rng = rand::thread_rng();
        let random_value: f32 = rng.gen();
        let mut a_biggest_prob = state.keys().next().unwrap();

        let mut cumulative_probability = 0.0;
        for (action, probability) in state {
            if state.get(&a_biggest_prob).unwrap() > probability {
                a_biggest_prob = action;
            }
            cumulative_probability += probability;
            if random_value < cumulative_probability {
                return action.clone();
            }
        }
        *a_biggest_prob
    }

    pub fn run_game_hashmap(&mut self, Pi: HashMap<i32, i32>) {
        self.reset();
        while !self.is_game_over(){
            let pos :i32 = self.agent_pos();
            let action = match Pi.get(&pos) {
                Some(&action) => action,
                None => {
                    continue;
                }
            };
            self.step(action);
            self.display();
        }
    }

    pub fn run_game_random_hashmap(&mut self, Pi: HashMap<i32, HashMap<i32, f32>>) {
        self.reset();
        while !self.is_game_over() {
            let pos: i32 = self.agent_pos();

            if let p = Pi.get(&pos).unwrap() {
                let action = self.select_action(&p);
                self.step(action);
                self.display();
            } else {
                continue;
            }
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


    pub fn monte_carlo_exploring_starts(&mut self,
                                        gamma: f32,
                                        nb_iter: i32,
                                        max_steps: i32) -> HashMap<i32, i32> {

        let mut rng = rand::thread_rng();

        let mut Pi = HashMap::new();
        let mut Q: HashMap<(i32, i32), f32> = HashMap::new();
        let mut returns: HashMap<(i32, i32), Vec<f32>> = HashMap::new();

        for _ in 0..nb_iter {
            self.env = self.from_random_state();

            let mut is_first_action: bool = true;
            let mut trajectory: Vec<(i32, i32, f32, Vec<i32>)> = Vec::new();
            let mut steps_count: i32 = 0;

            while steps_count < max_steps && !self.is_game_over() {
                let s = self.agent_pos();
                let aa:Vec<i32> = self.autorized_actions();

                if aa.is_empty() {
                    println!("No available actions from state {}", s);
                    break;
                }

                if !Pi.contains_key(&s) {
                    let random_index = rng.gen_range(0..aa.len());
                    Pi.insert(s.clone(), aa[random_index]);
                }

                let a = if is_first_action {
                    let random_index = rng.gen_range(0..aa.len());
                    is_first_action = false;
                    aa[random_index]
                } else {
                    Pi[&s]
                };

                let prev_score = self.score();
                self.step(a);
                let r = self.score() - prev_score;

                trajectory.push((s, a, r, aa));
                steps_count += 1;
            }

            if trajectory.is_empty() {
                // println!("Trajectory is empty after max_steps: {}", max_steps);
                // println!("Is game over: {}", self.is_game_over());
                continue;
            }

            let mut G = 0.0;
            let mut t = trajectory.len() - 1;

            for ((s, a, r, aa)) in trajectory.iter().rev() {
                G = gamma * G + r;


                let mut is_in = false;
                if t > 1 {
                    for (s_t, a_t, c_t, d_t) in Vec::from(&trajectory[..t]) {
                        if s_t == *s && a_t == *a {
                            is_in = true;
                            break;
                        }
                    }
                    t -= 1;
                }

                if !is_in{
                    let entry = returns.entry((*s, *a)).or_insert(Vec::new());
                    entry.push(G);

                    let sum: f32 = entry.iter().sum();
                    let mean = sum / entry.len() as f32;

                    Q.insert((*s, *a), mean);

                    let mut best_a: Option<i32> = None;
                    let mut best_a_score: Option<f32> = None;

                    for &a in aa {
                        if !Q.contains_key(&(*s, a)) {
                            Q.insert((*s, a), rng.gen());
                        }
                        if best_a == None || Q.get(&(*s, a)) > best_a_score.as_ref() {
                            best_a = Option::from(a);
                            best_a_score = Q.get(&(*s, a)).cloned();
                        }
                    }

                    Pi.insert(*s, best_a.unwrap());
                }
            }
        }
        Pi
    }

    pub fn monte_carlo_fv_on_policy(&mut self,
                                    gamma: f32,
                                    epsilon: f32,
                                    nb_iter: i32,
                                    max_steps: i32) -> HashMap<i32, HashMap<i32, f32>> {

        let mut rng = rand::thread_rng();

        let mut Pi: HashMap<i32, HashMap<i32, f32>> = Default::default();

        let mut Q: HashMap<(i32, i32), f32> = HashMap::new();
        let mut returns: HashMap<(i32, i32), Vec<f32>> = HashMap::new();

        for _ in 0..nb_iter {
            self.reset();

            let mut trajectory: Vec<(i32, i32, f32, Vec<i32>)> = Vec::new();
            let mut steps_count: i32 = 0;

            while steps_count < max_steps && !self.is_game_over() {
                let s = self.agent_pos();
                let aa = self.autorized_actions();

                if !Pi.contains_key(&s) {
                    let random_Vec = self.generate_random_probabilities();
                    let mut prob_per_action : HashMap<i32, f32> = HashMap::new();
                    for (index, action) in aa.iter().enumerate() {
                        prob_per_action.insert(*action, random_Vec[index]);
                    }
                    Pi.insert(s.clone(), prob_per_action);
                }

                let a = self.select_action(&Pi.get(&s).unwrap());

                let prev_score = self.score();
                self.step(a);
                let r = self.score() - prev_score;

                trajectory.push((s, a, r as f32, aa));
                steps_count += 1;
            }

            let mut G = 0.0;
            let mut t = trajectory.len() - 1;

            for ((s, a, r, aa)) in trajectory.iter().rev() {
                G = gamma * G + r;


                let mut is_in = false;
                if t > 1 {
                    for (s_t, a_t, c_t, d_t) in Vec::from(&trajectory[..t]) {
                        if s_t == *s && a_t == *a {
                            is_in = true;
                            break;
                        }
                    }
                    t -= 1;
                }

                if !is_in{
                    let entry = returns.entry((*s, *a)).or_insert(Vec::new());
                    entry.push(G);

                    let sum: f32 = entry.iter().sum();
                    let mean = sum / entry.len() as f32;

                    Q.insert((*s, *a), mean);

                    let mut best_a: Option<i32> = None;
                    let mut best_a_score: Option<f32> = None;

                    for &a in aa {
                        if !Q.contains_key(&(*s, a)) {
                            Q.insert((*s, a), rng.gen());
                        }
                        if best_a == None || Q.get(&(*s, a)) > best_a_score.as_ref() {
                            best_a = Option::from(a);
                            best_a_score = Q.get(&(*s, a)).cloned();
                        }
                    }

                    let mut A:  HashMap<i32, i32> = HashMap::new();
                    A.insert(*s, best_a.unwrap());

                    for (state, action) in &A {
                        if let Some(actions) = Pi.get_mut(state) {
                            for (a, p) in actions.iter_mut() {
                                if *a == *action {
                                    *p = 1.0 - epsilon + epsilon / self.num_actions() as f32;
                                } else {
                                    *p = epsilon / self.num_actions() as f32;
                                }
                            }
                        }
                    }
                }
            }
        }
        Pi
    }

    pub fn monte_carlo_off_policy(&mut self,
                                  gamma: f32,
                                  epsilon: f32,
                                  nb_iter: i32,
                                  max_steps: i32) -> HashMap<i32, i32> {

        let mut rng = rand::thread_rng();

        let mut Q: HashMap<(i32, i32), f32> = HashMap::new();
        let mut C: HashMap<(i32, i32), f32> = HashMap::new();
        let mut Pi: HashMap<i32, i32> = HashMap::new();

        for _ in 0..nb_iter {
            self.reset();

            let mut trajectory: Vec<(i32, i32, f32, Vec<i32>)> = Vec::new();
            let mut steps_count: i32 = 0;
            let mut b: HashMap<i32, HashMap<i32, f32>> = HashMap::new();


            while steps_count < max_steps && !self.is_game_over() {
                let s = self.agent_pos();
                let aa = self.autorized_actions();

                if !b.contains_key(&s) {
                    let random_Vec = self.generate_random_probabilities();
                    let mut prob_per_action: HashMap<i32, f32> = HashMap::new();
                    for (index, action) in aa.iter().enumerate() {
                        prob_per_action.insert(*action, random_Vec[index]);
                    }
                    b.insert(s.clone(), prob_per_action);

                    if Pi.contains_key(&s) {
                        let action = Pi.get(&s).unwrap();
                        if let Some(actions) = b.get_mut(&s) {
                            for (a, p) in actions.iter_mut() {
                                if *a == *action {
                                    *p = 1.0 - epsilon + epsilon / self.num_actions() as f32;
                                } else {
                                    *p = epsilon / self.num_actions() as f32;
                                }
                            }
                        }
                    }
                }

                let a = self.select_action(&b.get(&s).unwrap());

                let prev_score = self.score();

                self.step(a);
                let r = self.score() - prev_score;

                trajectory.push((s, a, r, aa));
                steps_count += 1;

                if !C.contains_key(&(s, a)) {
                    Q.insert((s, a), rng.gen_range(0f32..1f32));
                    C.insert((s, a), 0.0f32);
                }

                if !Pi.contains_key(&s) {
                    let mut best_a: Option<i32> = None;
                    let mut best_a_score: Option<f32> = None;
                    for a in self.autorized_actions() {
                        if best_a == None || Q.get(&(s, a)) > best_a_score.as_ref() {
                            best_a = Option::from(a);
                            best_a_score = Q.get(&(s, a)).cloned();
                        }
                    }
                    Pi.insert(s, best_a.unwrap());
                }
            }

            let mut W = 1.0f32;
            let mut G = 0.0;

            for ((s, a, r, aa)) in trajectory.iter().rev() {
                G = gamma * G + r;

                C.insert((*s, *a), C.get(&(*s, *a)).unwrap() +  W);

                Q.insert((*s, *a),
                         Q.get(&(*s, *a)).unwrap()
                             + W/C.get(&(*s, *a)).unwrap()
                             * (G - Q.get(&(*s, *a)).unwrap())
                );

                let mut best_a: Option<i32> = None;
                let mut best_a_score: Option<f32> = None;

                for &a in aa {
                    if !Q.contains_key(&(*s, a)) {
                        Q.insert((*s, a), rng.gen());
                    }
                    if best_a == None || Q.get(&(*s, a)) > best_a_score.as_ref() {
                        best_a = Option::from(a);
                        best_a_score = Q.get(&(*s, a)).cloned();
                    }
                }

                Pi.insert(*s, best_a.unwrap());

                if a != Pi.get(&s).unwrap() {continue;}
                else {
                    if let Some(actions) = b.get_mut(s){
                        W = W * 1.0f32/actions.get(a).unwrap();
                    }
                }
            }
        }
        Pi
    }

    pub fn Q_learning_off_policy(&mut self,
                                 gamma: f32,
                                 epsilon: f32,
                                 alpha: f32,
                                 nb_iter: i32,
                                 max_steps: i32) -> HashMap<i32, i32> {
        let mut rng = rand::thread_rng();

        let mut Q: HashMap<(i32, i32), f32> = HashMap::new();
        let mut Pi: HashMap<i32, i32> = HashMap::new();

        for _ in 0..nb_iter {
            self.reset();
            let mut steps_count: i32 = 0;

            while steps_count < max_steps && !self.is_game_over() {

                let s = self.agent_pos();
                let aa = self.autorized_actions();

                for a in aa {
                    if !Q.contains_key(&(s, a)) {
                        Q.insert((s, a), rng.gen());
                    }
                }

                let random_value: f32 = rng.gen();
                let mut a ;
                if random_value < epsilon {
                    a = *self.autorized_actions().choose(&mut rng).unwrap()
                } else {
                    let mut best_a: Option<i32> = None;
                    let mut best_a_score: Option<f32> = None;

                    for a in self.autorized_actions() {
                        if best_a == None || Q.get(&(s, a)) > best_a_score.as_ref() {
                            best_a = Option::from(a);
                            best_a_score = Q.get(&(s, a)).cloned();
                        }
                    }
                    a = best_a.unwrap();
                }

                let prev_score = self.score();
                self.step(a);
                let r = self.score() - prev_score;

                let s_p = self.agent_pos();
                let aa_p = self.autorized_actions();
                let target: f32;

                if self.is_game_over(){
                    target = r;
                } else {
                    let mut best_a_p: Option<i32> = None;
                    let mut best_a_score_p: Option<f32> = None;
                    for a_p in aa_p {
                        if !Q.contains_key(&(s_p, a_p)) {
                            Q.insert((s_p, a_p), rng.gen());
                        }
                        if best_a_p == None || Q.get(&(s_p, a_p)) > best_a_score_p.as_ref() {
                            best_a_p = Option::from(a_p);
                            best_a_score_p = Q.get(&(s_p, a_p)).cloned();
                        }
                    }
                    target = r + gamma * best_a_score_p.unwrap();
                }

                let updated_gain = (1.00 - alpha) * Q.get(&(s, a)).unwrap() + alpha * target;
                Q.insert((s, a), updated_gain);

                steps_count += 1;
            }
        }
        let mut All_States_Actions: HashMap<i32, Vec<i32>> = HashMap::new();

        for (s, a) in Q.keys(){
            if !All_States_Actions.contains_key(s) {
                All_States_Actions.entry(*s).or_insert_with(Vec::new).push(*a);
            }
            if All_States_Actions.contains_key(s){
                let myVec = All_States_Actions.get(&s).unwrap();
                if !myVec.contains(a){
                    All_States_Actions.entry(*s).or_insert_with(Vec::new).push(*a);
                }
            }
        }

        for (s, a_Vec) in All_States_Actions.iter() {

            let mut best_a: Option<i32> = None;
            let mut best_a_score: Option<f32> = None;
            for action in a_Vec {
                if best_a == None || Q.get(&(*s, *action)) > best_a_score.as_ref() {
                    best_a = Option::from(*action);
                    best_a_score = Q.get(&(*s, *action)).cloned();
                }
            }

            Pi.insert(*s, best_a.unwrap());
        }
        Pi
    }
}
