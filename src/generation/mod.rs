pub mod blank_generator;
pub mod prim_generator;
pub mod actual_prim_generator;

use std::fmt::Debug;

use crate::Point;
use crate::execution::{Tile, TileType};

/// Something which Generates Mazes
pub trait Generator {

    type Options: Default;
    type Tiles: TileType + Default;

    /// Creates a new generator of this type
    fn new() -> Self;

    /// Provides the name of the maze generator
    fn get_name(&self) -> &str;

    /// Provides the description of the generator
    fn get_description(&self) -> &str {
        "A maze generator"
    }

    /// Generates the maze
    fn generate_maze(&mut self) -> Maze<Self::Tiles>; // TODO: Add return type

    //Methods to do with configuring this generator

    /// Set the options on this maze
    fn set_options(&mut self, _options: Self::Options) { }

    /// Get the options set
    fn get_options(&self) -> Self::Options {
        Self::Options::default()
    }

    /// Get the UI components for the configuration
    fn get_options_ui(&self) {
        unimplemented!("get_options_ui")
    }
}

#[derive(Debug, Clone)]
pub struct Maze<T: TileType + Default> {
    width: usize,
    height: usize,
    grid: Vec<Vec<T>>,
    start: Point,
    goal: Point
}

impl <T: TileType + Default + Debug + Clone> Maze<T> {
    /// Create the Maze struct
    pub fn new(width: usize, height: usize) -> Self {

        let mut grid: Vec<Vec<T>> = Vec::new();
        for _ in 0..height {
            let mut row = Vec::new();
            for _ in 0..width {
                row.push(Default::default());
            }
            grid.push(row);
        }

        Maze { width, height, grid, start: Point(0, 0), goal: Point(0,0) }
    }

    pub fn fill(&mut self, tile: T) {
        for i in 0..self.height {
            for j in 0..self.width {
                self.grid[i][j] = tile.clone();
            }
        }
    }

    /// Read Maze from a file 
    pub fn read_maze() -> Self {
        unimplemented!("read_maze")
    }

    /// Write maze to a file
    pub fn write_maze() {
        unimplemented!("write_maze");
    }

    /// Get the width of the maze
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Get the height of the maze
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Get the cell at the given point
    /// 0 <= point.x < width
    /// 0 <= point.y < height
    pub fn get_cell(&self, point: Point) -> Option<&T> {
        let col = self.grid.get(point.get_y() as usize)?;
        col.get(point.get_x() as usize)
    }

    /// Set the cell at the point to a type
    pub fn set_cell(&mut self, point: Point, typ: T) {
        if let Some(col) = self.grid.get_mut(point.get_y() as usize) {
            col[point.get_x() as usize] = typ;
        }
    }

    pub fn get_grid(&self) -> &Vec<Vec<T>> {
        &self.grid
    }

    pub fn can_move(&self, point: Point) -> bool {
        match self.get_cell(point) {
            None => false, // not a valid cell
            Some(tile) => tile.can_walk() // can you walk on the cell
        }
    }

    /// Get the start location of the maze
    pub fn get_start(&self) -> Point {
        self.start
    }

    /// Set the start point of the maze
    pub fn set_start(&mut self, point: Point) {
        self.start = point
    }

    /// Get the finish point of the maze
    pub fn get_finish(&self) -> Point {
        self.goal
    }

    /// Set the finish point of the maze
    pub fn set_finish(&mut self, point: Point) {
        self.goal = point
    }
}