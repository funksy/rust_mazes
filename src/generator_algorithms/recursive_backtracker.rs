use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::maze::Maze;
use crate::cell::Coord;
use crate::generator_algorithms::generator_helpers::{random_grid_position, remove_walls_between_cells};

const DIR_X: [isize; 4] = [0, 1, 0, -1];
const DIR_Y: [isize; 4] = [-1, 0, 1, 0];

pub fn create_maze(maze: &mut Maze) {
    let start: Coord = random_grid_position(maze);
    carve_passages_from(&start, maze);
}

fn carve_passages_from(cell: &Coord, maze: &mut Maze) {
    maze.visit_cell(cell);

    let mut directions = [0, 1, 2, 3];
    directions.shuffle(&mut thread_rng());

    for direction in directions {
        let next_cell = Coord{
            y: (cell.y as isize + DIR_Y[direction]) as usize,
            x: (cell.x as isize + DIR_X[direction]) as usize
        };

        if (0..maze.height()).contains(&next_cell.y) && (0..maze.width()).contains(&next_cell.x) && !maze.get_cell_ref(&next_cell).visited() {
            remove_walls_between_cells(maze ,cell, direction);
            carve_passages_from(&next_cell, maze)
        }
    }
}