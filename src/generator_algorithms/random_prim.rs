use rand::{thread_rng, Rng};
use rand::seq::SliceRandom;

use indexmap::IndexSet;

use crate::maze::Maze;
use crate::generator_algorithms::{random_grid_position, remove_walls_between_cells};

//apply Prim's algorithm to an initialized Maze
pub fn create_maze(maze: &mut Maze) {
    //indexSet is a hash that is indexable, allowing for quick lookup characteristics of a hash
    //but still allowing for choosing one randomly using a random number generator
    let mut frontier: IndexSet<(usize, usize)> = IndexSet::new();

    //establish a starting cell, mark it as visited, and add it's adjacent cells to the frontier
    let (start_y, start_x): (usize, usize) = random_grid_position(maze);
    maze.visit_cell(start_y, start_x);
    add_cells_to_frontier(&maze, (start_y, start_x), &mut frontier);

    //main loop to continually apply the algorithm until no frontier cells are left
    //which means all cells have been visited
    while frontier.len() > 0 {
        let (rand_frontier_y, rand_frontier_x) = rand_frontier(&mut frontier);
        let direction_of_rand_visited_neighbor: usize = choose_rand_neighbor(&maze, (rand_frontier_y, rand_frontier_x));
        remove_walls_between_cells(maze, (rand_frontier_y, rand_frontier_x), direction_of_rand_visited_neighbor);
        maze.visit_cell(rand_frontier_y, rand_frontier_x);
        add_cells_to_frontier(&maze, (rand_frontier_y, rand_frontier_x), &mut frontier);
    }
}

//add appropriate adjacent cells to the frontier Vec
fn add_cells_to_frontier(maze: &Maze, (origin_cell_y, origin_cell_x): (usize, usize), frontier: &mut IndexSet<(usize, usize)>) {
    let mut new_frontier_cells: Vec<(usize, usize)> = Vec::new();

    if origin_cell_y > 0 {
        if maze.get_cell_ref(origin_cell_y - 1, origin_cell_x).visited == false {
            new_frontier_cells.push((origin_cell_y - 1, origin_cell_x))
        }
    }
    if origin_cell_y < maze.height - 1 {
        if maze.get_cell_ref(origin_cell_y + 1, origin_cell_x).visited == false {
            new_frontier_cells.push((origin_cell_y + 1, origin_cell_x));
        }
    }
    if origin_cell_x > 0 {
        if maze.get_cell_ref(origin_cell_y , origin_cell_x - 1).visited == false {
            new_frontier_cells.push((origin_cell_y, origin_cell_x - 1));
        }
    }
    if origin_cell_x < maze.width - 1 {
        if maze.get_cell_ref(origin_cell_y , origin_cell_x + 1).visited == false {
            new_frontier_cells.push((origin_cell_y, origin_cell_x + 1));
        }
    }

    for (cell_y, cell_x) in new_frontier_cells {
        frontier.insert((cell_y, cell_x));
    }
}

//chooses a random cell within the frontier
fn rand_frontier (frontier: &mut IndexSet<(usize, usize)>) -> (usize, usize) {
    frontier.swap_remove_index(thread_rng().gen_range(0..frontier.len())).unwrap()
}

// chooses a random cell adjacent to the cell indicated, respecting the boundaries of the provided Maze
fn choose_rand_neighbor(maze: &Maze, (frontier_cell_y, frontier_cell_x): (usize, usize)) -> usize {
    let mut directions = [0, 1, 2, 3];
    directions.shuffle(&mut thread_rng());

    for direction in directions {
        if direction == 0 && frontier_cell_y > 0 {
            if maze.get_cell_ref(frontier_cell_y - 1, frontier_cell_x).visited {
                return direction;
            }
        }
        if direction == 1 && frontier_cell_x < maze.width - 1 {
            if maze.get_cell_ref(frontier_cell_y, frontier_cell_x + 1).visited {
                return direction;
            }
        }
        if direction == 2 && frontier_cell_y < maze.height - 1 {
            if maze.get_cell_ref(frontier_cell_y + 1, frontier_cell_x).visited {
                return direction;
            }
        }
        if direction == 3 && frontier_cell_x > 0 {
            if maze.get_cell_ref(frontier_cell_y, frontier_cell_x - 1).visited {
                return direction;
            }
        }
    }
    0
}