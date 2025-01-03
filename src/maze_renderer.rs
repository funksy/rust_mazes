use std::collections::HashSet;

use svg::Document;
use svg::node::element::{Group, Line, Rectangle};
use crate::maze::Maze;
use crate::cell::Coord;

#[derive(Hash, Eq, PartialEq)]
struct Wall {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

#[derive(Debug)]
pub enum CellState {
    Normal,
    Frontier,
    Explored,
    Solution,
    Start,
    Finish,
}

struct Cell {
    x: i32,
    y: i32,
    state: CellState,
}

pub struct MazeRenderer {
    cells: Vec<Cell>,
    walls: HashSet<Wall>,
    maze_height: i32,
    maze_width: i32,
}

const CELL_SIZE: i32 = 3;

impl MazeRenderer {
    pub fn new(maze: &Maze) -> Self {
        let mut cells: Vec<Cell> = Vec::new();
        let mut walls: HashSet<Wall> = HashSet::new();

        let left_border = Wall {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: maze.height() as i32 * CELL_SIZE
        };
        walls.insert(left_border);

        let bottom_border = Wall {
            x1: 0,
            y1: maze.height() as i32 * CELL_SIZE,
            x2: maze.width() as i32 * CELL_SIZE,
            y2: maze.height() as i32 * CELL_SIZE
        };
        walls.insert(bottom_border);

        for cell in maze.grid() {
            let x = cell.coord().x as i32;
            let y = cell.coord().y as i32;

            cells.push(Cell { x, y, state: CellState::Normal });

            let top_wall = Wall {
                x1: x * CELL_SIZE,
                y1: y * CELL_SIZE,
                x2: x * CELL_SIZE + CELL_SIZE,
                y2: y * CELL_SIZE
            };
            walls.insert(top_wall);

            let right_wall = Wall {
                x1: x * CELL_SIZE + CELL_SIZE,
                y1: y * CELL_SIZE,
                x2: x * CELL_SIZE + CELL_SIZE,
                y2: y * CELL_SIZE + CELL_SIZE,
            };
            walls.insert(right_wall);
        }

        MazeRenderer {
            cells,
            walls,
            maze_width: maze.width() as i32,
            maze_height: maze.height() as i32,
        }
    }

    pub fn update_walls(&mut self, maze: &Maze) {
        for cell in maze.grid() {
            let x = cell.coord().x as i32;
            let y = cell.coord().y as i32;

            if !cell.walls()[0] {
                let top_wall = Wall {
                    x1: x * CELL_SIZE,
                    y1: y * CELL_SIZE,
                    x2: x * CELL_SIZE + CELL_SIZE,
                    y2: y * CELL_SIZE
                };
                self.walls.remove(&top_wall);
            }

            if !cell.walls()[1] {
                let right_wall = Wall {
                    x1: x * CELL_SIZE + CELL_SIZE,
                    y1: y * CELL_SIZE,
                    x2: x * CELL_SIZE + CELL_SIZE,
                    y2: y * CELL_SIZE + CELL_SIZE,
                };
                self.walls.remove(&right_wall);
            }
        }
    }

    fn render_wall_group(&self) -> Group {
        let mut group = Group::new().set("id", "g_cell_wall");
        for wall in &self.walls {
            let line = Line::new()
                .set("x1", wall.x1)
                .set("y1", wall.y1)
                .set("x2", wall.x2)
                .set("y2", wall.y2)
                .set("stroke", "black")
                .set("stroke-width", 0.5)
                .set("stroke-linecap", "square");
            group = group.add(line);
        }
        group
    }

    fn render_cell_group(&self) -> Group {
        let mut group = Group::new().set("id", "g_cell_body");
        for cell in &self.cells {
            let color = match &cell.state {
                CellState::Normal => "white",
                CellState::Frontier => "yellow",
                CellState::Explored=> "orange",
                CellState::Solution=> "lightcoral",
                CellState::Start=> "green",
                CellState::Finish=> "red",
            };

            let rectangle = Rectangle::new()
                .set("x", cell.x * CELL_SIZE)
                .set("y", cell.y * CELL_SIZE)
                .set("width", CELL_SIZE)
                .set("height", CELL_SIZE)
                .set("fill", color)
                .set("stroke", color)
                .set("stroke-width", "0.1%");
            group = group.add(rectangle);
        }
        group
    }

    pub fn update_cell_state(&mut self, cell: &Coord, new_state: CellState) {
        self.cells[cell.y * self.maze_width as usize + cell.x].state = new_state;
    }

    pub fn generate_document(&self) -> Document {
        let view_box = (
            -CELL_SIZE,
            -CELL_SIZE,
            self.maze_width * CELL_SIZE + 2 * CELL_SIZE,
            self.maze_height * CELL_SIZE + CELL_SIZE * 2
        );
        let document = Document::new()
            .set("viewBox", view_box)
            .add(self.render_cell_group())
            .add(self.render_wall_group());
        document
    }
}