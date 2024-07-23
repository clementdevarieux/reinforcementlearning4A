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

    let number_of_tests = 5;

    //////// LineWorld


    let mut lineworld = Env::LineWorld::LineWorld::init();
    fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("LineWorld.txt");
    fs::write(&file_path, "LineWorld\n");

    let mut durations_lineworld: Vec<u64> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = lineworld.policy_iteration(theta, gamma);
        durations_lineworld.push(now.elapsed().as_secs());
        lineworld.run_game_vec(res);
        lineworld.reset()
    }
    let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_lineworld.iter().sum::<u64>() / durations_lineworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_lineworld: Vec<u64> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = lineworld.value_iteration(0.10f32, 0.99f32);
        durations_lineworld.push(now.elapsed().as_secs());
        lineworld.run_game_vec(res);
        lineworld.reset()
    }
    let output = format!("Average elapsed time over {} value_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_lineworld.iter().sum::<u64>() / durations_lineworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_lineworld: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = lineworld.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_lineworld.push(now.elapsed().as_secs());
        lineworld.run_game_hashmap(res);
        lineworld.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, nb_iter, max_steps, durations_lineworld.iter().sum::<u64>() / durations_lineworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_lineworld: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = lineworld.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_lineworld.push(now.elapsed().as_secs());
        lineworld.run_game_random_hashmap(res);
        lineworld.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_lineworld.iter().sum::<u64>() / durations_lineworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_lineworld: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = lineworld.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_lineworld.push(now.elapsed().as_secs());
        lineworld.run_game_hashmap(res);
        lineworld.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_lineworld.iter().sum::<u64>() / durations_lineworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_lineworld: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = lineworld.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_lineworld.push(now.elapsed().as_secs());
        lineworld.run_game_hashmap(res);
        lineworld.reset()
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_lineworld.iter().sum::<u64>() / durations_lineworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");


    //////// GridWorld


    let mut gridworld = Env::GridWorld::GridWorld::init();
    fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("GridWorld.txt");
    fs::write(&file_path, "GridWorld\n");

    let mut durations_gridworld: Vec<u64> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = gridworld.policy_iteration(0.1f32, 0.99f32);
        durations_gridworld.push(now.elapsed().as_secs());
        gridworld.run_game_vec(res);
        gridworld.reset()
    }
    let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_gridworld.iter().sum::<u64>() / durations_gridworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");
    
    let mut durations_gridworld: Vec<u64> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = gridworld.value_iteration(0.10f32, 0.99f32);
        durations_gridworld.push(now.elapsed().as_secs());
        gridworld.run_game_vec(res);
        gridworld.reset()
    }
    let output = format!("Average elapsed time over {} value_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_gridworld.iter().sum::<u64>() / durations_gridworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");
    
    let mut durations_gridworld: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = gridworld.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_gridworld.push(now.elapsed().as_secs());
        gridworld.run_game_hashmap(res);
        gridworld.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, nb_iter, max_steps, durations_gridworld.iter().sum::<u64>() / durations_gridworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");
    
    let mut durations_gridworld: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = gridworld.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_gridworld.push(now.elapsed().as_secs());
        gridworld.run_game_random_hashmap(res);
        gridworld.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_gridworld.iter().sum::<u64>() / durations_gridworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");
    
    let mut durations_gridworld: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = gridworld.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_gridworld.push(now.elapsed().as_secs());
        gridworld.run_game_hashmap(res);
        gridworld.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_gridworld.iter().sum::<u64>() / durations_gridworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");
    
    let mut durations_gridworld: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = gridworld.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_gridworld.push(now.elapsed().as_secs());
        gridworld.run_game_hashmap(res);
        gridworld.reset()
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_gridworld.iter().sum::<u64>() / durations_gridworld.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");
    
    
    //////// Shifumi
    
    
    let mut shifumi = Env::Shifumi::Shifumi::init();
    fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("Shifumi.txt");
    fs::write(&file_path, "Shifumi\n");

    let mut durations_shifumi: Vec<u64> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = shifumi.policy_iteration(0.1f32, 0.99f32);
        durations_shifumi.push(now.elapsed().as_secs());
        shifumi.run_game_vec(res);
        shifumi.reset()
    }
    let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_shifumi.iter().sum::<u64>() / durations_shifumi.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_shifumi: Vec<u64> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = shifumi.value_iteration(0.10f32, 0.99f32);
        durations_shifumi.push(now.elapsed().as_secs());
        shifumi.run_game_vec(res);
        shifumi.reset()
    }
    let output = format!("Average elapsed time over {} value_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_shifumi.iter().sum::<u64>() / durations_shifumi.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_shifumi: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = shifumi.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_shifumi.push(now.elapsed().as_secs());
        shifumi.run_game_hashmap(res);
        shifumi.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, nb_iter, max_steps, durations_shifumi.iter().sum::<u64>() / durations_shifumi.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_shifumi: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = shifumi.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_shifumi.push(now.elapsed().as_secs());
        shifumi.run_game_random_hashmap(res);
        shifumi.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_shifumi.iter().sum::<u64>() / durations_shifumi.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_shifumi: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = shifumi.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_shifumi.push(now.elapsed().as_secs());
        shifumi.run_game_hashmap(res);
        shifumi.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_shifumi.iter().sum::<u64>() / durations_shifumi.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_shifumi: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = shifumi.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_shifumi.push(now.elapsed().as_secs());
        shifumi.run_game_hashmap(res);
        shifumi.reset()
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_shifumi.iter().sum::<u64>() / durations_shifumi.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");


    //////// MontyHall
    

    let mut montyhall = Env::MontyHall::MontyHall::init();
    fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("MontyHall.txt");
    fs::write(&file_path, "MontyHall\n");

    let mut durations_montyhall: Vec<u64> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = montyhall.policy_iteration(0.1f32, 0.99f32);
        durations_montyhall.push(now.elapsed().as_secs());
        montyhall.run_game_vec(res);
        montyhall.reset()
    }
    let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_montyhall.iter().sum::<u64>() / durations_montyhall.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_montyhall: Vec<u64> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = montyhall.value_iteration(0.10f32, 0.99f32);
        durations_montyhall.push(now.elapsed().as_secs());
        montyhall.run_game_vec(res);
        montyhall.reset()
    }
    let output = format!("Average elapsed time over {} value_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_montyhall.iter().sum::<u64>() / durations_montyhall.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_montyhall: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = montyhall.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_montyhall.push(now.elapsed().as_secs());
        montyhall.run_game_hashmap(res);
        montyhall.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, nb_iter, max_steps, durations_montyhall.iter().sum::<u64>() / durations_montyhall.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_montyhall: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = montyhall.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_montyhall.push(now.elapsed().as_secs());
        montyhall.run_game_random_hashmap(res);
        montyhall.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_montyhall.iter().sum::<u64>() / durations_montyhall.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_montyhall: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = montyhall.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_montyhall.push(now.elapsed().as_secs());
        montyhall.run_game_hashmap(res);
        montyhall.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_montyhall.iter().sum::<u64>() / durations_montyhall.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_montyhall: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = montyhall.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_montyhall.push(now.elapsed().as_secs());
        montyhall.run_game_hashmap(res);
        montyhall.reset()
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_montyhall.iter().sum::<u64>() / durations_montyhall.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");


    //////// SecretEnv0


    let mut secretenv0 = Env::SecretEnv0::SecretEnv0::new();
    fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("SecretEnv0.txt");
    fs::write(&file_path, "SecretEnv0\n");

    // let mut durations_secretenv0: Vec<u64> = Vec::new();
    // let theta = 0.1f32;
    // let gamma = 0.99f32;
    // for _ in 0..number_of_tests {
    //     let now = time::Instant::now();
    //     let res = secretenv0.policy_iteration(0.1f32, 0.99f32);
    //     durations_secretenv0.push(now.elapsed().as_secs());
    //     secretenv0.run_game_vec(res);
    //     secretenv0.reset()
    // }
    // let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_secretenv0.iter().sum::<u64>() / durations_secretenv0.len() as u64);
    // println!("{}", output);
    // fs::write(&file_path, "output");

    // let mut durations_secretenv0: Vec<u64> = Vec::new();
    // let theta = 0.1f32;
    // let gamma = 0.99f32;
    // for _ in 0..number_of_tests {
    //     let now = time::Instant::now();
    //     let res = secretenv0.value_iteration(0.10f32, 0.99f32);
    //     durations_secretenv0.push(now.elapsed().as_secs());
    //     secretenv0.run_game_vec(res);
    //     secretenv0.reset()
    // }
    // let output = format!("Average elapsed time over {} value_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_secretenv0.iter().sum::<u64>() / durations_secretenv0.len() as u64);
    // println!("{}", output);
    // fs::write(&file_path, "output");

    let mut durations_secretenv0: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv0.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_secretenv0.push(now.elapsed().as_secs());
        secretenv0.run_game_hashmap(res);
        secretenv0.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, nb_iter, max_steps, durations_secretenv0.iter().sum::<u64>() / durations_secretenv0.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_secretenv0: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv0.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv0.push(now.elapsed().as_secs());
        secretenv0.run_game_random_hashmap(res);
        secretenv0.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv0.iter().sum::<u64>() / durations_secretenv0.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_secretenv0: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv0.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv0.push(now.elapsed().as_secs());
        secretenv0.run_game_hashmap(res);
        secretenv0.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv0.iter().sum::<u64>() / durations_secretenv0.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_secretenv0: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv0.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_secretenv0.push(now.elapsed().as_secs());
        secretenv0.run_game_hashmap(res);
        secretenv0.reset()
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_secretenv0.iter().sum::<u64>() / durations_secretenv0.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");


    //////// SecretEnv1


    let mut secretenv1 = Env::SecretEnv1::SecretEnv1::new();
    fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("SecretEnv31.txt");
    fs::write(&file_path, "SecretEnv1\n");

    // let mut durations_secretenv1: Vec<u64> = Vec::new();
    // let theta = 0.1f32;
    // let gamma = 0.99f32;
    // for _ in 0..number_of_tests {
    //     let now = time::Instant::now();
    //     let res = secretenv1.policy_iteration(0.1f32, 0.99f32);
    //     durations_secretenv1.push(now.elapsed().as_secs());
    //     secretenv1.run_game_vec(res);
    //     secretenv1.reset()
    // }
    // let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_secretenv1.iter().sum::<u64>() / durations_secretenv1.len() as u64);
    // println!("{}", output);
    // fs::write(&file_path, "output");

    // let mut durations_secretenv1: Vec<u64> = Vec::new();
    // let theta = 0.1f32;
    // let gamma = 0.99f32;
    // for _ in 0..number_of_tests {
    //     let now = time::Instant::now();
    //     let res = secretenv1.value_iteration(0.10f32, 0.99f32);
    //     durations_secretenv1.push(now.elapsed().as_secs());
    //     secretenv1.run_game_vec(res);
    //     secretenv1.reset()
    // }
    // let output = format!("Average elapsed time over {} value_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_secretenv1.iter().sum::<u64>() / durations_secretenv1.len() as u64);
    // println!("{}", output);
    // fs::write(&file_path, "output");

    let mut durations_secretenv1: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv1.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_secretenv1.push(now.elapsed().as_secs());
        secretenv1.run_game_hashmap(res);
        secretenv1.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, nb_iter, max_steps, durations_secretenv1.iter().sum::<u64>() / durations_secretenv1.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_secretenv1: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv1.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv1.push(now.elapsed().as_secs());
        secretenv1.run_game_random_hashmap(res);
        secretenv1.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv1.iter().sum::<u64>() / durations_secretenv1.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_secretenv1: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv1.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv1.push(now.elapsed().as_secs());
        secretenv1.run_game_hashmap(res);
        secretenv1.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv1.iter().sum::<u64>() / durations_secretenv1.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_secretenv1: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv1.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_secretenv1.push(now.elapsed().as_secs());
        secretenv1.run_game_hashmap(res);
        secretenv1.reset()
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_secretenv1.iter().sum::<u64>() / durations_secretenv1.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");


    //////// SeceretEnv2


    let mut secretenv2 = Env::SecretEnv2::SecretEnv2::new();
    fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("SecretEnv2.txt");
    fs::write(&file_path, "SecretEnv2\n");

    // let mut durations_secretenv2: Vec<u64> = Vec::new();
    // let theta = 0.1f32;
    // let gamma = 0.99f32;
    // for _ in 0..number_of_tests {
    //     let now = time::Instant::now();
    //     let res = secretenv2.policy_iteration(0.1f32, 0.99f32);
    //     durations_secretenv2.push(now.elapsed().as_secs());
    //     secretenv2.run_game_vec(res);
    //     secretenv2.reset()
    // }
    // let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_secretenv2.iter().sum::<u64>() / durations_secretenv2.len() as u64);
    // println!("{}", output);
    // fs::write(&file_path, "output");

    // let mut durations_secretenv2: Vec<u64> = Vec::new();
    // let theta = 0.1f32;
    // let gamma = 0.99f32;
    // for _ in 0..number_of_tests {
    //     let now = time::Instant::now();
    //     let res = secretenv2.value_iteration(0.10f32, 0.99f32);
    //     durations_secretenv2.push(now.elapsed().as_secs());
    //     secretenv2.run_game_vec(res);
    //     secretenv2.reset()
    // }
    // let output = format!("Average elapsed time over {} value_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_secretenv2.iter().sum::<u64>() / durations_secretenv2.len() as u64);
    // println!("{}", output);
    // fs::write(&file_path, "output");

    let mut durations_secretenv2: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv2.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_secretenv2.push(now.elapsed().as_secs());
        secretenv2.run_game_hashmap(res);
        secretenv2.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, nb_iter, max_steps, durations_secretenv2.iter().sum::<u64>() / durations_secretenv2.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_secretenv2: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv2.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv2.push(now.elapsed().as_secs());
        secretenv2.run_game_random_hashmap(res);
        secretenv2.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv2.iter().sum::<u64>() / durations_secretenv2.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_secretenv2: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv2.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv2.push(now.elapsed().as_secs());
        secretenv2.run_game_hashmap(res);
        secretenv2.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv2.iter().sum::<u64>() / durations_secretenv2.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_secretenv2: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv2.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_secretenv2.push(now.elapsed().as_secs());
        secretenv2.run_game_hashmap(res);
        secretenv2.reset()
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_secretenv2.iter().sum::<u64>() / durations_secretenv2.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");


    //////// SecretEnv3
    
    
    let mut secretenv3 = Env::SecretEnv3::SecretEnv3::new();
    fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("SecretEnv3.txt");
    fs::write(&file_path, "SecretEnv3\n");

    // let mut durations_secretenv3: Vec<u64> = Vec::new();
    // let theta = 0.1f32;
    // let gamma = 0.99f32;
    // for _ in 0..number_of_tests {
    //     let now = time::Instant::now();
    //     let res = secretenv3.policy_iteration(0.1f32, 0.99f32);
    //     durations_secretenv3.push(now.elapsed().as_secs());
    //     secretenv3.run_game_vec(res);
    //     secretenv3.reset()
    // }
    // let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_secretenv3.iter().sum::<u64>() / durations_secretenv3.len() as u64);
    // println!("{}", output);
    // fs::write(&file_path, "output");

    // let mut durations_secretenv3: Vec<u64> = Vec::new();
    // let theta = 0.1f32;
    // let gamma = 0.99f32;
    // for _ in 0..number_of_tests {
    //     let now = time::Instant::now();
    //     let res = secretenv3.value_iteration(0.10f32, 0.99f32);
    //     durations_secretenv3.push(now.elapsed().as_secs());
    //     secretenv3.run_game_vec(res);
    //     secretenv3.reset()
    // }
    // let output = format!("Average elapsed time over {} value_iteration (theta = {}, gammma = {}): {}", number_of_tests, theta, gamma, durations_secretenv3.iter().sum::<u64>() / durations_secretenv3.len() as u64);
    // println!("{}", output);
    // fs::write(&file_path, "output");

    let mut durations_secretenv3: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv3.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_secretenv3.push(now.elapsed().as_secs());
        secretenv3.run_game_hashmap(res);
        secretenv3.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, nb_iter, max_steps, durations_secretenv3.iter().sum::<u64>() / durations_secretenv3.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_secretenv3: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv3.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv3.push(now.elapsed().as_secs());
        secretenv3.run_game_random_hashmap(res);
        secretenv3.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv3.iter().sum::<u64>() / durations_secretenv3.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_secretenv3: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv3.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv3.push(now.elapsed().as_secs());
        secretenv3.run_game_hashmap(res);
        secretenv3.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv3.iter().sum::<u64>() / durations_secretenv3.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

    let mut durations_secretenv3: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 0.99f32;
    let max_steps = 0.99f32;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv3.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_secretenv3.push(now.elapsed().as_secs());
        secretenv3.run_game_hashmap(res);
        secretenv3.reset()
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_secretenv3.iter().sum::<u64>() / durations_secretenv3.len() as u64);
    println!("{}", output);
    fs::write(&file_path, "output");

}
