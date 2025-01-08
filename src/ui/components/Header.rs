use dioxus::prelude::*;

pub fn Header() -> Element {
    rsx!{
        h1{
            id: "header",
            "Maze Generator and Solver"
        }
    }
}