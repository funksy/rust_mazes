use std::collections::{HashMap};

use dioxus::prelude::*;

use crate::maze::Maze;
use crate::cell::Coord;
use crate::solver_algorithms::solver_helpers::SolverStatus;

pub struct Djikstras {
    start: Coord,
    finish: Coord,
    explored: HashMap<Coord, Coord>,
    current_cell: Coord,
    pub status: SolverStatus,
}

impl Djikstras {
    pub fn new(start: &Coord, finish: &Coord) -> Self {
        Djikstras {
            start: start.clone(),
            finish: finish.clone(),
            explored: HashMap::new(),
            current_cell: start.clone(),
            status: SolverStatus::Initialized,
        }
    }

    pub fn find_solution(&mut self, maze: &mut Signal<Maze>) {
        let maze: &mut Maze = &mut maze.write();

        match self.status {
            SolverStatus::Initialized => {}
            SolverStatus::InProgress => {}
            SolverStatus::Solved => {}
            SolverStatus::Done => {
                panic!("You shouldn't be here");
            }
        }
    }
}