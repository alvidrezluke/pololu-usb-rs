use libm::{cos, sin};
use ndarray::{arr2, Array1, Array2};

/*
/// Truncates the float to whatever decimal is specified
    x:          input number
    decimals:   number of decimals returned
*/
fn precision_f64(x: f64, decimals: u32) -> f64 {
    if x == 0. || decimals == 0 {
        0.
    } else {
        let shift = decimals as i32 - x.abs().log10().ceil() as i32;
        let shift_factor = 10_f64.powi(shift);

        (x * shift_factor).round() / shift_factor
    }
}
/*
/// Gets Euclidean distance of a vector
    x: vector
*/

pub fn l2_norm(x: Array1<f64>) -> f64 {
    x.dot(&x).sqrt()
}

/*
/// Calculates the rotation matrix for platform
     gamma: angle of roll
     beta : angle of pitch
     alpha: angle of yaw
*/
pub fn calc_rot_matrix(gamma: f64, beta: f64, alpha: f64) -> Array2<f64> {
    arr2(&[
        [
            precision_f64(cos(alpha) * cos(beta), 4),
            precision_f64(
                cos(alpha) * sin(beta) * sin(gamma) - cos(gamma) * sin(alpha),
                4,
            ),
            precision_f64(
                sin(alpha) * sin(gamma) + cos(alpha) * cos(gamma) * sin(beta),
                4,
            ),
        ],
        [
            precision_f64(cos(beta) * sin(alpha), 4),
            precision_f64(
                cos(alpha) * cos(gamma) + sin(alpha) * sin(beta) * sin(gamma),
                4,
            ),
            precision_f64(
                cos(gamma) * sin(alpha) * sin(beta) - cos(alpha) * sin(gamma),
                4,
            ),
        ],
        [
            precision_f64(-sin(beta), 4),
            precision_f64(cos(beta) * sin(gamma), 4),
            precision_f64(cos(beta) * cos(gamma), 4),
        ],
    ])
}
