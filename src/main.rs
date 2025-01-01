use std::time::{Instant};

use maze_generator::maze::Maze;
use maze_generator::random_prim;
use maze_generator::maze_renderer::MazeRenderer;

fn main() {
    let mut maze = Maze::new();

    let start = Instant::now();
    
    random_prim::create_maze(&mut maze);

    let duration = start.elapsed();
    println!("It took {:?} to complete a maze of grid size {}x{}", duration, maze.height, maze.width);
    // maze.show();

    let mut renderer = MazeRenderer::new(&maze);
    renderer.update_walls(&maze);
    let document = renderer.generate_document();
    svg::save("test.svg", &document).unwrap();

    let duration = start.elapsed();
    println!("It took {:?} to complete the svg render", duration);
}