use rand::Rng;
use std::collections::HashMap;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;

pub struct LineWorld {
    pub agent_pos: i32,
    pub num_states: i32,
    pub num_actions: i32,
    pub S: Vec<i32>,
    pub A: Vec<i32>,
    pub R: Vec<i32>,
    pub T: Vec<i32>,
    pub p: Vec<Vec<Vec<Vec<f32>>>>
}

impl LineWorld {
    pub fn init() -> Self {
        Self {
            agent_pos: 2,
            num_states: 5,
            num_actions: 2,
            S: vec![0, 1, 2, 3, 4],
            A: vec![0, 1],
            R: vec![-1, 0, 1],
            T: vec![0, 4],
            p: {
                vec![
                    vec![
                        vec![vec![0f32; 3]; 5];
                        2
                    ];
                    5
                ]
            }
        }
    }

    fn generate_random_probabilities(&self) -> Vec<f32> {
        let mut rng = rand::thread_rng();
        let between = Uniform::from(0.0..1.0);
        let mut probabilities: Vec<f32> = (0..self.available_actions().len()).map(|_| between.sample(&mut rng)).collect();
        let sum: f32 = probabilities.iter().sum();

        for prob in probabilities.iter_mut() {
            *prob /= sum;
        }

        probabilities
    }


