use std::collections::{HashMap, VecDeque};

use crate::maze::Maze;
use crate::maze_renderer::{MazeRenderer, CellState};

pub fn find_solution(maze: &Maze, (start_y, start_x): (usize, usize), (finish_y, finish_x): (usize, usize), renderer: &mut MazeRenderer) {
    let mut frontier: VecDeque<(usize, usize)> = VecDeque::new();
    let mut explored: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    frontier.push_back((start_y, start_x));

    while !explored.contains_key(&(finish_y, finish_x)) {
        let current_cell = frontier.pop_front().unwrap();
        add_adjacent_cells(&maze, &mut explored, &mut frontier, current_cell);
    }

    renderer.update_cell_state(start_y, start_x, CellState::Start);
    renderer.update_cell_state(finish_y, finish_x, CellState::Finish);
    let (mut solution_cell_y, mut solution_cell_x) = *explored.get(&(finish_y, finish_x)).unwrap();
    while (solution_cell_y, solution_cell_x) != (start_y, start_x) {
        renderer.update_cell_state(solution_cell_y, solution_cell_x, CellState::Solution);
        (solution_cell_y, solution_cell_x) = *explored.get(&(solution_cell_y, solution_cell_x)).unwrap();
    }
}

fn add_adjacent_cells(maze: &Maze, explored: &mut HashMap<(usize, usize), (usize, usize)>, frontier: &mut VecDeque<(usize, usize)>,(cell_y, cell_x): (usize, usize)) {
    let cell = maze.get_cell_ref(cell_y, cell_x);
    if !cell.walls[0] && !explored.contains_key(&(cell_y - 1, cell_x)) {
        explored.insert((cell_y - 1, cell_x),(cell_y, cell_x));
        frontier.push_back((cell_y - 1, cell_x));
    }
    if !cell.walls[1] && !explored.contains_key(&(cell_y, cell_x + 1)) {
        explored.insert((cell_y, cell_x + 1),(cell_y, cell_x));
        frontier.push_back((cell_y, cell_x + 1));
    }
    if !cell.walls[2] && !explored.contains_key(&(cell_y + 1, cell_x)) {
        explored.insert((cell_y + 1, cell_x),(cell_y, cell_x));
        frontier.push_back((cell_y + 1, cell_x));
    }
    if !cell.walls[3] && !explored.contains_key(&(cell_y, cell_x - 1)) {
        explored.insert((cell_y, cell_x - 1),(cell_y, cell_x));
        frontier.push_back((cell_y, cell_x - 1));
    }
}