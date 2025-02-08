use std::collections::HashSet;
use rayon::prelude::*;
use crate::cell::{Cell, CellState, Coord};
use crate::maze_svg_render::MazeSvg;

#[derive(PartialEq)]
pub struct Maze {
    height: usize,
    width: usize,
    grid: Vec<Cell>,
    svg: MazeSvg,
}

const CELL_SIZE: i32 = 3;

impl Maze {
    pub fn new(height: usize, width: usize) -> Self {
        let mut grid = Vec::with_capacity(height * width);
        for y in 0..height {
            for x in 0..width {
                grid.push(Cell::new(Coord{ y, x }));
            }
        }

        Self {
            height,
            width,
            grid,
            svg: MazeSvg::new(height, width),
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

    pub fn svg_elements(&self) -> &MazeSvg {
        &self.svg
    }

    pub fn get_cell_ref(&self, coord: &Coord) -> &Cell {
        &self.grid[coord.y * self.width + coord.x]
    }

    pub fn visit_cell(&mut self, coord: &Coord) {
        self.grid[coord.y * self.width + coord.x].visit();
        self.svg.update_cell_color(coord, CellState::Path);
    }

    pub fn change_cell_state(&mut self, coord: &Coord, new_state: CellState) {
        self.grid[coord.y * self.width + coord.x].change_state(new_state);
        self.svg.update_cell_color(coord, new_state);
    }

    pub fn remove_cell_wall(&mut self, coord: &Coord, wall_side: &str) {
        self.grid[coord.y * self.width + coord.x].remove_wall(wall_side);
        self.svg.remove_cell_wall(coord, wall_side);

        // match wall_side {
        //     "top" => {
        //         let wall_to_remove = SvgLine {
        //             x1: coord.x as i32 * CELL_SIZE,
        //             y1: coord.y as i32 * CELL_SIZE,
        //             x2: coord.x as i32 * CELL_SIZE + CELL_SIZE,
        //             y2: coord.y as i32 * CELL_SIZE
        //         };
        //
        //         let (containing_wall, new_walls) = self.split_wall(&wall_to_remove, WallDirection::Horizontal, coord);
        //         self.svg.horiz_walls[coord.y].remove(&containing_wall);
        //         self.svg.horiz_walls[coord.y].extend(&new_walls);
        //     },
        //     "right" => {
        //         let wall_to_remove = SvgLine {
        //             x1: coord.x as i32 * CELL_SIZE + CELL_SIZE,
        //             y1: coord.y as i32 * CELL_SIZE,
        //             x2: coord.x as i32 * CELL_SIZE + CELL_SIZE,
        //             y2: coord.y as i32 * CELL_SIZE + CELL_SIZE,
        //         };
        //
        //         let (containing_wall, new_walls) = self.split_wall(&wall_to_remove, WallDirection::Vertical, coord);
        //         self.svg.vert_walls[coord.x + 1].remove(&containing_wall);
        //         self.svg.vert_walls[coord.x + 1].extend(&new_walls);
        //     },
        //     _ => {}
        // }
    }

    // fn split_wall(&self, wall_to_remove: &SvgLine, wall_direction: WallDirection, cell_coord: &Coord) -> (SvgLine, Vec<SvgLine>) {
    //     let (walls_vec, walls_vec_i): (&Vec<HashSet<SvgLine>>, usize) = match wall_direction {
    //         WallDirection::Vertical => (&self.svg.vert_walls, cell_coord.x + 1),
    //         WallDirection::Horizontal => (&self.svg.horiz_walls, cell_coord.y),
    //     };
    //
    //     let containing_wall = match walls_vec[walls_vec_i]
    //         .par_iter()
    //         .find_any(|containing_wall| self.contains_wall(&wall_to_remove, containing_wall))
    //         .cloned() {
    //         Some(wall) => wall,
    //         None => panic!("Containing wall doesn't exist."),
    //     };
    //
    //     let mut new_walls: Vec<SvgLine> = Vec::new();
    //
    //     if !(containing_wall.x1 == wall_to_remove.x1 && containing_wall.y1 == wall_to_remove.y1) {
    //         let new_line_1 = SvgLine {
    //             x1: containing_wall.x1,
    //             y1: containing_wall.y1,
    //             x2: wall_to_remove.x1,
    //             y2: wall_to_remove.y1,
    //         };
    //         new_walls.push(new_line_1);
    //     }
    //
    //     if !(containing_wall.x2 == wall_to_remove.x2 && containing_wall.y2 == wall_to_remove.y2) {
    //         let new_line_2 = SvgLine {
    //             x1: wall_to_remove.x2,
    //             y1: wall_to_remove.y2,
    //             x2: containing_wall.x2,
    //             y2: containing_wall.y2,
    //         };
    //         new_walls.push(new_line_2);
    //     }
    //
    //     (containing_wall, new_walls)
    // }
    //
    // fn contains_wall(&self, inside_wall: &SvgLine, containing_wall: &SvgLine) -> bool {
    //     let x_in_range = (inside_wall.x1 >= containing_wall.x1) && (inside_wall.x2 <= containing_wall.x2);
    //     let y_in_range = (inside_wall.y1 >= containing_wall.y1) && (inside_wall.y2 <= containing_wall.y2);
    //
    //     x_in_range && y_in_range
    // }
}