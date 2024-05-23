mod environments;
mod algorithms;

use std::fmt;
use rand::prelude::*;
use std::iter::IntoIterator;
use rand::seq::SliceRandom;

fn main() {

    let (S, A, R, T, p) = environments::grid_world();

    algorithms::policy_iteration(S, A, R, T, p, 0.0001f32, 0.999f32)

}
