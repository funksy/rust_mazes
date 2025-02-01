use std::collections::HashMap;
use dioxus::prelude::*;
use rand::Rng;
use crate::cell::{Cell, Coord};
use crate::generator_algorithms::generator_helpers::{GeneratorAlgo, GeneratorStatus};
use crate::maze::Maze;

pub struct Ellers {
    current_row: usize,
    set_identifier: usize,
    sets: HashMap<usize, Vec<Coord>>,
    cells: HashMap<Coord, usize>,
    status: GeneratorStatus,
}

impl GeneratorAlgo for Ellers {
    fn create_maze(&mut self, maze: &mut Signal<Maze>) {
        let maze: &mut Maze = &mut maze.write();

        match self.status {
            GeneratorStatus::Initialized => {}
            GeneratorStatus::InProgress => {}
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
        //for each cell, if it does not already belong to a set, assign it a new set
        for x in 0..maze.width() {
            let cell_coord = Coord{ x, y: self.current_row };
            maze.visit_cell(&cell_coord);
            if !self.cells.contains_key(&cell_coord) {
                self.sets.insert(self.set_identifier, vec![cell_coord]);
                self.cells.insert(cell_coord, self.set_identifier);
                self.set_identifier += 1;
            }
        }

        //for each pair of cells that belong to different sets, randomly join them
        for x in 0..(maze.width() - 1) {
            let current_cell = Coord{ x, y: self.current_row };
            let next_cell = Coord{ x: x + 1, y: self.current_row };

            if self.cells.get(&current_cell) != self.cells.get(&next_cell) {
                self.randomly_join_cells(current_cell, next_cell, 50);
            }
        }

        //if this is not the last row, process vertical connections
        if self.current_row < maze.width() - 1 {}
        //for each group of cells that belong to the same set, randomly add the cell below it.
        //ensure that each group has at least one of these connections

        //if this is the last row, make sure all cells are merged into the same set
        else {}
    }

    fn randomly_join_cells(&mut self, first_cell: Coord, second_cell: Coord, weight: usize) {
        if weight > rand::thread_rng().gen_range(0..100) {
            let first_set_id = match self.cells.get(&first_cell).cloned() {
                Some(first_set_id) => first_set_id,
                None => panic!("There should be an entry here")
            };
            let second_set_id = match self.cells.get(&second_cell).cloned() {
                Some(second_set_id) => second_set_id,
                None => panic!("There should be an entry here")
            };
            let second_set_cells = match self.sets.get(&second_set_id).cloned() {
                Some(second_set_cells) => second_set_cells,
                None => panic!("There should be an entry here")
            };
            self.cells.insert(second_cell, first_set_id);
            self.sets.entry(first_set_id).and_modify(|existing_cells| existing_cells.extend(second_set_cells));
            self.sets.remove(&second_set_id);
        }
    }
}