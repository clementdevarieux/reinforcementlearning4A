/*
use rand::Rng;
use std::collections::HashMap;
use rand::distributions::{Distribution, Uniform};
use std::ffi::c_void;

pub struct SecretEnv0 {
    pub agent_pos: i32,
    pub num_states: i32,
    pub num_actions: i32,
    pub S: Vec<i32>,
    pub A: Vec<i32>,
    pub R: Vec<i32>,
    pub T: Vec<i32>,
    pub p: Vec<Vec<Vec<Vec<f32>>>>
}

impl SecretEnv0 {
    pub fn init() -> *mut c_void {
        let secret_env_0_new: libloading::Symbol<unsafe extern fn() -> *mut c_void> =
            unsafe { LIB.get(b"secret_env_0_new") }.expect("Failed to load function `secret_env_0_new`");
        unsafe {
            let env = secret_env_0_new();
            env
        }
    }

}

 */