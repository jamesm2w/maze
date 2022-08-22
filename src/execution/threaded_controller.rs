use std::{
    sync::{mpsc::Sender, Arc, Mutex, RwLock},
};

use crate::{Point, execution::Maze};

use super::{polled_controller::PolledController, private, Controller, Heading, Robot, threaded_robot::ThreadedRobot};
use super::Tile;
/// ThreadedController implementation
pub trait ThreadedController: Default {
    fn control_robot(&mut self, robot: &mut ThreadedRobot);

    fn reset(&self) {}
}

/// Blanket implementation to allow PolledControllers to be used as ThreadedControllers with no issue
impl<T> ThreadedController for T
where
    T: PolledController<ThreadedRobot>,
{
    fn control_robot(&mut self, robot: &mut ThreadedRobot) {
        self.control_robot(robot);
    }

    fn reset(&self) {
        self.reset()
    }
}

#[derive(Debug, Clone)]
pub struct ThreadedRobotProgress {
    pub finished: bool,
    pub robot_pos: Point,
    pub target_loc: Point,
    pub robot_head: Heading
}

#[derive(Default)]
pub struct ThreadedControllerWrapper<C: ThreadedController>
{
    robot: ThreadedRobot,
    active: Arc<Mutex<bool>>,
    thread_delay: Arc<Mutex<i32>>,
    progress_sender: Arc<Mutex<Option<Sender<ThreadedRobotProgress>>>>,
    latest_robot_update: Arc<Mutex<Option<ThreadedRobotProgress>>>,
    controller: C,
}

impl<C> Controller<ThreadedRobot, Tile> for ThreadedControllerWrapper<C>
where 
    C: ThreadedController
{
    fn get_name(&self) -> &str {
        "Threaded Polled Controller"
    }

    fn get_description(&self) -> &str {
        "Based Multi-Threaded Robot Controller"
    }

    fn get_robot(&self) -> &ThreadedRobot {
        &self.robot
    }

    fn set_robot(&mut self, robot: ThreadedRobot) {
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

    fn set_maze(&mut self, maze: crate::generation::Maze<Tile>) {
        self.robot.set_maze(Arc::new(RwLock::from(maze)));
    }

    fn start(&mut self) {

        while !Robot::get_location(&self.robot).eq(&self.robot.get_goal_location())
            && match self.active.lock() {
                Ok(val) => *val,
                Err(_) => false,
            }
        {
            self.controller.control_robot(&mut self.robot);

            private::Robot::advance(&mut self.robot);

            // (self.callback)(&mut self.robot);
            // println!("loc {:?}\ttarget_loc {:?}\theading {:?}", self.robot.get_location(), self.robot.get_goal_location(), self.robot.get_heading());
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

impl<C: ThreadedController> ThreadedControllerWrapper<C> {
    pub fn new(
        active: Arc<Mutex<bool>>,
        thread_delay: Arc<Mutex<i32>>,
        latest_robot_update: Arc<Mutex<Option<ThreadedRobotProgress>>>,
    ) -> Self {
        Self {
            active,
            thread_delay,
            latest_robot_update,
            ..Default::default()
        }
    }

    pub fn set_maze_ref(&mut self, maze: Arc<RwLock<Maze<Tile>>>) {
        if !*self.active.lock().unwrap() {
            self.robot.set_maze(maze);
        } 
    }

    pub fn set_sender(&mut self, tx: Sender<ThreadedRobotProgress>) {
        self.progress_sender = Arc::from(Mutex::from(Some(tx)));
    }

    pub fn send_robot_update(&mut self, finished: bool) {
        let message = ThreadedRobotProgress {
            finished,
            robot_head: self.robot.get_heading(),
            robot_pos: self.robot.get_location(),
            target_loc: self.robot.get_goal_location()
        };

        match self.progress_sender.lock() {
            Ok(lock) => {
                if let Some(sender) = &*lock {
                    let send_res = sender.send(message.clone());
                    match send_res {
                        Ok(_) => match self.latest_robot_update.lock() {
                            Ok(mut opt_val) => *opt_val = Some(message),
                            Err(_) => (),
                        },
                        Err(err) => println!("{:?}", err),
                    }
                }
            }
            _ => (),
        }
    }
}
