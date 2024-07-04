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
use crate::secret_env::lib_secret_env::getlib;

fn main() {

    getlib();
    // unsafe {
    //     #[cfg(target_os = "linux")]
    //         let path = "./libs/libsecret_envs.so";
    //     #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    //         let path = "./libs/libsecret_envs_intel_macos.dylib";
    //     #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    //         let path = "./libs/libsecret_envs.dylib";
    //     #[cfg(windows)]
    //         let path = "./libs/secret_envs.dll";
    //     let lib = libloading::Library::new(path).expect("Failed to load library");
    //
    //     println!("Secret env 0 functions available for Dynamic Programming ------------------------------------------------------");
    //
    //     let secret_env_0_num_states: libloading::Symbol<unsafe extern fn() -> usize> =
    //         lib.get(b"secret_env_0_num_states").expect("Failed to load function `secret_env_0_num_states`");
    //     dbg!(secret_env_0_num_states());
    //
    //     let secret_env_0_num_actions: libloading::Symbol<unsafe extern fn() -> usize> =
    //         lib.get(b"secret_env_0_num_actions").expect("Failed to load function `secret_env_0_num_actions`");
    //     dbg!(secret_env_0_num_actions());
    //
    //     let secret_env_0_num_rewards: libloading::Symbol<unsafe extern fn() -> usize> =
    //         lib.get(b"secret_env_0_num_rewards").expect("Failed to load function `secret_env_0_num_rewards`");
    //     dbg!(secret_env_0_num_rewards());
    //
    //     let secret_env_0_reward: libloading::Symbol<unsafe extern fn(usize) -> f32> =
    //         lib.get(b"secret_env_0_reward").expect("Failed to load function `secret_env_0_reward`");
    //
    //     for i in 0..secret_env_0_num_rewards() {
    //         dbg!(secret_env_0_reward(i));
    //     }
    //
    //     let secret_env_0_transition_probability: libloading::Symbol<unsafe extern fn(usize, usize, usize, usize) -> f32> =
    //         lib.get(b"secret_env_0_transition_probability").expect("Failed to load function `secret_env_0_transition_probability`");
    //     dbg!(secret_env_0_transition_probability(0, 0, 0, 0));
    //
    //     println!("Secret env 0 functions available for Monte Carlo or TD or Methods ------------------------------------------------------");
    //
    //     let secret_env_0_new: libloading::Symbol<unsafe extern fn() -> *mut c_void> =
    //         lib.get(b"secret_env_0_new").expect("Failed to load function `secret_env_0_new`");
    //     let env = secret_env_0_new();
    //     dbg!(env);
    //
    //     let secret_env_0_reset: libloading::Symbol<unsafe extern fn(*mut c_void)> =
    //         lib.get(b"secret_env_0_reset").expect("Failed to load function `secret_env_0_reset`");
    //     secret_env_0_reset(env);
    //
    //     let secret_env_0_state_id: libloading::Symbol<unsafe extern fn(*const c_void) -> usize> =
    //         lib.get(b"secret_env_0_state_id").expect("Failed to load function `secret_env_0_state_id`");
    //     dbg!(secret_env_0_state_id(env));
    //
    //     let secret_env_0_is_forbidden: libloading::Symbol<unsafe extern fn(*const c_void, usize) -> bool> =
    //         lib.get(b"secret_env_0_is_forbidden").expect("Failed to load function `secret_env_0_is_forbidden`");
    //     dbg!(secret_env_0_is_forbidden(env, 0));
    //
    //     let secret_env_0_is_game_over: libloading::Symbol<unsafe extern fn(*const c_void) -> bool> =
    //         lib.get(b"secret_env_0_is_game_over").expect("Failed to load function `secret_env_0_is_game_over`");
    //     dbg!(secret_env_0_is_game_over(env));
    //
    //     while !secret_env_0_is_game_over(env) {
    //         let secret_env_0_available_actions: libloading::Symbol<unsafe extern fn(*const c_void) -> *const usize> =
    //             lib.get(b"secret_env_0_available_actions").expect("Failed to load function `secret_env_0_available_actions`");
    //         let actions = secret_env_0_available_actions(env);
    //         dbg!(actions);
    //
    //         let secret_env_0_available_actions_len: libloading::Symbol<unsafe extern fn(*const c_void) -> usize> =
    //             lib.get(b"secret_env_0_available_actions_len").expect("Failed to load function `secret_env_0_available_actions_len`");
    //         dbg!(secret_env_0_available_actions_len(env));
    //
    //         // show all available actions
    //         for i in 0..secret_env_0_available_actions_len(env) {
    //             dbg!(*actions.add(i));
    //         }
    //
    //         let first_available_action = *actions;
    //
    //         let secret_env_0_available_actions_delete: libloading::Symbol<unsafe extern fn(*const usize, usize)> =
    //             lib.get(b"secret_env_0_available_actions_delete").expect("Failed to load function `secret_env_0_available_actions_delete`");
    //         secret_env_0_available_actions_delete(actions, secret_env_0_available_actions_len(env));
    //
    //         let secret_env_0_step: libloading::Symbol<unsafe extern fn(*mut c_void, usize)> =
    //             lib.get(b"secret_env_0_step").expect("Failed to load function `secret_env_0_step`");
    //         secret_env_0_step(env, first_available_action);
    //
    //         let secret_env_0_state_id: libloading::Symbol<unsafe extern fn(*const c_void) -> usize> =
    //             lib.get(b"secret_env_0_state_id").expect("Failed to load function `secret_env_0_state_id`");
    //         dbg!(secret_env_0_state_id(env));
    //     }
    //
    //     let secret_env_0_score: libloading::Symbol<unsafe extern fn(*const c_void) -> f32> =
    //         lib.get(b"secret_env_0_score").expect("Failed to load function `secret_env_0_score`");
    //     dbg!(secret_env_0_score(env));
    //
    //     let secret_env_0_delete: libloading::Symbol<unsafe extern fn(*mut c_void)> =
    //         lib.get(b"secret_env_0_delete").expect("Failed to load function `secret_env_0_delete`");
    //     secret_env_0_delete(env);
    //
    //
    //     let secret_env_0_from_random_state: libloading::Symbol<unsafe extern fn() -> *mut c_void> =
    //         lib.get(b"secret_env_0_from_random_state").expect("Failed to load function `secret_env_0_from_random_state`");
    //     let env = secret_env_0_from_random_state();
    //     dbg!(env);
    //
    //     secret_env_0_delete(env);
    //
    //     println!("Secret env 1 functions available for Dynamic Programming ------------------------------------------------------");
    //
    //     let secret_env_1_num_states: libloading::Symbol<unsafe extern fn() -> usize> =
    //         lib.get(b"secret_env_1_num_states").expect("Failed to load function `secret_env_1_num_states`");
    //     dbg!(secret_env_1_num_states());
    //
    //     let secret_env_1_num_actions: libloading::Symbol<unsafe extern fn() -> usize> =
    //         lib.get(b"secret_env_1_num_actions").expect("Failed to load function `secret_env_1_num_actions`");
    //     dbg!(secret_env_1_num_actions());
    //
    //     let secret_env_1_num_rewards: libloading::Symbol<unsafe extern fn() -> usize> =
    //         lib.get(b"secret_env_1_num_rewards").expect("Failed to load function `secret_env_1_num_rewards`");
    //     dbg!(secret_env_1_num_rewards());
    //
    //     let secret_env_1_reward: libloading::Symbol<unsafe extern fn(usize) -> f32> =
    //         lib.get(b"secret_env_1_reward").expect("Failed to load function `secret_env_1_reward`");
    //
    //     for i in 0..secret_env_1_num_rewards() {
    //         dbg!(secret_env_1_reward(i));
    //     }
    //
    //     let secret_env_1_transition_probability: libloading::Symbol<unsafe extern fn(usize, usize, usize, usize) -> f32> =
    //         lib.get(b"secret_env_1_transition_probability").expect("Failed to load function `secret_env_1_transition_probability`");
    //     dbg!(secret_env_1_transition_probability(0, 0, 0, 0));
    //
    //     println!("Secret env 1 functions available for Monte Carlo or TD Methods ------------------------------------------------------");
    //
    //     let secret_env_1_new: libloading::Symbol<unsafe extern fn() -> *mut c_void> =
    //         lib.get(b"secret_env_1_new").expect("Failed to load function `secret_env_1_new`");
    //     let env = secret_env_1_new();
    //     dbg!(env);
    //
    //     let secret_env_1_reset: libloading::Symbol<unsafe extern fn(*mut c_void)> =
    //         lib.get(b"secret_env_1_reset").expect("Failed to load function `secret_env_1_reset`");
    //     secret_env_1_reset(env);
    //
    //     let secret_env_1_state_id: libloading::Symbol<unsafe extern fn(*const c_void) -> usize> =
    //         lib.get(b"secret_env_1_state_id").expect("Failed to load function `secret_env_1_state_id`");
    //     dbg!(secret_env_1_state_id(env));
    //
    //     let secret_env_1_is_forbidden: libloading::Symbol<unsafe extern fn(*const c_void, usize) -> bool> =
    //         lib.get(b"secret_env_1_is_forbidden").expect("Failed to load function `secret_env_1_is_forbidden`");
    //     dbg!(secret_env_1_is_forbidden(env, 0));
    //
    //     let secret_env_1_is_game_over: libloading::Symbol<unsafe extern fn(*const c_void) -> bool> =
    //         lib.get(b"secret_env_1_is_game_over").expect("Failed to load function `secret_env_1_is_game_over`");
    //     dbg!(secret_env_1_is_game_over(env));
    //
    //     while !secret_env_1_is_game_over(env) {
    //         let secret_env_1_available_actions: libloading::Symbol<unsafe extern fn(*const c_void) -> *const usize> =
    //             lib.get(b"secret_env_1_available_actions").expect("Failed to load function `secret_env_1_available_actions`");
    //         let actions = secret_env_1_available_actions(env);
    //         dbg!(actions);
    //
    //         let secret_env_1_available_actions_len: libloading::Symbol<unsafe extern fn(*const c_void) -> usize> =
    //             lib.get(b"secret_env_1_available_actions_len").expect("Failed to load function `secret_env_1_available_actions_len`");
    //         dbg!(secret_env_1_available_actions_len(env));
    //
    //         // show all available actions
    //         for i in 0..secret_env_1_available_actions_len(env) {
    //             dbg!(*actions.add(i));
    //         }
    //
    //         let first_available_action = *actions;
    //
    //         let secret_env_1_available_actions_delete: libloading::Symbol<unsafe extern fn(*const usize, usize)> =
    //             lib.get(b"secret_env_1_available_actions_delete").expect("Failed to load function `secret_env_1_available_actions_delete`");
    //         secret_env_1_available_actions_delete(actions, secret_env_1_available_actions_len(env));
    //
    //         let secret_env_1_step: libloading::Symbol<unsafe extern fn(*mut c_void, usize)> =
    //             lib.get(b"secret_env_1_step").expect("Failed to load function `secret_env_1_step`");
    //         secret_env_1_step(env, first_available_action);
    //
    //         let secret_env_1_state_id: libloading::Symbol<unsafe extern fn(*const c_void) -> usize> =
    //             lib.get(b"secret_env_1_state_id").expect("Failed to load function `secret_env_1_state_id`");
    //         dbg!(secret_env_1_state_id(env));
    //     }
    //
    //     let secret_env_1_score: libloading::Symbol<unsafe extern fn(*const c_void) -> f32> =
    //         lib.get(b"secret_env_1_score").expect("Failed to load function `secret_env_1_score`");
    //     dbg!(secret_env_1_score(env));
    //
    //     let secret_env_1_delete: libloading::Symbol<unsafe extern fn(*mut c_void)> =
    //         lib.get(b"secret_env_1_delete").expect("Failed to load function `secret_env_1_delete`");
    //     secret_env_1_delete(env);
    //
    //     let secret_env_1_from_random_state: libloading::Symbol<unsafe extern fn() -> *mut c_void> =
    //         lib.get(b"secret_env_1_from_random_state").expect("Failed to load function `secret_env_1_from_random_state`");
    //     let env = secret_env_1_from_random_state();
    //     dbg!(env);
    //
    //     secret_env_1_delete(env);
    // };
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
    // let res = lineworld.policy_iteration(0.0001f32, 0.999f32);
    //
    // lineworld.run_game_vec(res);
    // // println!("{:?}",res);
    // let mut grid = Env::GridWorld::GridWorld::init();

    // let mut shifumi = Env::Shifumi::Shifumi::init();
    // for _ in 0..10 {
    //     println!("coucou");
    //     let res = grid.monte_carlo_exploring_starts(0.99999f32, 10000, 10);
    //     println!("{:?}", res);
    // }
    //println!("{:?}", res);

    // for _ in 0..10 {
    //     let res = shifumi.monte_carlo_exploring_starts(0.99999f32, 10000, 10);
    //     shifumi.run_game_hashmap(res);
    //     shifumi.reset();
    // }
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
