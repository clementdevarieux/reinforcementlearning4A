mod environments;
mod algorithms;
mod Env;

use std::fmt;
use rand::prelude::*;
use std::iter::IntoIterator;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use colored::*;

fn main() {


    pub fn display_matrix(matrix: &[i32], rows: usize, cols: usize) {
        for i in 0..rows {
            for j in 0..cols {
                print!("{:3}", matrix[i * cols + j]); // {:3} pour aligner les colonnes
            }
            println!(); // Pour aller à la ligne après chaque ligne de la matrice
        }
    }

    // let mut lineworld = Env::LineWorld::LineWorld::init();

    // let mut grid = Env::GridWorld::GridWorld::init();

    let mut shifumi = Env::Shifumi::Shifumi::init();
    // for _ in 0..10 {
    //     println!("coucou");
    //     let res = grid.monte_carlo_exploring_starts(0.99999f32, 10000, 10);
    //     println!("{:?}", res);
    // }
    //println!("{:?}", res);

    for _ in 0..10 {
        let res = shifumi.monte_carlo_exploring_starts(0.99999f32, 10000, 10);
        shifumi.run_game_hashmap(res);
        shifumi.reset();
    }
    // gridwold.step(1);
    // gridwold.display();
    // gridwold.step(3);
    // gridwold.display();
    //for _ in 0..10 {
    //    println!("{:?}", gridwold.policy_iteration(0.0001f32, 0.999f32));
    //}


    // println!("{:?}", lineworld.state_desc());

    // println!("{:?}", lineworld.policy_iteration(0.0001f32, 0.999f32));

    // println!("{:?}", lineworld.value_iteration(0.0001f32, 0.999f32));

    //
    // print!("{:?}", lineworld.p[1][0][0])
    // lineworld.step(0);
    //
    // println!("{:?}", lineworld.state_desc());
    //
    // lineworld.step(1);
    //
    // println!("{:?}", lineworld.state_desc());
    //
    // println!("{}", lineworld.agent_pos);
    //
    // println!("{:?}", lineworld.available_actions());
    //
    // lineworld.display();

    // println!("{:?}", lineworld.p);


    // let (S, A, R, T, p) = environments::line_world();
    // let (S, A, R, T, p) = environments::shifumi();
    //
    // // println!("{:?}", S)
    //
    // let res = algorithms::value_iteration(S, A, R, T, p, 0.0001f32, 0.999f32);
    //
    // println!("{:?}", res);
    //
    // let (S_grid, A_grid, R_grid, T_grid, p_grid) = environments::montyhall_standard();
    // let grid_res = algorithms::value_iteration(S_grid,A_grid,R_grid,T_grid,p_grid, 0.0001f32, 0.999f32);
    //
    // println!("{:?}", grid_res)


}
