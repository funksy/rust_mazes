use crate::cell::{Cell, CellState, Coord};
use crate::maze_svg_render::MazeSvg;

#[derive(PartialEq)]
pub struct Maze {
    height: usize,
    width: usize,
    grid: Vec<Cell>,
    svg: MazeSvg,
}

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
    }
}