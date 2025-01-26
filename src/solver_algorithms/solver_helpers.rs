use dioxus::prelude::Signal;
use crate::cell::{CellState, Coord};
use crate::maze::Maze;
use crate::solver_algorithms::breadth_first_search::BreadthFirstSearch;
use crate::solver_algorithms::depth_first_search::DepthFirstSearch;
use crate::solver_algorithms::djikstras::Djikstras;

#[derive(PartialEq)]
pub enum SolverStatus {
    Initialized,
    InProgress,
    Solved,
    Done,
}

pub trait SolverAlgo {
    fn find_solution(&mut self, maze: &mut Signal<Maze>);

    fn status(&self) -> &SolverStatus;

    fn reset(&self, maze: &mut Signal<Maze>);
}

pub fn get_solver_algo(algo: &str, start: &Coord, finish: &Coord) -> Box<dyn SolverAlgo> {
    match algo {
        "breadth_first_search" => Box::new(BreadthFirstSearch::new(start, finish)),
        "djikstras" => Box::new(Djikstras::new(start, finish)),
        "depth_first_search" => Box::new(DepthFirstSearch::new(start, finish)),
        _ => panic!("you shouldn't be here"),
    }
}

pub fn reset_solver(maze: &mut Maze) {
    for y in 0..maze.height() {
        for x in 0..maze.width() {
            maze.change_cell_state(&Coord{ x, y}, CellState::Path);
        }
    }
}