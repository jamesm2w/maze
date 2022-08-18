use super::{Controller, Robot, private, TileType};

use crate::execution::Maze;

/// A Polled Controller is what most people want to be writing
/// where the controller has a function which gets repeatedly called for each movement
/// 
pub trait PolledController<R: Robot> {

    fn control_robot(&mut self, robot: &mut R);

    fn reset(&self) { }
}

/// The wrapper implements the normal functionality of the controller for the Polled Controller
/// so we just need a PolledController struct to create this wrapping
pub struct PolledControllerWrapper<R: Robot, P: PolledController<R> + Default>{
    controller: Box<P>,
    robot: Option<R>,
    active: bool,
    delay: i32,
    callback: Box<dyn Fn(&R) -> ()>
}

impl <R: Robot, P: PolledController<R> + Default> PolledControllerWrapper<R, P> {
    pub fn new(controller: P) -> Self {
        PolledControllerWrapper {controller: Box::new(controller), robot: None, active: false, delay: 0, callback: Box::new(|_| {})}
    }

    pub fn set_poll_callback(&mut self, cb: Box<dyn Fn(&R) -> ()>) {
        self.callback = cb;
    } 
}

impl <R: Robot<Tiles=K> + private::Robot, P: PolledController<R> + Default, K: TileType + Default> Controller<R, K> for PolledControllerWrapper<R, P> {

    /// Set the robot the controller operates on
    fn set_robot(&mut self, robot: R) {
        self.robot = Some(robot);
    }

    fn set_maze(&mut self, maze: Maze<K>) {
        let maze = Box::new(maze);
    
        if let Some(robot) = &mut self.robot {
            robot.set_maze(maze);
        }
    }

    /// Called when the controller is started
    fn start(&mut self) {
        self.active = true;

        if let Some(robot) = &mut self.robot {

            while !robot.get_location().eq(&robot.get_goal_location()) && self.active {

                self.controller.control_robot(robot);
                
                robot.advance(); // TODO: Not sure the robot obj should be calling "advance"

                (self.callback)(&robot);

                if self.delay > 0 {
                    robot.sleep(self.delay);
                }
            }

        }

        println!("Robot reached goal. ");
        // todo!("Reset & Active check in end of start");
    }

    /// Called when the controller is reset
    fn reset(&mut self) {
        self.active = false;
    }

    /// Change the delay inbetween moves
    fn set_delay(&mut self, delay: i32) {
        self.delay = delay
    }

    /// Get the current delay between moves
    fn get_delay(&self) -> i32 {
        self.delay
    }

    /// Returns the name which will be shown
    fn get_name(&self) -> &str {
        "Robot Controller"
    }

    /// Returns a description which can be shown
    fn get_description(&self) -> &str {
        "A Polled Robot Controller"
    }
}