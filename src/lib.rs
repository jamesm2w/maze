// Maze Controller

// Maze Generation
// Maze Running

pub mod generation;

pub mod execution;


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn get_x(&self) -> usize {
        self.0
    } 

    pub fn get_y(&self) -> usize {
        self.1
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GeneratorOptions {
    pub width: i32,
    pub height: i32,
}

impl Default for GeneratorOptions {
    fn default() -> Self {
        GeneratorOptions {
            width: 30,
            height: 30,
        }
    }
}
