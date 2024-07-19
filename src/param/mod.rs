use serde_yml::generate_file;
use std::fs;
use crate::robot::RobotConfig;

pub fn get_robot_params(path: &str) -> RobotConfig {
    let file = std::fs::read_to_string(path).unwrap().to_string();

    let data: RobotConfig = serde_yml::from_str(&file).expect("YAML was not well-formatted");
    // println!("{:?}", data);
    return data;
}

pub fn set_robot_params(config:RobotConfig) {
    generate_file!("yaml", &config, |content| {
        fs::write("description/robot_param.yml", content)
    });
}
