use dioxus::prelude::*;

use crate::ui::components::{GeneratorConfig::GeneratorConfig, MazeRender::MazeRender, SolverConfig::SolverConfig};
use crate::structures::maze::Maze;

pub fn launch_app() {
    dioxus::launch(App);
}

static CSS: Asset = asset!("src/ui/assets/main.css");
// static favicon: Asset = asset!("src/ui/assets/favicon.ico");

fn App() -> Element {
    let maze: Signal<Maze> = use_signal(|| Maze::new(15, 15));
    let generated: Signal<bool> = use_signal(|| false);
    let working: Signal<bool> = use_signal(|| false);

    rsx!{
        document::Stylesheet { href: CSS }
        style { "@import url('https://fonts.googleapis.com/css2?family=Titillium+Web:ital,wght@0,200;0,300;0,400;0,600;0,700;0,900;1,200;1,300;1,400;1,600;1,700&display=swap');" }

        head {
            link {
                rel: "icon",
                "type": "image/x-icon",
                // href: favicon;
            }
        }
        div {
            id: "sidebar",
            h1 { "Mazer" },
            GeneratorConfig {
                maze: maze,
                generated: generated,
                working: working,
            }
            SolverConfig {
                maze: maze,
                generated: generated,
                working: working,
            }
        }
        div {
            id: "maze",
            MazeRender { maze: maze }
        }
    }
}