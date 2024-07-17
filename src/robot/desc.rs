use ndarray::{array, Array1};
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
    base_pos: Array1<f64>,
    direction: Direction,
    end_effector_offset: Array1<f64>,
}

/*
    centroid_pos: with respect to the world frame
    orientation: with respect to the world frame
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Platform {
    centroid_pos: Array1<f64>,
    orientation: Array1<f64>,
}

/*
    Group together parts
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Leg {
    motor: Motor,
    top_leg_length: f64,
    bottom_leg_length: f64,
}

/*
    Whole Robot
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct RobotConfig {
    platform: Platform,
    legs: Vec<Leg>,
}

pub fn get_robot_params() {
    let file = std::fs::read_to_string("description/robot_param.yml")
        .unwrap()
        .to_string();

    let data: RobotConfig = serde_yml::from_str(&file).expect("YAML was not well-formatted");
    println!("{:?}", data)
}

// top plate: radius of 150/2 mm
// spaced evenly 120 deg apart
pub fn set_robot_params() {
    let p = Platform {
        centroid_pos: array![1.0, 2.0, 3.0],
        orientation: array![1.0, 2.0, 0.0],
    };
    let m1 = Motor {
        base_pos: array![1.0, 2.0, 3.0],
        direction: Direction::Left,
        end_effector_offset: array![1.0, 2.0, 0.0],
    };
    let m2 = Motor {
        base_pos: array![2.0, 2.0, 2.0],
        direction: Direction::Right,
        end_effector_offset: array![9.0, 2.0, 5.0],
    };
    let l1 = Leg {
        motor: m1,
        top_leg_length: 175.0,
        bottom_leg_length: 41.14,
    };
    let l2 = Leg {
        motor: m2,
        top_leg_length: 175.0,
        bottom_leg_length: 41.14,
    };

    let config = RobotConfig {
        platform: p,
        legs: vec![l1, l2],
    };
    generate_file!("yaml", &config, |content| {
        fs::write("description/robot_param.yml", content)
    });
}
