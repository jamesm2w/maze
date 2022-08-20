use crate::{
    execution::Tile,
    generation::{Generator, Maze},
    Point, GeneratorOptions
};
use rand::{rngs::ThreadRng, Rng};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellType {
    Wall,
    Passage,
}

/// Simple generator which uses a Randomised Prim's Algorithm to generate passageways and a maze. 
#[derive(Debug, Clone)]
pub struct PrimGenerator {
    options: GeneratorOptions,
}

impl Generator for PrimGenerator {
    type Options = GeneratorOptions;
    type Tiles = Tile;

    fn new() -> Self {
        PrimGenerator {
            options: Default::default(),
        }
    }

    fn get_name(&self) -> &str {
        "Prim Generator"
    }

    fn get_description(&self) -> &str {
        "Generate Mazes with Prim Algorithm"
    }

    fn get_options(&self) -> Self::Options {
        self.options
    }

    fn set_options(&mut self, _options: Self::Options) {
        // Prims grid needs to be at least 3x3, but even then should be much larger to get a better maze
        // Stuff probably breaks on some funky sizes
        if _options.width < 3 || _options.height < 3 {
            return;
        }
        self.options = _options;
    }

    fn generate_maze(&mut self) -> Maze<Self::Tiles> {
        let mut maze: Maze<Tile> = Maze::new(
            self.options.width as usize + 1,
            self.options.height as usize + 1,
        );
        maze.fill(Tile::Wall);
        let mut frontier: Vec<(usize, usize)> = Vec::new();
        let mut grid: Vec<Vec<CellType>> = Vec::new();
        let mut visited_cells: HashSet<(usize, usize)> = HashSet::new();
        let mut thread_rng: ThreadRng = rand::thread_rng();

        // Maze (0, 0) -> (width, height)
        // Grid (1, 1) -> (width - 1, height - 1) on the maze; (0, 0) -> (width-2, height-2) on the grid
        // mapping for grid co-ord -> maze co-ord => G_x+1, G_y+1

        // Fill grid with walls
        for i in 0..self.options.height - 1 {
            let mut row = Vec::new();
            for j in 0..self.options.width - 1 {
                row.push(CellType::Wall);
                maze.set_cell(Point(j as usize + 1, i as usize + 1), Tile::Wall);
            }
            grid.push(row);
        }
        // TODO: probably a bug where there's 2 rows/cols of passage after the maze. Maybe fix that.

        maze.set_start(Point(1, 1)); // grid point (0, 0)
        maze.set_finish(Point(
            self.options.width as usize - 1,
            self.options.height as usize - 1,
        )); // grid point (w-2, h-2)

        // Start by setting up the goal as the first item in grid
        grid[self.options.height as usize - 2][self.options.width as usize - 2] = CellType::Passage;
        maze.set_cell(maze.get_finish(), Tile::Passage);

        visited_cells.insert((
            self.options.width as usize - 2,
            self.options.height as usize - 2,
        ));

        // Add neighbours to frontier
        let h = self.options.height as usize - 2;
        let w = self.options.width as usize - 2;
        vec![(w - 1, h), (w, h - 1)]
            .iter()
            .for_each(|p| frontier.push(*p));

        // Prims Algo:
        //  While frontier not empty,
        //  select random point from the frontier
        //  if wall only neighbours 1 passageway -- make passage and add neighbours to the frontier
        //  remove from frontier
        let mut rand_index;
        while let Some((x, y)) = {
            rand_index = if frontier.len() > 1 {
                thread_rng.gen_range(0..=frontier.len()-1)
            } else {
                0
            };
            frontier.get(rand_index)
        } {
            if visited_cells.contains(&(*x, *y)) {
                frontier.remove(rand_index);
                continue;
            }

            // Check surrounding elements
            let neighbours = vec![
                PrimGenerator::grid_get_point(&grid, (*x, y.saturating_sub(1))),
                PrimGenerator::grid_get_point(&grid, (x.saturating_sub(1), *y)),
                PrimGenerator::grid_get_point(&grid, (*x, y + 1)),
                PrimGenerator::grid_get_point(&grid, (x + 1, *y)),
            ];

            // Number of neighbours which are passages
            let passages = neighbours
                .iter()
                .filter(|(cell, _)| cell.is_some() && cell.unwrap() == &CellType::Passage)
                .map(|(cell, point)| (cell.unwrap(), *point))
                .collect::<Vec<(&CellType, (usize, usize))>>();

            // Note this cell as visited and expanded
            visited_cells.insert((*x, *y));

            if passages.len() == 1 {
                let mut valid_neighbour_points = Vec::new();
                // Add all neighbours to the frontier list if this doesnt form loops
                for (el, point) in neighbours {
                    match el {
                        Some(CellType::Wall) if !visited_cells.contains(&point) => {
                            valid_neighbour_points.push(point)
                        }
                        _ => (),
                    }
                }
                // Mark as passage
                grid[*y][*x] = CellType::Passage;
                maze.set_cell(Point(x + 1, y + 1), Tile::Passage);
                frontier.extend(valid_neighbour_points);
            };

            frontier.swap_remove(rand_index);
        }

        if let Some(Tile::Wall) = maze.get_cell(Point(1, 1)) {

            if let Some(Tile::Passage) = maze.get_cell(Point(2, 1)) {
                maze.set_cell(Point(1, 1), Tile::Passage);
                maze.set_cell(Point(2, 1), Tile::Wall);
            }

        }

        maze
    }
}

impl PrimGenerator {
    fn grid_get_point(
        grid: &Vec<Vec<CellType>>,
        point: (usize, usize),
    ) -> (Option<&CellType>, (usize, usize)) {
        let (x, y) = point;
        if y < grid.len() {
            (grid[y].get(x), (x, y))
        } else {
            (None, (x, y))
        }
    }
}
