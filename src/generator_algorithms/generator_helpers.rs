use dioxus::prelude::Signal;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

use crate::maze::Maze;
use crate::cell::Coord;
use crate::generator_algorithms::random_prim::RandomPrim;
use crate::generator_algorithms::recursive_backtracker::RecursiveBacktracker;

#[derive(PartialEq)]
pub enum GeneratorStatus {
    Initialized,
    InProgress,
    Done,
}

pub trait GeneratorAlgo {
    fn create_maze(&mut self, maze: &mut Signal<Maze>);

    fn status(&self) -> &GeneratorStatus;
}


pub fn get_generator_algo(algo: &str) -> Box<dyn GeneratorAlgo> {
    match algo {
        "random_prim" => Box::new(RandomPrim::new()),
        "recursive_backtracker" => Box::new(RecursiveBacktracker::new()),
        _ => panic!("you shouldn't be here"),
    }
}

pub fn random_grid_position (maze: &Maze) -> Coord {
    let y = thread_rng().gen_range(0..maze.height());
    let x = thread_rng().gen_range(0..maze.width());
    Coord {
        y,
        x,
    }
}

pub fn remove_walls_between_cells(maze: &mut Maze, frontier_cell: &Coord, direction: usize) {
    match direction {
        0 => {
            maze.remove_cell_wall(frontier_cell, "top");
            maze.remove_cell_wall(&Coord{ y: frontier_cell.y - 1, x: frontier_cell.x }, "bottom");
        }
        1 => {
            maze.remove_cell_wall(frontier_cell, "right");
            maze.remove_cell_wall(&Coord{ y: frontier_cell.y, x: frontier_cell.x + 1 }, "left");
        }
        2 => {
            maze.remove_cell_wall(frontier_cell, "bottom");
            maze.remove_cell_wall(&Coord{ y: frontier_cell.y + 1, x: frontier_cell.x }, "top");
        }
        3 => {
            maze.remove_cell_wall(frontier_cell, "left");
            maze.remove_cell_wall(&Coord{ y: frontier_cell.y, x: frontier_cell.x - 1 }, "right");
        }
        _ => {}
    }
}

pub fn choose_rand_neighbor(maze: &Maze, frontier_cell: &Coord, visited_status: bool) -> Result<usize, &'static str> {
    let mut directions = [0, 1, 2, 3];
    directions.shuffle(&mut thread_rng());

    for direction in directions {
        if direction == 0 && frontier_cell.y > 0 {
            if maze.get_cell_ref(&Coord{ y: frontier_cell.y - 1, x: frontier_cell.x }).visited() == visited_status {
                return Ok(direction);
            }
        }
        if direction == 1 && frontier_cell.x < maze.width() - 1 {
            if maze.get_cell_ref(&Coord{ y: frontier_cell.y, x: frontier_cell.x + 1 }).visited() == visited_status {
                return Ok(direction);
            }
        }
        if direction == 2 && frontier_cell.y < maze.height() - 1 {
            if maze.get_cell_ref(&Coord{ y: frontier_cell.y + 1, x: frontier_cell.x }).visited() == visited_status {
                return Ok(direction);
            }
        }
        if direction == 3 && frontier_cell.x > 0 {
            if maze.get_cell_ref(&Coord{ y: frontier_cell.y, x: frontier_cell.x - 1 }).visited() == visited_status {
                return Ok(direction);
            }
        }
    }
    Err("no neighboring cells in that state")
}