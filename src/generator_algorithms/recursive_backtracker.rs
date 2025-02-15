use dioxus::prelude::*;

use crate::structures::maze::Maze;
use crate::structures::cell::{Coord};
use crate::generator_algorithms::generator_helpers::{choose_rand_neighbor, random_grid_position, remove_walls_between_cells, GeneratorAlgo, GeneratorStatus};

pub struct RecursiveBacktracker {
    stack: Vec<Coord>,
    status: GeneratorStatus,
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
                if !self.stack.is_empty() {
                    let current_cell = match self.stack.pop() {
                        Some(coord) => coord,
                        None => panic!("why isn't there a value here?")
                    };
                    if let Ok(dir) = choose_rand_neighbor(maze, &current_cell, false) { match dir {
                            0 => self.next_cell(maze, current_cell, Coord{ x: current_cell.x, y: current_cell.y - 1 }, dir),
                            1 => self.next_cell(maze, current_cell, Coord{ x: current_cell.x + 1, y: current_cell.y }, dir),
                            2 => self.next_cell(maze, current_cell, Coord{ x: current_cell.x, y: current_cell.y + 1 }, dir),
                            _ => self.next_cell(maze, current_cell, Coord{ x: current_cell.x - 1, y: current_cell.y }, dir),
                        }
                    } }
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

impl RecursiveBacktracker {
    pub fn new() -> Self {
        let stack: Vec<Coord> = Vec::new();

        RecursiveBacktracker {
            stack,
            status: GeneratorStatus::Initialized,
        }
    }

    fn next_cell(&mut self, maze: &mut Maze, current_cell: Coord, next_cell: Coord, dir: usize) {
        self.stack.push(current_cell);
        remove_walls_between_cells(maze, &current_cell, dir);
        maze.visit_cell(&next_cell);
        self.stack.push(next_cell);
    }
}