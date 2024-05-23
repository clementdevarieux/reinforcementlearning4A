use std::fmt;
use rand::prelude::*;
use std::iter::IntoIterator;
use rand::seq::SliceRandom;

pub fn policy_iteration(S: Vec<i32>, A:Vec<i32>, R:Vec<i32>, T:Vec<i32>, p:Vec<Vec<Vec<Vec<f32>>>>, theta: f32, gamma: f32) {

    let len_S= S.clone().len();
    let mut rng = rand::thread_rng();
    let mut V: Vec<f32> = Vec::with_capacity(len_S);

    for _ in 0..len_S {
        V.push(rng.gen_range(0f32..1f32));
    }

    let mut Pi= Vec::with_capacity(len_S);

    for _ in 0..len_S {
        Pi.push(A.choose(&mut rand::thread_rng())); // mettre des valeurs aléatoires de A
    }

    while true {
        // policy evaluation
        while true {
            let mut delta = 0;
            for s in 0..S.len() {
                let mut v = V[s];
                let mut total: f32 = 0f32;
                for s_p in 0..S.len() {
                    for r in 0..R.len() {
                        total = total + p[s][Pi[s]][s_p][r] * (R[r] + gamma * V[s_p]);
                    }
                }
                V[s] = total;
                // delta = // np.maximum(delta, np.abs(v - V[s]))
            }
            false;
        }
        false;
    }
}