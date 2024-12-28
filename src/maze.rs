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

        let mut top = "┏━".to_string();
        for col in 0..(self.width - 1) {
            top.push(if self.get_cell_ref(0, col).walls[1] {'┳'} else {'━'});
            top.push('━');
        }
        top.push('┓');
        println!("\n{}", top);

        for y in 0..(self.height - 1) {
            let mut row = if self.get_cell_ref(y, 0).walls[2] {
                "┣━".to_string()
            } else {
                "┃ ".to_string()
            };
            for x in 1..self.width {
                row.push(
                    self.get_inner_junction(
                        self.get_cell_ref(y, x - 1),
                        self.get_cell_ref(y + 1, x),
                    )
                );
                row.push(if self.get_cell_ref(y, x).walls[2] {'━'} else {' '});
            }
            row.push(if self.get_cell_ref(y, self.width - 1).walls[2] {
                '┫'
            } else {
                '┃'
            });
            println!("{}", row);
        }

        let mut bot = "┗━".to_string();
        for col in 0..(self.width - 1) {
            bot.push(if self.get_cell_ref(self.height - 1, col).walls[1] {'┻'} else {'━'});
            bot.push('━');
        }
        bot.push('┛');
        println!("{}\n", bot);
    }

    fn get_inner_junction(&self, top_left_cell: &Cell, bottom_right_cell: &Cell) -> char {
        let lookup: i8 =
            ((top_left_cell.walls[2] as i8) << 3 | (top_left_cell.walls[1] as i8) << 1) |
            ((bottom_right_cell.walls[0] as i8) << 2 | (bottom_right_cell.walls[3] as i8));

        [' ', '╻', '╹', '┃', '╺', '┏', '┗', '┣',
            '╸', '┓', '┛', '┫', '━', '┳', '┻', '╋'][lookup as usize]
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