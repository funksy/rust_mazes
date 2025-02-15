use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures;

use crate::generator_algorithms::generator_helpers::{get_generator_algo, get_generator_options, GeneratorStatus};
use crate::solver_algorithms::solver_helpers::{get_solver_algo, get_solver_options, SolverStatus};
use crate::ui::components::{GeneratorConfig, SolverConfig, MazeRender, Button};
use crate::structures::maze::Maze;
use crate::structures::cell::Coord;

pub fn launch_app() {
    dioxus::launch(App);
}

static CSS: Asset = asset!("src/ui/assets/main.css");

fn App() -> Element {
    let height: Signal<usize> = use_signal(|| 15);
    let width: Signal<usize> = use_signal(|| 15);
    let mut maze: Signal<Maze> = use_signal(|| Maze::new(*height.read(), *width.read()));

    let mut start_coord_x: Signal<usize> = use_signal(|| 0);
    let mut start_coord_y: Signal<usize> = use_signal(|| 0);
    let mut finish_coord_x: Signal<usize> = use_signal(|| width - 1);
    let mut finish_coord_y: Signal<usize> = use_signal(|| height - 1);
    let start_coord: Memo<Coord> = use_memo(move || {
        Coord{ x: start_coord_x(), y: start_coord_y() }
    });
    let finish_coord: Memo<Coord> = use_memo(move || {
        Coord{ x: finish_coord_x(), y: finish_coord_y() }
    });

    let generator_algo_choice: Signal<String> = use_signal(|| "ellers".to_string());
    let mut generator_algo = use_signal(|| get_generator_algo(generator_algo_choice.read().as_str()));
    let generator_delay: Signal<usize> = use_signal(|| 10);
    let batch_size: Memo<usize> = use_memo(move || (height() * width()) / generator_delay());
    let mut generated: Signal<bool> = use_signal(|| false);

    let solver_algo_choice: Signal<String> = use_signal(|| "breadth_first_search".to_string());
    let mut solver_algo = use_signal(|| get_solver_algo(solver_algo_choice.read().as_str(), &start_coord(), &finish_coord()));
    let solver_delay: Signal<usize> = use_signal(|| 10);
    let mut solved: Signal<bool> = use_signal(|| false);

    let mut working: Signal<bool> = use_signal(|| false);

    use_effect(move || {
        generator_algo_choice();
        generator_algo.set(get_generator_algo(generator_algo_choice.read().as_str()));
    });

    use_effect(move || {
        solver_algo_choice();
        solver_algo.set(get_solver_algo(solver_algo_choice.read().as_str(), &start_coord(), &finish_coord()));
    });

    rsx!{
        document::Stylesheet { href: CSS }
        style { "@import url('https://fonts.googleapis.com/css2?family=Titillium+Web:ital,wght@0,200;0,300;0,400;0,600;0,700;0,900;1,200;1,300;1,400;1,600;1,700&display=swap');" }

        div {
            id: "sidebar",
            h1 { "Mazer" },
            div {
                id: "generator-config",
                class: "config-div",
                GeneratorConfig::GeneratorConfig {
                    dropdown_options: get_generator_options(),
                    generator_algo_choice: generator_algo_choice,
                    height: height,
                    width: width,
                    disabled: false,
                    generator_delay: generator_delay,
                    working: working,
                }
                Button::Button {
                    button_text: "Generate maze".to_string(),
                    disabled: *working.read(),
                    onclick: move |_| {
                        start_coord_x.set(0);
                        start_coord_y.set(0);
                        finish_coord_x.set(width - 1);
                        finish_coord_y.set(height - 1);
                        generated.set(false);
                        generator_algo.set(get_generator_algo(generator_algo_choice.read().as_str()));
                        working.set(true);

                        wasm_bindgen_futures::spawn_local(async move {
                            maze.set(Maze::new(*height.read(), *width.read()));
                            TimeoutFuture::new(200).await;

                            let batch = *batch_size.read();

                            while generator_algo.read().status() != &GeneratorStatus::Done {
                                for _ in 0..batch {
                                    if generator_algo.read().status() == &GeneratorStatus::Done {
                                        break;
                                    }
                                    generator_algo.write().create_maze(&mut maze);
                                }

                                if *generator_delay.read() > 0 {
                                        TimeoutFuture::new(*generator_delay.read() as u32).await;
                                }

                            }
                            if generator_algo.read().status() == &GeneratorStatus::Done {
                                generated.set(true);
                                working.set(false);
                            }
                        });
                    }
                }
            }
            div {
                id: "solver-config",
                class: "config-div",
                SolverConfig::SolverConfig {
                    dropdown_options: get_solver_options(),
                    solver_algo_choice: solver_algo_choice,
                    height: height,
                    width: width,
                    start_coord_x: start_coord_x,
                    start_coord_y: start_coord_y,
                    finish_coord_x: finish_coord_x,
                    finish_coord_y: finish_coord_y,
                    solver_delay: solver_delay,
                    working: working,
                }
                Button::Button {
                    button_text: "Solve maze".to_string(),
                    disabled: !*generated.read() || *working.read(),
                    onclick: move |_| {
                        working.set(true);

                        wasm_bindgen_futures::spawn_local(async move {
                            if solved() {
                                solver_algo.write().reset(&mut maze);
                                solver_algo.set(get_solver_algo(solver_algo_choice.read().as_str(), &start_coord(), &finish_coord()));
                                solved.set(false);
                            }

                            let batch = *batch_size.read();

                            while solver_algo.read().status() != &SolverStatus::Done {
                                for _ in 0..batch {
                                    if solver_algo.read().status() == &SolverStatus::Done {
                                        break;
                                    }
                                    solver_algo.write().find_solution(&mut maze);
                                }

                                if *solver_delay.read() > 0 {
                                    TimeoutFuture::new(*solver_delay.read() as u32).await;
                                }
                            }
                            if solver_algo.read().status() == &SolverStatus::Done {
                                solved.set(true);
                                working.set(false);
                            }
                        });
                    }
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

