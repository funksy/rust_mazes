use rand::{thread_rng, Rng};

use crate::maze::Maze;

pub mod random_prim;
pub mod recursive_backtracker;

pub trait GeneratorAlgorithm {
    fn create_maze(maze: &mut Maze);
}

//chooses a random cell within the Maze
pub fn random_grid_position (maze: &Maze) -> (usize, usize) {
    let start_row = thread_rng().gen_range(0..maze.height);
    let start_col = thread_rng().gen_range(0..maze.width);
    (start_row, start_col)
}

//removes the common wall between the indicated cell and the one in the indicated direction from that cell
pub fn remove_walls_between_cells(maze: &mut Maze, (frontier_cell_y, frontier_cell_x): (usize, usize), direction: usize) {
    match direction {
        0 => {
            maze.remove_cell_wall(frontier_cell_y, frontier_cell_x, "top");
            maze.remove_cell_wall(frontier_cell_y - 1, frontier_cell_x, "bottom");
        }
        1 => {
            maze.remove_cell_wall(frontier_cell_y, frontier_cell_x, "right");
            maze.remove_cell_wall(frontier_cell_y, frontier_cell_x + 1, "left");
        }
        2 => {
            maze.remove_cell_wall(frontier_cell_y, frontier_cell_x, "bottom");
            maze.remove_cell_wall(frontier_cell_y + 1, frontier_cell_x, "top");
        }
        3 => {
            maze.remove_cell_wall(frontier_cell_y, frontier_cell_x, "left");
            maze.remove_cell_wall(frontier_cell_y, frontier_cell_x - 1, "right");
        }
        _ => {}
    }
}