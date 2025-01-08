use std::io::{self, Write};
use crate::cell::{Cell, CellState, Coord};

pub struct Maze {
    height: usize,
    width: usize,
    grid: Vec<Cell>,
    pub frame: String,
}

impl Maze {
    pub fn new(height: usize, width: usize) -> Self {
        let mut grid = Vec::new();

        // let (height, width) = Self::get_maze_dimensions();

        for y in 0..height {
            for x in 0..width {
                grid.push(Cell::new(Coord{ y, x }));
            }
        }

        Self {
            height,
            width,
            grid,
            frame: "".to_string()
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn grid(&self) -> &Vec<Cell> {
        &self.grid
    }

    pub fn get_cell_ref(&self, coord: &Coord) -> &Cell {
        &self.grid[coord.y * self.width + coord.x]
    }

    pub fn visit_cell(&mut self, coord: &Coord) {
        self.grid[coord.y * self.width + coord.x].visit();
    }

    pub fn remove_cell_wall(&mut self, coord: &Coord, wall: &str) {
        self.grid[coord.y * self.width + coord.x].remove_wall(wall);
    }

    pub fn change_cell_state(&mut self, coord: &Coord, new_state: CellState) {
        self.grid[coord.y * self.width + coord.x].change_state(new_state);
    }

    fn update_frame(&mut self) {
        let mut top = "┏━".to_string();
        for x in 0..(self.width - 1) {
            top.push(if self.get_cell_ref(&Coord{ y: 0, x }).walls()[1] {'┳'} else {'━'});
            top.push('━');
        }
        top.push('┓');

        let mut rows = Vec::new();
        for y in 0..(self.height - 1) {
            let mut row = if self.get_cell_ref(&Coord{ y, x: 0 }).walls()[2] {
                "┣━".to_string()
            } else {
                "┃ ".to_string()
            };
            for x in 1..self.width {
                row.push(
                    self.get_inner_junction(
                        self.get_cell_ref(&Coord{ y, x: x - 1 }),
                        self.get_cell_ref(&Coord{ y: y + 1, x }),
                    )
                );
                row.push(if self.get_cell_ref(&Coord{ y, x }).walls()[2] {'━'} else {' '});
            }
            row.push(if self.get_cell_ref(&Coord{ y, x: self.width - 1 }).walls()[2] {
                '┫'
            } else {
                '┃'
            });
            rows.push(row);
        }

        let mut bot = "┗━".to_string();
        for x in 0..(self.width - 1) {
            bot.push(if self.get_cell_ref(&Coord{ y: self.height - 1, x }).walls()[1] {'┻'} else {'━'});
            bot.push('━');
        }
        bot.push('┛');

        self.frame = top;
        self.frame.push_str("\n");
        for row in rows {
            self.frame.push_str(&row);
            self.frame.push_str("\n");
        }
        self.frame.push_str(&bot);
    }

    pub fn show(&mut self) {
        self.update_frame();
        println!("{}", self.frame);
    }

    fn get_inner_junction(&self, top_left_cell: &Cell, bottom_right_cell: &Cell) -> char {
        let lookup: i8 =
            ((top_left_cell.walls()[2] as i8) << 3 | (top_left_cell.walls()[1] as i8) << 1) |
            ((bottom_right_cell.walls()[0] as i8) << 2 | (bottom_right_cell.walls()[3] as i8));

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