mod environments;
mod algorithms;

use std::fmt;
use rand::prelude::*;
use std::iter::IntoIterator;
use rand::seq::SliceRandom;
use libloading::{Library, Symbol};

fn main() {

    let (S, A, R, T, p) = environments::line_world();

    let res = algorithms::monte_carlo_exploring_starts(S, A, R, T, p, 0.0001f32, 0.999f32,10,10);

    println!("{:?}", res);
}
