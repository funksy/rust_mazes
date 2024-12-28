use indexmap::IndexSet;
use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;
use crate::maze::Maze;

pub fn create_maze(maze: &mut Maze) {
    let mut frontier: IndexSet<(usize, usize)> = IndexSet::new();

    {
        let start: (usize, usize) = find_start(maze);
        maze.visit_cell(start.0, start.1);
        add_cells_to_frontier(&maze, (start.0, start.1), &mut frontier);
    }

    while frontier.len() > 0 {
        let rand_frontier = rand_frontier(&mut frontier);
        let direction_of_rand_visited_neighbor: usize = choose_rand_neighbor(&maze, rand_frontier);
        remove_walls_between_cells(maze, rand_frontier, direction_of_rand_visited_neighbor);
        maze.visit_cell(rand_frontier.0, rand_frontier.1);
        add_cells_to_frontier(&maze, rand_frontier, &mut frontier);
    }
}

//chooses a random cell within the Maze to act as the starting point
fn find_start (maze: &Maze) -> (usize, usize) {
    let start_row = thread_rng().gen_range(0..maze.height);
    let start_col = thread_rng().gen_range(0..maze.width);
    (start_row, start_col)
}

//add appropriate adjacent cells to the frontier Vec
fn add_cells_to_frontier(maze: &Maze, cell_coords: (usize, usize), frontier: &mut IndexSet<(usize, usize)>) {
    let mut new_frontier_cells: Vec<(usize, usize)> = Vec::new();

    if cell_coords.0 > 0 {
        if maze.get_cell_ref(cell_coords.0 - 1, cell_coords.1).visited == false {
            new_frontier_cells.push((cell_coords.0 - 1, cell_coords.1))
        }
    }
    if cell_coords.0 < maze.height - 1 {
        if maze.get_cell_ref(cell_coords.0 + 1, cell_coords.1).visited == false {
            new_frontier_cells.push((cell_coords.0 + 1, cell_coords.1));
        }
    }
    if cell_coords.1 > 0 {
        if maze.get_cell_ref(cell_coords.0 , cell_coords.1 - 1).visited == false {
            new_frontier_cells.push((cell_coords.0, cell_coords.1 - 1));
        }
    }
    if cell_coords.1 < maze.width - 1 {
        if maze.get_cell_ref(cell_coords.0 , cell_coords.1 + 1).visited == false {
            new_frontier_cells.push((cell_coords.0, cell_coords.1 + 1));
        }
    }

    for cell in new_frontier_cells {
        frontier.insert((cell.0, cell.1));
    }
}

fn rand_frontier (frontier: &mut IndexSet<(usize, usize)>) -> (usize, usize) {
    frontier.swap_remove_index(thread_rng().gen_range(0..frontier.len())).unwrap()
}

// chooses a random cell adjacent to the cell indicated, respecting the boundaries of the provided Maze
fn choose_rand_neighbor(maze: &Maze, frontier_cell_coords: (usize, usize)) -> usize {
    let mut directions = [0, 1, 2, 3];
    directions.shuffle(&mut thread_rng());

    for direction in directions {
        if direction == 0 && frontier_cell_coords.0 > 0 {
            if maze.get_cell_ref(frontier_cell_coords.0 - 1, frontier_cell_coords.1).visited {
                return direction;
            }
        }
        if direction == 1 && frontier_cell_coords.1 < maze.width - 1 {
            if maze.get_cell_ref(frontier_cell_coords.0, frontier_cell_coords.1 + 1).visited {
                return direction;
            }
        }
        if direction == 2 && frontier_cell_coords.0 < maze.height - 1 {
            if maze.get_cell_ref(frontier_cell_coords.0 + 1, frontier_cell_coords.1).visited {
                return direction;
            }
        }
        if direction == 3 && frontier_cell_coords.1 > 0 {
            if maze.get_cell_ref(frontier_cell_coords.0, frontier_cell_coords.1 - 1).visited {
                return direction;
            }
        }
    }
    0
}

fn remove_walls_between_cells(maze: &mut Maze, frontier_cell: (usize, usize), direction: usize) {
    match direction {
        0 => {
            maze.remove_cell_wall(frontier_cell.0, frontier_cell.1, 0);
            maze.remove_cell_wall(frontier_cell.0 - 1, frontier_cell.1, 2);
        }
        1 => {
            maze.remove_cell_wall(frontier_cell.0, frontier_cell.1, 1);
            maze.remove_cell_wall(frontier_cell.0, frontier_cell.1 + 1, 3);
        }
        2 => {
            maze.remove_cell_wall(frontier_cell.0, frontier_cell.1, 2);
            maze.remove_cell_wall(frontier_cell.0 + 1, frontier_cell.1, 0);
        }
        3 => {
            maze.remove_cell_wall(frontier_cell.0, frontier_cell.1, 3);
            maze.remove_cell_wall(frontier_cell.0, frontier_cell.1 - 1, 1);
        }
        _ => {}
    }
}