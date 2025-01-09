use dioxus::prelude::*;
use crate::generator_algorithms::random_prim;
use crate::ui::components::{Header, MazeRender, Dropdown, Button};
use crate::maze::Maze;

pub fn launch_app() {
    dioxus::launch(App);
}

static CSS: Asset = asset!("src/ui/assets/main.css");

fn App() -> Element {
    let mut maze = use_signal(|| Maze::new(100, 100));

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
        div{
            id: "dropdowns",
            Dropdown::Dropdown {
                id: "generator_dropdown",
                options: gen_dropdown_props,
                helper_text: "Maze Generator Algo".to_string()
            }
            Dropdown::Dropdown {
                id: "solver_dropdown",
                options: solve_dropdown_props,
                helper_text: "Maze Solver Algo".to_string()
            }
        }
        div {
            id: "buttons",
            Button::Button {}
        }
    }
}