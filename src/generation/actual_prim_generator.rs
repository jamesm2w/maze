use rand::Rng;

use crate::{execution::Tile, GeneratorOptions, Point};

use super::{Generator, Maze};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GridCell {
    Passage,
    Wall,
}

pub struct GappedPrimGenerator {
    options: GeneratorOptions,
    grid: Vec<Vec<GridCell>>,
}

impl Generator for GappedPrimGenerator {
    type Options = GeneratorOptions;
    type Tiles = Tile;

    fn new() -> Self {
        GappedPrimGenerator {
            options: Default::default(),
            grid: vec![],
        }
    }

    fn get_name(&self) -> &str {
        "Prim Generator"
    }

    fn set_options(&mut self, _options: Self::Options) {
        self.options = _options;
    }

    fn get_options(&self) -> Self::Options {
        self.options
    }

    fn generate_maze(&mut self) -> super::Maze<Self::Tiles> {
        let maze: Maze<Tile> = Maze::new(
            self.get_options().width as usize,
            self.get_options().height as usize,
        );

        for _ in 0..self.get_options().height * 2 + 1 {
            let mut row = Vec::new();
            for _ in 0..self.get_options().width * 2 + 1 {
                row.push(GridCell::Wall);
            }
            self.grid.push(row);
        }

        let mut thread_rng = rand::thread_rng();
        let mut frontier = Vec::new();
        let mut random_index;
        while let Some(Point(x, y)) = {
            random_index = if frontier.len() > 0 {
                thread_rng.gen_range(0..frontier.len())
            } else {
                0
            };
            frontier.get(random_index)
        } {
            self.grid[*y][*x] = GridCell::Passage;
            let mut frontier_around = self.get_frontier_around_point(Point(*x, *y));
            self.connect_random_neighbour(Point(*x, *y));
            frontier.swap_remove(random_index);
            frontier.append(&mut frontier_around);
        }

        maze
    }
}

impl GappedPrimGenerator {
    pub fn get_grid(&self, point: Point) -> GridCell {
        self.grid[point.get_y()][point.get_x()]
    }

    pub fn get_real_width(&self) -> usize {
        self.get_options().width as usize * 2 + 1
    }

    pub fn get_real_height(&self) -> usize {
        self.get_options().height as usize * 2 + 1
    }

    pub fn is_point_legal(&self, point: Point) -> bool {
        point.get_x() > 0
            && point.get_x() < self.get_real_width() - 1
            && point.get_y() > 0
            && point.get_y() < self.get_real_height() - 1
    }

    pub fn get_two_gapped_cells(&self, point: Point) -> Vec<Point> {
        let Point(x, y) = point;
        vec![(x - 2, y), (x + 2, y), (x, y - 2), (x, y + 2)]
            .into_iter()
            .map(|(x, y)| Point(x, y))
            .collect()
    }

    pub fn get_frontier_around_point(&self, point: Point) -> Vec<Point> {
        self.get_two_gapped_cells(point)
            .into_iter()
            .filter(|x| self.is_point_legal(*x) && self.get_grid(*x) == GridCell::Wall)
            .collect()
    }

    pub fn get_neighbours_around_point(&self, point: Point) -> Vec<Point> {
        self.get_two_gapped_cells(point)
            .into_iter()
            .filter(|x| self.is_point_legal(*x) && self.get_grid(*x) == GridCell::Passage)
            .collect()
    }

    pub fn point_between(&self, point_a: Point, point_b: Point) -> Point {
        let Point(xa, ya) = point_a;
        let Point(xb, yb) = point_b;

        let x = match xb as isize - xa as isize {
            0 => xa,
            2 => 1 + xa,
            -2 => xa - 1,
            _ => panic!("Calling point_between on non-2-gapped points"),
        };

        let y = match yb as isize - ya as isize {
            0 => ya,
            2 => 1 + ya,
            -2 => ya - 1,
            _ => panic!("Calling point_between on non-2-gapped points"),
        };

        Point(x, y)
    }

    pub fn connect_random_neighbour(&mut self, point: Point) {
        let neighbours = self.get_neighbours_around_point(point);
        let picked_index = rand::thread_rng().gen_range(0..neighbours.len());
        let neighbour = neighbours[picked_index];

        let Point(x, y) = self.point_between(point, neighbour);
        self.grid[y][x] = GridCell::Passage;
    }
}
