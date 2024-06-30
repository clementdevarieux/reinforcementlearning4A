use rand::Rng;
use std::collections::HashMap;
use std::io::Write;
use colored::*;

pub struct Shifumi {
    pub agent_pos: i32,
    pub num_states: i32,
    pub num_actions: i32,
    pub S: Vec<i32>,
    pub A: Vec<i32>,
    pub R: Vec<i32>,
    pub T: Vec<i32>,
    pub p: Vec<Vec<Vec<Vec<f32>>>>
}

impl Shifumi {
    pub fn init() -> Self {
        Self {
            agent_pos: 0,
            num_states: 37,
            num_actions: 4,
            S: (0..37).collect(),
            A: vec![0, 1, 2], // P F C
            R: vec![-1, 0, 1],
            T: (10..37).collect(),
            p: {
                vec![
                    vec![
                        vec![vec![0f32; 3]; 37];
                        3
                    ];
                    37
                ]
            }
        }
    }

    pub fn update_p(&mut self) {
        self.pp[0][0][1][1] = 1f32/3f32; //# PP
        self.pp[0][0][2][0] = 1f32/3f32; //# PF
        self.pp[0][0][3][2] = 1f32/3f32; //# PC

        self.pp[0][1][4][2] = 1f32/3f32; //# FP
        self.pp[0][1][5][1] = 1f32/3f32; // # FF
        self.pp[0][1][6][0] = 1f32/3f32; // # FC

        self.pp[0][2][7][0] = 1f32/3f32; // # CP
        self.pp[0][2][8][2] = 1f32/3f32; // # CF
        self.pp[0][2][9][1] = 1f32/3f32; // # CC
        // ###############
        self.pp[1][0][10][1] = 1f32; // # PP PP
        self.pp[1][1][11][2] = 1f32; // # PP FP
        self.pp[1][2][12][0] = 1f32; // # PP CP

        self.pp[2][0][13][1] = 1f32; // # PF PP
        self.pp[2][1][14][2] = 1f32; // # PF FP
        self.pp[2][2][15][0] = 1f32; // # PF CP

        self.pp[3][0][16][1] = 1f32; // # PC PP
        self.pp[3][1][17][2] = 1f32; // # PC FP
        self.pp[3][2][18][0] = 1f32; // # PC CP
        //###############
        self.pp[4][0][19][0] = 1f32; // # FP PF
        self.pp[4][1][20][1] = 1f32; // FP FF
        self.pp[4][2][21][2] = 1f32; //# FP CF

        self.pp[5][0][22][0] = 1f32; // FF PF
        self.pp[5][1][23][1] = 1f32; // FF FF
        self.pp[5][2][24][2] = 1f32; // FF CF

        self.pp[6][0][25][0] = 1f32; // FC PF
        self.pp[6][1][26][1] = 1f32; // FC FF
        self.pp[6][2][27][2] = 1f32; // FC CF
        //###############
        self.pp[7][0][28][2] = 1f32; // CP PC
        self.pp[7][1][29][0] = 1f32; // CP FC
        self.pp[7][2][30][1] = 1f32; // CP CC

        self.pp[8][0][31][2] = 1f32; // CF PC
        self.pp[8][1][32][0] = 1f32; // CF FC
        self.pp[8][2][33][1] = 1f32; // CF CC

        self.pp[9][0][34][2] = 1f32; // CC PC
        self.pp[9][1][35][0] = 1f32; // CC FC
        self.pp[9][2][36][1] = 1f32; // CC CC
    }

