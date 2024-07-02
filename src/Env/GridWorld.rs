use rand::Rng;
use std::collections::HashMap;
use std::io::Write;
use colored::*;
use rand::distributions::{Distribution, Uniform};

pub struct GridWorld {
    pub agent_pos: i32,
    pub num_states: i32,
    pub num_actions: i32,
    pub S: Vec<i32>,
    pub A: Vec<i32>,
    pub R: Vec<i32>,
    pub T: Vec<i32>,
    pub p: Vec<Vec<Vec<Vec<f32>>>>
}


impl GridWorld {
    pub fn init() -> Self {
        Self {
            agent_pos: 8,
            num_states: 49,
            num_actions: 4,
            S: (0..49).collect(),
            A: vec![0, 1, 2, 3], // left right up down
            R: vec![-3, -1, 0, 1],
            T: vec![0, 1, 2, 3, 4, 5, 6, 7, 12, 13, 14, 20, 21, 27, 28, 34, 35, 40, 41, 42, 43, 44, 45, 46, 47, 48],
            p: {
                vec![
                    vec![
                        vec![vec![0f32; 4]; 49];
                        4
                    ];
                    49
                ]
            }
        }
    }

    fn generate_random_probabilities(&self) -> Vec<f32> {
        let mut rng = rand::thread_rng();
        let between = Uniform::from(0.0..1.0);
        let mut probabilities: Vec<f32> = (0..self.num_actions).map(|_| between.sample(&mut rng)).collect();
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
                        // actions terminales :
                        // si on monte depuis la premiere ligne :
                        if 7 < s && s < 12 && a == 2 && s_p == s - 7 && self.R[r] == -1 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on descend depuis la derniere ligne :
                        if 35 < s && s < 40 && a == 3 && s_p == s + 7 && self.R[r] == -1 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on va à gauche depuis la première colonne :
                        if s % 7 == 1 && 7 < s && s < 37 && a == 0 && s_p == s - 1 && self.R[r] == -1 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on va à droite depuis la dernière colonne :
                        if s % 7 == 5 && 7 < s && s < 37 && a == 1 && s_p == s + 1 && self.R[r] == -1 {
                            self.p[s][a][s_p][r] = 1f32;
                        }

                        // actions banales :
                        // si on est sur la premiere ligne:
                        // si on descend
                        if 7 < s && s < 12 && a == 3 && s_p == s + 7 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on va à gauche
                        if 8 < s && s < 12 && a == 0 && s_p == s - 1 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on va à droite
                        if 7 < s && s < 11 && a == 1 && s_p == s + 1 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }

