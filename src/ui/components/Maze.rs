use std::collections::HashSet;

use dioxus::prelude::*;

use crate::maze::Maze;
use crate::cell::{CellState, Coord};
use crate::generator_algorithms::random_prim;

#[derive(Hash, Eq, PartialEq)]
struct Wall {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32
}

struct Cell {
    x: i32,
    y: i32,
    state: CellState,
}

const CELL_SIZE: i32 = 3;

// pub struct MazeProps {
//     pub generator: String,
//     pub solver: String,
// }


pub fn Maze() -> Element {
    let mut maze = use_signal(|| Maze::new(10, 10));
    let mut walls = use_signal(|| HashSet::new());


    //TODO build grid of top and right walls spanning height/width of maze


    //TODO iterate over walls in maze, removing sections as needed and replacing with leftover walls
    use_effect(move || {
        let left_border = Wall {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: maze.read().height() as i32 * CELL_SIZE
        };
        walls.write().insert(left_border);

        let bottom_border = Wall {
            x1: 0,
            y1: maze.read().height() as i32 * CELL_SIZE,
            x2: maze.read().width() as i32 * CELL_SIZE,
            y2: maze.read().height() as i32 * CELL_SIZE
        };
        walls.write().insert(bottom_border);

        for cell in maze.read().grid() {
            let x = cell.coord().x as i32;
            let y = cell.coord().y as i32;

            if cell.walls()[0] {
                let top_wall = Wall {
                    x1: x * CELL_SIZE,
                    y1: y * CELL_SIZE,
                    x2: x * CELL_SIZE + CELL_SIZE,
                    y2: y * CELL_SIZE
                };
                walls.write().insert(top_wall);
            }

            if cell.walls()[1] {
                let right_wall = Wall {
                    x1: x * CELL_SIZE + CELL_SIZE,
                    y1: y * CELL_SIZE,
                    x2: x * CELL_SIZE + CELL_SIZE,
                    y2: y * CELL_SIZE + CELL_SIZE,
                };
                walls.write().insert(right_wall);
            }
        }
    });

    rsx! {
        svg {
            view_box: "{-CELL_SIZE} {-CELL_SIZE} {maze.read().width() as i32 * CELL_SIZE + 2 * CELL_SIZE} {maze.read().height() as i32 * CELL_SIZE + 2 * CELL_SIZE}",

            g {
                id: "cells",
                for cell in maze.read().grid() {
                    rect {
                        id: "{cell.coord().y * maze.read().width() + cell.coord().x}",
                        x: "{cell.coord().x as i32 * CELL_SIZE}",
                        y: "{cell.coord().y as i32 * CELL_SIZE}",
                        width: "{CELL_SIZE}",
                        height: "{CELL_SIZE}",
                        fill: "white"
                    }
                }
            }

            g {
                id: "walls",
                for wall in walls.read().iter() {
                    line {
                        id: "{wall.x1} {wall.y1} {wall.x2} {wall.y2}",
                        x1: "{wall.x1}",
                        y1: "{wall.y1}",
                        x2: "{wall.x2}",
                        y2: "{wall.y2}",
                    }
                }
            }
        }
    }
}