#![warn(missing_docs)]

mod debug;
mod gui;
mod math;
mod param;
mod robot;

use crate::robot::get_robot_params;

use libm::cos;
use ndarray::{arr1, Array, Array1, Array2, ArrayBase, ArrayView, Dim, OwnedRepr};
use rust_decimal_macros::dec;
use std::f64::consts::PI;

const FILE_PATH: &str = "description/robot_param.yml";
fn main() {
    // let path: &str = "COM6";
    // let baud_rate: u32 = 9600;
    //
    // let robot = get_robot_params(FILE_PATH);
    // let t = math::inverse_kinematics(robot);
    // let config: RobotConfig = get_robot_params();crate::math::kinematics::
    // inverse_kinematics(robot);
    // let mut a = Array::zeros((1, 0));
}
