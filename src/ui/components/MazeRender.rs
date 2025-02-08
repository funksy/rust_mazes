use dioxus::prelude::*;

use crate::maze::Maze;

const CELL_SIZE: i32 = 3;

#[component]
pub fn MazeRender(maze: ReadOnlySignal<Maze>) -> Element {
    let mut cells = use_signal(|| maze.read().svg_elements().cells().clone());
    let mut vert_walls = use_signal(|| maze.read().svg_elements().vert_walls().clone());
    let mut horiz_walls = use_signal(|| maze.read().svg_elements().horiz_walls().clone());

    use_effect(move || {
        let maze = maze.read();
        let svg_elements = maze.svg_elements();
        cells.set(svg_elements.cells().clone());
        vert_walls.set(svg_elements.vert_walls().clone());
        horiz_walls.set(svg_elements.horiz_walls().clone());
    });

    rsx! {
        svg {
            view_box: "{-0.5} {0} {maze.read().width() as i32 * CELL_SIZE + 1} {maze.read().height() as i32 * CELL_SIZE}",

            g {
                id: "cells",
                {
                    cells.read().iter().map(|(id, cell)| {
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
                    horiz_walls.read().iter().flat_map(|horiz_wall_vec| {
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
                    vert_walls.read().iter().flat_map(|vert_wall_vec| {
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