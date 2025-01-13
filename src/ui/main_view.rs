use dioxus::prelude::*;

use crate::generator_algorithms::random_prim;
use crate::solver_algorithms::breadth_first_search;
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


    let gen_dropdown_props = vec![
        ("random_prim".to_string(),"Random Prim".to_string()),
        ("recursive_backtracker".to_string(),"Recursive Backtracker".to_string())
        , ("ellers".to_string(),"Ellers".to_string())
    ];

    let solve_dropdown_props = vec![
        ("breadth_first_search".to_string(),"Breadth First Search".to_string()),
    ];

    rsx!{
        document::Stylesheet { href: CSS }

        Header::Header{}
        MazeRender::MazeRender { maze: maze }
        div {
            id: "maze-config",
            NumInput::NumInput {
                value: height,
                max_val: 150
            }
            NumInput::NumInput {
                value: width,
                max_val: 150
            }

            Button::Button {
                button_text: "New Maze",
                disabled: false,
                onclick: move |_| {
                    maze.set(Maze::new(*height.read(), *width.read()));
                    generated.set(false);
                    solved.set(false);
                },
            }
        }
        div {
            id: "dropdowns",
            Dropdown::Dropdown {
                id: "generator-dropdown",
                options: gen_dropdown_props,
                helper_text: "Maze Generator Algo".to_string()
            }
            Dropdown::Dropdown {
                id: "solver-dropdown",
                options: solve_dropdown_props,
                helper_text: "Maze Solver Algo".to_string()
            }
        }
        div {
            id: "algo-buttons",
            Button::Button {
                button_text: "Generate maze".to_string(),
                disabled: *generated.read(),
                onclick: move |_| {
                    random_prim::create_maze(&mut maze);
                    generated.set(true);
                },
            }
            Button::Button {
                button_text: "Solve maze".to_string(),
                disabled: !*generated.read() || *solved.read(),
                onclick: move |_| {
                    let start = &Coord { x: 0, y: 0 };
                    let finish = &Coord { x: maze.read().width() - 1, y: maze.read().height() - 1 };
                    breadth_first_search::find_solution(&mut maze, start, finish);
                    solved.set(true);
                },
            }
        }
    }
}