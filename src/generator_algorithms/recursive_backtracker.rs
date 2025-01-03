use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::maze::Maze;
use crate::generator_algorithms::{random_grid_position, remove_walls_between_cells};

const DIR_X: [isize; 4] = [0, 1, 0, -1];
const DIR_Y: [isize; 4] = [-1, 0, 1, 0];

pub fn create_maze(maze: &mut Maze) {
    let (start_y, start_x): (usize, usize) = random_grid_position(maze);
    carve_passages_from(start_y, start_x, maze);
}

fn carve_passages_from(cell_y: usize, cell_x: usize, maze: &mut Maze) {
    maze.visit_cell(cell_y, cell_x);

    let mut directions = [0, 1, 2, 3];
    directions.shuffle(&mut thread_rng());

    for direction in directions {
        let next_cell_y = (cell_y as isize + DIR_Y[direction]) as usize;
        let next_cell_x = (cell_x as isize + DIR_X[direction]) as usize;

        if (0..maze.height).contains(&next_cell_y) && (0..maze.width).contains(&next_cell_x) && !maze.get_cell_ref(next_cell_y, next_cell_x).visited {
            remove_walls_between_cells(maze ,(cell_y, cell_x), direction);
            carve_passages_from(next_cell_y, next_cell_x, maze)
        }
    }
}