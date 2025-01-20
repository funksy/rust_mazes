use dioxus::prelude::*;
use maze_generator::generator_algorithms::generator_helpers::{GeneratorAlgo, GeneratorStatus};
use maze_generator::generator_algorithms::recursive_backtracker::RecursiveBacktracker;
use maze_generator::maze::Maze;
use maze_generator::ui::main_view::launch_app;

fn main() {
    launch_app();
}