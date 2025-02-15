use rand::{thread_rng, Rng};
use dioxus::prelude::*;
use indexmap::IndexSet;

use crate::maze::Maze;
use crate::cell::Coord;
use crate::generator_algorithms::generator_helpers::{GeneratorStatus, random_grid_position, remove_walls_between_cells, choose_rand_neighbor, GeneratorAlgo};

pub struct RandomPrim {
    frontier: IndexSet<Coord>,
    status: GeneratorStatus,
}

impl GeneratorAlgo for RandomPrim {
    fn create_maze(&mut self, maze: &mut Signal<Maze>) {
        let maze: &mut Maze = &mut maze.write();

        match self.status {
            GeneratorStatus::Initialized => {
                let start: Coord = random_grid_position(maze);
                maze.visit_cell(&start);
                self.add_cells_to_frontier(maze, &start);
                self.status = GeneratorStatus::InProgress;
            }
            GeneratorStatus::InProgress => {
                let rand_frontier = self.rand_frontier();
                let direction_of_rand_visited_neighbor: usize = choose_rand_neighbor(maze, &rand_frontier, true).unwrap();
                remove_walls_between_cells(maze, &rand_frontier, direction_of_rand_visited_neighbor);
                maze.visit_cell(&rand_frontier);
                self.add_cells_to_frontier(maze, &rand_frontier);
                if self.frontier.is_empty() {
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

impl RandomPrim {
    pub fn new() -> Self {
        let frontier = IndexSet::new();

        RandomPrim {
            frontier,
            status: GeneratorStatus::Initialized,
        }
    }

    fn add_cells_to_frontier(&mut self, maze: &mut Maze, origin: &Coord) {
        let mut new_frontier_cells: Vec<Coord> = Vec::new();

        if origin.y > 0 && !maze.get_cell_ref(&Coord{ y: origin.y - 1, x: origin.x }).visited() {
            new_frontier_cells.push(Coord{ y: origin.y - 1, x: origin.x })
        }
        if origin.y < maze.height() - 1 && !maze.get_cell_ref(&Coord{ y: origin.y + 1, x: origin.x }).visited() {
            new_frontier_cells.push(Coord{ y: origin.y + 1, x: origin.x });
        }
        if origin.x > 0 && !maze.get_cell_ref(&Coord{ y: origin.y, x: origin.x - 1 }).visited() {
            new_frontier_cells.push(Coord{ y: origin.y, x: origin.x - 1 });
        }
        if origin.x < maze.width() - 1 && !maze.get_cell_ref(&Coord{ y: origin.y, x: origin.x + 1 }).visited() {
            new_frontier_cells.push(Coord{ y: origin.y, x: origin.x + 1 });
        }

        for cell in new_frontier_cells {
            self.frontier.insert(cell);
        }
    }

    fn rand_frontier (&mut self) -> Coord {
        self.frontier.swap_remove_index(thread_rng().gen_range(0..self.frontier.len())).unwrap()
    }
}