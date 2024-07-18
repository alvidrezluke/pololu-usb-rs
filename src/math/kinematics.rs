use libm::{acos, atan2};
use ndarray::{arr2, array, Array1, Array2};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::{Decimal, MathematicalOps};
use rust_decimal_macros::dec;
// use rust_decimal::Decimal;
use crate::param::{Direction, Leg};

// in mm
const LOWER_LEG_LENGTH: f64 = 119.0;
const UPPER_LEG_LENGTH: f64 = 21.1;

/*
/// Truncates the float to whatever decimal is specified
    x:          input number
    decimals:   number of decimals returned
*/
pub(crate) fn precision_f64(x: f64, decimals: u32) -> f64 {
    if x == 0. || decimals == 0 {
        0.
    } else {
        let shift = decimals as i32 - x.abs().log10().ceil() as i32;
        let shift_factor = 10_f64.powi(shift);

        (x * shift_factor).round() / shift_factor
    }
}

pub(crate) fn slice_to_dec(vector: [f64; 3]) -> [Decimal; 3] {
    let mut v: [Decimal; 3] = [dec!(0); 3];
    for (i, x) in vector.iter().enumerate() {
        v[i] = Decimal::from_f64(*x).unwrap();
    }
    return v;
}
/*
/// Gets Euclidean distance of a vector
    x: vector
*/

fn l2_norm(x: Array1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

/*
/// Calculates the rotation matrix for platform
     gamma: angle of roll
     beta : angle of pitch
     alpha: angle of yaw
*/

pub fn calc_rot_matrix(gamma: Decimal, beta: Decimal, alpha: Decimal) -> Array2<Decimal> {
    arr2(&[
        [
            (alpha.cos() * beta.cos()).round_dp(4),
            (alpha.cos() * beta.sin() * gamma.sin() - gamma.sin() * alpha.sin()).round_dp(4),
            (alpha.sin() * gamma.sin() + alpha.cos() * gamma.cos() * beta.sin()).round_dp(4),
        ],
        [
            (beta.cos() * alpha.sin()).round_dp(4),
            (alpha.cos() * gamma.cos() + alpha.sin() * beta.sin() * gamma.sin()).round_dp(4),
            (gamma.cos() * alpha.sin() * beta.sin() - alpha.cos() * gamma.sin()).round_dp(4),
        ],
        [
            (-beta.sin()).round_dp(4),
            (beta.cos() * gamma.sin()).round_dp(4),
            (beta.cos() * gamma.cos()).round_dp(4),
        ],
    ])
}

/*
    end_pos: end effector desired position
    dir:     Direction of servo
    returns angle needed for servo
*/
pub fn calc_servo_pos(end_pos: &Array1<Decimal>, dir: &Direction) -> f64 {
    let ep: Array1<f64> = array![
        end_pos[0].to_f64().unwrap(),
        end_pos[1].to_f64().unwrap(),
        end_pos[2].to_f64().unwrap()
    ];
    let temp: Array1<f64> = array![ep[0], ep[1]];
    let phi: f64 = (l2_norm(ep.clone())
        + (LOWER_LEG_LENGTH.powf(2.0) - UPPER_LEG_LENGTH.powf(2.0)))
        / (2.0 * LOWER_LEG_LENGTH * l2_norm(temp));
    return match dir {
        Direction::Left => atan2(ep[2], ep[1]) + acos(phi % 1.0),
        Direction::Right => atan2(ep[2], ep[1]) - acos(phi % 1.0),
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

    a_i: the vector from the world frame to the pos of motor
    S_i = p_c + R_c * b_i - a_i
*/
//ArrayBase<OwnedRepr<Decimal>, Ix1>
pub fn inverse_kinematics(lengths: Vec<Array1<Decimal>>, legs: [Leg; 6]) -> Array1<f64> {
    let q: Array1<f64> = lengths
        .iter()
        .zip(legs.iter())
        .map(|(length, leg)| calc_servo_pos(length, &leg.motor.direction))
        .collect::<Array1<f64>>();
    q
}
