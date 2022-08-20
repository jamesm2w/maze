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
        let mut maze: Maze<Tile> = Maze::new(
            self.get_real_width(),
            self.get_real_height(),
        );

        for _ in 0..self.get_real_height() {
            let mut row = Vec::new();
            for _ in 0..self.get_real_width() {
                row.push(GridCell::Wall);
            }
            self.grid.push(row);
        }

        let mut thread_rng = rand::thread_rng();
        let mut frontier = Vec::new();
        let mut random_index;
        
        let (ix, iy) = (1, 1);
        self.grid[iy][ix] = GridCell::Passage;
        frontier.push(Point(ix, iy));
        
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

        for y in 0..self.get_real_height() {
            for x in 0..self.get_real_width() {
                maze.set_cell(Point(x, y), match self.grid[y][x] {
                    GridCell::Passage => Tile::Passage,
                    GridCell::Wall => Tile::Wall
                });
            }
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
        vec![(x.wrapping_sub(2), y), (x + 2, y), (x, y.wrapping_sub(2)), (x, y + 2)]
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
        
        if neighbours.len() == 0 {
            return;
        }

        let picked_index = if neighbours.len() > 1 { rand::thread_rng().gen_range(0..neighbours.len()) } else { 0 };
        let neighbour = neighbours[picked_index];

        let Point(x, y) = self.point_between(point, neighbour);
        self.grid[y][x] = GridCell::Passage;
    }
}

#[test]
fn test() {
    let mut generator = GappedPrimGenerator::new();

    let maze = generator.generate_maze();

    for i in maze.get_grid().iter() {
        for j in i.iter() {
            print!("{}", match j {
                Tile::Wall => "#",
                Tile::Passage => " ",
                Tile::BeenBefore => "*"
            });
        }
        println!()
    }
    println!();

    assert_eq!(1, 1);
}