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
use crate::secret_env::lib_secret_env::LIB;
use std::time;
use std::fs;
use std::path::PathBuf;

fn main() {

    // let secret_env_3_new: libloading::Symbol<unsafe extern fn() -> *mut c_void> =
    //     unsafe { LIB.get(b"secret_env_3_new") }.expect("Failed to load function `secret_env_3_new`");
    // unsafe {
    //     let env = secret_env_3_new();
    //     dbg!(env);
    //     let secret_env_3_display: libloading::Symbol<unsafe extern fn(*const c_void)> =
    //         unsafe { LIB.get(b"secret_env_3_display") }.expect("Failed to load function `secret_env_3_display`");
    //     dbg!(secret_env_3_display(env));
    // }


    // let mut secret3 = Env::SecretEnv3::SecretEnv3::new();

    // let policy = secret0.value_iteration(0.0001f32, 0.999f32);
    // let res = secret0.monte_carlo_off_policy(0.99999f32,0.5, 1000, 10000);
    // let res = secret3.Q_learning_off_policy(0.99f32,0.10, 0.1, 10000, 10000);
    // println!("{:?}", res);

    // secret3.run_game_hashmap(res);

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
    
    // secret1.run_game_hashmap(res);


    // let mut secret0 = Env::SecretEnv0::SecretEnv0::new();
    // // let policy = secret0.value_iteration(0.0001f32, 0.999f32);
    // // let res = secret0.monte_carlo_off_policy(0.99999f32,0.5, 1000, 10000);
    // let res = secret0.Q_learning_off_policy(0.99f32,0.10, 0.10, 1000, 10000);
    // println!("{:?}", res);
    //
    // secret0.run_game_hashmap(res);


    //////// GridWorld


    let mut lineworld = Env::LineWorld::LineWorld::init();
    fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("LineWorld.txt");
    fs::write(&file_path, "LineWorld\n");
    let mut durations_lineworld: Vec<u64> = Vec::new();
    for _ in 0..5 {
        let now = time::Instant::now();
        let res = lineworld.policy_iteration(0.1f32, 0.99f32);
        durations_lineworld.push(now.elapsed().as_secs());
        lineworld.run_game_vec(res);
        lineworld.reset()
    }
    let output = format!("Average elapsed time over 5 policy_iteration: {}", durations_lineworld.iter().sum::<u64>() / durations_lineworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_lineworld: Vec<u64> = Vec::new();
    for _ in 0..5 {
        let now = time::Instant::now();
        let res = lineworld.value_iteration(0.10f32, 0.99f32);
        durations_lineworld.push(now.elapsed().as_secs());
        lineworld.run_game_vec(res);
        lineworld.reset()
    }
    let output = format!("Average elapsed time over 5 value_iteration: {}", durations_lineworld.iter().sum::<u64>() / durations_lineworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_lineworld: Vec<u64> = Vec::new();
    for _ in 0..5 {
        let now = time::Instant::now();
        let res = lineworld.monte_carlo_exploring_starts(0.99f32, 1000, 10);
        durations_lineworld.push(now.elapsed().as_secs());
        lineworld.run_game_hashmap(res);
        lineworld.reset()
    }
    let output = format!("Average elapsed time over 5 monte_carlo_exploring_starts: {}", durations_lineworld.iter().sum::<u64>() / durations_lineworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_lineworld: Vec<u64> = Vec::new();
    for _ in 0..5 {
        let now = time::Instant::now();
        let res = lineworld.monte_carlo_fv_on_policy(0.99f32,0.10, 1000, 10);
        durations_lineworld.push(now.elapsed().as_secs());
        lineworld.run_game_random_hashmap(res);
        lineworld.reset()
    }
    let output = format!("Average elapsed time over 5 monte_carlo_fv_on_policy: {}", durations_lineworld.iter().sum::<u64>() / durations_lineworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_lineworld: Vec<u64> = Vec::new();
    for _ in 0..5 {
        let now = time::Instant::now();
        let res = lineworld.monte_carlo_off_policy(0.99f32,0.10, 1000, 10);
        durations_lineworld.push(now.elapsed().as_secs());
        lineworld.run_game_hashmap(res);
        lineworld.reset()
    }
    let output = format!("Average elapsed time over 5 monte_carlo_off_policy: {}", durations_lineworld.iter().sum::<u64>() / durations_lineworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_lineworld: Vec<u64> = Vec::new();
    for _ in 0..5 {
        let now = time::Instant::now();
        let res = lineworld.Q_learning_off_policy(0.99f32,0.10, 0.10, 1000, 10);
        durations_lineworld.push(now.elapsed().as_secs());
        lineworld.run_game_hashmap(res);
        lineworld.reset()
    }
    let output = format!("Average elapsed time over 5 Q_learning_off_policy: {}", durations_lineworld.iter().sum::<u64>() / durations_lineworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");


    //////// GridWorld


    //
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
