# maze

Rust library for generating and running robots in mazes

## Usage: 

The library is intended to be used as a dependency for a bin program which implements a `PolledController`. It provides `DefaultRobot`, `Tile`, `Facing` types as defaults for the maze and robot.

An example `main` method could be something like this:
```rust
fn main() {
    println!("Starting my Maze App");

    let mut gen = PrimGenerator::new();
    let maze = gen.generate_maze();
    println!("Robot Start Pos {:?}", maze.get_start());
    println!("Robot End Pos {:?}", maze.get_finish());

    let mut controller = PolledControllerWrapper::<DefaultRobot, MyController>::new();

    controller.set_maze(maze);
    controller.set_delay(10000);
    print!("{esc}[2J{esc}[H", esc = 27 as char); // If you want to clear the screen before running
    controller.set_poll_callback(Box::new(|robot: &DefaultRobot| {
      // Could put some debug or maze drawing code here
      robot.print();
    }));
    controller.start();
}
```

Where you define `MyController` as the Polled Controller as something like this random example: 
```rust
#[derive(Default)]
struct MyController;

impl<R: Robot<Tiles = Tile>> PolledController<R> for MyController {
    fn control_robot(&mut self, robot: &mut R) {
        let mut random_gen = rand::thread_rng();

        let direction = match random_gen.gen_range(0..=3) {
            0 => Facing::Ahead,
            1 => Facing::Left,
            2 => Facing::Right,
            _ => Facing::Behind,
        };
        
        robot.face(direction);
    }
}
```
