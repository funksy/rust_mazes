use std::collections::HashMap;
use rand::{Rng};
use rand::seq::SliceRandom;
use dioxus::prelude::*;
use crate::maze::Maze;
use crate::cell::Coord;
use crate::generator_algorithms::generator_helpers::{remove_walls_between_cells, GeneratorAlgo, GeneratorStatus};

pub struct Ellers {
    current_row: usize,
    set_identifier: usize,
    sets: HashMap<usize, Vec<Coord>>,
    cells: HashMap<Coord, usize>,
    status: GeneratorStatus,
}

// impl GeneratorAlgo for Ellers {
//     fn create_maze(&mut self, maze: &mut Signal<Maze>) {
//         let maze: &mut Maze = &mut maze.write();
//
//         match self.status {
//             GeneratorStatus::Initialized => {
//                 self.process_maze_row(maze);
//                 self.status = GeneratorStatus::InProgress;
//             }
//             GeneratorStatus::InProgress => {
//                 self.process_maze_row(maze);
//                 if self.current_row == maze.width() {
//                     self.status = GeneratorStatus::Done;
//                 }
//             }
//             GeneratorStatus::Done => {
//                 panic!("You shouldn't be here");
//             }
//         }
//     }
//
//     fn status(&self) -> &GeneratorStatus {
//         &self.status
//     }
// }

impl Ellers {
    pub fn new() -> Self {
        Ellers {
            current_row: 0,
            set_identifier: 0,
            sets: HashMap::new(),
            cells: HashMap::new(),
            status: GeneratorStatus::Initialized,
        }
    }

    pub fn create_maze(&mut self, maze: &mut Maze) {
        // let maze: &mut Maze = &mut maze.write();

        match self.status {
            GeneratorStatus::Initialized => {
                self.process_maze_row(maze);
                self.status = GeneratorStatus::InProgress;
            }
            GeneratorStatus::InProgress => {
                self.process_maze_row(maze);
                if self.current_row == maze.height() {
                    self.status = GeneratorStatus::Done;
                }
            }
            GeneratorStatus::Done => {
                panic!("You shouldn't be here");
            }
        }
    }

    pub fn status(&self) -> &GeneratorStatus {
        &self.status
    }

    fn process_maze_row(&mut self, maze: &mut Maze) {
        //for each cell, if it does not already belong to a set, assign it a new set
        for x in 0..maze.width() {
            let cell_coord = &Coord{ x, y: self.current_row };
            if !self.cells.contains_key(&cell_coord) {
                maze.visit_cell(&cell_coord);
                self.add_new_cell(&cell_coord, false, 0);
            }
        }

        //for each pair of cells that belong to different sets, randomly join them
        for x in 0..(maze.width() - 1) {
            let current_cell = &Coord{ x, y: self.current_row };
            let next_cell = &Coord{ x: x + 1, y: self.current_row };

            if self.cells.get(current_cell) != self.cells.get(next_cell) {
                if self.randomly_join_cells(current_cell, next_cell, 50) {
                    // remove_walls_between_cells(maze, current_cell,1);
                    let first_set_id = match self.cells.get(current_cell).cloned() {
                        Some(first_set_id) => first_set_id,
                        None => panic!("There should be an entry here, line 96")
                    };
                    let second_set_id = match self.cells.get(next_cell).cloned() {
                        Some(second_set_id) => second_set_id,
                        None => panic!("There should be an entry here, line 100")
                    };
                    self.merge_sets(first_set_id, second_set_id);
                }
            }
        }

        //if this is not the last row, process vertical connections
        if self.current_row < maze.width() - 1 {
            let mut vert_connections: Vec<usize> = Vec::new();
            for x in 0..maze.width() {
                let current_cell = &Coord{ x, y: self.current_row };
                let next_cell = &Coord{ x, y: self.current_row + 1 };



                if self.randomly_join_cells(current_cell, next_cell, 50) {
                    let set_id = match self.cells.get(current_cell).cloned() {
                        Some(first_set_id) => first_set_id,
                        None => panic!("There should be an entry here, line 95")
                    };
                    maze.visit_cell(&next_cell);
                    remove_walls_between_cells(maze, current_cell, 2);
                    vert_connections.push(set_id);
                }
            }

            let mut sets_without_vert_connection: Vec<(Coord, usize)> = Vec::new();
            for (set, cells) in self.sets.iter_mut() {
                if !vert_connections.contains(set) {
                    let mut current_cell = &cells[0];

                    while current_cell.y != self.current_row {
                        current_cell = match cells.choose(&mut rand::thread_rng()) {
                            Some(cell_coord) => cell_coord,
                            None => panic!("There should be an entry here, line 113")
                        };
                    }
                    let next_cell = Coord{ x: current_cell.x, y: current_cell.y + 1 };
                    sets_without_vert_connection.push((next_cell, set.clone()));
                }
            }

            sets_without_vert_connection.iter().for_each(|(cell, set_id)| {
                maze.visit_cell(cell);
                remove_walls_between_cells(maze, cell, 2);
                self.add_new_cell(cell, true, set_id.clone());
            });
        }
        //if this is the last row, make sure all cells are merged into the same set

        else {
            for x in 0..maze.width() - 1 {
                let current_cell = &Coord{ x, y: self.current_row };
                let next_cell = &Coord{ x: x + 1, y: self.current_row };

                if self.cells.get(current_cell) != self.cells.get(next_cell) {
                    self.randomly_join_cells(current_cell, next_cell, 100);
                    remove_walls_between_cells(maze, current_cell, 1);
                    let first_set_id = match self.cells.get(current_cell).cloned() {
                        Some(first_set_id) => first_set_id,
                        None => panic!("There should be an entry here, line 163")
                    };
                    let second_set_id = match self.cells.get(next_cell).cloned() {
                        Some(second_set_id) => second_set_id,
                        None => panic!("There should be an entry here, line 167")
                    };
                    self.merge_sets(first_set_id, second_set_id);
                }
            }
        }
        self.current_row += 1;
    }

    fn add_new_cell(&mut self, cell_coord: &Coord, existing_set: bool, set_id: usize) {
        match existing_set {
            true => {
                self.sets.entry(set_id).and_modify(|existing_cells| existing_cells.push(cell_coord.clone()));
                self.cells.insert(cell_coord.clone(), set_id);
            },
            false => {
                self.sets.insert(self.set_identifier, vec![cell_coord.clone()]);
                self.cells.insert(cell_coord.clone(), self.set_identifier);
                self.set_identifier += 1;
            }
        }
    }

    fn randomly_join_cells(&mut self, first_cell: &Coord, second_cell: &Coord, weight: usize) -> bool {
        if weight > rand::thread_rng().gen_range(0..100) {
            let first_set_id = match self.cells.get(first_cell).cloned() {
                Some(first_set_id) => first_set_id,
                None => panic!("There should be an entry here, line 190")
            };
            self.add_new_cell(second_cell, true, first_set_id);
            true
        } else {
            false
        }
    }

    fn merge_sets(&mut self, first_set: usize, second_set: usize) {
        let second_set_cells = match self.sets.get(&second_set).cloned() {
            Some(second_set_cells) => second_set_cells,
            None => panic!("There should be an entry here, line 204, second:{second_set}")
        };
        self.sets.entry(first_set).and_modify(|existing_cells| existing_cells.extend(second_set_cells));
        self.sets.remove(&second_set);
    }
}