mod desc;
mod kinematics;
mod math;

use crate::robot::desc::Motor;

use ndarray::{Array1, Array2};

// in mm
const LOWER_LEG_LENGTH: f64 = 41.14;
const UPPER_LEG_LENGTH: f64 = 175.0;
/*
/// Loads robot parameters from .yml file
    todo!()
*/
pub fn get_robot_params() {
    desc::get_robot_params();
}

/*
/// Sets robot parameters based on app
    todo!()
*/
pub fn set_robot_params() {
    desc::set_robot_params();
}
/*
    gamma: angle of roll
    beta : angle of pitch
    alpha: angle of yaw
*/
pub fn calc_rot_matrix(gamma: f64, beta: f64, alpha: f64) -> Array2<f64> {
    math::calc_rot_matrix(gamma, beta, alpha)
}

/*
/// Computes Inverse kinematics
    target: center of platform
    [x, y, z, roll, pitch, yaw]
    motors: vector of motors
    returns vector of angles
*/
pub fn inverse_kinematics(target: Array1<f64>, motors: Array1<Motor>) {
    kinematics::inverse_kinematics(target, motors)
}
