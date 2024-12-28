use std::io::{self, Write};
use crate::cell::Cell;

pub struct Maze {
    pub height: usize,
    pub width: usize,
    pub grid: Vec<Cell>,
}

impl Maze {
    pub fn new() -> Self {
        let (height, width): (usize, usize) = Self::get_maze_dimensions();

        let mut grid = Vec::new();
        for y in 0..height {
            for x in 0..width {
                grid.push(Cell::new(x, y));
            }
        }

        Maze {
            height,
            width,
            grid,
        }
    }

    pub fn get_cell_ref(&self, row: usize, col: usize) -> &Cell {
        &self.grid[row * self.width + col]
    }

    pub fn visit_cell(&mut self, row: usize, col: usize) {
        self.grid[row * self.width + col].visit();
    }

    pub fn remove_cell_wall(&mut self,  row: usize, col: usize, wall: usize) {
        self.grid[row * self.width + col].remove_wall(wall);
    }

    pub fn show(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.get_cell_ref(row, col).visited == false {
                    print!("0");
                } else {
                    print!("1");
                }
            }
            println!();
        }
    }

    fn get_maze_dimensions() -> (usize, usize) {
        let rows: usize;
        let cols: usize;

        loop {
            print!("Enter the desired number of rows: ");
            io::stdout().flush().unwrap();

            let mut row_input = String::new();
            io::stdin()
                .read_line(&mut row_input)
                .expect("Failed to read line.");

            match row_input.trim().parse() {
                Ok(num) => {
                    rows = num;
                    break;
                },
                Err(_) => println!("\nPlease enter a valid positive number!\n"),
            }
        }

        loop {
            print!("Enter the desired number of cols: ");
            io::stdout().flush().unwrap();

            let mut col_input = String::new();
            io::stdin()
                .read_line(&mut col_input)
                .expect("Failed to read line.");

            match col_input.trim().parse() {
                Ok(num) => {
                    cols = num;
                    break;
                },
                Err(_) => println!("\nPlease enter a valid positive number!\n"),
            }
        }

        (rows, cols)
    }
}