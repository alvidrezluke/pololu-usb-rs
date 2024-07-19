use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use crate::param;

/*
/// Direction servo arm is set up
*/
#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Serialize, Deserialize, Debug)]
enum MotorID{
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}
/*
    dir:                Direction servo arm is set up
    base_pos:           Center of servo rotation
    end_effector:       Point relative to the centroid of the top plate
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Motor {
    pub(crate) id: u8,
    pub(crate) base_pos: [Decimal; 3],
    pub(crate) direction: Direction,
    // end_effector_offset: [f64; 3],
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Corner {
    pub(crate) pos: [Decimal; 3],
    pub(crate) id: u8,
}
/*
    centroid_pos: with respect to the world frame
    orientation: with respect to the world frame
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Platform {
    pub(crate) corners: [Corner; 6],
    pub(crate) centroid_pos: [Decimal; 3],
    pub(crate) orientation: [Decimal; 3],
}

/*
    Group together parts
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Leg {
    pub(crate) motor: Motor,
    pub(crate) top_leg_length: Decimal,
    pub(crate) bottom_leg_length: Decimal,
}

/*
    Whole Robot
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RobotConfig {
    pub(crate) platform: Platform,
    pub(crate) legs: [Leg; 6],
}

impl RobotConfig {
    pub fn get_corners(&self) -> &[Corner; 6] {
        &self.platform.corners
    }
    pub fn get_center(&self) -> [Decimal; 3] {
        self.platform.centroid_pos
    }
    // pub fn init_pos(&self) ->
}




