use std::collections::{HashMap, HashSet};
use rand::{Rng};
use rand::seq::SliceRandom;
use dioxus::prelude::*;
use crate::maze::Maze;
use crate::cell::Coord;
use crate::generator_algorithms::generator_helpers::{remove_walls_between_cells, GeneratorAlgo, GeneratorStatus};

pub struct Ellers {
    current_row: usize,
    current_col: usize,
    set_identifier: usize,
    sets_needing_vertical_connection: HashSet<usize>,
    sets: HashMap<usize, Vec<Coord>>,
    cells: HashMap<Coord, usize>,
    stage: Stage,
    status: GeneratorStatus,
}

enum AddCell {
    NewSet,
    AddToSet,
}

enum Stage {
    Horizontal,
    Vertical,
    LastRow
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
            current_col: 0,
            set_identifier: 0,
            sets_needing_vertical_connection: HashSet::new(),
            sets: HashMap::new(),
            cells: HashMap::new(),
            stage: Stage::Horizontal,
            status: GeneratorStatus::Initialized,
        }
    }

    fn process_maze_row(&mut self, maze: &mut Maze) {
        match self.stage {
            Stage::Horizontal => {
                let mut rng = rand::thread_rng();

                if self.current_col < maze.width() {
                    let current_cell = &Coord{ x: self.current_col, y: self.current_row };
                    if !self.cells.contains_key(current_cell) {
                        maze.visit_cell(current_cell);
                        self.add_new_cell(current_cell, 0, AddCell::NewSet);
                    }
                    if self.current_col > 0 {
                        let previous_cell = &Coord{ x: self.current_col - 1, y: self.current_row };
                        if self.cells.get(current_cell) != self.cells.get(previous_cell) && rng.gen_bool(0.5) {
                            self.merge_cell_sets(current_cell, previous_cell);
                            remove_walls_between_cells(maze, current_cell, 3);
                        }
                    }
                    self.current_col += 1;
                }
                else {
                    self.current_col = 0;
                    if self.current_row < maze.height() - 1 {
                        self.sets_needing_vertical_connection = self.sets.keys().copied().collect();
                        self.stage = Stage::Vertical;
                    }
                    else {
                        self.stage = Stage::LastRow;
                    }
                }
            },
            Stage::Vertical => {
                let mut rng = rand::thread_rng();

                if self.current_col < maze.width() {
                    if rng.gen_bool(0.5) {
                        let current_cell = &Coord { x: self.current_col, y: self.current_row };
                        let next_cell = &Coord { x: self.current_col, y: self.current_row + 1 };

                        let current_set_id = *self.cells.get(current_cell)
                            .expect("No self.cells entry for current_cell");

                        maze.visit_cell(next_cell);
                        self.add_new_cell(next_cell, current_set_id, AddCell::AddToSet);
                        remove_walls_between_cells(maze, current_cell, 2);

                        self.sets_needing_vertical_connection.remove(&current_set_id);
                    }
                    self.current_col += 1;
                }
                else if self.sets_needing_vertical_connection.len() > 0 {
                    let (set_id) = match self.sets_needing_vertical_connection.iter().next().cloned() {
                        Some(set_id) => set_id,
                        None => panic!("No value in sets_needing_vertical_connection")
                    };
                    self.sets_needing_vertical_connection.remove(&set_id);

                    let cells = match self.sets.get(&set_id) {
                        Some(cells) => cells.clone(),
                        None => panic!("no entry in sets")
                    };
                    let current_row_cells: Vec<&Coord> = cells.iter()
                        .filter(|cell| cell.y == self.current_row)
                        .collect();
                    let random_cell = match current_row_cells.choose(&mut rng) {
                        Some(coord) => coord,
                        None => panic!("No cell in current_row_cells")
                    };

                    let next_cell = Coord{ x: random_cell.x, y: self.current_row + 1 };
                    maze.visit_cell(&next_cell);
                    self.add_new_cell(&next_cell, set_id, AddCell::AddToSet);
                    remove_walls_between_cells(maze, random_cell, 2);
                }
                else {
                    self.current_col = 0;
                    self.current_row += 1;
                    self.stage = Stage::Horizontal;
                }
            },
            Stage::LastRow => {
                if self.current_col < maze.width() - 1 {
                    let current_cell = &Coord { x: self.current_col, y: self.current_row };
                    let next_cell = &Coord { x: self.current_col + 1, y: self.current_row };
                    if self.cells.get(current_cell) != self.cells.get(next_cell) {
                        self.merge_cell_sets(current_cell, next_cell);
                        remove_walls_between_cells(maze, current_cell, 1);
                    }
                    self.current_col += 1;
                }
                else {
                    self.current_row += 1;
                }
            }
        }
    }

    #[inline]
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

    #[inline]
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