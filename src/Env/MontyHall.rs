use rand::Rng;
use std::collections::HashMap;

pub struct MontyHall {
    pub agent_pos: i32,
    pub num_states: i32,
    pub num_actions: i32,
    pub S: Vec<i32>,
    pub A: Vec<i32>,
    pub R: Vec<i32>,
    pub T: Vec<i32>,
    pub p: Vec<Vec<Vec<Vec<f32>>>>,
    pub deleted_door: i32
}

impl MontyHall {
    pub fn init() -> Self {
        Self {
            agent_pos: 0,
            num_states: 6,
            num_actions: 5,
            S: vec![0, 1, 2, 3, 4, 5], // état initial / on a choisi la porte A / B / C / même porte / on change
            A: vec![0, 1, 2, 3, 4], // porte A / B / C / on reste / on change
            R: vec![0, 1],
            T: vec![4, 5],
            p: {
                vec![
                    vec![
                        vec![vec![0f32; 2]; 6];
                        5
                    ];
                    6
                ]
            },
            deleted_door: 0
        }
    }

    pub fn update_p(&mut self) {
        // état initiaux
        self.p[0][0][1][0] = 1.0f32;
        self.p[0][1][2][0] = 1.0f32;
        self.p[0][2][3][0] = 1.0f32;

        // on reste
        // on perd
        self.p[1][3][4][0] = 2f32/3f32;
        self.p[2][3][4][0] = 2f32/3f32;
        self.p[3][3][4][0] = 2f32/3f32;

        // on gagne
        self.p[1][3][4][1] = 1f32/3f32;
        self.p[2][3][4][1] = 1f32/3f32;
        self.p[3][3][4][1] = 1f32/3f32;

        // on change
        // on gagne
        self.p[1][4][5][1] = 2f32/3f32;
        self.p[2][4][5][1] = 2f32/3f32;
        self.p[3][4][5][1] = 2f32/3f32;

        // on perd
        self.p[1][4][5][0] = 1f32/3f32;
        self.p[2][4][5][0] = 1f32/3f32;
        self.p[3][4][5][0] = 1f32/3f32;
    }

    pub fn from_random_state(&mut self) {
        let ok_states: Vec<i32> = vec![0,1,2,3];
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
        if self.agent_pos == 0 {
            vec![0, 1, 2]
        } else if self.agent_pos == 1 || self.agent_pos == 2 || self.agent_pos == 3 {
            vec![3, 4]
        } else {
            vec![]
        }
    }

    pub fn score(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let chances_stay = vec![0, 0, 1];
        let chances_move = vec![0, 1, 1];
        let mut result = 0;
        if self.agent_pos == 4 {
            result = chances_stay[rng.gen_range(0..3)];
            result
        } else if self.agent_pos == 5 {
            result = chances_move[rng.gen_range(0..3)];
            result
        } else {
            0
        }
    }

