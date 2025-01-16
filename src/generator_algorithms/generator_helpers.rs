use rand::{thread_rng, Rng};

use crate::maze::Maze;
use crate::cell::Coord;

#[derive(PartialEq)]
pub enum GeneratorStatus {
    Initialized,
    InProgress,
    Done,
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