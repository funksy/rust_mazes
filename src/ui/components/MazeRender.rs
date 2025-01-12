use dioxus::prelude::*;

use crate::maze::Maze;

#[derive(PartialEq, Props, Clone)]
pub struct MazeRenderProps {
    maze: Signal<Maze>,
}

const CELL_SIZE: i32 = 3;

pub fn MazeRender(props: MazeRenderProps) -> Element {
    let maze_svg = use_memo(move || props.maze.read().svg_render().clone());

    rsx! {
        svg {
            view_box: "{-CELL_SIZE} {-CELL_SIZE} {props.maze.read().width() as i32 * CELL_SIZE + 2 * CELL_SIZE} {props.maze.read().height() as i32 * CELL_SIZE + 2 * CELL_SIZE}",

            g {
                id: "cells",
                for cell in &maze_svg.read().cells {
                    rect {
                        x: "{cell.x}",
                        y: "{cell.y}",
                        width: "{cell.width}",
                        height: "{cell.height}",
                        fill: "{cell.fill}"
                    }
                }
            }

            g {
                id: "walls",
                for horiz_wall_vec in &maze_svg.read().horiz_walls {
                    for wall in horiz_wall_vec {
                        line {
                            x1: "{wall.x1}",
                            y1: "{wall.y1}",
                            x2: "{wall.x2}",
                            y2: "{wall.y2}",
                        }
                    }
                }
                for vert_wall_vec in &maze_svg.read().vert_walls {
                    for wall in vert_wall_vec {
                        line {
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
}

// fn generate_walls(maze: &Maze) -> HashSet<Wall> {
//     let mut walls: HashSet<Wall> = HashSet::new();
//
//     for y in 0..=maze.height() {
//         let horiz_wall = Wall {
//             x1: 0,
//             y1: y as i32 * CELL_SIZE,
//             x2: maze.width() as i32 * CELL_SIZE,
//             y2: y as i32 * CELL_SIZE,
//         };
//         walls.insert(horiz_wall);
//     }
//
//     for x in 0..=maze.width() {
//         let vert_wall = Wall{
//             x1: x as i32 * CELL_SIZE,
//             y1: 0,
//             x2: x as i32 * CELL_SIZE,
//             y2: maze.height() as i32 * CELL_SIZE,
//         };
//         walls.insert(vert_wall);
//     }
//
//     for cell in maze.grid() {
//         let x = cell.coord().x as i32;
//         let y = cell.coord().y as i32;
//
//         if !cell.walls()[0] {
//             let top_wall = Wall {
//                 x1: x * CELL_SIZE,
//                 y1: y * CELL_SIZE,
//                 x2: x * CELL_SIZE + CELL_SIZE,
//                 y2: y * CELL_SIZE
//             };
//             remove_wall(&top_wall, &mut walls);
//         }
//
//         if !cell.walls()[1] {
//             let right_wall = Wall {
//                 x1: x * CELL_SIZE + CELL_SIZE,
//                 y1: y * CELL_SIZE,
//                 x2: x * CELL_SIZE + CELL_SIZE,
//                 y2: y * CELL_SIZE + CELL_SIZE,
//             };
//             remove_wall(&right_wall, &mut walls);
//         }
//     }
//
//     walls
// }
//
// fn remove_wall(wall_to_remove: &Wall, walls: &mut HashSet<Wall>) {
//     let old_wall = match walls
//         .iter()
//         .find(|containing_wall| contains_wall(&wall_to_remove, containing_wall))
//         .cloned() {
//         Some(wall) => wall,
//         None => return
//     };
//
//     if !(old_wall.x1 == wall_to_remove.x1 && old_wall.y1 == wall_to_remove.y1) {
//         let new_line_1 = Wall {
//             x1: old_wall.x1,
//             y1: old_wall.y1,
//             x2: wall_to_remove.x1,
//             y2: wall_to_remove.y1,
//         };
//         walls.insert(new_line_1);
//     }
//
//     if !(old_wall.x2 == wall_to_remove.x2 && old_wall.y2 == wall_to_remove.y2) {
//         let new_line_2 = Wall {
//             x1: wall_to_remove.x2,
//             y1: wall_to_remove.y2,
//             x2: old_wall.x2,
//             y2: old_wall.y2,
//         };
//         walls.insert(new_line_2);
//     }
//
//     walls.remove(&old_wall);
// }
//
// fn contains_wall(inside_wall: &Wall, containing_wall: &Wall) -> bool {
//     let x_in_range = (inside_wall.x1 >= containing_wall.x1) && (inside_wall.x2 <= containing_wall.x2);
//     let y_in_range = (inside_wall.y1 >= containing_wall.y1) && (inside_wall.y2 <= containing_wall.y2);
//
//     return x_in_range && y_in_range
// }
//
// fn get_cell_color(cell_state: &CellState) -> String {
//     match cell_state {
//         CellState::Unvisited => "white".to_string(),
//         CellState::Path => "white".to_string(),
//         CellState::Frontier => "white".to_string(),
//         CellState::Solution => "pink".to_string(),
//         CellState::Start => "green".to_string(),
//         CellState::Finish => "red".to_string(),
//     }
// }