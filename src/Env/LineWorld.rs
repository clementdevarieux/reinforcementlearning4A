pub struct LineWorld{
    pub agent_pos: i32,
    pub num_states: i32,
    pub num_actions: i32,
    pub S: Vec<i32>,
    pub A: Vec<i32>,
    pub R: Vec<i32>,
    pub T: Vec<i32>,
    // pub p: Vec<Vec<Vec<Vec<f32>>>>

}

/// TODO: Modifier les "if" par un "match"


impl LineWorld {
    pub fn init() -> Self{
        Self {
            agent_pos: 2,
            num_states: 5,
            num_actions: 2,
            S: vec![0, 1, 2, 3, 4],
            A: vec![0,1],
            R: vec![-1,0,1],
            T: vec![0,4],
            // p: vec![
            //     vec![
            //         vec![vec![0f32; Self.R.len()]; Self.S.len()];
            //         Self.A.len()
            //     ];
            //     Self.S.len()
            // ];
        }
    }

    pub fn state_desc(&self) -> Vec<f32> {
        let mut one_hot = vec![0.0; self.num_states as usize];
        one_hot[self.agent_pos as usize] = 1.0;
        one_hot
    }

    pub fn is_game_over(&self) -> bool {
        if self.agent_pos == 0 || self.agent_pos == 4 {
            true
        } else {false}
    }

    pub fn available_actions(&self) -> Vec<i32> {
        if self.is_game_over(){
            vec![]
        } else {
            vec![0, 1]
        }
    }

    pub fn score(&self) -> f32 {
        if self.agent_pos == 0 {
            -1.0
        } else if self.agent_pos == 4 {
                1.0
        } else {0.0}
    }

    pub fn step(&mut self, action: i32) {
        assert!(self.A.contains(&action));
        assert!(!self.is_game_over());

        if action == 0 {
            self.agent_pos -= 1;
        } else {
            self.agent_pos += 1;
        }
    }

    pub fn reset(&mut self) {
        self.agent_pos = 2;
    }

    pub fn display(&self) {
        for i in 0..5 {
            if i == self.agent_pos{
                print!("X");
            } else {
                print!("_");
            }
        }
        println!();
    }

}

