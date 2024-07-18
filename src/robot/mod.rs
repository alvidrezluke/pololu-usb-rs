use crate::param::RobotConfig;

use crate::param;
/*
/// Loads robot parameters from .yml file
*/
pub fn get_robot_params(path: &str) -> RobotConfig {
    param::get_robot_params(path)
}

/*
/// Sets robot parameters based on app
    todo!()
*/
pub fn set_robot_params() {
    param::set_robot_params();
}
/*
    gamma: angle of roll
    beta : angle of pitch
    alpha: angle of yaw
*/

/*
/// Computes Inverse kinematics
    target: center of platform
    [x, y, z, roll, pitch, yaw]
    motors: vector of motors
    returns vector of angles
*/

// pub fn inverse_kinematics(robot_config: RobotConfig) {
//     // kinematics::inverse_kinematics(robot_config)
//     println!("{:?}", robot_config.get_corners())
// }
