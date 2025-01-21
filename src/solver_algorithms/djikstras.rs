use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;

use dioxus::prelude::*;

use crate::maze::Maze;
use crate::cell::{CellState, Coord};
use crate::solver_algorithms::solver_helpers::SolverStatus;

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
    current_cell: DistanceToStart,
    pub status: SolverStatus,
}

impl Djikstras {
    pub fn new(start: &Coord, finish: &Coord) -> Self {
        Djikstras {
            start: start.clone(),
            finish: finish.clone(),
            explored: HashMap::new(),
            frontier: BinaryHeap::new(),
            current_cell: DistanceToStart {
                cell_coord: start.clone(),
                distance: 0,
            },
            status: SolverStatus::Initialized,
        }
    }

    pub fn find_solution(&mut self, maze: &mut Signal<Maze>) {
        let maze: &mut Maze = &mut maze.write();

        match self.status {
            SolverStatus::Initialized => {
                maze.visit_cell(&self.start);
                self.frontier.push(DistanceToStart {
                    cell_coord: self.start,
                    distance: 0,
                });
                self.status = SolverStatus::InProgress;
            }
            SolverStatus::InProgress => {
                if self.current_cell.cell_coord != self.finish {
                    self.current_cell = self.frontier.pop().unwrap();
                    // TODO collect valid neighbor cells
                    // TODO add entries to explored and frontier
                } else {
                    self.status == SolverStatus::Solved;
                }
            }
            SolverStatus::Solved => {}
            SolverStatus::Done => {
                panic!("You shouldn't be here");
            }
        }
    }

    fn get_valid_neighbors(&self, maze: &Maze) -> Vec<Coord> {
        let mut valid_neighbors: Vec<Coord> = Vec::new();
        
    }
}