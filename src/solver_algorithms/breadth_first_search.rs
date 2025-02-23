use std::collections::{HashMap, VecDeque};

use dioxus::prelude::*;

use crate::structures::maze::Maze;
use crate::structures::cell::{CellState, Coord};
use crate::solver_algorithms::solver_helpers::{reset_solver, solved, SolverAlgo, SolverStatus};

pub struct BreadthFirstSearch {
    start: Coord,
    finish: Coord,
    frontier: VecDeque<Coord>,
    explored: HashMap<Coord, Coord>,
    current_cell: Coord,
    status: SolverStatus,
}

impl SolverAlgo for BreadthFirstSearch {
    fn find_solution(&mut self, maze: &mut Signal<Maze>) {
        let maze: &mut Maze = &mut maze.write();

        match self.status {
            SolverStatus::Initialized => {
                maze.change_cell_state(&self.start, CellState::Start);
                maze.change_cell_state(&self.finish, CellState::Finish);
                self.frontier.push_back(self.start);
                self.status = SolverStatus::InProgress;
            }
            SolverStatus::InProgress => {
                self.current_cell = self.frontier.pop_front().unwrap();
                self.add_adjacent_cells_to_frontier(maze);
                if solved(&self.explored, &self.finish) {
                    self.current_cell = *self.explored.get(&self.finish).unwrap();
                    self.status = SolverStatus::Solved;
                }
            }
            SolverStatus::Solved => {
                maze.change_cell_state(&self.current_cell, CellState::Solution);
                self.current_cell = *self.explored.get(&self.current_cell).unwrap();
                if self.current_cell == self.start {
                    self.status = SolverStatus::Done;
                }
            }
            SolverStatus::Done => {
                panic!("You shouldn't be here");
            }
        }
    }

    fn status(&self) -> &SolverStatus {
        &self.status
    }

    fn reset(&self, maze: &mut Signal<Maze>) {
        let maze = &mut maze.write();
        reset_solver(maze);
    }
}

impl BreadthFirstSearch {
    pub fn new(start: &Coord, finish: &Coord) -> Self {
        BreadthFirstSearch {
            start: *start,
            finish: *finish,
            frontier: VecDeque::new(),
            explored: HashMap::new(),
            current_cell: *start,
            status: SolverStatus::Initialized,
        }
    }

    fn add_adjacent_cells_to_frontier(&mut self, maze: &mut Maze) {
        let cell = *maze.get_cell_ref(&self.current_cell);

        if !cell.walls()[0] && !self.explored.contains_key(&Coord{ y: cell.coord().y - 1, x: cell.coord().x }) {
            let new_frontier_cell = Coord{ y: cell.coord().y - 1, x: cell.coord().x };
            self.process_frontier_cell(maze, new_frontier_cell);
        }
        if !cell.walls()[1] && !self.explored.contains_key(&Coord{ y: cell.coord().y, x: cell.coord().x + 1 }) {
            let new_frontier_cell = Coord{ y: cell.coord().y, x: cell.coord().x + 1 };
            self.process_frontier_cell(maze, new_frontier_cell);
        }
        if !cell.walls()[2] && !self.explored.contains_key(&Coord{ y: cell.coord().y + 1, x: cell.coord().x }) {
            let new_frontier_cell = Coord{ y: cell.coord().y + 1, x: cell.coord().x };
            self.process_frontier_cell(maze, new_frontier_cell);
        }
        if !cell.walls()[3] && !self.explored.contains_key(&Coord{ y: cell.coord().y, x: cell.coord().x - 1 }) {
            let new_frontier_cell = Coord{ y: cell.coord().y, x: cell.coord().x - 1 };
            self.process_frontier_cell(maze, new_frontier_cell);
        }
    }

    fn process_frontier_cell(&mut self, maze: &mut Maze, new_frontier_cell: Coord) {
        self.explored.insert(new_frontier_cell, self.current_cell);
        self.frontier.push_back(new_frontier_cell);
        if new_frontier_cell != self.start && new_frontier_cell != self.finish {
            maze.change_cell_state(&new_frontier_cell, CellState::Frontier);
        }
    }
}