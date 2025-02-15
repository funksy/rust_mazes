use std::collections::{HashSet, HashMap};
use rayon::prelude::*;

use crate::cell::{CellState, Coord};

#[derive(PartialEq)]
pub struct MazeSvg {
    pub cells: HashMap<(usize, usize), SvgRect>,
    pub vert_walls: Vec<HashSet<SvgLine>>,
    pub horiz_walls: Vec<HashSet<SvgLine>>,
}

#[derive(PartialEq, Clone)]
pub struct SvgRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub fill: String,
    pub stroke: String,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct SvgLine {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

#[derive(Debug)]
enum WallDirection {
    Vertical,
    Horizontal,
}

const CELL_SIZE: i32 = 3;

impl MazeSvg {
    pub fn new(height: usize, width: usize) -> Self {
        let mut cells: HashMap<(usize, usize), SvgRect> = HashMap::with_capacity(width * height);
        for y in 0..height {
            for x in 0..width {
                cells.insert((x, y), SvgRect {
                    x: x as i32 * CELL_SIZE,
                    y: y as i32 * CELL_SIZE,
                    width: CELL_SIZE,
                    height: CELL_SIZE,
                    fill: "lightgrey".to_string(),
                    stroke: "lightgrey".to_string(),
                });
            }
        }

        let mut horiz_walls: Vec<HashSet<SvgLine>> = Vec::with_capacity(width);
        for y in 0..=height {
            let mut horiz_wall_set: HashSet<SvgLine> = HashSet::new();
            horiz_wall_set.insert(SvgLine{
                x1: 0,
                y1: y as i32 * CELL_SIZE,
                x2: width as i32 * CELL_SIZE,
                y2: y as i32 * CELL_SIZE,
            });
            horiz_walls.push(horiz_wall_set);
        }

        let mut vert_walls: Vec<HashSet<SvgLine>> = Vec::with_capacity(height);
        for x in 0..=width {
            let mut vert_wall_set: HashSet<SvgLine> = HashSet::new();
            vert_wall_set.insert(SvgLine{
                x1: x as i32 * CELL_SIZE,
                y1: 0,
                x2: x as i32 * CELL_SIZE,
                y2: height as i32 * CELL_SIZE,
            });
            vert_walls.push(vert_wall_set);
        }

        Self {
            cells,
            horiz_walls,
            vert_walls
        }
    }

    pub fn cells(&self) -> &HashMap<(usize, usize), SvgRect> {
        &self.cells
    }

    pub fn vert_walls(&self) -> &Vec<HashSet<SvgLine>> {
        &self.vert_walls
    }

    pub fn horiz_walls(&self) -> &Vec<HashSet<SvgLine>> {
        &self.horiz_walls
    }

    pub fn update_cell_color(&mut self, coord: &Coord, cell_state: CellState) {
        let color: String = self.get_cell_color(&cell_state);
        self.cells.entry((coord.x, coord.y)).and_modify(|cell| {
            cell.fill = color.clone();
            cell.stroke = color;
        });
    }

    fn get_cell_color(&self, cell_state: &CellState) -> String {
        match cell_state {
            CellState::Unvisited => "lightgrey".to_string(),
            CellState::Path => "white".to_string(),
            CellState::Frontier => "papayawhip".to_string(),
            CellState::Solution => "dodgerblue".to_string(),
            CellState::Start => "green".to_string(),
            CellState::Finish => "red".to_string(),
        }
    }

    pub fn remove_cell_wall(&mut self, coord: &Coord, wall_side: &str) {
        match wall_side {
            "top" => {
                let wall_to_remove = SvgLine {
                    x1: coord.x as i32 * CELL_SIZE,
                    y1: coord.y as i32 * CELL_SIZE,
                    x2: coord.x as i32 * CELL_SIZE + CELL_SIZE,
                    y2: coord.y as i32 * CELL_SIZE
                };
                let (containing_wall, new_walls) = self.split_wall(&wall_to_remove, WallDirection::Horizontal, coord);
                self.horiz_walls[coord.y].remove(&containing_wall);
                self.horiz_walls[coord.y].extend(&new_walls);
            },
            "right" => {
                let wall_to_remove = SvgLine {
                    x1: coord.x as i32 * CELL_SIZE + CELL_SIZE,
                    y1: coord.y as i32 * CELL_SIZE,
                    x2: coord.x as i32 * CELL_SIZE + CELL_SIZE,
                    y2: coord.y as i32 * CELL_SIZE + CELL_SIZE,
                };
                let (containing_wall, new_walls) = self.split_wall(&wall_to_remove, WallDirection::Vertical, coord);
                self.vert_walls[coord.x + 1].remove(&containing_wall);
                self.vert_walls[coord.x + 1].extend(&new_walls);
            }
            _ => {},
        }
    }

    fn split_wall(&self, wall_to_remove: &SvgLine, wall_direction: WallDirection, cell_coord: &Coord) -> (SvgLine, Vec<SvgLine>) {
        let (walls_vec, walls_vec_i): (&Vec<HashSet<SvgLine>>, usize) = match wall_direction {
            WallDirection::Vertical => (&self.vert_walls, cell_coord.x + 1),
            WallDirection::Horizontal => (&self.horiz_walls, cell_coord.y),
        };

        let containing_wall = match walls_vec[walls_vec_i]
            .par_iter()
            .find_any(|containing_wall| self.contains_wall(wall_to_remove, containing_wall))
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