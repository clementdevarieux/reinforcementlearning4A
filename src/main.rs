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
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::PathBuf;

fn main() {

    let number_of_tests = 5;

    /*
    //////// LineWorld



    //fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("LineWorld.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "LineWorld\n");

    let mut durations_lineworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let mut lineworld = Env::LineWorld::LineWorld::init();
        let now = time::Instant::now();
        let res = lineworld.policy_iteration(theta, gamma);
        durations_lineworld.push(now.elapsed().as_millis());
        lineworld.run_game_vec(res);
        final_score.push(lineworld.score());
    }
    let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}ms", number_of_tests, theta, gamma, durations_lineworld.iter().sum::<u128>() / durations_lineworld.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    ///////////////////
    let mut durations_lineworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let mut lineworld = Env::LineWorld::LineWorld::init();
        let now = time::Instant::now();
        let res = lineworld.value_iteration(theta, gamma);
        durations_lineworld.push(now.elapsed().as_millis());
        lineworld.run_game_vec(res);
        final_score.push(lineworld.score());
    }
    let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}ms", number_of_tests, theta, gamma, durations_lineworld.iter().sum::<u128>() / durations_lineworld.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);
    ////////////////
    let mut durations_lineworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut lineworld = Env::LineWorld::LineWorld::init();
        let now = time::Instant::now();
        let res = lineworld.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_lineworld.push(now.elapsed().as_millis());
        lineworld.run_game_hashmap(res);
        final_score.push(lineworld.score());
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, nb_iter, max_steps, durations_lineworld.iter().sum::<u128>() / durations_lineworld.len() as u128);
    println!("{}", output);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);
    ////////////////////

    let mut durations_lineworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut lineworld = Env::LineWorld::LineWorld::init();
        let now = time::Instant::now();
        let res = lineworld.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_lineworld.push(now.elapsed().as_millis());
        lineworld.run_game_random_hashmap(res);
        final_score.push(lineworld.score());
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_lineworld.iter().sum::<u128>() / durations_lineworld.len() as u128);
    println!("{}", output);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    ///////////////
    let mut durations_lineworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut lineworld = Env::LineWorld::LineWorld::init();
        let now = time::Instant::now();
        let res = lineworld.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_lineworld.push(now.elapsed().as_millis());
        lineworld.run_game_hashmap(res);
        final_score.push(lineworld.score());
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_lineworld.iter().sum::<u128>() / durations_lineworld.len() as u128);
    println!("{}", output);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    ///////////

    let mut durations_lineworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut lineworld = Env::LineWorld::LineWorld::init();
        let now = time::Instant::now();
        let res = lineworld.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_lineworld.push(now.elapsed().as_millis());
        lineworld.run_game_hashmap(res);
        final_score.push(lineworld.score());
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_lineworld.iter().sum::<u128>() / durations_lineworld.len() as u128);
    println!("{}", output);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);
    ///////////

    let mut durations_lineworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut lineworld = Env::LineWorld::LineWorld::init();
        let now = time::Instant::now();
        let res = lineworld.sarsa(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_lineworld.push(now.elapsed().as_millis());
        lineworld.run_game_hashmap(res);
        final_score.push(lineworld.score());
    }
    let output = format!("Average elapsed time over {} Sarsa (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_lineworld.iter().sum::<u128>() / durations_lineworld.len() as u128);
    println!("{}", output);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);





    //////// GridWorld


    let file_path = PathBuf::from("./2024-07-23").join("GridWorld.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "GridWorld\n");

    let mut durations_gridworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let mut gridworld = Env::GridWorld::GridWorld::init();
        let now = time::Instant::now();
        let res = gridworld.policy_iteration(theta, gamma);
        durations_gridworld.push(now.elapsed().as_millis());
        gridworld.run_game_vec(res);
        final_score.push(gridworld.score());
    }
    let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}ms", number_of_tests, theta, gamma, durations_gridworld.iter().sum::<u128>() / durations_gridworld.len() as u128);

    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    //////
    
    let mut durations_gridworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let theta = 0.01f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let mut gridworld = Env::GridWorld::GridWorld::init();
        let now = time::Instant::now();
        let res = gridworld.value_iteration(theta, gamma);
        durations_gridworld.push(now.elapsed().as_millis());
        gridworld.run_game_vec(res);
        final_score.push(gridworld.score());
    }
    let output = format!("Average elapsed time over {} value_iteration (theta = {}, gammma = {}): {}ms", number_of_tests, theta, gamma, durations_gridworld.iter().sum::<u128>() / durations_gridworld.len() as u128);

    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    //////
    
    let mut durations_gridworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 10000;
    let max_steps = 1000;
    for _ in 0..number_of_tests {
        let mut gridworld = Env::GridWorld::GridWorld::init();
        let now = time::Instant::now();
        let res = gridworld.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_gridworld.push(now.elapsed().as_millis());
        gridworld.run_game_hashmap(res);
        final_score.push(gridworld.score());
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, nb_iter, max_steps, durations_gridworld.iter().sum::<u128>() / durations_gridworld.len() as u128);

    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    /////
    
    let mut durations_gridworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 10000;
    let max_steps = 1000;
    for _ in 0..number_of_tests {
        let mut gridworld = Env::GridWorld::GridWorld::init();
        let now = time::Instant::now();
        let res = gridworld.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_gridworld.push(now.elapsed().as_millis());
        gridworld.run_game_random_hashmap(res);
        final_score.push(gridworld.score());
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_gridworld.iter().sum::<u128>() / durations_gridworld.len() as u128);

    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    ////
    
    let mut durations_gridworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 10000;
    let max_steps = 1000;
    for _ in 0..number_of_tests {
        let mut gridworld = Env::GridWorld::GridWorld::init();
        let now = time::Instant::now();
        let res = gridworld.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_gridworld.push(now.elapsed().as_millis());
        gridworld.run_game_hashmap(res);
        final_score.push(gridworld.score());
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_gridworld.iter().sum::<u128>() / durations_gridworld.len() as u128);

    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);
    ///////
    
    let mut durations_gridworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 10000;
    let max_steps = 1000;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let mut gridworld = Env::GridWorld::GridWorld::init();
        let res = gridworld.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_gridworld.push(now.elapsed().as_millis());
        gridworld.run_game_hashmap(res);
        final_score.push(gridworld.score());
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_gridworld.iter().sum::<u128>() / durations_gridworld.len() as u128);

    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    ///////

    let mut durations_gridworld: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 10000;
    let max_steps = 1000;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let mut gridworld = Env::GridWorld::GridWorld::init();
        let res = gridworld.sarsa(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_gridworld.push(now.elapsed().as_millis());
        gridworld.run_game_hashmap(res);
        final_score.push(gridworld.score());
    }
    let output = format!("Average elapsed time over {} sarsa (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_gridworld.iter().sum::<u128>() / durations_gridworld.len() as u128);

    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);



    //////// Shifumi
    

    let file_path = PathBuf::from("./2024-07-23").join("Shifumi.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "Shifumi\n");

    let mut durations_shifumi: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let mut shifumi = Env::Shifumi::Shifumi::init();
        let now = time::Instant::now();
        let res = shifumi.policy_iteration(theta, gamma);
        durations_shifumi.push(now.elapsed().as_millis());
        shifumi.run_game_vec(res);
        final_score.push(shifumi.score() as f32);
    }
    let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}ms", number_of_tests, theta, gamma, durations_shifumi.iter().sum::<u128>() / durations_shifumi.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    ////

    let mut durations_shifumi: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let theta = 0.01f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let mut shifumi = Env::Shifumi::Shifumi::init();
        let now = time::Instant::now();
        let res = shifumi.value_iteration(theta, gamma);
        durations_shifumi.push(now.elapsed().as_millis());
        shifumi.run_game_vec(res);
        final_score.push(shifumi.score() as f32);
    }
    let output = format!("Average elapsed time over {} value_iteration (theta = {}, gammma = {}): {}ms", number_of_tests, theta, gamma, durations_shifumi.iter().sum::<u128>() / durations_shifumi.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    ////

    let mut durations_shifumi: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut shifumi = Env::Shifumi::Shifumi::init();
        let now = time::Instant::now();
        let res = shifumi.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_shifumi.push(now.elapsed().as_millis());
        shifumi.run_game_hashmap(res);
        final_score.push(shifumi.score() as f32);
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, nb_iter, max_steps, durations_shifumi.iter().sum::<u128>() / durations_shifumi.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);
    /////////

    let mut durations_shifumi: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut shifumi = Env::Shifumi::Shifumi::init();
        let now = time::Instant::now();
        let res = shifumi.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_shifumi.push(now.elapsed().as_millis());
        shifumi.run_game_random_hashmap(res);
        final_score.push(shifumi.score() as f32);
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_shifumi.iter().sum::<u128>() / durations_shifumi.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    //////////

    let mut durations_shifumi: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut shifumi = Env::Shifumi::Shifumi::init();
        let now = time::Instant::now();
        let res = shifumi.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_shifumi.push(now.elapsed().as_millis());
        shifumi.run_game_hashmap(res);
        final_score.push(shifumi.score() as f32);
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_shifumi.iter().sum::<u128>() / durations_shifumi.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);
    //////

    let mut durations_shifumi: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut shifumi = Env::Shifumi::Shifumi::init();
        let now = time::Instant::now();
        let res = shifumi.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_shifumi.push(now.elapsed().as_millis());
        shifumi.run_game_hashmap(res);
        final_score.push(shifumi.score() as f32);
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_shifumi.iter().sum::<u128>() / durations_shifumi.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    //////

    let mut durations_shifumi: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut shifumi = Env::Shifumi::Shifumi::init();
        let now = time::Instant::now();
        let res = shifumi.sarsa(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_shifumi.push(now.elapsed().as_millis());
        shifumi.run_game_hashmap(res);
        final_score.push(shifumi.score() as f32);
    }
    let output = format!("Average elapsed time over {} sarsa (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_shifumi.iter().sum::<u128>() / durations_shifumi.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);



    //////// MontyHall
    

    let file_path = PathBuf::from("./2024-07-23").join("MontyHall.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "MontyHall\n");

    let mut durations_montyhall: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let theta = 0.1f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let mut montyhall = Env::MontyHall::MontyHall::init();
        let now = time::Instant::now();
        let res = montyhall.policy_iteration(theta, gamma);
        durations_montyhall.push(now.elapsed().as_millis());
        montyhall.run_game_vec(res);
        final_score.push(montyhall.score() as f32);
    }
    let output = format!("Average elapsed time over {} policy_iteration (theta = {}, gammma = {}): {}ms", number_of_tests, theta, gamma, durations_montyhall.iter().sum::<u128>() / durations_montyhall.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    /////

    let mut durations_montyhall: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let theta = 0.01f32;
    let gamma = 0.99f32;
    for _ in 0..number_of_tests {
        let mut montyhall = Env::MontyHall::MontyHall::init();
        let now = time::Instant::now();
        let res = montyhall.value_iteration(theta, gamma);
        durations_montyhall.push(now.elapsed().as_millis());
        montyhall.run_game_vec(res);
        final_score.push(montyhall.score() as f32);
    }
    let output = format!("Average elapsed time over {} value_iteration (theta = {}, gammma = {}): {}ms", number_of_tests, theta, gamma, durations_montyhall.iter().sum::<u128>() / durations_montyhall.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    /////

    let mut durations_montyhall: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut montyhall = Env::MontyHall::MontyHall::init();
        let now = time::Instant::now();
        let res = montyhall.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_montyhall.push(now.elapsed().as_millis());
        montyhall.run_game_hashmap(res);
        final_score.push(montyhall.score() as f32);
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, nb_iter, max_steps, durations_montyhall.iter().sum::<u128>() / durations_montyhall.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    /////

    // let mut durations_montyhall: Vec<u128> = Vec::new();
    // let mut final_score: Vec<f32> = Vec::new();
    // let gamma = 0.99f32;
    // let epsilon = 0.10;
    // let nb_iter = 1000;
    // let max_steps = 10;
    // for _ in 0..number_of_tests {
    //     let mut montyhall = Env::MontyHall::MontyHall::init();
    //     let now = time::Instant::now();
    //     let res = montyhall.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
    //     durations_montyhall.push(now.elapsed().as_millis());
    //     montyhall.run_game_random_hashmap(res);
    //     final_score.push(montyhall.score() as f32);
    // }
    // let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_montyhall.iter().sum::<u128>() / durations_montyhall.len() as u128);
    // let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    // let output_score = format!("Score moyen= {}", average_score);
    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open(&file_path)
    //     .unwrap();
    // writeln!(file, "{}", output);
    // writeln!(file, "{}", output_score);

    //////

    // let mut durations_montyhall: Vec<u128> = Vec::new();
    // let mut final_score: Vec<f32> = Vec::new();
    // let gamma = 0.99f32;
    // let epsilon = 0.10;
    // let nb_iter = 1000;
    // let max_steps = 10;
    // for _ in 0..number_of_tests {
    //     let mut montyhall = Env::MontyHall::MontyHall::init();
    //     let now = time::Instant::now();
    //     let res = montyhall.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
    //     durations_montyhall.push(now.elapsed().as_millis());
    //     montyhall.run_game_hashmap(res);
    //     final_score.push(montyhall.score() as f32);
    // }
    // let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_montyhall.iter().sum::<u128>() / durations_montyhall.len() as u128);
    // let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    // let output_score = format!("Score moyen= {}", average_score);
    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open(&file_path)
    //     .unwrap();
    // writeln!(file, "{}", output);
    // writeln!(file, "{}", output_score);

    /////

    let mut durations_montyhall: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut montyhall = Env::MontyHall::MontyHall::init();
        let now = time::Instant::now();
        let res = montyhall.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_montyhall.push(now.elapsed().as_millis());
        montyhall.run_game_hashmap(res);
        final_score.push(montyhall.score() as f32);
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_montyhall.iter().sum::<u128>() / durations_montyhall.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    /////

    let mut durations_montyhall: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let mut montyhall = Env::MontyHall::MontyHall::init();
        let now = time::Instant::now();
        let res = montyhall.sarsa(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_montyhall.push(now.elapsed().as_millis());
        montyhall.run_game_hashmap(res);
        final_score.push(montyhall.score() as f32);
    }
    let output = format!("Average elapsed time over {} sarsa (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_montyhall.iter().sum::<u128>() / durations_montyhall.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    */

    //////// SecretEnv0


    let file_path = PathBuf::from("./2024-07-23").join("SecretEnv0.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "SecretEnv0\n");

    let mut durations_secretenv0: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 10000;
    let max_steps = 1000;
    for _ in 0..number_of_tests {
        let mut secretenv0 = Env::SecretEnv0::SecretEnv0::new();
        let now = time::Instant::now();
        let res = secretenv0.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_secretenv0.push(now.elapsed().as_millis());
        secretenv0.run_game_hashmap(res);
        final_score.push(secretenv0.score());
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, nb_iter, max_steps, durations_secretenv0.iter().sum::<u128>() / durations_secretenv0.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    ///////

    let mut durations_secretenv0: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 10000;
    let max_steps = 1000;
    for _ in 0..number_of_tests {
        let mut secretenv0 = Env::SecretEnv0::SecretEnv0::new();
        let now = time::Instant::now();
        let res = secretenv0.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv0.push(now.elapsed().as_millis());
        secretenv0.run_game_random_hashmap(res);
        final_score.push(secretenv0.score());
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv0.iter().sum::<u128>() / durations_secretenv0.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    //////

    let mut durations_secretenv0: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 10000;
    let max_steps = 1000;
    for _ in 0..number_of_tests {
        let mut secretenv0 = Env::SecretEnv0::SecretEnv0::new();
        let now = time::Instant::now();
        let res = secretenv0.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv0.push(now.elapsed().as_millis());
        secretenv0.run_game_hashmap(res);
        final_score.push(secretenv0.score());
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv0.iter().sum::<u128>() / durations_secretenv0.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);

    ///////

    let mut durations_secretenv0: Vec<u128> = Vec::new();
    let mut final_score: Vec<f32> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 10000;
    let max_steps = 1000;
    for _ in 0..number_of_tests {
        let mut secretenv0 = Env::SecretEnv0::SecretEnv0::new();
        let now = time::Instant::now();
        let res = secretenv0.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_secretenv0.push(now.elapsed().as_millis());
        secretenv0.run_game_hashmap(res);
        final_score.push(secretenv0.score());
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}ms", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_secretenv0.iter().sum::<u128>() / durations_secretenv0.len() as u128);
    let average_score: f32 = final_score.iter().sum::<f32>() / final_score.len() as f32;
    let output_score = format!("Score moyen= {}", average_score);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);
    writeln!(file, "{}", output_score);



    /*
    //////// SecretEnv1


    let mut secretenv1 = Env::SecretEnv1::SecretEnv1::new();
    fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("SecretEnv31.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "SecretEnv1\n");

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
    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open(&file_path)
    //     .unwrap();
    // writeln!(file, "{}", output);

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
    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open(&file_path)
    //     .unwrap();
    // writeln!(file, "{}", output);

    let mut durations_secretenv1: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv1.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_secretenv1.push(now.elapsed().as_secs());
        secretenv1.run_game_hashmap(res);
        secretenv1.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, nb_iter, max_steps, durations_secretenv1.iter().sum::<u64>() / durations_secretenv1.len() as u64);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);

    let mut durations_secretenv1: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv1.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv1.push(now.elapsed().as_secs());
        secretenv1.run_game_random_hashmap(res);
        secretenv1.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv1.iter().sum::<u64>() / durations_secretenv1.len() as u64);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);

    let mut durations_secretenv1: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv1.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv1.push(now.elapsed().as_secs());
        secretenv1.run_game_hashmap(res);
        secretenv1.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv1.iter().sum::<u64>() / durations_secretenv1.len() as u64);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);

    let mut durations_secretenv1: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv1.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_secretenv1.push(now.elapsed().as_secs());
        secretenv1.run_game_hashmap(res);
        secretenv1.reset()
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_secretenv1.iter().sum::<u64>() / durations_secretenv1.len() as u64);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);


    //////// SeceretEnv2


    let mut secretenv2 = Env::SecretEnv2::SecretEnv2::new();
    fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("SecretEnv2.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "SecretEnv2\n");

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
    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open(&file_path)
    //     .unwrap();
    // writeln!(file, "{}", output);

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
    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open(&file_path)
    //     .unwrap();
    // writeln!(file, "{}", output);

    let mut durations_secretenv2: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv2.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_secretenv2.push(now.elapsed().as_secs());
        secretenv2.run_game_hashmap(res);
        secretenv2.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, nb_iter, max_steps, durations_secretenv2.iter().sum::<u64>() / durations_secretenv2.len() as u64);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);

    let mut durations_secretenv2: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv2.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv2.push(now.elapsed().as_secs());
        secretenv2.run_game_random_hashmap(res);
        secretenv2.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv2.iter().sum::<u64>() / durations_secretenv2.len() as u64);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);

    let mut durations_secretenv2: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv2.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv2.push(now.elapsed().as_secs());
        secretenv2.run_game_hashmap(res);
        secretenv2.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv2.iter().sum::<u64>() / durations_secretenv2.len() as u64);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);

    let mut durations_secretenv2: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv2.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_secretenv2.push(now.elapsed().as_secs());
        secretenv2.run_game_hashmap(res);
        secretenv2.reset()
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_secretenv2.iter().sum::<u64>() / durations_secretenv2.len() as u64);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);


    //////// SecretEnv3
    
    
    let mut secretenv3 = Env::SecretEnv3::SecretEnv3::new();
    fs::create_dir("2024-07-23").unwrap();
    let file_path = PathBuf::from("./2024-07-23").join("SecretEnv3.txt");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "SecretEnv3\n");

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
    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open(&file_path)
    //     .unwrap();
    // writeln!(file, "{}", output);

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
    // let mut file = OpenOptions::new()
    //     .write(true)
    //     .append(true)
    //     .open(&file_path)
    //     .unwrap();
    // writeln!(file, "{}", output);

    let mut durations_secretenv3: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv3.monte_carlo_exploring_starts(gamma, nb_iter, max_steps);
        durations_secretenv3.push(now.elapsed().as_secs());
        secretenv3.run_game_hashmap(res);
        secretenv3.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_exploring_starts (gammma = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, nb_iter, max_steps, durations_secretenv3.iter().sum::<u64>() / durations_secretenv3.len() as u64);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);

    let mut durations_secretenv3: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv3.monte_carlo_fv_on_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv3.push(now.elapsed().as_secs());
        secretenv3.run_game_random_hashmap(res);
        secretenv3.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_fv_on_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv3.iter().sum::<u64>() / durations_secretenv3.len() as u64);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);

    let mut durations_secretenv3: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv3.monte_carlo_off_policy(gamma, epsilon, nb_iter, max_steps);
        durations_secretenv3.push(now.elapsed().as_secs());
        secretenv3.run_game_hashmap(res);
        secretenv3.reset()
    }
    let output = format!("Average elapsed time over {} monte_carlo_off_policy (gammma = {}, epsilon = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, nb_iter, max_steps, durations_secretenv3.iter().sum::<u64>() / durations_secretenv3.len() as u64);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);

    let mut durations_secretenv3: Vec<u64> = Vec::new();
    let gamma = 0.99f32;
    let epsilon = 0.10;
    let alpha = 0.10;
    let nb_iter = 1000;
    let max_steps = 10;
    for _ in 0..number_of_tests {
        let now = time::Instant::now();
        let res = secretenv3.Q_learning_off_policy(gamma, epsilon, alpha, nb_iter, max_steps);
        durations_secretenv3.push(now.elapsed().as_secs());
        secretenv3.run_game_hashmap(res);
        secretenv3.reset()
    }
    let output = format!("Average elapsed time over {} Q_learning_off_policy (gammma = {}, epsilon = {}, alpha = {}, nb_iter = {}, max_steps = {}): {}", number_of_tests, gamma, epsilon, alpha, nb_iter, max_steps, durations_secretenv3.iter().sum::<u64>() / durations_secretenv3.len() as u64);
    println!("{}", output);
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();
    writeln!(file, "{}", output);

     */

}
