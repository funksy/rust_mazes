use dioxus::prelude::*;

use maze_generator::generator_algorithms::ellers::Ellers;
use maze_generator::generator_algorithms::generator_helpers::{GeneratorAlgo, GeneratorStatus};
use maze_generator::maze::Maze;
use maze_generator::ui::main_view::launch_app;

fn main() {

    let mut maze = Maze::new(3, 3);
    let mut ellers = Ellers::new();

    while ellers.status() != &GeneratorStatus::Done {
        ellers.create_maze(&mut maze);
    }
    // launch_app();
}