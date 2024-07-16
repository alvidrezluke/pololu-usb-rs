use libm::{acosf, atan2f};
use nalgebra::{Vector2, Vector3, Vector6};

const LOWER_LEG_LENGTH: f32 = 2.5;
const UPPER_LEG_LENGTH: f32 = 4.0;

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Motor {
    pub(crate) direction: Direction,
    pub(crate) end_effector_offset: Vector3<f32>,
    pub(crate) angle: f32,
}

/*
    end_pos: end effector desired position
    dir:     Direction of servo
    returns angle needed for servo
*/
pub fn calc_servo_pos(end_pos: &Vector3<f32>, dir: Direction) -> f32 {
    let temp: Vector2<f32> = Vector2::new(end_pos.x, end_pos.y);
    let phi = (end_pos.norm().powf(2.0)
        + (LOWER_LEG_LENGTH.powf(2.0) - UPPER_LEG_LENGTH.powf(2.0)))
        / (2.0 * LOWER_LEG_LENGTH * temp.norm());

    return match dir {
        Direction::Left => {
            atan2f(end_pos.z, end_pos.y) + acosf(phi)
        },
        Direction::Right => {
            atan2f(end_pos[(2, 0)], end_pos[(1, 0)]) - acosf(phi)
        }
    }
}

/*
    end_pos: center of platform
    motors: vector of motors
    returns angle needed for servo
*/

// 1 2 3 4 5 6
// 1 2 3 4 5 6
pub fn calc_target_pos(end_pos: Vector3<f32>, motors:Vector6<Motor>) -> Vector6<Vector3<f32>> {
    Vector6::new(
        end_pos - motors[0].end_effector_offset,
        end_pos - motors[1].end_effector_offset,
        end_pos - motors[2].end_effector_offset,
        end_pos - motors[3].end_effector_offset,
        end_pos - motors[4].end_effector_offset,
        end_pos - motors[5].end_effector_offset,
    )
}

/*
    target: center of platform
    motors: vector of motors
    returns vector of angles
*/
pub fn inverse_kinematics(target:Vector3<f32>, motors:Vector6<Motor>){
    let end_positions:Vector6<Vector3<f32>> = calc_target_pos(target, motors);
    let angles:Vector6<f32> = Vector6::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    for (i, pos) in end_positions.iter().enumerate(){
        // have to rewrite
        angles[i] = calc_servo_pos(pos, Direction::Right);
    }

}
