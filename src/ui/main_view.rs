use dioxus::prelude::*;

use crate::ui::components::{Header, Maze, Dropdown};

pub fn launch_app() {
    dioxus::launch(App);
}

static CSS: Asset = asset!("src/ui/assets/main.css");

fn App() -> Element {
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
        Maze::Maze{}
        div{
            Dropdown::Dropdown{
                id: "generator_dropdown",
                options: gen_dropdown_props,
                helper_text: "Choose the generator algorithm".to_string()
            }
            Dropdown::Dropdown{
                id: "solver_dropdown",
                options: solve_dropdown_props,
                helper_text: "Choose the solver algorithm".to_string()
            }
        }
    }
}