mod kinematics;

use nalgebra::{Matrix, Vector3, Vector6};
use pololu_usb_rs::serial;
use crate::kinematics::{Direction, Motor};
use crate::kinematics::Direction::Right;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let path: &str = "COM6";
    let baud_rate: u32 = 9600;

}
// start
