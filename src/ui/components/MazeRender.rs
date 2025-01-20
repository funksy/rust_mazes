use dioxus::prelude::*;

use crate::maze::Maze;

const CELL_SIZE: i32 = 3;

#[component]
pub fn MazeRender(maze: ReadOnlySignal<Maze>) -> Element {
    let maze_svg = use_memo(move || maze.read().svg_render().clone());

    use_effect(move || {
       maze.read();
    });

    rsx! {
        svg {
            view_box: "{-0.5} {0} {maze.read().width() as i32 * CELL_SIZE + 1} {maze.read().height() as i32 * CELL_SIZE}",

            g {
                id: "cells",
                {
                    maze_svg.read().cells.iter().map(|cell| {
                        rsx!{
                            rect {
                                x: "{&cell.x}",
                                y: "{&cell.y}",
                                width: "{&cell.width}",
                                height: "{&cell.height}",
                                fill: "{&cell.fill}",
                                stroke: "{&cell.stroke}"
                            }
                        }
                    })
                }
            }

            g {
                id: "walls",
                {
                    maze_svg.read().horiz_walls.iter().flat_map(|horiz_wall_vec| {
                        horiz_wall_vec.iter().map(|wall| {
                            rsx! {
                                line {
                                    x1: "{wall.x1}",
                                    y1: "{wall.y1}",
                                    x2: "{wall.x2}",
                                    y2: "{wall.y2}",
                                }
                            }
                        })
                    })
                }

                {
                    maze_svg.read().vert_walls.iter().flat_map(|vert_wall_vec| {
                        vert_wall_vec.iter().map(|wall| {
                            rsx! {
                                line {
                                    x1: "{wall.x1}",
                                    y1: "{wall.y1}",
                                    x2: "{wall.x2}",
                                    y2: "{wall.y2}",
                                }
                            }
                        })
                    })
                }
            }
        }
    }
}