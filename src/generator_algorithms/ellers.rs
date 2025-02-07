use std::collections::{HashMap, HashSet};
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

enum AddCell {
    NewSet,
    AddToSet,
}

impl GeneratorAlgo for Ellers {
    fn create_maze(&mut self, maze: &mut Signal<Maze>) {
        let maze: &mut Maze = &mut maze.write();

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

    fn status(&self) -> &GeneratorStatus {
        &self.status
    }
}

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

    fn process_maze_row(&mut self, maze: &mut Maze) {
        let mut rng = rand::thread_rng();

        for x in 0..maze.width() {
            let current_cell = &Coord{ x, y: self.current_row };
            if !self.cells.contains_key(current_cell) {
                maze.visit_cell(current_cell);
                self.add_new_cell(current_cell, 0, AddCell::NewSet);
            }

            if x > 0 {
                let previous_cell = &Coord{ x: x - 1, y: self.current_row };
                if self.cells.get(current_cell) != self.cells.get(previous_cell) && rng.gen_bool(0.5) {
                    self.merge_cell_sets(current_cell, previous_cell);
                    remove_walls_between_cells(maze, current_cell, 3);
                }
            }
        }

        if self.current_row < maze.height() - 1 {
            let mut sets_needing_connection: HashSet<usize> = self.sets.keys().copied().collect();

            for x in 0..maze.width() {
                if rng.gen_bool(0.5) {
                    let current_cell = &Coord { x, y: self.current_row };
                    let next_cell = &Coord { x, y: self.current_row + 1 };

                    let current_set_id = *self.cells.get(current_cell)
                        .expect("No self.cells entry for current_cell");

                    maze.visit_cell(next_cell);
                    self.add_new_cell(next_cell, current_set_id, AddCell::AddToSet);
                    remove_walls_between_cells(maze, current_cell, 2);

                    sets_needing_connection.remove(&current_set_id);
                }
            }

            let connections_needed: Vec<(usize, Coord)> = sets_needing_connection
                .iter()
                .filter_map(|&set_id| {
                    let cells = self.sets.get(&set_id)?;
                    let current_row_cells: Vec<&Coord> = cells.iter()
                        .filter(|cell| cell.y == self.current_row)
                        .collect();
                    current_row_cells.choose(&mut rng)
                        .map(|&cell| (set_id, cell.clone()))
                })
                .collect();

            for (set_id, random_cell) in connections_needed {
                let next_cell = Coord {
                    x: random_cell.x,
                    y: self.current_row + 1
                };
                maze.visit_cell(&next_cell);
                self.add_new_cell(&next_cell, set_id, AddCell::AddToSet);
                remove_walls_between_cells(maze, &random_cell, 2);
            }
        }

        else {
            for x in 0..maze.width() - 1 {
                let current_cell = &Coord { x, y: self.current_row };
                let next_cell = &Coord { x: x + 1, y: self.current_row };
                if self.cells.get(current_cell) != self.cells.get(next_cell) {
                    self.merge_cell_sets(current_cell, next_cell);
                    remove_walls_between_cells(maze, current_cell, 1);
                }
            }
        }
        self.current_row += 1;
    }

    fn add_new_cell(&mut self, cell_coord: &Coord, set_id: usize, add_type: AddCell) {
        match add_type {
            AddCell::NewSet => {
                self.sets.insert(self.set_identifier, vec![*cell_coord]);
                self.cells.insert(*cell_coord, self.set_identifier);
                self.set_identifier += 1;
            },
            AddCell::AddToSet => {
                self.sets.entry(set_id).and_modify(|existing_cells| existing_cells.push(*cell_coord));
                self.cells.insert(*cell_coord, set_id);
            }
        }
    }

    fn merge_cell_sets(&mut self, first_cell: &Coord, second_cell: &Coord) {
        let first_set_id = *self.cells.get(first_cell).
            expect("No self.cells entry for first_cell");
        let second_set_id = *self.cells.get(second_cell).
            expect("No self.cells entry for second_cell");
        let second_set_cells = self.sets.remove(&second_set_id)
            .expect("No self.sets entry for second_set_id");

        for cell in &second_set_cells {
            self.cells.insert(*cell, first_set_id);
        }

        self.sets.entry(first_set_id).and_modify(|existing_cells| existing_cells.extend(second_set_cells));
        self.sets.remove(&second_set_id);
    }
}