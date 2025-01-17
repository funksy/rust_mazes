use std::collections::{HashMap, VecDeque};

use dioxus::prelude::*;

use crate::maze::Maze;
use crate::cell::{CellState, Coord};
use crate::solver_algorithms::solver_helpers::SolverStatus;

pub struct BreadthFirstSearch {
    start: Coord,
    finish: Coord,
    frontier: VecDeque<Coord>,
    explored: HashMap<Coord, Coord>,
    current_cell: Coord,
    pub status: SolverStatus,
}

impl BreadthFirstSearch {
    pub fn new(start: &Coord, finish: &Coord) -> Self {
        let frontier: VecDeque<Coord> = VecDeque::new();
        let explored: HashMap<Coord, Coord> = HashMap::new();
        let start = start.clone();
        let finish = finish.clone();
        let current_cell = start.clone();

        BreadthFirstSearch {
            start,
            finish,
            frontier,
            explored,
            current_cell,
            status: SolverStatus::Initialized,
        }
    }

    pub fn find_solution(&mut self, maze: &mut Signal<Maze>) {
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
                self.add_adjacent_cells(maze, &self.current_cell.clone());
                if self.explored.contains_key(&self.finish) {
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

    fn add_adjacent_cells(&mut self, maze: &mut Maze, current_cell: &Coord) {
        let cell = *maze.get_cell_ref(current_cell);
        if !cell.walls()[0] && !self.explored.contains_key(&Coord{ y: cell.coord().y - 1, x: cell.coord().x }) {
            let new_frontier_cell = Coord{ y: cell.coord().y - 1, x: cell.coord().x };
            self.explored.insert(new_frontier_cell, current_cell.clone());
            self.frontier.push_back(new_frontier_cell);
            if new_frontier_cell != self.start && new_frontier_cell != self.finish {
                maze.change_cell_state(&new_frontier_cell, CellState::Frontier);
            }
        }
        if !cell.walls()[1] && !self.explored.contains_key(&Coord{ y: cell.coord().y, x: cell.coord().x + 1 }) {
            let new_frontier_cell = Coord{ y: cell.coord().y, x: cell.coord().x + 1 };
            self.explored.insert(new_frontier_cell, current_cell.clone());
            self.frontier.push_back(new_frontier_cell);
            if new_frontier_cell != self.start && new_frontier_cell != self.finish {
                maze.change_cell_state(&new_frontier_cell, CellState::Frontier);
            }
        }
        if !cell.walls()[2] && !self.explored.contains_key(&Coord{ y: cell.coord().y + 1, x: cell.coord().x }) {
            let new_frontier_cell = Coord{ y: cell.coord().y + 1, x: cell.coord().x };
            self.explored.insert(new_frontier_cell, current_cell.clone());
            self.frontier.push_back(new_frontier_cell);
            if new_frontier_cell != self.start && new_frontier_cell != self.finish {
                maze.change_cell_state(&new_frontier_cell, CellState::Frontier);
            }
        }
        if !cell.walls()[3] && !self.explored.contains_key(&Coord{ y: cell.coord().y, x: cell.coord().x - 1 }) {
            let new_frontier_cell = Coord{ y: cell.coord().y, x: cell.coord().x - 1 };
            self.explored.insert(new_frontier_cell, current_cell.clone());
            self.frontier.push_back(new_frontier_cell);
            if new_frontier_cell != self.start && new_frontier_cell != self.finish {
                maze.change_cell_state(&new_frontier_cell, CellState::Frontier);
            }
        }
    }
}