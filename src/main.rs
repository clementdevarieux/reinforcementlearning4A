mod environments;
mod algorithms;
mod Env;

use std::fmt;
use rand::prelude::*;
use std::iter::IntoIterator;
use rand::seq::SliceRandom;

fn main() {

    let mut lineworld = Env::LineWorld::LineWorld::init();

    lineworld.step(0);

    println!("{:?}", lineworld.state_desc());

    lineworld.step(0);

    println!("{:?}", lineworld.state_desc());

    println!("{}", lineworld.agent_pos);

    println!("{:?}", lineworld.available_actions());

    lineworld.display();

    // println!("{:?}", lineworld.p);


    // let (S, A, R, T, p) = environments::shifumi();
    //
    // // println!("{:?}", S)
    //
    // let res = algorithms::policy_iteration(S, A, R, T, p, 0.0001f32, 0.999f32);
    //
    // println!("{:?}", res);
    //
    // let (S_grid, A_grid, R_grid, T_grid, p_grid) = environments::montyhall_standard();
    // let grid_res = algorithms::value_iteration(S_grid,A_grid,R_grid,T_grid,p_grid, 0.0001f32, 0.999f32);
    //
    // println!("{:?}", grid_res)
}
