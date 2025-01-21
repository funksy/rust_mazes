use dioxus::prelude::*;

use crate::maze::Maze;
use crate::cell::{Coord};
use crate::generator_algorithms::generator_helpers::{choose_rand_neighbor, random_grid_position, remove_walls_between_cells, GeneratorAlgo, GeneratorStatus};

pub struct RecursiveBacktracker {
    stack: Vec<Coord>,
    pub status: GeneratorStatus,
}

impl RecursiveBacktracker {
    pub fn new() -> Self {
        let stack: Vec<Coord> = Vec::new();

        RecursiveBacktracker {
            stack,
            status: GeneratorStatus::Initialized,
        }
    }
}

impl GeneratorAlgo for RecursiveBacktracker {
    fn create_maze(&mut self, maze: &mut Signal<Maze>) {
        let maze: &mut Maze = &mut maze.write();

        match self.status {
            GeneratorStatus::Initialized => {
                let start: Coord = random_grid_position(maze);
                maze.visit_cell(&start);
                self.stack.push(start);
                self.status = GeneratorStatus::InProgress;
            }
            GeneratorStatus::InProgress => {
                if self.stack.len() > 0 {
                    let current_cell = match self.stack.pop() {
                        Some(coord) => coord,
                        None => panic!("why isn't there a value here?")
                    };
                    match choose_rand_neighbor(&maze, &current_cell, false) {
                        Ok(dir) => match dir {
                            0 => {
                                self.stack.push(current_cell);
                                remove_walls_between_cells(maze, &current_cell, dir);
                                let next_cell = Coord { x: current_cell.x, y: current_cell.y - 1 };
                                maze.visit_cell(&next_cell);
                                self.stack.push(next_cell);
                            },
                            1 => {
                                self.stack.push(current_cell);
                                remove_walls_between_cells(maze, &current_cell, dir);
                                let next_cell = Coord { x: current_cell.x + 1, y: current_cell.y };
                                maze.visit_cell(&next_cell);
                                self.stack.push(next_cell);
                            },
                            2 => {
                                self.stack.push(current_cell);
                                remove_walls_between_cells(maze, &current_cell, dir);
                                let next_cell = Coord { x: current_cell.x, y: current_cell.y + 1 };
                                maze.visit_cell(&next_cell);
                                self.stack.push(next_cell);
                            },
                            _ => {
                                self.stack.push(current_cell);
                                remove_walls_between_cells(maze, &current_cell, dir);
                                let next_cell = Coord { x: current_cell.x - 1, y: current_cell.y };
                                maze.visit_cell(&next_cell);
                                self.stack.push(next_cell);
                            },
                        }
                        Err(_) => {}
                    }
            }
            else {
            self.status = GeneratorStatus::Done;
                }
            }
            GeneratorStatus::Done => {
                panic!("You shouldn't be here");
            }
        }
    }

    fn status(&self) -> &GeneratorStatus {
        &self.status
    }
}