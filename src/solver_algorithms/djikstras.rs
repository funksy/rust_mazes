use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

use dioxus::prelude::*;

use crate::structures::maze::Maze;
use crate::structures::cell::{CellState, Coord};
use crate::solver_algorithms::solver_helpers::{reset_solver, solved, SolverAlgo, SolverStatus};

#[derive(Copy, Clone, Eq, PartialEq)]
struct DistanceToStart {
    cell_coord: Coord,
    distance: usize,
}

impl Ord for DistanceToStart {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for DistanceToStart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Djikstras {
    start: Coord,
    finish: Coord,
    explored: HashMap<Coord, Coord>,
    frontier: BinaryHeap<DistanceToStart>,
    current_cell: Coord,
    status: SolverStatus,
}

impl SolverAlgo for Djikstras {
    fn find_solution(&mut self, maze: &mut Signal<Maze>) {
        let maze: &mut Maze = &mut maze.write();

        match self.status {
            SolverStatus::Initialized => {
                maze.change_cell_state(&self.start, CellState::Start);
                maze.change_cell_state(&self.finish, CellState::Finish);
                self.frontier.push(DistanceToStart {
                    cell_coord: self.start,
                    distance: 0,
                });
                self.status = SolverStatus::InProgress;
            }
            SolverStatus::InProgress => {
                println!("Frontier has {} coords", self.frontier.len());
                let temp: DistanceToStart = self.frontier.pop().unwrap();
                self.current_cell = temp.cell_coord;
                let distance: usize = temp.distance;
                self.add_adjacent_cells_to_frontier(maze, distance);
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
                    println!("Solved?");
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
impl Djikstras {
    pub fn new(start: &Coord, finish: &Coord) -> Self {
        Djikstras {
            start: *start,
            finish: *finish,
            explored: HashMap::new(),
            frontier: BinaryHeap::new(),
            current_cell: *start,
            status: SolverStatus::Initialized,
        }
    }

    fn add_adjacent_cells_to_frontier(&mut self, maze: &mut Maze, distance: usize) {
        let cell = *maze.get_cell_ref(&self.current_cell);

        if !cell.walls()[0] && !self.explored.contains_key(&Coord{ y: cell.coord().y - 1, x: cell.coord().x }) {
            let new_frontier_cell = Coord{ y: cell.coord().y - 1, x: cell.coord().x };
            self.explored.insert(new_frontier_cell, self.current_cell);
            self.frontier.push(DistanceToStart{ cell_coord: new_frontier_cell, distance: distance + 1 });
            if new_frontier_cell != self.start && new_frontier_cell != self.finish {
                maze.change_cell_state(&new_frontier_cell, CellState::Frontier);
            }
        }
        if !cell.walls()[1] && !self.explored.contains_key(&Coord{ y: cell.coord().y, x: cell.coord().x + 1 }) {
            let new_frontier_cell = Coord{ y: cell.coord().y, x: cell.coord().x + 1 };
            self.explored.insert(new_frontier_cell, self.current_cell);
            self.frontier.push(DistanceToStart{ cell_coord: new_frontier_cell, distance: distance + 1 });
            if new_frontier_cell != self.start && new_frontier_cell != self.finish {
                maze.change_cell_state(&new_frontier_cell, CellState::Frontier);
            }
        }
        if !cell.walls()[2] && !self.explored.contains_key(&Coord{ y: cell.coord().y + 1, x: cell.coord().x }) {
            let new_frontier_cell = Coord{ y: cell.coord().y + 1, x: cell.coord().x };
            self.explored.insert(new_frontier_cell, self.current_cell);
            self.frontier.push(DistanceToStart{ cell_coord: new_frontier_cell, distance: distance + 1 });
            if new_frontier_cell != self.start && new_frontier_cell != self.finish {
                maze.change_cell_state(&new_frontier_cell, CellState::Frontier);
            }
        }
        if !cell.walls()[3] && !self.explored.contains_key(&Coord{ y: cell.coord().y, x: cell.coord().x - 1 }) {
            let new_frontier_cell = Coord{ y: cell.coord().y, x: cell.coord().x - 1 };
            self.explored.insert(new_frontier_cell, self.current_cell);
            self.frontier.push(DistanceToStart{ cell_coord: new_frontier_cell, distance: distance + 1 });
            if new_frontier_cell != self.start && new_frontier_cell != self.finish {
                maze.change_cell_state(&new_frontier_cell, CellState::Frontier);
            }
        }
    }
}