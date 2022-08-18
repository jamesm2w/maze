use crate::{generation::Maze, Point};
use std::{borrow::{Borrow}, thread, time::Duration};

use super::{private, Facing, Heading, Robot, Tile};

pub struct DefaultRobot {
    active: bool,
    location: Point,
    target: Point,
    maze: Box<Maze<Tile>>,
    heading: Heading,
    steps: i64,
    collisions: i64,
    runs: i32,
}

impl Default for DefaultRobot {
    fn default() -> Self {
        DefaultRobot {
            active: true,
            location: Point(1, 1),
            target: Point(0, 0),
            maze: Box::new(Maze::new(0, 0)),
            heading: Heading::South,
            steps: 0,
            collisions: 0,
            runs: 0,
        }
    }
}

impl Robot for DefaultRobot {
    type Tiles = Tile;

    /// Look at the relative facing direction and get the tile
    fn look(&self, face: Facing) -> Self::Tiles {
        let heading = self.heading.augment_heading(face);
        let Point(locx, locy) = self.get_location();
        let pos = match heading { // Panic waiting to happen with these subs if the robot tries to look off the maze
            Heading::North => Point(locx, locy - 1),
            Heading::East => Point(locx + 1, locy),
            Heading::South => Point(locx, locy + 1),
            Heading::West => Point(locx - 1, locy),
        };

        *self.get_maze().get_cell(pos).unwrap_or(&Tile::Wall)
    }

    /// Change the robot to face a certain way
    fn face(&mut self, face: Facing) {
        self.set_heading(self.heading.augment_heading(face))
    }

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
        self.target
    }
    
    /// Get the current location of the robot
    fn get_location(&self) -> Point {
        self.location
    }
    
    /// Get the current maze
    fn get_maze(&self) -> &Maze<Self::Tiles> {
        self.maze.borrow()
    }

    /// Sleep for a bit. Time is a microsecond value.
    fn sleep(&self, time: i32) {
        thread::sleep(Duration::from_micros(time as u64))
    }

    fn get_runs(&self) -> i32 { self.runs }

    fn get_steps(&self) -> i64 { self.steps  }

    fn get_collisions(&self) -> i64 { self.collisions }
}

impl private::Robot for DefaultRobot {
    /// Set the maze currently being used by the robot
    fn set_maze(&mut self, maze: Box<Maze<Self::Tiles>>) {
        self.maze = maze;
        self.set_target_location(self.maze.get_finish());
        self.set_location(self.maze.get_start());
    }

    /// Set the current location of the robot
    fn set_location(&mut self, loc: Point) {
        self.location = loc
    }

    /// Reset everything
    fn reset(&self) {}

    /// Set where the robots target is
    fn set_target_location(&mut self, loc: Point) {
        self.target = loc
    }

    /// Advance the robot on. Define at crate level privacy
    /// to stop an external robot controller calling this at the wrong time
    /// Should be implemented by the concrete Robot impl.
    // BUG: This will panic if robot tries to exit off the top or left side of the maze. 
    fn advance(&mut self) {
        if self.active {
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

            if self.maze.can_move(new_loc) {
                self.steps += 1;
                self.set_location(new_loc);
            } else {
                self.collisions += 1
            }

            self.maze.set_cell(Point(locx, locy), Tile::BeenBefore);
        }
    }
}
