use std::time::{Instant};

use maze_generator::maze::Maze;
use maze_generator::random_prim;
use maze_generator::svg_generator::generate_svg;

fn main() {
    let mut maze = Maze::new();

    let start = Instant::now();
    random_prim::create_maze(&mut maze);
    generate_svg(&maze);
    println!(".svg was created and saved in the top level directory");
    let duration = start.elapsed();

    println!("It took {:?} to complete a maze of grid size {}x{}", duration, maze.height, maze.width);
    maze.show();

}