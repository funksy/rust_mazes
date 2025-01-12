use std::collections::{HashMap, VecDeque};

use dioxus::prelude::*;

use crate::maze::Maze;
use crate::cell::{CellState, Coord};

pub fn find_solution(maze: &mut Signal<Maze>, start: &Coord, finish: &Coord) {
    let maze: &mut Maze = &mut maze.write();

    maze.change_cell_state(start, CellState::Start);
    maze.change_cell_state(finish, CellState::Finish);

    let mut frontier: VecDeque<Coord> = VecDeque::new();
    let mut explored: HashMap<Coord, Coord> = HashMap::new();

    frontier.push_back(start.clone());

    while !explored.contains_key(&finish) {
        let current_cell = frontier.pop_front().unwrap();
        add_adjacent_cells(maze, &mut explored, &mut frontier, &current_cell);
    }

    let mut solution_cell = *explored.get(&finish).unwrap();
    while solution_cell != *start {
        maze.change_cell_state(&solution_cell, CellState::Solution);
        solution_cell = *explored.get(&solution_cell).unwrap();
    }
}

fn add_adjacent_cells(maze: &mut Maze, explored: &mut HashMap<Coord, Coord>, frontier: &mut VecDeque<Coord>, current_cell: &Coord) {
    let cell = *maze.get_cell_ref(current_cell);
    if !cell.walls()[0] && !explored.contains_key(&Coord{ y: cell.coord().y - 1, x: cell.coord().x }) {
        explored.insert(Coord{ y: cell.coord().y - 1, x: cell.coord().x }, current_cell.clone());
        frontier.push_back(Coord{ y: cell.coord().y - 1, x: cell.coord().x });
        // maze.change_cell_state(&Coord{ y: cell.coord().y - 1, x: cell.coord().x }, CellState::Frontier);
    }
    if !cell.walls()[1] && !explored.contains_key(&Coord{ y: cell.coord().y, x: cell.coord().x + 1 }) {
        explored.insert(Coord{ y: cell.coord().y, x: cell.coord().x + 1 }, current_cell.clone());
        frontier.push_back(Coord{ y: cell.coord().y, x: cell.coord().x + 1 });
        // maze.change_cell_state(&Coord{ y: cell.coord().y, x: cell.coord().x + 1 }, CellState::Frontier);
    }
    if !cell.walls()[2] && !explored.contains_key(&Coord{ y: cell.coord().y + 1, x: cell.coord().x }) {
        explored.insert(Coord{ y: cell.coord().y + 1, x: cell.coord().x }, current_cell.clone());
        frontier.push_back(Coord{ y: cell.coord().y + 1, x: cell.coord().x });
        // maze.change_cell_state(&Coord{ y: cell.coord().y + 1, x: cell.coord().x }, CellState::Frontier);
    }
    if !cell.walls()[3] && !explored.contains_key(&Coord{ y: cell.coord().y, x: cell.coord().x - 1 }) {
        explored.insert(Coord{ y: cell.coord().y, x: cell.coord().x - 1 }, current_cell.clone());
        frontier.push_back(Coord{ y: cell.coord().y, x: cell.coord().x - 1 });
        // maze.change_cell_state(&Coord{ y: cell.coord().y, x: cell.coord().x -1 }, CellState::Frontier);
    }
}