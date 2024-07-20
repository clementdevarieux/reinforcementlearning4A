mod environments;
mod algorithms;
mod Env;
mod secret_env;

use std::ffi::c_void;

use std::fmt;
use rand::prelude::*;
use std::iter::IntoIterator;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use colored::*;

fn main() {

    let mut secret0 = Env::SecretEnv0::SecretEnv0::new();
    //let policy = secret0.value_iteration(0.0001f32, 0.999f32);
    let res = secret0.monte_carlo_exploring_starts(0.99999f32, 1000, 10000);
    println!("{:?}", res);

    secret0.display();
    //
    // pub fn display_matrix(matrix: &[i32], rows: usize, cols: usize) {
    //     for i in 0..rows {
    //         for j in 0..cols {
    //             print!("{:3}", matrix[i * cols + j]); // {:3} pour aligner les colonnes
    //         }
    //         println!(); // Pour aller à la ligne après chaque ligne de la matrice
    //     }
    // }
    //
    // let mut lineworld = Env::LineWorld::LineWorld::init();
    //
    // for _ in 0..10 {
    //     let res = lineworld.monte_carlo_off_policy(0.99f32,0.10, 10000, 10);
    //     // lineworld.run_game_hashmap(res);
    //     println!("{:?}", res);
    // }
    // //
    // let mut grid = Env::GridWorld::GridWorld::init();

    // for _ in 0..1 {
    //     let res = grid.monte_carlo_off_policy(0.99f32,0.1, 10000, 20);
    //     grid.run_game_hashmap(res);
    //     // println!("{:?}", res);
    // }
    //
    // let mut shifumi = Env::Shifumi::Shifumi::init();
    //
    // for _ in 0..1 {
    //     let res = shifumi.monte_carlo_off_policy(0.99f32,0.10, 1000, 20);
    //     shifumi.run_game_hashmap(res);
    //     // println!("{:?}", res);
    // }

    // let mut monty = Env::MontyHall::MontyHall::init();
    // for _ in 0..10 {
    //     let res = monty.monte_carlo_off_policy(0.99f32,0.10, 10000, 100);
    //     monty.run_game_hashmap(res);
    //     // monty.run_game_random_hashmap(res);
    //     // println!("{:?}", res);
    // }

    // for _ in 0..10 {
    //     let res = monty.monte_carlo_exploring_starts(0.99999f32, 10000, 10);
    //     monty.run_game_hashmap(res);
    //     monty.reset();
    // }

}
