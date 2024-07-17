#![warn(missing_docs)]
mod debug;
mod robot;

use crate::robot::{calc_rot_matrix, set_robot_params};
use ndarray::Array2;
use std::f64::consts::PI;

fn main() {
    // let path: &str = "COM6";
    // let baud_rate: u32 = 9600;
    let roll: f64 = PI / 4.0;
    let pitch: f64 = PI / 3.0;
    let yaw: f64 = PI / 2.0;
    let _a: Array2<f64> = calc_rot_matrix(roll, pitch, yaw);
    // println!("{:?}", a);
    // let g: Array1<f64> = array![22.12, 55.14];
    // println!("{:?}", g)
    set_robot_params();
    // get_robot_params();
}
// start
