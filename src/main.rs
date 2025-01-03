use std::time::{Instant};

use maze_generator::maze::Maze;
use maze_generator::maze_renderer::MazeRenderer;

use maze_generator::generator_algorithms::{random_prim, recursive_backtracker };
use maze_generator::solver_algorithms::{ breadth_first_search };

fn main() {
    let mut maze = Maze::new();

    let start = Instant::now();
    recursive_backtracker::create_maze(&mut maze);
    let duration = start.elapsed();
    println!("It took {:?} to complete a maze of grid size {}x{}", duration, maze.height, maze.width);

    let start = Instant::now();
    let mut renderer = MazeRenderer::new(&maze);
    renderer.update_walls(&maze);
    let document = renderer.generate_document();
    svg::save("test.svg", &document).unwrap();
    let duration = start.elapsed();
    println!("It took {:?} to complete the svg render", duration);

    let start = Instant::now();
    breadth_first_search::find_solution(&maze, (0, 0), (maze.height - 1, maze.width - 1), &mut renderer);
    let duration = start.elapsed();
    println!("It took {:?} to find a solution for the maze", duration);

    let document = renderer.generate_document();
    svg::save("solved.svg", &document).unwrap();
}