    pub fn step(&mut self, action: i32) {
        if self.A.contains(&action) && !self.is_game_over() {
            if self.agent_pos == 0 {
                match action {
                    0 => self.agent_pos = 1,
                    1 => self.agent_pos = 2,
                    2 => self.agent_pos = 3,
                    _ => println!("Action impossible")
                }
            } else {
                match action {
                    3 => self.agent_pos = 4,
                    _ => self.agent_pos = 5
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.agent_pos = 0;
        self.deleted_door = 0;
    }


    pub fn display(&mut self) {
        if self.agent_pos == 0 {
            println!("Bienvenue au Monty Hall ! Choisissez une des portes suivantes:");
            println!("|A| |B| |C|");
        } else if self.agent_pos == 1 {
            let mut rng = rand::thread_rng();
            self.deleted_door = rng.gen_range(0..2);
            if self.deleted_door == 0 {
                println!("Vous avez choisi la porte A");
                println!("|A| |x| |C|"); // delete door 0
            } else {
                println!("Vous avez choisi la porte A");
                println!("|A| |B| |x|"); // delete door 1
            }
        } else if self.agent_pos == 2 {
            let mut rng = rand::thread_rng();
            self.deleted_door = rng.gen_range(2..4);
            if self.deleted_door == 2 {
                println!("Vous avez choisi la porte B");
                println!("|x| |B| |C|"); // delete door 2
            } else {
                println!("Vous avez choisi la porte B");
                println!("|A| |B| |x|"); // delete door 3
            }
        } else if self.agent_pos == 3 {
            let mut rng = rand::thread_rng();
            self.deleted_door = rng.gen_range(4..6);
            if self.deleted_door == 4 {
                println!("Vous avez choisi la porte C");
                println!("|x| |B| |C|"); // delete door 4
            } else {
                println!("Vous avez choisi la porte C");
                println!("|A| |x| |C|"); // delete door 5
            }
        } else if self.agent_pos == 4 {
            if self.deleted_door == 0 {
                if self.score() == 0 {
                    println!("Vous n'avez pas changé de porte, et gardez donc la A");
                    println!("Dommage, c'est perdu !");
                    println!("|L| |x| |W|")
                } else {
                    println!("Vous n'avez pas changé de porte, et gardez donc la A");
                    println!("Quelle chance !");
                    println!("|W| |x| |L|")
                }
            } else if self.deleted_door == 1 {
                if self.score() == 0 {
                    println!("Vous n'avez pas changé de porte, et gardez donc la A");
                    println!("Dommage, c'est perdu !");
                    println!("|L| |W| |x|")
                } else {
                    println!("Vous n'avez pas changé de porte, et gardez donc la A");
                    println!("Quelle chance !");
                    println!("|W| |L| |x|")
                }
            } else if self.deleted_door == 3 {
                if self.score() == 0 {
                    println!("Vous n'avez pas changé de porte, et gardez donc la B");
                    println!("Dommage, c'est perdu !");
                    println!("|W| |L| |x|")
                } else {
                    println!("Vous n'avez pas changé de porte, et gardez donc la B");
                    println!("Quelle chance !");
                    println!("|L| |W| |x|")
                }
            } else if self.deleted_door == 2 {
                if self.score() == 0 {
                    println!("Vous n'avez pas changé de porte, et gardez donc la B");
                    println!("Dommage, c'est perdu !");
                    println!("|x| |L| |W|")
                } else {
                    println!("Vous n'avez pas changé de porte, et gardez donc la B");
                    println!("Quelle chance !");
                    println!("|x| |W| |L|")
                }
            } else if self.deleted_door == 4 {
                if self.score() == 0 {
                    println!("Vous n'avez pas changé de porte, et gardez donc la C");
                    println!("Dommage, c'est perdu !");
                    println!("|x| |W| |L|")
                } else {
                    println!("Vous n'avez pas changé de porte, et gardez donc la C");
                    println!("Quelle chance !");
                    println!("|x| |L| |W|")
                }
            } else {
                if self.score() == 0 {
                    println!("Vous n'avez pas changé de porte, et gardez donc la C");
                    println!("Dommage, c'est perdu !");
                    println!("|W| |x| |L|")
                } else {
                    println!("Vous n'avez pas changé de porte, et gardez donc la C");
                    println!("Quelle chance !");
                    println!("|L| |x| |W|")
                }
            }
        } else {
            if self.deleted_door == 0 {
                if self.score() == 0 {
                    println!("Vous avez changé de porte, et passez donc sur la C");
                    println!("Dommage, c'est perdu !");
                    println!("|W| |x| |L|")
                } else {
                    println!("Vous avez changé de porte, et passez donc sur la C");
                    println!("Quelle chance !");
                    println!("|L| |x| |W|")
                }
            } else if self.deleted_door == 1 {
                if self.score() == 0 {
                    println!("Vous avez changé de porte, et passez donc sur la B");
                    println!("Dommage, c'est perdu !");
                    println!("|W| |L| |x|")
                } else {
                    println!("Vous avez changé de porte, et passez donc sur la B");
                    println!("Quelle chance !");
                    println!("|L| |W| |x|")
                }
            } else if self.deleted_door == 2 {
                if self.score() == 0 {
                    println!("Vous avez changé de porte, et passez donc sur la C");
                    println!("Dommage, c'est perdu !");
                    println!("|x| |W| |L|")
                } else {
                    println!("Vous avez changé de porte, et passez donc sur la C");
                    println!("Quelle chance !");
                    println!("|x| |L| |W|")
                }
            } else if self.deleted_door == 3 {
                if self.score() == 0 {
                    println!("Vous avez changé de porte, et passez donc sur la A");
                    println!("Dommage, c'est perdu !");
                    println!("|L| |W| |x|")
                } else {
                    println!("Vous avez changé de porte, et passez donc sur la A");
                    println!("Quelle chance !");
                    println!("|W| |L| |x|")
                }
            } else if self.deleted_door == 4 {
                if self.score() == 0 {
                    println!("Vous avez changé de porte, et passez donc sur la B");
                    println!("Dommage, c'est perdu !");
                    println!("|x| |L| |W|")
                } else {
                    println!("Vous avez changé de porte, et passez donc sur la B");
                    println!("Quelle chance !");
                    println!("|x| |W| |L|")
                }
            } else {
                if self.score() == 0 {
                    println!("Vous avez changé de porte, et passez donc sur la A");
                    println!("Dommage, c'est perdu !");
                    println!("|L| |x| |W|")
                } else {
                    println!("Vous avez changé de porte, et passez donc sur la A");
                    println!("Quelle chance !");
                    println!("|W| |x| |L|")
                }
            }
        }
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
}