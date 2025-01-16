use dioxus::prelude::*;

use crate::generator_algorithms::generator_helpers::GeneratorStatus;
use crate::generator_algorithms::random_prim::RandomPrim;
use crate::solver_algorithms::solver_helpers::SolverStatus;
use crate::solver_algorithms::breadth_first_search::BreadthFirstSearch;
use crate::ui::components::{Header, MazeRender, Dropdown, Button, NumInput};
use crate::maze::Maze;
use crate::cell::Coord;

pub fn launch_app() {
    dioxus::launch(App);
}

static CSS: Asset = asset!("src/ui/assets/main.css");

fn App() -> Element {
    let height: Signal<usize> = use_signal(|| 10);
    let width: Signal<usize> = use_signal(|| 10);

    let mut maze: Signal<Maze> = use_signal(|| Maze::new(*height.read(), *width.read()));
    let mut generated: Signal<bool> = use_signal(|| false);
    let mut solved: Signal<bool> = use_signal(|| false);

    let mut generator_algo = RandomPrim::new();

    let mut starting_coord_x: Signal<usize> = use_signal(|| 0);
    let mut starting_coord_y: Signal<usize> = use_signal(|| 0);
    let mut finishing_coord_x: Signal<usize> = use_signal(|| width - 1);
    let mut finishing_coord_y: Signal<usize> = use_signal(|| height - 1);
    let mut starting_coord: Signal<Coord> = use_signal(|| Coord { x: *starting_coord_x.read(), y: *starting_coord_y.read() });
    let mut finishing_coord: Signal<Coord> = use_signal(|| Coord { x: *finishing_coord_x.read(), y: *finishing_coord_y.read() });
    let mut solver_algo = BreadthFirstSearch::new(&starting_coord.read(), &finishing_coord.read());

    use_effect(move || {
        starting_coord_x();
        starting_coord_y();
        finishing_coord_x();
        finishing_coord_y();

        starting_coord.set(Coord { x: *starting_coord_x.read(), y: *starting_coord_y.read() });
        finishing_coord.set(Coord { x: *finishing_coord_x.read(), y: *finishing_coord_y.read() });
    });

    let gen_dropdown_props = vec![
        ("random_prim".to_string(),"Random Prim".to_string()),
        ("recursive_backtracker".to_string(),"Recursive Backtracker".to_string()),
        ("ellers".to_string(),"Ellers".to_string())
    ];

    let solve_dropdown_props = vec![
        ("breadth_first_search".to_string(),"Breadth First Search".to_string()),
    ];

    rsx!{
        document::Stylesheet { href: CSS }
        style { "@import url('https://fonts.googleapis.com/css2?family=Titillium+Web:ital,wght@0,200;0,300;0,400;0,600;0,700;0,900;1,200;1,300;1,400;1,600;1,700&display=swap');" }

        Header::Header{}
        MazeRender::MazeRender { maze: maze }
        div {
            id: "maze-controls",
            div {
                id: "maze-config",
                align_items: "center",
                h2 { "Maze Options" }
                div {
                    id: "height-config",
                    p { "Height:" }
                    NumInput::NumInput {
                        id: "height-input",
                        value: height,
                        max_val: 200,
                        min_val: 2,
                    }
                }
                div {
                    id: "width-config",
                    p { "Width:" }
                    NumInput::NumInput {
                        id: "width-input",
                        value: width,
                        max_val: 200,
                        min_val: 2,
                    }
                }

                Button::Button {
                    button_text: "New Maze",
                    disabled: false,
                    onclick: move |_| {
                        maze.set(Maze::new(*height.read(), *width.read()));
                        starting_coord_x.set(0);
                        starting_coord_y.set(0);
                        finishing_coord_x.set(width - 1);
                        finishing_coord_y.set(height - 1);
                        generated.set(false);
                        solved.set(false);
                    },
                }
            }
            div {
                id: "generator-config",
                h2 { "Generator Options" }
                Dropdown::Dropdown {
                    id: "generator-dropdown",
                    options: gen_dropdown_props,
                    helper_text: "Maze Generator Algo".to_string()
                }
                Button::Button {
                    button_text: "Generate maze".to_string(),
                    disabled: *generated.read(),
                    onclick: move |_| {
                        while generator_algo.status != GeneratorStatus::Done {
                            generator_algo.create_maze(&mut maze);
                        }
                        if generator_algo.status == GeneratorStatus::Done {
                            generated.set(true);
                        }
                    },
                }
            }
            div {
                id: "solver-config",
                h2 { "Solver Options" }
                Dropdown::Dropdown {
                    id: "solver-dropdown",
                    options: solve_dropdown_props,
                    helper_text: "Maze Solver Algo".to_string()
                }
                div {
                    id: "start-finish-config",
                    h4 { "START" }
                    div {
                        id: "starting-coord-config",
                        p { "x:" }
                        NumInput::NumInput {
                            id: "starting-coord-x",
                            value: starting_coord_x,
                            max_val: *width.read(),
                            min_val: 0,
                        }
                        p { "y:" }
                        NumInput::NumInput {
                            id: "starting-coord-y",
                            value: starting_coord_y,
                            max_val: *height.read(),
                            min_val: 0,
                        }
                    }
                    h4 { "FINISH" }
                    div {
                        id: "finishing-coord-config",
                        p { "x:" }
                        NumInput::NumInput {
                            id: "finishing-coord-x",
                            value: finishing_coord_x,
                            max_val: *width.read(),
                            min_val: 0,
                        }
                        p { "y:" }
                        NumInput::NumInput {
                            id: "finishing-coord-y",
                            value: finishing_coord_y,
                            max_val: *height.read(),
                            min_val: 0,
                        }
                    }
                }
                Button::Button {
                    button_text: "Solve maze".to_string(),
                    disabled: !*generated.read() || *solved.read(),
                    onclick: move |_| {
                        while solver_algo.status != SolverStatus::Done {
                            solver_algo.find_solution(&mut maze);
                        }
                        if solver_algo.status == SolverStatus::Done {
                            solved.set(true);
                        }
                    },
                }
            }
        }
    }
}