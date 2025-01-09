use std::collections::HashSet;

use dioxus::prelude::*;

use crate::maze::Maze;
use crate::cell::{CellState, Coord};

#[derive(Hash, Eq, PartialEq, Clone)]
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

#[derive(PartialEq, Props, Clone)]
pub struct MazeRenderProps {
    maze: Signal<Maze>,
}

pub fn MazeRender(props: MazeRenderProps) -> Element {
    let mut walls = HashSet::new();

    for y in 0..=props.maze.read().height() {
        let horiz_wall = Wall {
            x1: 0,
            y1: y as i32 * CELL_SIZE,
            x2: props.maze.read().width() as i32 * CELL_SIZE,
            y2: y as i32 * CELL_SIZE,
        };
        walls.insert(horiz_wall);
    }

    for x in 0..=props.maze.read().width() {
        let vert_wall = Wall{
            x1: x as i32 * CELL_SIZE,
            y1: 0,
            x2: x as i32 * CELL_SIZE,
            y2: props.maze.read().height() as i32 * CELL_SIZE,
        };
        walls.insert(vert_wall);
    }

    let mut walls = use_signal(|| walls);

    use_effect(move || {
        for cell in props.maze.read().grid() {
            let x = cell.coord().x as i32;
            let y = cell.coord().y as i32;

            if !cell.walls()[0] {
                let top_wall = Wall {
                    x1: x * CELL_SIZE,
                    y1: y * CELL_SIZE,
                    x2: x * CELL_SIZE + CELL_SIZE,
                    y2: y * CELL_SIZE
                };
                remove_wall(&top_wall, &mut walls);
            }

            if !cell.walls()[1] {
                let right_wall = Wall {
                    x1: x * CELL_SIZE + CELL_SIZE,
                    y1: y * CELL_SIZE,
                    x2: x * CELL_SIZE + CELL_SIZE,
                    y2: y * CELL_SIZE + CELL_SIZE,
                };
                remove_wall(&right_wall, &mut walls);
            }
        }
    });

    rsx! {
        svg {
            view_box: "{-CELL_SIZE} {-CELL_SIZE} {props.maze.read().width() as i32 * CELL_SIZE + 2 * CELL_SIZE} {props.maze.read().height() as i32 * CELL_SIZE + 2 * CELL_SIZE}",

            g {
                id: "cells",
                for cell in props.maze.read().grid() {
                    rect {
                        id: "{cell.coord().y * props.maze.read().width() + cell.coord().x}",
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

fn remove_wall(wall_to_remove: &Wall, walls_set: &mut Signal<HashSet<Wall>>) {
    let old_wall = match walls_set.read()
        .iter()
        .find(|containing_wall| contains_wall(&wall_to_remove, containing_wall))
        .cloned() {
        Some(wall) => wall,
        None => return
    };

    if !(old_wall.x1 == wall_to_remove.x1 && old_wall.y1 == wall_to_remove.y1) {
        let new_line_1 = Wall {
            x1: old_wall.x1,
            y1: old_wall.y1,
            x2: wall_to_remove.x1,
            y2: wall_to_remove.y1,
        };
        walls_set.write().insert(new_line_1);
    }

    if !(old_wall.x2 == wall_to_remove.x2 && old_wall.y2 == wall_to_remove.y2) {
        let new_line_2 = Wall {
            x1: wall_to_remove.x2,
            y1: wall_to_remove.y2,
            x2: old_wall.x2,
            y2: old_wall.y2,
        };
        walls_set.write().insert(new_line_2);
    }

    walls_set.write().remove(&old_wall);
}

fn contains_wall(inside_wall: &Wall, containing_wall: &Wall) -> bool {
    let x_in_range = (inside_wall.x1 >= containing_wall.x1) && (inside_wall.x2 <= containing_wall.x2);
    let y_in_range = (inside_wall.y1 >= containing_wall.y1) && (inside_wall.y2 <= containing_wall.y2);

    return x_in_range && y_in_range
}