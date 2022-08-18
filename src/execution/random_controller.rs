use super::{polled_controller::PolledController, Facing};

use rand::Rng;

#[derive(Default)]
pub struct RandomController;

impl<Robot: crate::execution::Robot> PolledController<Robot> for RandomController {

    fn control_robot(&mut self, robot: &mut Robot) {
    
        let mut random_gen = rand::thread_rng();
        let direction = match random_gen.gen_range(0..=3) {
            0 => Facing::Ahead,
            1 => Facing::Left,
            2 => Facing::Right,
            _ => Facing::Behind,
        };

        robot.face(direction);
    }
}