    fn select_action(&self, state: &HashMap<i32, f32>) -> i32 {
        let mut rng = rand::thread_rng();
        let random_value: f32 = rng.gen();
        let mut a_biggest_prob: i32 = 0;

        let mut cumulative_probability = 0.0;
        for (action, probability) in state {
            if state.get(&a_biggest_prob).unwrap() > probability {
                a_biggest_prob = *action;
            }
            cumulative_probability += probability;
            if random_value < cumulative_probability {
                return action.clone();
            }
        }
        a_biggest_prob
    }
    pub fn update_p(&mut self) {
        for s in 0..self.S.len() {
            for a in 0..self.A.len() {
                for s_p in 0..self.S.len() {
                    for r in 0..self.R.len() {
                        if s_p == (s + 1) && a == 1 && self.R[r] == 0 && [1, 2].contains(&self.S[s]) {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        if s > 0 && s_p == (s - 1) && a == 0 && self.R[r] == 0 && [2, 3].contains(&self.S[s]) {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                    }
                }
            }
        };

        self.p[3][1][4][2] = 1f32;
        self.p[1][0][0][0] = 1f32;
    }

    pub fn from_random_state(&mut self) {
        let mut rng = rand::thread_rng();
        self.agent_pos = rng.gen_range(1..4)
    }

    pub fn state_desc(&self) -> Vec<f32> {
        let mut one_hot = vec![0.0; self.num_states as usize];
        one_hot[self.agent_pos as usize] = 1.0;
        one_hot
    }

    pub fn is_game_over(&self) -> bool {
        if self.agent_pos == 0 || self.agent_pos == 4 {
            true
        } else { false }
    }

    pub fn available_actions(&self) -> Vec<i32> {
        match self.is_game_over() {
            true => vec![],
            false => vec![0, 1]
        }
    }

    pub fn score(&self) -> f32 {
        match self.agent_pos {
            0 => -1.0,
            4 => 1.0,
            _ => 0.0
        }
    }

    pub fn step(&mut self, action: i32) {
        if self.A.contains(&action) && !self.is_game_over() {
            match action {
                0 => self.agent_pos -= 1,
                _ => self.agent_pos += 1
            }
        }
    }

    pub fn reset(&mut self) {
        self.agent_pos = 2;
    }

    pub fn display(&self) {
        for i in 0..5 {
            if i == self.agent_pos {
                print!("X");
            } else {
                print!("_");
            }
        }
        println!();
    }

    pub fn run_game_vec(&mut self, Pi: Vec<i32>) {
        println!("Etat initial :\n");
        self.display();
        println!("\n");
        let mut step: i32 = 1;
        while !self.is_game_over() {
            println!("Step {:?}: \n", step);
            self.step(Pi[self.agent_pos as usize]);
            self.display();
            println!("\n");
            step += 1;
        }
    }

    pub fn run_game_hashmap(&mut self, Pi: HashMap<i32, i32>) {
        println!("Etat initial :\n");
        self.reset();
        self.display();
        println!("\n");
        let mut step: i32 = 1;
        while !self.is_game_over() && step <= 50 {
            println!("Step {:?}: \n", step);
            if let Some(&action) = Pi.get(&self.agent_pos) {
                println!("Action for position {}: {}", self.agent_pos, action);
                self.step(action);
            } else {
                println!("No action found for position {}. Ending game.", self.agent_pos);
                break;
            }
            self.display();
            println!("\n");
            step += 1;
        }
    }

    pub fn run_game_random_hashmap(&mut self, Pi: HashMap<i32, HashMap<i32, f32>>) {
        println!("Etat initial :\n");
        self.reset();
        self.display();
        println!("\n");
        let mut step: i32 = 1;
        while !self.is_game_over() && step <= 50 {
            println!("Step {:?}: \n", step);
            if let p = Pi.get(&self.agent_pos).unwrap() {
                let action = self.select_action(&p);
                println!("Action for position {}: {}", self.agent_pos, action);
                self.step(action);
            } else {
                println!("No action found for position {}. Ending game.", self.agent_pos);
                break;
            }
            self.display();
            println!("\n");
            step += 1;
        }
    }

    pub fn policy_iteration(&mut self,
                            theta: f32,
                            gamma: f32) -> Vec<i32> {
        let len_S: usize = self.num_states as usize;
        let mut rng = rand::thread_rng();
        let mut V: Vec<f32> = Vec::with_capacity(len_S);

        for _ in 0..len_S {
            V.push(rng.gen_range(0f32..1f32));
        }

        let mut Pi = Vec::with_capacity(len_S);

        for _ in 0..len_S {
            let random_index = rng.gen_range(0..self.num_actions) as usize;
            Pi.push(self.A[random_index]); // mettre des valeurs aléatoires de A
        }

        self.update_p();
        loop {
            // policy evaluation
            loop {
                let mut delta: f32 = 0.0;
                for s in 0..len_S {
                    // self.agent_pos = s as i32;
                    let mut v = V[s];
                    let mut total: f32 = 0f32;
                    for s_p in 0..len_S {
                        for r in 0..self.R.len() {
                            /*
                            let mut p = 0.0f32;
                            if !self.is_game_over() && (s_p == (s-1) || s_p == (s+1)){
                                        self.step(Pi[s]);
                                        if self.score() == r as f32 && self.agent_pos == s_p as i32 {
                                            p = 1.0;
                                }
                            }

                             */

                            total = total + self.p[s][Pi[s] as usize][s_p][r] * (self.R[r] as f32 + gamma * V[s_p]);
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

            for s in 0..self.num_states {
                if self.T.contains(&(s)) {
                    continue;
                }

                let mut old_action = Pi[s as usize];

                let mut argmax_a: i32 = -9999999;
                let mut max_a: f32 = -9999999.0;

                for a in 0..self.num_actions {
                    let mut total: f32 = 0.0;
                    // let mut p = 0.0f32;
                    // self.step(a);
                    for s_p in 0..self.num_states {
                        for r_index in 0..self.R.len() {
                            /*
                            if !self.is_game_over() && (s_p == (s-1) || s_p == (s+1)){
                                self.step(Pi[s as usize]);
                                if self.score() == r_index as f32 && self.agent_pos == s_p {
                                    p = 1.0;
                                }
                            }

                             */
                            total += self.p[s as usize][a as usize][s_p as usize][r_index] * (self.R[r_index] as f32 + gamma * V[s_p as usize])
                        }
                    }

                    if argmax_a == -9999999 || total >= max_a {
                        argmax_a = a as i32;
                        max_a = total;
                    }
                }

                Pi[s as usize] = argmax_a;

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
        self.update_p();

        let len_S = self.num_states as usize;
        let mut rng = rand::thread_rng();
        let mut V: Vec<f32> = Vec::with_capacity(len_S);

        for i in 0..len_S {
            if self.T.contains(&(i as i32)) {
                V.push(0f32);
            } else {
                V.push(rng.gen_range(0f32..1f32));
            }
        }

        loop {
            let mut delta = 0f32;
            for s in 0..len_S {
                if self.T.contains(&(s as i32)) {
                    continue;
                }

                let v = V[s];
                let mut max_value: f32 = -9999f32;
                for a in 0..self.A.len() {
                    let mut total: f32 = 0.0;
                    for s_p in 0..len_S {
                        for r in 0..self.R.len() {
                            total += self.p[s][a][s_p][r] * (self.R[r] as f32 + gamma * V[s_p]);
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
        for s in 0..self.S.len() {
            if self.T.contains(&(s as i32)) {
                continue;
            }

            let mut argmax_a: i32 = -1;
            let mut max_value: f32 = -99999f32;

            for a in 0..self.A.len() {
                let mut total: f32 = 0.0;
                for s_p in 0..self.S.len() {
                    for r in 0..self.R.len() {
                        total += self.p[s][a][s_p][r] * (self.R[r] as f32 + gamma * V[s_p]);
                    }
                }

                if total > max_value {
                    max_value = total;
                    argmax_a = a as i32;
                }
            }

            Pi[s] = argmax_a;
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
            self.from_random_state();

            let mut is_first_action: bool = true;
            let mut trajectory: Vec<(i32, i32, f32, Vec<i32>)> = Vec::new();
            let mut steps_count: i32 = 0;

            while steps_count < max_steps && !self.is_game_over() {
                let s = self.agent_pos;
                let aa = self.available_actions();

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

                if !is_in {
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
                let s = self.agent_pos;
                let aa = self.available_actions();

                if !Pi.contains_key(&s) {
                    let random_Vec = self.generate_random_probabilities();
                    let mut prob_per_action: HashMap<i32, f32> = HashMap::new();
                    for action in 0..random_Vec.len() {
                        prob_per_action.insert(action as i32, random_Vec[action]);
                    }
                    Pi.insert(s.clone(), prob_per_action);
                }

                let a = self.select_action(&Pi.get(&s).unwrap());

                let prev_score = self.score();
                self.step(a);
                let r = self.score() - prev_score;

                trajectory.push((s, a, r, aa));
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

                if !is_in {
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

                    let mut A: HashMap<i32, i32> = HashMap::new();
                    A.insert(*s, best_a.unwrap());

                    for (state, action) in &A {
                        if let Some(actions) = Pi.get_mut(state) {
                            for (a, p) in actions.iter_mut() {
                                if *a == *action {
                                    *p = 1.0 - epsilon + epsilon / self.num_actions as f32;
                                } else {
                                    *p = epsilon / self.num_actions as f32;
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

        for s in 0..self.num_states {
            for a in 0..self.num_actions {
                Q.insert((s, a), rng.gen_range(0f32..1f32));
                C.insert((s, a), 0.0f32);
            }
        }

        for s in 0..self.num_states {
            if !self.T.contains(&s){
                let mut best_a: Option<i32> = None;
                let mut best_a_score: Option<f32> = None;
                for a in 0..self.num_actions {
                    if best_a == None || Q.get(&(s, a)) > best_a_score.as_ref() {
                        best_a = Option::from(a);
                        best_a_score = Q.get(&(s, a)).cloned();
                    }
                }
                Pi.insert(s, best_a.unwrap());
            }
        }


        for _ in 0..nb_iter {
            self.reset();

            let mut trajectory: Vec<(i32, i32, f32, Vec<i32>)> = Vec::new();
            let mut steps_count: i32 = 0;
            let mut b: HashMap<i32, HashMap<i32, f32>> = HashMap::new();

            for s in 0..self.num_states {
                let random_Vec = self.generate_random_probabilities();
                let mut prob_per_action: HashMap<i32, f32> = HashMap::new();
                for a in 0..self.num_actions {
                    prob_per_action.insert(a as i32, random_Vec[a as usize]);
                }
                b.insert(s, prob_per_action);
            }

            for (state, action) in &Pi {
                if let Some(actions) = b.get_mut(state) {
                    for (a, p) in actions.iter_mut() {
                        if *a == *action {
                            *p = 1.0 - epsilon + epsilon / self.num_actions as f32;
                        } else {
                            *p = epsilon / self.num_actions as f32;
                        }
                    }
                }
            }

            while steps_count < max_steps && !self.is_game_over() {
                let s = self.agent_pos;
                let aa = self.available_actions();

                let a = self.select_action(&b.get(&s).unwrap());

                let prev_score = self.score();
                self.step(a);
                let r = self.score() - prev_score;

                trajectory.push((s, a, r, aa));
                steps_count += 1;
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

                let s = self.agent_pos;
                let aa = self.available_actions();

                for a in aa {
                    if !Q.contains_key(&(s, a)) {
                        Q.insert((s, a), rng.gen());
                    }
                }

                let random_value: f32 = rng.gen();
                let mut a ;
                if random_value < epsilon {
                    a = *self.available_actions().choose(&mut rng).unwrap()
                } else {
                    let mut best_a: Option<i32> = None;
                    let mut best_a_score: Option<f32> = None;

                    for a in self.available_actions() {
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

                let s_p = self.agent_pos;
                let aa_p = self.available_actions();
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
