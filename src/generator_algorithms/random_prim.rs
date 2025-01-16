use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

use dioxus::prelude::*;
use indexmap::IndexSet;

use crate::maze::Maze;
use crate::cell::{CellState, Coord};
use crate::generator_algorithms::generator_helpers::{GeneratorStatus, random_grid_position, remove_walls_between_cells};

pub struct RandomPrim {
    frontier: IndexSet<Coord>,
    pub status: GeneratorStatus,
}

impl RandomPrim {
    pub fn new() -> Self {
        let frontier = IndexSet::new();

        RandomPrim {
            frontier,
            status: GeneratorStatus::Initialized,
        }
    }

    pub fn create_maze(&mut self, maze: &mut Signal<Maze>) {
        let maze: &mut Maze = &mut maze.write();

        match self.status {
            GeneratorStatus::Initialized => {
                let start: Coord = random_grid_position(&maze);
                maze.visit_cell(&start);
                self.add_cells_to_frontier(maze, &start);
                self.status = GeneratorStatus::InProgress;
            }
            GeneratorStatus::InProgress => {
                let rand_frontier = self.rand_frontier();
                let direction_of_rand_visited_neighbor: usize = self.choose_rand_neighbor(maze, &rand_frontier);
                remove_walls_between_cells(maze, &rand_frontier, direction_of_rand_visited_neighbor);
                maze.visit_cell(&rand_frontier);
                self.add_cells_to_frontier(maze, &rand_frontier);
                if self.frontier.len() == 0 {
                    self.status = GeneratorStatus::Done;
                }
            }
            GeneratorStatus::Done => {
                panic!("You shouldn't be here");
            }
        }
    }

    fn add_cells_to_frontier(&mut self, maze: &mut Maze, origin: &Coord) {
        let mut new_frontier_cells: Vec<Coord> = Vec::new();

        if origin.y > 0 {
            if maze.get_cell_ref(&Coord{ y: origin.y - 1, x: origin.x }).visited() == false {
                maze.change_cell_state(&Coord{ y: origin.y - 1, x: origin.x }, CellState::Frontier);
                new_frontier_cells.push(Coord{ y: origin.y - 1, x: origin.x })
            }
        }
        if origin.y < maze.height() - 1 {
            if maze.get_cell_ref(&Coord{ y: origin.y + 1, x: origin.x }).visited() == false {
                maze.change_cell_state(&Coord{ y: origin.y + 1, x: origin.x }, CellState::Frontier);
                new_frontier_cells.push(Coord{ y: origin.y + 1, x: origin.x });
            }
        }
        if origin.x > 0 {
            if maze.get_cell_ref(&Coord{ y: origin.y, x: origin.x - 1 }).visited() == false {
                maze.change_cell_state(&Coord{ y: origin.y, x: origin.x - 1 }, CellState::Frontier);
                new_frontier_cells.push(Coord{ y: origin.y, x: origin.x - 1 });
            }
        }
        if origin.x < maze.width() - 1 {
            if maze.get_cell_ref(&Coord{ y: origin.y, x: origin.x + 1 }).visited() == false {
                maze.change_cell_state(&Coord{ y: origin.y, x: origin.x + 1 }, CellState::Frontier);
                new_frontier_cells.push(Coord{ y: origin.y, x: origin.x + 1 });
            }
        }

        for cell in new_frontier_cells {
            self.frontier.insert(cell);
        }
    }

    fn rand_frontier (&mut self) -> Coord {
        self.frontier.swap_remove_index(thread_rng().gen_range(0..self.frontier.len())).unwrap()
    }

    fn choose_rand_neighbor(&self, maze: &Maze, frontier_cell: &Coord) -> usize {
        let mut directions = [0, 1, 2, 3];
        directions.shuffle(&mut thread_rng());

        for direction in directions {
            if direction == 0 && frontier_cell.y > 0 {
                if maze.get_cell_ref(&Coord{ y: frontier_cell.y - 1, x: frontier_cell.x }).visited() {
                    return direction;
                }
            }
            if direction == 1 && frontier_cell.x < maze.width() - 1 {
                if maze.get_cell_ref(&Coord{ y: frontier_cell.y, x: frontier_cell.x + 1 }).visited() {
                    return direction;
                }
            }
            if direction == 2 && frontier_cell.y < maze.height() - 1 {
                if maze.get_cell_ref(&Coord{ y: frontier_cell.y + 1, x: frontier_cell.x }).visited() {
                    return direction;
                }
            }
            if direction == 3 && frontier_cell.x > 0 {
                if maze.get_cell_ref(&Coord{ y: frontier_cell.y, x: frontier_cell.x - 1 }).visited() {
                    return direction;
                }
            }
        }
        0
    }
}