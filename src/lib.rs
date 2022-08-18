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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
