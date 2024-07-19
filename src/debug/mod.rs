use rust_decimal_macros::dec;
use crate::math::kinematics::slice_to_dec;
use crate::robot::{Corner, Direction, Leg, Motor, Platform, RobotConfig};

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn test_set_robot(){
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
    crate::param::set_robot_params(config);
}
