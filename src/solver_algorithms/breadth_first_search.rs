use std::collections::{HashMap, VecDeque};

use crate::maze::Maze;
use crate::cell::Coord;
use crate::maze_renderer::{MazeRenderer, CellState};

pub fn find_solution(maze: &Maze, start: &Coord, finish: &Coord, renderer: &mut MazeRenderer) {
    let mut frontier: VecDeque<Coord> = VecDeque::new();
    let mut explored: HashMap<Coord, Coord> = HashMap::new();

    frontier.push_back(start.clone());

    while !explored.contains_key(&finish) {
        let current_cell = frontier.pop_front().unwrap();

        // renderer.update_cell_state(current_cell_y, current_cell_x, CellState::Explored);

        add_adjacent_cells(&maze, &mut explored, &mut frontier, &current_cell, renderer);
    }

    renderer.update_cell_state(start, CellState::Start);
    renderer.update_cell_state(finish, CellState::Finish);

    let mut solution_cell = *explored.get(&finish).unwrap();
    while solution_cell != *start {
        renderer.update_cell_state(&solution_cell, CellState::Solution);
        solution_cell = *explored.get(&solution_cell).unwrap();
    }
}

fn add_adjacent_cells(maze: &Maze, explored: &mut HashMap<Coord, Coord>, frontier: &mut VecDeque<Coord>, current_cell: &Coord, renderer: &mut MazeRenderer) {
    let cell = maze.get_cell_ref(current_cell);
    if !cell.walls()[0] && !explored.contains_key(&Coord{ y: cell.coord().y - 1, x: cell.coord().x }) {
        explored.insert(Coord{ y: cell.coord().y - 1, x: cell.coord().x }, current_cell.clone());
        frontier.push_back(Coord{ y: cell.coord().y - 1, x: cell.coord().x });
        // renderer.update_cell_state(cell_y - 1, cell_x, CellState::Frontier);
    }
    if !cell.walls()[1] && !explored.contains_key(&Coord{ y: cell.coord().y, x: cell.coord().x + 1 }) {
        explored.insert(Coord{ y: cell.coord().y, x: cell.coord().x + 1 }, current_cell.clone());
        frontier.push_back(Coord{ y: cell.coord().y, x: cell.coord().x + 1 });
        // renderer.update_cell_state(cell_y , cell_x + 1, CellState::Frontier);
    }
    if !cell.walls()[2] && !explored.contains_key(&Coord{ y: cell.coord().y + 1, x: cell.coord().x }) {
        explored.insert(Coord{ y: cell.coord().y + 1, x: cell.coord().x }, current_cell.clone());
        frontier.push_back(Coord{ y: cell.coord().y + 1, x: cell.coord().x });
        // renderer.update_cell_state(cell_y + 1, cell_x, CellState::Frontier);
    }
    if !cell.walls()[3] && !explored.contains_key(&Coord{ y: cell.coord().y, x: cell.coord().x - 1 }) {
        explored.insert(Coord{ y: cell.coord().y, x: cell.coord().x - 1 }, current_cell.clone());
        frontier.push_back(Coord{ y: cell.coord().y, x: cell.coord().x - 1 });
        // renderer.update_cell_state(cell_y, cell_x - 1, CellState::Frontier);
    }
}