                        // si on est sur la deuxième ligne:
                        // si on monte
                        if 14 < s && s < 19 && a == 2 && s_p == s - 7 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on descend
                        if 14 < s && s < 20 && a == 3 && s_p == s + 7 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on va à droite
                        if 14 < s && s < 19 && a == 1 && s_p == s + 1 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on va à gauche
                        if 15 < s && s < 20 && a == 0 && s_p == s - 1 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }

                        // si on est sur la troisième ligne:
                        // si on monte
                        if 21 < s && s < 27 && a == 2 && s_p == s - 7 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on descend
                        if 21 < s && s < 27 && a == 3 && s_p == s + 7 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on va à gauche
                        if 22 < s && s < 27 && a == 0 && s_p == s - 1 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on va à droite
                        if 21 < s && s < 26 && a == 1 && s_p == s + 1 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }

                        // si on est sur la quatrième ligne:
                        // si on monte
                        if 28 < s && s < 34 && a == 2 && s_p == s - 7 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on descend
                        if 28 < s && s < 33 && a == 3 && s_p == s + 7 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on va à gauche
                        if 29 < s && s < 34 && a == 0 && s_p == s - 1 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on va à droite
                        if 28 < s && s < 33 && a == 1 && s_p == s + 1 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }

                        // si on est sur la dernière ligne
                        // si on monte
                        if 35 < s && s < 40 && a == 2 && s_p == s - 7 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on va à gauche
                        if 36 < s && s < 40 && a == 0 && s_p == s - 1 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                        // si on va à droite
                        if 35 < s && s < 39 && a == 0 && s_p == s - 1 && self.R[r] == 0 {
                            self.p[s][a][s_p][r] = 1f32;
                        }
                    }
                }
            }
        }
        self.p[11][1][12][0] = 1f32;
        self.p[19][2][12][0] = 1f32;
        self.p[39][1][40][3] = 1f32;
        self.p[33][3][40][3] = 1f32;
    }


    pub fn from_random_state(&mut self) {
        let ok_states: Vec<i32> = vec![8, 9, 10, 11, 15, 16,
                                       17, 18, 19, 22, 23,
                                       24, 25, 26, 29, 30,
                                       31, 32, 33, 36, 37, 38, 39];
        let mut rng = rand::thread_rng();
        self.agent_pos = ok_states[rng.gen_range(0..ok_states.len())]
    }

    pub fn state_desc(&self) -> Vec<f32> {
        let mut one_hot = vec![0.0; self.num_states as usize];
        one_hot[self.agent_pos as usize] = 1.0;
        one_hot
    }

    pub fn is_game_over(&self) -> bool {
        if self.T.contains(&self.agent_pos){
            true
        } else {false}
    }

    pub fn available_actions(&self) -> Vec<i32> {
        match self.is_game_over(){
            true => vec![],
            false => vec![0, 1, 2, 3]
        }
    }

    pub fn score(&self) -> i32 {
        let minus_1: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7,
                                     13, 14, 20, 21, 27,
                                     28, 34, 35, 41, 42,
                                     43, 44, 45, 46, 47, 48];

        if minus_1.contains(&self.agent_pos) {
            -1
        } else if self.agent_pos == 12 {
            -3
        } else if self.agent_pos == 40 {
            1
        } else {
            0
        }
    }

    pub fn step(&mut self, action: i32) {
        if self.A.contains(&action) && !self.is_game_over() {
            match action {
                0 => self.agent_pos -= 1,
                1 => self.agent_pos += 1,
                2 => self.agent_pos -= 7,
                _ => self.agent_pos += 7
            }
        }
    }

    pub fn reset(&mut self) {
        self.agent_pos = 8;
    }


    pub fn display(&self) {
        let left_side: Vec<i32> = vec![7, 14, 21, 28, 35];
        let right_side: Vec<i32> = vec![13, 20, 27, 34, 41];
        let top_bottom: Vec<i32> = vec![1, 2, 3, 4, 5, 43, 44, 45, 45, 46, 47];
        for i in 0..49 {
            if i == self.agent_pos {
                print!("{}", " X ".red());
            } else if left_side.contains(&i){
                print!("{}", "| ".green());
            } else if right_side.contains(&i){
                print!("{}", " |\n".green());
            } else if top_bottom.contains(&i){
                print!("{}", "---".green());
            } else if i == 0 {
                print!("{}", "/ ".green());
            } else if  i == 48 {
                print!("{}", " /".green());
            } else if i == 6 {
                print!("{}", " \\\n".green());
            } else if i == 42 {
                print!("{}", "\\ ".green());
            } else {
                print!("{}", " 0 ".blue());
            }
        }
        println!();
    }

    pub fn run_game_vec(&mut self, Pi: Vec<i32>){
        println!("Etat initial :\n");
        self.display();
        println!("\n");
        let mut step: i32 = 1;
        while !self.is_game_over() {
            println!("Step {:?}: \n",step);
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

        let mut Pi= Vec::with_capacity(len_S);

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

        let len_S= self.num_states as usize;
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
        let len_S = self.num_states as usize;
        let len_A = self.A.len();
        let len_R = self.R.len();
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
                    let mut prob_per_action : HashMap<i32, f32> = HashMap::new();
                    for action in 0..random_Vec.len() {
                        prob_per_action.insert(action as i32, random_Vec[action]);
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
}