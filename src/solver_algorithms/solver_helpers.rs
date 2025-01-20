use crate::cell::{CellState, Coord};
use crate::maze::Maze;

#[derive(PartialEq)]
pub enum SolverStatus {
    Initialized,
    InProgress,
    Solved,
    Done,
}

pub fn reset_solver(maze: &mut Maze) {
    for y in 0..maze.height() {
        for x in 0..maze.width() {
            maze.change_cell_state(&Coord{ x, y}, CellState::Path);
        }
    }
}