use std::{
    sync::{mpsc::Sender, Arc, Mutex},
};

use crate::{Point, generation::Maze};

use super::{polled_controller::PolledController, private, Controller, Heading, Robot, TileType};

/// ThreadedController implementation
pub trait ThreadedController<R: Robot>: Default {
    fn control_robot(&mut self, robot: &mut R);

    fn reset(&self) {}
}

/// Blanket implementation to allow PolledControllers to be used as ThreadedControllers with no issue
impl<R: Robot, T> ThreadedController<R> for T
where
    T: PolledController<R>,
{
    fn control_robot(&mut self, robot: &mut R) {
        self.control_robot(robot);
    }

    fn reset(&self) {
        self.reset()
    }
}

#[derive(Debug, Clone)]
pub struct ThreadedRobotProgress<T: TileType + Default + Clone> {
    pub finished: bool,
    pub robot_pos: Point,
    pub target_loc: Point,
    pub robot_head: Heading,
    pub maze: Maze<T>
}

#[derive(Default)]
pub struct ThreadedControllerWrapper<C, R, T>
where
    C: ThreadedController<R> + Default,
    R: Robot,
    T: TileType + Default + Clone
{
    controller: C,
    robot: R,
    active: Arc<Mutex<bool>>,
    thread_delay: Arc<Mutex<i32>>,
    progress_sender: Option<Sender<ThreadedRobotProgress<T>>>,
    latest_robot_update: Arc<Mutex<Option<ThreadedRobotProgress<T>>>>,
}

impl<T, R, C> Controller<R, T> for ThreadedControllerWrapper<C, R, T>
where
    T: TileType + Default + Clone,
    R: Robot<Tiles = T> + private::Robot,
    C: ThreadedController<R>,
{
    fn get_name(&self) -> &str {
        "Threaded Polled Controller"
    }

    fn get_description(&self) -> &str {
        "Based Multi-Threaded Robot Controller"
    }

    fn get_robot(&self) -> &R {
        &self.robot
    }

    fn set_robot(&mut self, robot: R) {
        self.robot = robot;
    }

    fn get_delay(&self) -> i32 {
        match self.thread_delay.lock() {
            Ok(val) => val.clone(),
            Err(_) => 0,
        }
    }

    fn set_delay(&mut self, delay: i32) {
        let mut del = self.thread_delay.lock().unwrap();
        *del = delay;
    }

    fn set_maze(&mut self, maze: crate::generation::Maze<T>) {
        self.robot.set_maze(Box::new(maze));
    }

    fn start(&mut self) {
        match self.active.lock() {
            Ok(mut val) => *val = true,
            Err(_) => (),
        };

        while !self
            .robot
            .get_location()
            .eq(&self.robot.get_goal_location())
            && match self.active.lock() {
                Ok(val) => *val,
                Err(_) => false,
            }
        {
            self.controller.control_robot(&mut self.robot);

            self.robot.advance();

            // (self.callback)(&mut self.robot);
            self.send_robot_update(false);
            
            if match self.thread_delay.lock() {
                Ok(val) => *val > 0,
                Err(_) => false,
            } {
                self.robot.sleep(self.get_delay());
            }
        }

        println!("Robot finished. ");
        self.send_robot_update(true);

        // todo!("Reset & Active check in end of start");
    }

    fn reset(&mut self) {
        match self.active.lock() {
            Ok(mut val) => *val = false,
            Err(_) => (),
        };

        self.controller.reset();
    }
}

impl<C, R, T> ThreadedControllerWrapper<C, R, T>
where
    C: ThreadedController<R>,
    R: Robot<Tiles = T>,
    T: TileType + Default + Clone
{
    pub fn new(
        active: Arc<Mutex<bool>>,
        thread_delay: Arc<Mutex<i32>>,
        progress_sender: Option<Sender<ThreadedRobotProgress<T>>>,
        latest_robot_update: Arc<Mutex<Option<ThreadedRobotProgress<T>>>>,
    ) -> Self {
        Self {
            active,
            thread_delay,
            progress_sender,
            latest_robot_update,
            ..Default::default()
        }
    }

    pub fn send_robot_update(&mut self, finished: bool) {
        let message = ThreadedRobotProgress {
            finished,
            robot_head: self.robot.get_heading(),
            robot_pos: self.robot.get_location(),
            target_loc: self.robot.get_goal_location(),
            maze: self.robot.get_maze().clone()
        };

        match &self.progress_sender {
            Some(sender) => {
                let send_res = sender.send(message.clone());
                match send_res {
                    Ok(_) => match self.latest_robot_update.lock() {
                        Ok(mut opt_val) => *opt_val = Some(message),
                        Err(_) => (),
                    },
                    Err(err) => println!("{:?}", err),
                }
            }
            None => (),
        }
    }
}
