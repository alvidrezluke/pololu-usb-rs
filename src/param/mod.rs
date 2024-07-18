use crate::math::kinematics::slice_to_dec;
use ndarray::{array, Array1};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use serde_yml::generate_file;
use std::fs;
/*
/// Direction servo arm is set up
*/
#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    Left,
    Right,
}

/*
    dir:                Direction servo arm is set up
    base_pos:           Center of servo rotation
    end_effector:       Point relative to the centroid of the top plate
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Motor {
    id: u8,
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

pub fn get_robot_params(path: &str) -> RobotConfig {
    let file = std::fs::read_to_string(path).unwrap().to_string();

    let data: RobotConfig = serde_yml::from_str(&file).expect("YAML was not well-formatted");
    // println!("{:?}", data);
    return data;
}

pub fn set_robot_params() {
    /*
        Temporary
    */
    let config: RobotConfig = RobotConfig {
        platform: Platform {
            centroid_pos: slice_to_dec([0.0, 0.0, 170.07]),
            orientation: slice_to_dec([0.0, 0.0, 0.0]),
            corners: [
                Corner {
                    id: 0,
                    pos: slice_to_dec([10.0, -75.0, 0.0]),
                },
                Corner {
                    id: 1,
                    pos: slice_to_dec([10.0, -75.0, 0.0]),
                },
                Corner {
                    id: 2,
                    pos: slice_to_dec([10.0, -75.0, 0.0]),
                },
                Corner {
                    id: 3,
                    pos: slice_to_dec([10.0, -75.0, 0.0]),
                },
                Corner {
                    id: 4,
                    pos: slice_to_dec([10.0, -75.0, 0.0]),
                },
                Corner {
                    id: 5,
                    pos: slice_to_dec([10.0, -75.0, 0.0]),
                },
            ],
        },
        legs: [
            Leg {
                motor: Motor {
                    id: 0,
                    base_pos: slice_to_dec([28.30, -94.45, 10.0]),
                    direction: Direction::Right,
                    // end_effector_offset: [10.0, -75.0, 0.0],
                },
                top_leg_length: dec!(119.0),
                bottom_leg_length: dec!(21.10),
            },
            Leg {
                motor: Motor {
                    id: 1,
                    base_pos: slice_to_dec([95.95, 22.72, 10.0]),
                    direction: Direction::Left,
                    // end_effector_offset: [69.9519, 28.8397, 0.0],
                },
                top_leg_length: dec!(119.0),
                bottom_leg_length: dec!(21.10),
            },
            Leg {
                motor: Motor {
                    id: 2,
                    base_pos: slice_to_dec([67.65, 71.73, 10.0]),
                    direction: Direction::Right,
                    // end_effector_offset: [59.9519, -46.1603, 0.0],
                },
                top_leg_length: dec!(119.0),
                bottom_leg_length: dec!(21.10),
            },
            Leg {
                motor: Motor {
                    id: 3,
                    base_pos: slice_to_dec([-67.65, 71.73, 10.0]),
                    direction: Direction::Left,
                    // end_effector_offset: [-59.9519, 46.1603, 0.0],
                },
                top_leg_length: dec!(119.0),
                bottom_leg_length: dec!(21.10),
            },
            Leg {
                motor: Motor {
                    id: 4,
                    base_pos: slice_to_dec([-95.95, 22.72, 10.0]),
                    direction: Direction::Right,
                    // end_effector_offset: [-69.9519, 28.8397, 0.0],
                },
                top_leg_length: dec!(119.0),
                bottom_leg_length: dec!(21.10),
            },
            Leg {
                motor: Motor {
                    id: 5,
                    base_pos: slice_to_dec([-28.30, -94.45, 10.0]),
                    direction: Direction::Left,
                    // end_effector_offset: [-10.0, -75.0, 0.0],
                },
                top_leg_length: dec!(119.0),
                bottom_leg_length: dec!(21.10),
            },
        ],
    };
    generate_file!("yaml", &config, |content| {
        fs::write("description/robot_param.yml", content)
    });
}