    pub fn from_random_state(&mut self) {
        let ok_states: Vec<i32> = vec![0,1,2,3,4,5,6,7,8,9];
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
            false => vec![0, 1, 2]
        }
    }

    pub fn score(&self) -> i32 {
        let minus_1: Vec<i32> = vec![2, 6, 7, 12, 15, 18, 19, 22, 25, 29, 32, 35];
        let plus_1: Vec<i32> = vec![3, 4, 8, 11, 14, 17, 21, 24, 27, 28, 31, 34];

        if minus_1.contains(&self.agent_pos) {
            -1
        } else if plus_1.contains(&self.agent_pos) {
            1
        } else {
            0
        }
    }

    pub fn step(&mut self, action: i32) {
        let mut rng = rand::thread_rng();
        if self.A.contains(&action) && !self.is_game_over() {
            if self.agent_pos == 0 {
                let random_index = rng.gen_range(0..3); // 0 1 2
                match action {
                    0 => self.agent_pos = random_index + 1, // 1 2 3
                    1 => self.agent_pos = random_index + 4, // 4 5 6
                    _ => self.agent_pos = random_index + 7  // 7 8 9
                }
            } else if self.agent_pos == 1 {
                match action {
                    0 => self.agent_pos = 10, // 1 2 3
                    1 => self.agent_pos = 11, // 4 5 6
                    _ => self.agent_pos = 12  // 7 8 9
                }
            } else if self.agent_pos == 2 {
                match action {
                    0 => self.agent_pos = 13, // 1 2 3
                    1 => self.agent_pos = 14, // 4 5 6
                    _ => self.agent_pos = 15  // 7 8 9
                }
            } else if self.agent_pos == 3 {
                match action {
                    0 => self.agent_pos = 16, // 1 2 3
                    1 => self.agent_pos = 17, // 4 5 6
                    _ => self.agent_pos = 18  // 7 8 9
                }
            } else if self.agent_pos == 4 {
                match action {
                    0 => self.agent_pos = 19, // 1 2 3
                    1 => self.agent_pos = 20, // 4 5 6
                    _ => self.agent_pos = 21  // 7 8 9
                }
            } else if self.agent_pos == 5 {
                match action {
                    0 => self.agent_pos = 22, // 1 2 3
                    1 => self.agent_pos = 23, // 4 5 6
                    _ => self.agent_pos = 24  // 7 8 9
                }
            } else if self.agent_pos == 6 {
                match action {
                    0 => self.agent_pos = 25, // 1 2 3
                    1 => self.agent_pos = 26, // 4 5 6
                    _ => self.agent_pos = 27  // 7 8 9
                }
            } else if self.agent_pos == 7 {
                match action {
                    0 => self.agent_pos = 28, // 1 2 3
                    1 => self.agent_pos = 29, // 4 5 6
                    _ => self.agent_pos = 30  // 7 8 9
                }
            } else if self.agent_pos == 8 {
                match action {
                    0 => self.agent_pos = 31, // 1 2 3
                    1 => self.agent_pos = 32, // 4 5 6
                    _ => self.agent_pos = 33  // 7 8 9
                }
            } else if self.agent_pos == 9 {
                match action {
                    0 => self.agent_pos = 34, // 1 2 3
                    1 => self.agent_pos = 35, // 4 5 6
                    _ => self.agent_pos = 36  // 7 8 9
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.agent_pos = 0;
    }

    pub fn display(&self) {
        if self.agent_pos == 0 {
            println!("Début du jeu, choisissez Pierre, Feuille ou Ciseaux !");
        } else if self.agent_pos == 1 {
            println!("Round 1 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : P // Choix de l'adversaire : P");
            println!("Score : 0");
            println!("Moyen tout ça..");
        } else if self.agent_pos == 2 {
            println!("Round 1 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : P // Choix de l'adversaire : F");
            println!("Score : -1");
            println!("Pas de chance...");
        } else if self.agent_pos == 3 {
            println!("Round 1 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : P // Choix de l'adversaire : C");
            println!("Score : 1");
            println!("Bien joué !");
        } else if self.agent_pos == 4 {
            println!("Round 1 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : F // Choix de l'adversaire : P");
            println!("Score : 1");
            println!("Bien joué !");
        } else if self.agent_pos == 5 {
            println!("Round 1 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : F // Choix de l'adversaire : F");
            println!("Score : 0");
            println!("Moyen tout ça..");
        } else if self.agent_pos == 6 {
            println!("Round 1 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : F // Choix de l'adversaire : C");
            println!("Score : -1");
            println!("Pas de chance...");
        } else if self.agent_pos == 7 {
            println!("Round 1 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : C // Choix de l'adversaire : P");
            println!("Score : -1");
            println!("Pas de chance...");
        } else if self.agent_pos == 8 {
            println!("Round 1 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : C // Choix de l'adversaire : F");
            println!("Score : 1");
            println!("Bien joué !");
        } else if self.agent_pos == 9 {
            println!("Round 1 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : C // Choix de l'adversaire : C");
            println!("Score : 0");
            println!("Moyen tout ça..");
        } else if self.agent_pos == 10 {
            println!("Round 1 : P P");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : P // Choix de l'adversaire : P");
            println!("Score Final: 0");
            println!("Moyen tout ça..");
        } else if self.agent_pos == 11 {
            println!("Round 1 : P P");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : F // Choix de l'adversaire : P");
            println!("Score Final: 1");
            println!("Bien joué !");
        } else if self.agent_pos == 12 {
            println!("Round 1 : P P");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : C // Choix de l'adversaire : P");
            println!("Score Final: -1");
            println!("Mauvais choix...");
        } else if self.agent_pos == 13 {
            println!("Round 1 : P F");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : P // Choix de l'adversaire : P");
            println!("Score Final: -1");
            println!("Mauvais choix...");
        }  else if self.agent_pos == 14 {
            println!("Round 1 : P F");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : F // Choix de l'adversaire : P");
            println!("Score Final: 0");
            println!("Pas trop mal");
        } else if self.agent_pos == 15 {
            println!("Round 1 : P F");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : C // Choix de l'adversaire : P");
            println!("Score Final: -2");
            println!("Wasted !");
        } else if self.agent_pos == 16 {
            println!("Round 1 : P C");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : P // Choix de l'adversaire : P");
            println!("Score Final: 1");
            println!("Pas trop mal");
        } else if self.agent_pos == 17 {
            println!("Round 1 : P C");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : F // Choix de l'adversaire : P");
            println!("Score Final: 2");
            println!("Incroyable !");
        } else if self.agent_pos == 18 {
            println!("Round 1 : P C");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : C // Choix de l'adversaire : P");
            println!("Score Final: 0");
            println!("Mauvais choix !");
        } else if self.agent_pos == 19 {
            println!("Round 1 : F P");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : P // Choix de l'adversaire : F");
            println!("Score Final: 0");
            println!("Mauvais choix !");
        } else if self.agent_pos == 20 {
            println!("Round 1 : F P");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : F // Choix de l'adversaire : F");
            println!("Score Final: 1");
            println!("Okayokay");
        } else if self.agent_pos == 21 {
            println!("Round 1 : F P");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : C // Choix de l'adversaire : F");
            println!("Score Final: 2");
            println!("Incrédiblé!");
        } else if self.agent_pos == 22 {
            println!("Round 1 : F F");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : P // Choix de l'adversaire : F");
            println!("Score Final: -1");
            println!("Mauvais choix!");
        } else if self.agent_pos == 23 {
            println!("Round 1 : F F");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : F // Choix de l'adversaire : F");
            println!("Score Final: 0");
            println!("Moyen tout ça");
        } else if self.agent_pos == 24 {
            println!("Round 1 : F F");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : C // Choix de l'adversaire : F");
            println!("Score Final: 1");
            println!("Bien vu !");
        } else if self.agent_pos == 25 {
            println!("Round 1 : F C");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : P // Choix de l'adversaire : F");
            println!("Score Final: -2");
            println!("NUL");
        } else if self.agent_pos == 26 {
            println!("Round 1 : F C");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : F // Choix de l'adversaire : F");
            println!("Score Final: -1");
            println!("Dommage");
        } else if self.agent_pos == 27 {
            println!("Round 1 : F C");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : C // Choix de l'adversaire : F");
            println!("Score Final: 0");
            println!("Bien rattrapé");
        } else if self.agent_pos == 28 {
            println!("Round 1 : C P");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : P // Choix de l'adversaire : C");
            println!("Score Final: 0");
            println!("Bien rattrapé");
        } else if self.agent_pos == 29 {
            println!("Round 1 : C P");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : F // Choix de l'adversaire : C");
            println!("Score Final: -2");
            println!("Aie aie aie");
        } else if self.agent_pos == 30 {
            println!("Round 1 : C P");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : C // Choix de l'adversaire : C");
            println!("Score Final: -1");
            println!("Aie aie aie");
        } else if self.agent_pos == 31 {
            println!("Round 1 : C F");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : P // Choix de l'adversaire : C");
            println!("Score Final: 2");
            println!("Champion !");
        } else if self.agent_pos == 32 {
            println!("Round 1 : C F");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : F // Choix de l'adversaire : C");
            println!("Score Final: 0");
            println!("Oh non !");
        } else if self.agent_pos == 33 {
            println!("Round 1 : C F");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : C // Choix de l'adversaire : C");
            println!("Score Final: 1");
            println!("Pas mal");
        } else if self.agent_pos == 34 {
            println!("Round 1 : C C");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : P // Choix de l'adversaire : C");
            println!("Score Final: 1");
            println!("Pas mal");
        } else if self.agent_pos == 35 {
            println!("Round 1 : C C");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : F // Choix de l'adversaire : C");
            println!("Score Final: -1");
            println!("Mauvais choix");
        } else {
            println!("Round 1 : C C");
            println!("Round 2 :\nSHI...\nFU...\nMI!!!\n");
            println!("Votre choix : C // Choix de l'adversaire : C");
            println!("Score Final: 0");
            println!("Moyen tout ça");
        }
    }

    pub fn run_game_vec(&mut self, Pi: Vec<i32>){
        self.display();
        println!("\n");
        while !self.is_game_over() {
            self.step(Pi[self.agent_pos as usize]);
            self.display();
            println!("\n");
        }
    }

    pub fn run_game_hashmap(&mut self, Pi: HashMap<i32, i32>) {
        self.reset();
        self.display();
        println!("\n");
        let mut step: i32 = 1;
        while !self.is_game_over() && step <= 50 {
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

}