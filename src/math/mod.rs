pub(crate) mod kinematics;

use crate::debug::print_type_of;
use ndarray::linalg::Dot;
use ndarray::{arr1, Array, Array1, Array2, ArrayBase, ArrayView, Ix1, OwnedRepr};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use crate::robot::{Corner, RobotConfig};

// pub fn calc_rot_matrix(gamma: Decimal, beta: Decimal, alpha: Decimal) -> Array2<Decimal> {
//     kinematics::calc_rot_matrix(gamma, beta, alpha)
// }
// todo!()
pub fn inverse_kinematics(robot: RobotConfig) {
    let corners: &[Corner; 6] = &robot.platform.corners;
    let r: Array2<Decimal> = kinematics::calc_rot_matrix(
        robot.platform.orientation[0],
        robot.platform.orientation[1],
        robot.platform.orientation[2],
    );
    // let t = dec!(0.0);
    let d: Vec<Array1<Decimal>> = corners
        .iter()
        .zip(robot.legs.iter())
        .map(|(corner, leg)| {
            arr1(&robot.get_center()) + r.dot(&arr1(&corner.pos)) - arr1(&leg.motor.base_pos)
        })
        .collect();
    // print_type_of(&t);
    // print_type_of(&d);
    // println!("{:#?}", d);
    println!("{:?}", kinematics::inverse_kinematics(d, robot.legs))
}
