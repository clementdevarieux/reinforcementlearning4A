use std::fmt;
use rand::prelude::*;
use std::iter::IntoIterator;
use rand::seq::SliceRandom;
use std::cmp;
use std::f32;

pub fn policy_iteration(S: Vec<i32>,
                        A:Vec<i32>,
                        R:Vec<i32>,
                        T:Vec<i32>,
                        p:Vec<Vec<Vec<Vec<f32>>>>,
                        theta: f32,
                        gamma: f32) -> Vec<i32> {

    let len_S= S.clone().len();
    let mut rng = rand::thread_rng();
    let mut V: Vec<f32> = Vec::with_capacity(len_S);

    for _ in 0..len_S {
        V.push(rng.gen_range(0f32..1f32));
    }

    let mut Pi= Vec::with_capacity(len_S);

    for _ in 0..len_S {
        let random_index = rng.gen_range(0..A.len());
        Pi.push(A[random_index]); // mettre des valeurs aléatoires de A
    }

    loop {
        // policy evaluation
        loop {
            let mut delta: f32 = 0.0;
            for s in 0..S.len() {
                let mut v = V[s];
                let mut total: f32 = 0f32;
                for s_p in 0..S.len() {
                    for r in 0..R.len() {
                        total = total + p[s][Pi[s] as usize][s_p][r] * (R[r] as f32 + gamma * V[s_p]);
                    }
                }
                V[s] = total;
                delta = delta.max((v - V[s]).abs());
            }
            if delta < theta {
                break;
            }
        }

        let mut policy_stable = true;

        for s in 0..S.len() {
            if T.contains(&(s as i32)) {
                continue;
            }

            let mut old_action = Pi[s];

            let mut argmax_a: i32 = -9999999;
            let mut max_a: f32 = -9999999.0;

            for a in 0..A.len() {
                let mut total: f32 = 0.0;

                for s_p in 0..S.len() {
                    for r_index in 0..R.len() {
                        total += p[s][a][s_p][r_index] * (R[r_index] as f32 + gamma * V[s_p])
                    }
                }

                if argmax_a == -9999999 || total >= max_a {
                    argmax_a = a as i32;
                    max_a = total;
                }
            }

            Pi[s] = argmax_a;

            if old_action != Pi[s] {
                policy_stable = false;
            }
        }

        if policy_stable {
            break
        }
    }
    return Pi
}

pub fn value_iteration(S: Vec<i32>,
                       A:Vec<i32>,
                       R:Vec<i32>,
                       T:Vec<i32>,
                       p:Vec<Vec<Vec<Vec<f32>>>>,
                       theta: f32,
                       gamma: f32) -> Vec<i32> {

    let len_S= S.clone().len();
    let mut rng = rand::thread_rng();
    let mut V: Vec<f32> = Vec::with_capacity(len_S);

    for i in 0..len_S {
        if T.contains(&(i as i32)) {
            V.push(0f32);
        } else {
            V.push(rng.gen_range(0f32..1f32));
        }
    }


    loop {
        let mut delta = 0f32;
        for s in 0..len_S {
            if T.contains(&(s as i32)) {
                continue;
            }

            let v = V[s];
            let mut max_value: f32 = -9999f32;
            for a in 0..A.len() {
                let mut total: f32 = 0.0;
                for s_p in 0..S.len() {
                    for r in 0..R.len() {
                        total += p[s][a][s_p][r] * (R[r] as f32 + gamma * V[s_p]);
                    }
                }
                if total > max_value {
                    max_value = total;
                }
            }

            V[s] = max_value;
            delta = delta.max((v - V[s]).abs());
        }
        if delta < theta {
            break;
        }
    }

    let mut Pi: Vec<i32> = vec![-1; len_S];
    for s in 0..S.len() {
        if T.contains(&(s as i32)) {
            continue;
        }

        let mut argmax_a: i32 = -1;
        let mut max_value: f32 = -99999f32;

        for a in 0..A.len() {
            let mut total: f32 = 0.0;
            for s_p in 0..S.len() {
                for r in 0..R.len() {
                    total += p[s][a][s_p][r] * (R[r] as f32 + gamma * V[s_p]);
                }
            }

            if total > max_value {
                max_value = total;
                argmax_a = a as i32;
            }
        }

        Pi[s] = argmax_a;
    }

    Pi
}

pub fn monte_carlo_es(S: Vec<i32>,
                      A:Vec<i32>,
                      R:Vec<i32>,
                      T:Vec<i32>,
                      p:Vec<Vec<Vec<Vec<f32>>>>,
                      theta: f32,
                      gamma: f32) -> Vec<i32> {

  pass
}