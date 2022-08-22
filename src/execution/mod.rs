use crate::{Point, generation::Maze};

pub mod polled_controller;
pub mod threaded_controller;
pub mod random_controller;
pub mod robot;
pub mod threaded_robot;

/// This trait is what the student implements -- 
/// the brains of the robot is a type which can run certain
pub trait Controller<T: Robot<Tiles=K>, K: TileType + Default> {

    /// Set the robot the controller operates on
    fn set_robot(&mut self, robot: T);

    /// Get the current robot
    fn get_robot(&self) -> &T;

    /// Set the Maze the controller/robot operates on
    fn set_maze(&mut self, maze: Maze<K>);

    /// Called when the controller is started
    fn start(&mut self);

    /// Called when the controller is reset
    fn reset(&mut self);

    /// Change the delay inbetween moves
    fn set_delay(&mut self, delay: i32);

    /// Get the current delay between moves
    fn get_delay(&self) -> i32;

    /// Returns the name which will be shown
    fn get_name(&self) -> &str;

    /// Returns a description which can be shown
    fn get_description(&self) -> &str {
        "A robot controller"
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Heading {
    North = 1000,
    East,
    South,
    West,
}

impl From<u32> for Heading {
    fn from(i: u32) -> Self {
        match i {
            1000 => Heading::North,
            1001 => Heading::East,
            1002 => Heading::South,
            _ => Heading::West
        }
    }
}

impl Heading {
    pub fn augment_heading(&self, face: Facing) -> Self {

        let h = match face {
            Facing::Ahead => *self,
            Facing::Behind => match self {
                Heading::North => Heading::South,
                Heading::South => Heading::North,
                Heading::East => Heading::West,
                Heading::West => Heading::East
            },
            Facing::Left => match self {
                Heading::North => Heading::West,
                Heading::West => Heading::South,
                Heading::South => Heading::East,
                Heading::East => Heading::North
            },
            Facing::Right => match self {
                Heading::North => Heading::East,
                Heading::East => Heading::South,
                Heading::South => Heading::West,
                Heading::West => Heading::North
            }
        };
        h
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Facing {
    Ahead = 2000,
    Right,
    Behind,
    Left,
}

impl From<i32> for Facing {
    fn from(i: i32) -> Self {
        match i {
            0 => Facing::Ahead,
            1 => Facing::Right,
            2 => Facing::Behind,
            _ => Facing::Left
        }
    }
}

/// Enum which can represent the multiple types of tiles in the maze
/// At the basic level just need to know if the tile is a wall or if it can be walked over
/// this could be extended to have more exciting types if you so wanted
pub trait TileType {
    fn is_wall(&self) -> bool;

    fn can_walk(&self) -> bool {
        !self.is_wall()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Passage,
    BeenBefore,
    Wall,
}

impl Default for Tile {
    fn default() -> Self {
        Tile::Passage
    }
}

impl TileType for Tile {
    fn is_wall(&self) -> bool {
        match self {
            Tile::Wall => true,
            _ => false,
        }
    }
}

/// The robot which is moved about the maze. 
/// Has a fairly restricted API so most of the logic 
/// has to be implemented in a controller obejct
pub trait Robot: Default {
    type Tiles: TileType + Default;

    /// Look at the relative facing direction and get the tile
    fn look(&self, face: Facing) -> Self::Tiles;

    /// Change the robot to face a certain way
    fn face(&mut self, face: Facing);

    /// Set the robots absolute heading
    fn set_heading(&mut self, heading: Heading);

    /// Get the robots current heading
    fn get_heading(&self) -> Heading;

    /// Get the location of the robot
    fn get_goal_location(&self) -> Point;
    
    /// Get the current location of the robot
    fn get_location(&self) -> Point;

    /// Get the current maze
    fn get_maze(&self) -> &Maze<Self::Tiles>;

    /// Sleep for a bit
    fn sleep(&self, time: i32);

    /// Stats methods TODO!
    /// Get the number of times this robot has been run
    fn get_runs(&self) -> i32 { 0 }

    fn get_steps(&self) -> i64 { 0 }

    fn get_collisions(&self) -> i64 { 0 }

    // Print Robot & Maze state out to console
    fn print(&self) { }

}

mod private {
    use crate::{generation::Maze, Point};

    /// Internal methods for a Robot. Implementors must also be
    /// of trait Robot.
    pub(crate) trait Robot: super::Robot {

        /// Set the maze currently being used by the robot
        fn set_maze(&mut self, maze: Box<Maze<Self::Tiles>>);

        /// Set the current location of the robot
        fn set_location(&mut self, loc: Point);

        /// Reset everything
        fn reset(&mut self);

        /// Set where the robots target is
        fn set_target_location(&mut self, loc: Point);

        /// Advance the robot on. Define at crate level privacy 
        /// to stop an external robot controller calling this at the wrong time
        /// Should be implemented by the concrete Robot impl.
        fn advance(&mut self);
    }
}

// IRobotReport, RobotReport
#[derive(Default)]
pub struct RobotStatistics {
    steps: i64,
    collisions: i64,
    goal_reached: bool,
    runs: i32
}

impl RobotStatistics {
    pub fn set_steps(&mut self, steps: i64) {
        self.steps = steps
    }

    pub fn get_steps(&self) -> i64 {
        self.steps
    }

    pub fn set_collision(&mut self, collisions: i64) {
        self.collisions = collisions
    }

    pub fn get_collisions(&self) -> i64 {
        self.collisions
    }

    pub fn set_goal_reached(&mut self, status: bool) {
        self.goal_reached = status
    }

    pub fn goal_reached(&self) -> bool {
        self.goal_reached
    }

    pub fn set_run_number(&mut self, runs: i32) {
        self.runs = runs
    }

    pub fn get_run_number(&self) -> i32 {
        self.runs
    }
}