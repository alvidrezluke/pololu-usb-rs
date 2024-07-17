use crate::robot::desc::{Direction, Motor};
use crate::robot::{math, LOWER_LEG_LENGTH, UPPER_LEG_LENGTH};
use libm::{acos, atan2};
use ndarray::{array, Array1};

/*
    end_pos: end effector desired position
    dir:     Direction of servo
    returns angle needed for servo
*/
pub fn calc_servo_pos(end_pos: &Array1<f64>, dir: Direction) -> f64 {
    let temp: Array1<f64> = array![end_pos[0], end_pos[1]];
    let phi = (math::l2_norm(end_pos.clone())
        + (LOWER_LEG_LENGTH.powf(2.0) - UPPER_LEG_LENGTH.powf(2.0)))
        / (2.0 * LOWER_LEG_LENGTH * math::l2_norm(temp));

    return match dir {
        Direction::Left => atan2(end_pos[2], end_pos[1]) + acos(phi),
        Direction::Right => atan2(end_pos[2], end_pos[1]) - acos(phi),
    };
}
/*
/// Computes Inverse kinematics
    target: center of platform
    [x, y, z, roll, pitch, yaw]
    motors: vector of motors
    returns vector of angles

    c = centroid frame
    Given target_pos(x_c, y_c, z_c) and orientation()
    Inverse robot procedure:
    1) find the platform corners in platform frame (p_i^c)
    2) find the platform transformation relative to world frame (T_c^0)
    3) find platform corners in world frame (p_i^0 = T_c^0 p_i^c)
    4) find the Euclidean distance base to platform

    S_i = p_c + R_c * b_i - a_i
*/
pub fn inverse_kinematics(_target: Array1<f64>, _motors: Array1<Motor>) {}
