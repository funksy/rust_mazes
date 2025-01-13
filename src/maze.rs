use std::collections::HashSet;
use crate::cell::{Cell, CellState, Coord};

#[derive(PartialEq, Clone)]
pub struct SvgRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub fill: String,
    pub stroke: String,
}

#[derive(Debug)]
enum WallDirection {
    Vertical,
    Horizontal,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct SvgLine {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

#[derive(PartialEq, Clone)]
pub struct SvgRender {
    pub cells: Vec<SvgRect>,
    pub vert_walls: Vec<HashSet<SvgLine>>,
    pub horiz_walls: Vec<HashSet<SvgLine>>
}

pub struct Maze {
    height: usize,
    width: usize,
    grid: Vec<Cell>,
    svg: SvgRender,
}

const CELL_SIZE: i32 = 3;

impl Maze {
    pub fn new(height: usize, width: usize) -> Self {
        let mut grid = Vec::new();
        let mut cells: Vec<SvgRect> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                grid.push(Cell::new(Coord{ y, x }));
                cells.push(SvgRect {
                    x: x as i32 * CELL_SIZE,
                    y: y as i32 * CELL_SIZE,
                    width: CELL_SIZE,
                    height: CELL_SIZE,
                    fill: "lightgrey".to_string(),
                    stroke: "lightgrey".to_string(),
                });
            }
        }

        let mut horiz_walls: Vec<HashSet<SvgLine>> = Vec::new();
        for y in 0..=height {
            let horiz_wall = SvgLine {
                x1: 0,
                y1: y as i32 * CELL_SIZE,
                x2: width as i32 * CELL_SIZE,
                y2: y as i32 * CELL_SIZE,
            };
            let mut new_horiz_wall_set = HashSet::new();
            new_horiz_wall_set.insert(horiz_wall);
            horiz_walls.push(new_horiz_wall_set);
        }

        let mut vert_walls: Vec<HashSet<SvgLine>> = Vec::new();
        for x in 0..=width {
            let vert_wall = SvgLine {
                x1: x as i32 * CELL_SIZE,
                y1: 0,
                x2: x as i32 * CELL_SIZE,
                y2: height as i32 * CELL_SIZE,
            };
            let mut new_vert_wall_set = HashSet::new();
            new_vert_wall_set.insert(vert_wall);
            vert_walls.push(new_vert_wall_set);
        }

        Self {
            height,
            width,
            grid,
            svg: SvgRender {
                cells,
                horiz_walls,
                vert_walls,
            }
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

    pub fn svg_render(&self) -> &SvgRender {
        &self.svg
    }

    pub fn get_cell_ref(&self, coord: &Coord) -> &Cell {
        &self.grid[coord.y * self.width + coord.x]
    }

    pub fn visit_cell(&mut self, coord: &Coord) {
        self.grid[coord.y * self.width + coord.x].visit();
        self.svg.cells[coord.y * self.width + coord.x].fill = self.get_cell_color(&self.get_cell_ref(coord).state());
        self.svg.cells[coord.y * self.width + coord.x].stroke = self.get_cell_color(&self.get_cell_ref(coord).state());
    }

    pub fn change_cell_state(&mut self, coord: &Coord, new_state: CellState) {
        self.grid[coord.y * self.width + coord.x].change_state(new_state);
        self.svg.cells[coord.y * self.width + coord.x].fill = self.get_cell_color(&self.get_cell_ref(coord).state());
        self.svg.cells[coord.y * self.width + coord.x].stroke = self.get_cell_color(&self.get_cell_ref(coord).state());
    }

    fn get_cell_color(&self, cell_state: &CellState) -> String {
        match cell_state {
            CellState::Unvisited => "lightgrey".to_string(),
            CellState::Path => "white".to_string(),
            CellState::Frontier => "yellow".to_string(),
            CellState::Solution => "dodgerblue".to_string(),
            CellState::Start => "green".to_string(),
            CellState::Finish => "red".to_string(),
        }
    }

    pub fn remove_cell_wall(&mut self, coord: &Coord, wall_side: &str) {
        self.grid[coord.y * self.width + coord.x].remove_wall(wall_side);

        match wall_side {
            "top" => {
                let wall_to_remove = SvgLine {
                    x1: coord.x as i32 * CELL_SIZE,
                    y1: coord.y as i32 * CELL_SIZE,
                    x2: coord.x as i32 * CELL_SIZE + CELL_SIZE,
                    y2: coord.y as i32 * CELL_SIZE
                };

                let (containing_wall, new_walls) = self.split_wall(&wall_to_remove, WallDirection::Horizontal, coord);
                self.svg.horiz_walls[coord.y].remove(&containing_wall);
                self.svg.horiz_walls[coord.y].extend(&new_walls);
            },
            "right" => {
                let wall_to_remove = SvgLine {
                    x1: coord.x as i32 * CELL_SIZE + CELL_SIZE,
                    y1: coord.y as i32 * CELL_SIZE,
                    x2: coord.x as i32 * CELL_SIZE + CELL_SIZE,
                    y2: coord.y as i32 * CELL_SIZE + CELL_SIZE,
                };

                let (containing_wall, new_walls) = self.split_wall(&wall_to_remove, WallDirection::Vertical, coord);
                self.svg.vert_walls[coord.x + 1].remove(&containing_wall);
                self.svg.vert_walls[coord.x + 1].extend(&new_walls);
            },
            _ => {}
        }
    }

    fn split_wall(&self, wall_to_remove: &SvgLine, wall_direction: WallDirection, cell_coord: &Coord) -> (SvgLine, Vec<SvgLine>) {
        let (walls_vec, walls_vec_i): (&Vec<HashSet<SvgLine>>, usize) = match wall_direction {
            WallDirection::Vertical => (&self.svg.vert_walls, cell_coord.x + 1),
            WallDirection::Horizontal => (&self.svg.horiz_walls, cell_coord.y),
        };

        let containing_wall = match walls_vec[walls_vec_i]
            .iter()
            .find(|containing_wall| self.contains_wall(&wall_to_remove, containing_wall))
            .cloned() {
            Some(wall) => wall,
            None => panic!("Containing wall doesn't exist."),
        };

        let mut new_walls: Vec<SvgLine> = Vec::new();

        if !(containing_wall.x1 == wall_to_remove.x1 && containing_wall.y1 == wall_to_remove.y1) {
            let new_line_1 = SvgLine {
                x1: containing_wall.x1,
                y1: containing_wall.y1,
                x2: wall_to_remove.x1,
                y2: wall_to_remove.y1,
            };
            new_walls.push(new_line_1);
        }

        if !(containing_wall.x2 == wall_to_remove.x2 && containing_wall.y2 == wall_to_remove.y2) {
            let new_line_2 = SvgLine {
                x1: wall_to_remove.x2,
                y1: wall_to_remove.y2,
                x2: containing_wall.x2,
                y2: containing_wall.y2,
            };
            new_walls.push(new_line_2);
        }

        (containing_wall, new_walls)
    }

    fn contains_wall(&self, inside_wall: &SvgLine, containing_wall: &SvgLine) -> bool {
        let x_in_range = (inside_wall.x1 >= containing_wall.x1) && (inside_wall.x2 <= containing_wall.x2);
        let y_in_range = (inside_wall.y1 >= containing_wall.y1) && (inside_wall.y2 <= containing_wall.y2);

        x_in_range && y_in_range
    }
}