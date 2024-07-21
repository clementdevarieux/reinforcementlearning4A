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
    let mut secret3 = Env::SecretEnv3::SecretEnv3::new();

    secret3.reset();
    secret3.display();
    println!("{:?}", secret3.autorized_actions());
    println!("{:?}",secret3.A());
    println!("{:?}",secret3.available_actions_len());
    // let policy = secret0.value_iteration(0.0001f32, 0.999f32);
    // let res = secret0.monte_carlo_off_policy(0.99999f32,0.5, 1000, 10000);
    let res = secret3.monte_carlo_off_policy(0.99f32,0.10, 1000, 10000);
    // println!("{:?}", res);

    secret3.run_game_hashmap(res);

    // let mut secret2 = Env::SecretEnv2::SecretEnv2::new();
    // secret2.display();
    // println!("{:?}", secret2.autorized_actions());
    // println!("{:?}",secret2.A());
    // println!("{:?}",secret2.available_actions_len());
    // secret2.step(1);
    // secret2.display();
    // println!("{:?}", secret2.autorized_actions());
    // println!("{:?}",secret2.A());
    // println!("{:?}",secret2.available_actions_len());
    // // let policy = secret0.value_iteration(0.0001f32, 0.999f32);
    // // let res = secret0.monte_carlo_off_policy(0.99999f32,0.5, 1000, 10000);
    // let res = secret2.Q_learning_off_policy(0.99f32,0.10, 0.10, 1000, 10000);
    // println!("{:?}", res);
    //
    // secret2.run_game_hashmap(res);

    // let mut secret1 = Env::SecretEnv1::SecretEnv1::new();
    // // let policy = secret0.value_iteration(0.0001f32, 0.999f32);
    // // let res = secret0.monte_carlo_off_policy(0.99999f32,0.5, 1000, 10000);
    // let res = secret1.monte_carlo_off_policy(0.99f32,0.20, 1000000, 10000);
    // println!("{:?}", res);
    //
    // secret1.run_game_hashmap(res);


    // let mut secret0 = Env::SecretEnv0::SecretEnv0::new();
    // // let policy = secret0.value_iteration(0.0001f32, 0.999f32);
    // // let res = secret0.monte_carlo_off_policy(0.99999f32,0.5, 1000, 10000);
    // let res = secret0.Q_learning_off_policy(0.99f32,0.10, 0.10, 1000, 10000);
    // println!("{:?}", res);
    //
    // secret0.run_game_hashmap(res);

    // let mut lineworld = Env::LineWorld::LineWorld::init();
    // //
    // for _ in 0..1 {
    //     let res = lineworld.Q_learning_off_policy(0.99f32,0.10, 0.10, 1000, 10);
    //     lineworld.run_game_hashmap(res);
    //     // println!("{:?}", res);
    // }
    // //
    // let mut grid = Env::GridWorld::GridWorld::init();
    //
    // for _ in 0..1 {
    //     let res = grid.Q_learning_off_policy(0.99f32,0.10, 0.10, 10000, 10);
    //     grid.run_game_hashmap(res);
    //     // println!("{:?}", res);
    // }
    //
    // let mut shifumi = Env::Shifumi::Shifumi::init();
    //
    // for _ in 0..1 {
    //     let res = shifumi.Q_learning_off_policy(0.99f32,0.10, 0.1, 1000, 20);
    //     shifumi.run_game_hashmap(res);
    // }

    // let mut monty = Env::MontyHall::MontyHall::init();
    // for _ in 0..10 {
    //     let res = monty.Q_learning_off_policy(0.99f32,0.10, 0.1, 10000, 100);
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
