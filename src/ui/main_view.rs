use dioxus::prelude::*;

use crate::generator_algorithms::generator_helpers::GeneratorStatus;
use crate::generator_algorithms::random_prim::RandomPrim;
use crate::solver_algorithms::solver_helpers::SolverStatus;
use crate::solver_algorithms::breadth_first_search::BreadthFirstSearch;
use crate::ui::components::{Header, DimensionConfig, GeneratorConfig, SolverConfig, MazeRender, Dropdown, Button, NumInput};
use crate::maze::Maze;
use crate::cell::{CellState, Coord};

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

    let mut starting_coord: Signal<Coord> = use_signal(|| Coord { x: 0, y: 0 });
    let mut finishing_coord: Signal<Coord> = use_signal(|| Coord { x: *width.read() - 1, y: *height.read() - 1 });
    let mut solver_algo = BreadthFirstSearch::new(&starting_coord.read(), &finishing_coord.read());

    let gen_dropdown_options = vec![
        ("random_prim".to_string(),"Random Prim".to_string()),
        ("recursive_backtracker".to_string(),"Recursive Backtracker".to_string()),
        ("ellers".to_string(),"Ellers".to_string())
    ];

    let solve_dropdown_options = vec![
        ("breadth_first_search".to_string(),"Breadth First Search".to_string()),
    ];

    rsx!{
        document::Stylesheet { href: CSS }
        style { "@import url('https://fonts.googleapis.com/css2?family=Titillium+Web:ital,wght@0,200;0,300;0,400;0,600;0,700;0,900;1,200;1,300;1,400;1,600;1,700&display=swap');" }

        div {
            id: "sidebar",
            h1 { "Mazer" },
            div {
                id: "maze-config",
                class: "config-div",
                DimensionConfig::DimensionConfig {
                    height: height,
                    width: width,
                }
                Button::Button {
                    button_text: "New Maze",
                    disabled: false,
                    onclick: move |_| {
                        maze.set(Maze::new(*height.read(), *width.read()));
                        starting_coord.set(Coord { x: 0, y: 0 });
                        finishing_coord.set(Coord { x: *width.read() - 1, y: *height.read() - 1 });
                        generated.set(false);
                        solved.set(false);
                    },
                }
            }
            div {
                id: "generator-config",
                class: "config-div",
                GeneratorConfig::GeneratorConfig {
                    dropdown_options: gen_dropdown_options,
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
                class: "config-div",
                SolverConfig::SolverConfig {
                    dropdown_options: solve_dropdown_options,
                    height: height,
                    width: width,
                    starting_coord: starting_coord,
                    finishing_coord: finishing_coord,
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
        div
        {
            id: "maze",
            MazeRender::MazeRender { maze: maze }
        }
    }
}