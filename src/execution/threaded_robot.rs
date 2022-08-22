use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

use crate::{execution::Heading, generation::Maze, Point};

use super::{Tile, Robot, private};

pub struct ThreadedRobot {
    location: Point,
    target_loc: Point,
    heading: Heading,
    maze: Arc<RwLock<Maze<Tile>>>,
    steps: i64,
    collisions: i64,
    runs: i32,
}

impl ThreadedRobot {
    pub fn get_maze(&self) -> Arc<RwLock<Maze<Tile>>> {
        self.maze.clone()
    }

    pub(crate) fn set_maze(&mut self, maze: Arc<RwLock<Maze<Tile>>>) {
        self.location = maze.read().unwrap().get_start();
        self.maze = maze;
    }
}

impl Default for ThreadedRobot {
    fn default() -> Self {
        Self {
            location: Point(1, 1),
            target_loc: Point(1, 1),
            heading: Heading::East,
            maze: Arc::from(RwLock::from(Maze::new(15, 15))),
            steps: 0,
            collisions: 0,
            runs: 0,
        }
    }
}

impl Robot for ThreadedRobot {
    type Tiles = Tile;

    fn face(&mut self, face: super::Facing) {
        self.set_heading(self.heading.augment_heading(face))
    }

    fn look(&self, face: super::Facing) -> Self::Tiles {
        let heading = self.heading.augment_heading(face);
        let Point(locx, locy) = self.get_location();
        let pos = match heading { // Panic waiting to happen with these subs if the robot tries to look off the maze
            Heading::North => Point(locx, locy - 1),
            Heading::East => Point(locx + 1, locy),
            Heading::South => Point(locx, locy + 1),
            Heading::West => Point(locx - 1, locy),
        };

        match self.maze.read() {
            Ok(maze) => *maze.get_cell(pos).unwrap_or(&Tile::Wall),
            Err(_) => unreachable!()
        }
        // *self.get_maze().get_cell(pos).unwrap_or(&Tile::Wall)
    }

    fn get_maze(&self) -> &Maze<Self::Tiles> {
        unimplemented!()
    }
    
    /// Sleep for a bit. Time is a millisecond value.
    fn sleep(&self, time: i32) {
        thread::sleep(Duration::from_millis(time as u64))
    }

    fn get_runs(&self) -> i32 { self.runs }

    fn get_steps(&self) -> i64 { self.steps  }

    fn get_collisions(&self) -> i64 { self.collisions }

    /// Set the robots absolute heading
    fn set_heading(&mut self, heading: Heading) {
        self.heading = heading
    }

    /// Get the robots current heading
    fn get_heading(&self) -> Heading {
        self.heading
    }

    /// Get the location of the robot
    fn get_goal_location(&self) -> Point {
        self.maze.read().unwrap().get_finish()
    }
    
    /// Get the current location of the robot
    fn get_location(&self) -> Point {
        self.location
    }
}

impl private::Robot for ThreadedRobot {
    /// Set the maze currently being used by the robot
    fn set_maze(&mut self, maze: Box<Maze<Self::Tiles>>) {
        
        self.set_target_location(maze.get_finish());
        self.set_location(maze.get_start());

        // println!("set target {:?}; start {:?}", self.get_goal_location(), self.get_location());
        self.maze = Arc::from(RwLock::new(maze.as_ref().clone()));
    }

    /// Set the current location of the robot
    fn set_location(&mut self, loc: Point) {
        self.location = loc
    }

    /// Reset everything
    fn reset(&mut self) {
        self.location = match self.maze.read() {
            Ok(res) => res.get_start(),
            Err(_) => unreachable!()
        };
        self.runs += 1;
        self.steps = 0;
        self.collisions = 0;
    }

    /// Set where the robots target is
    fn set_target_location(&mut self, loc: Point) {
        self.target_loc = loc
    }

    /// Advance the robot on. Define at crate level privacy
    /// to stop an external robot controller calling this at the wrong time
    /// Should be implemented by the concrete Robot impl.
    // BUG: This will panic if robot tries to exit off the top or left side of the maze. 
    fn advance(&mut self) {

        let Point(locx, locy) = self.get_location();
        // println!("{:?} {:?}", locx, locy);
        let new_loc = match self.get_heading() {
            Heading::North => Point(locx, locy - 1),
            Heading::East => Point(locx + 1, locy),
            Heading::South => Point(locx, locy + 1),
            Heading::West => Point(locx - 1, locy)
        };
        // println!("heading {:?}", self.get_heading());
        // println!("new pos {:?}\t old pos {:?}", new_loc, Point(locx, locy));

        match self.maze.read() {
            Ok(maze) => {
                if maze.can_move(new_loc) {
                    self.steps += 1;
                    self.location = new_loc;
                } else {
                    self.collisions += 1;
                }
            },
            Err(_) => unreachable!()
        }

        match self.maze.write() {
            Ok(mut maze) => {
                maze.set_cell(Point(locx, locy), Tile::BeenBefore)
            }, 
            Err(_) => ()
        }
        
    }
}