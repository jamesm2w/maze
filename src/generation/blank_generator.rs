use crate::generation::{Generator, Tile, Maze};
use crate::Point;

/// Example Generator which generates nothing
pub struct BlankGenerator;

impl Generator for BlankGenerator {
    type Options = ();
    type Tiles = Tile;

    fn new() -> Self {
        BlankGenerator {}
    }

    fn get_name(&self) -> &str {
        "Blank Generator"
    }

    fn generate_maze(&mut self) -> Maze<Self::Tiles> {
        let mut maze = Maze::new(20, 20);
        maze.set_start(Point(1,1));
        maze.set_finish(Point(18,18));
        for i in 0..maze.get_height() {
            for j in 0..maze.get_width() {
                
                if i == 0 || j == 0 || i == maze.get_height() - 1 || j == maze.get_height() - 1 {
                    maze.set_cell(Point(i, j), Tile::Wall);
                } else {
                    maze.set_cell(Point(i, j), Tile::Passage);
                }

            }
        }
        
        maze
    }
}