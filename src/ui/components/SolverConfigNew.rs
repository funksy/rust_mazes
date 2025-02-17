use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures;

use crate::generator_algorithms::generator_helpers::get_generator_algo;
use crate::solver_algorithms::solver_helpers::{get_solver_algo, get_solver_options, SolverStatus};
use crate::structures::cell::Coord;
use crate::structures::maze::Maze;
use crate::ui::components::{Dropdown::Dropdown, NumInput::NumInput, Button::Button, NumSlider::NumSlider};

#[component]
pub fn SolverConfigNew(maze: Signal<Maze>, generated: Signal<bool>, working: Signal<bool>) -> Element {
    let mut start_coord_x: Signal<usize> = use_signal(|| 0);
    let mut start_coord_y: Signal<usize> = use_signal(|| 0);
    let mut finish_coord_x: Signal<usize> = use_signal(|| maze.read().width() - 1);
    let mut finish_coord_y: Signal<usize> = use_signal(|| maze.read().height() - 1);
    let start_coord: Memo<Coord> = use_memo(move || {
        Coord{ x: start_coord_x(), y: start_coord_y() }
    });
    let finish_coord: Memo<Coord> = use_memo(move || {
        Coord{ x: finish_coord_x(), y: finish_coord_y() }
    });
    let mut solved: Signal<bool> = use_signal(|| false);

    let solver_algo_choice: Signal<String> = use_signal(|| "breadth_first_search".to_string());
    let mut solver_algo = use_signal(|| get_solver_algo(solver_algo_choice.read().as_str(), &start_coord(), &finish_coord()));

    let mut solver_speed: Signal<usize> = use_signal(|| 2);
    let mut solver_delay: Signal<u32> = use_signal(|| *solver_speed.read() as u32 * 10);
    let mut batch_size: Signal<usize> = use_signal(|| (maze.read().width() * maze.read().height()) / 150);

    use_effect(move || {
        solver_algo_choice();
        solver_algo.set(get_solver_algo(solver_algo_choice.read().as_str(), &start_coord(), &finish_coord()));
    });

    use_effect(move || {
        generated();
        finish_coord_x.set(maze.read().width() - 1);
        finish_coord_y.set(maze.read().height() - 1);
    });

    use_effect(move || {
        solver_delay.set(*solver_speed.read() as u32 * 10);
    });

    rsx! {
        div {
            id: "solver-config",
            class: "config-div",
            form {
                fieldset {
                    id: "solver-algo-config",
                    legend { "Solver Config" },
                    Dropdown {
                        id: "solver-dropdown",
                        options: get_solver_options(),
                        helper_text: "Maze Solver Algo".to_string(),
                        value: solver_algo_choice,
                        disabled: *working.read(),
                    }
                    div {
                        id: "start-finish-config",
                        label { for: "start-coord-config", "Starting Cell" }
                        div {
                            id: "start-coord-config",
                            label { for: "start-coord-x", "x:" },
                            NumInput {
                                id: "start-coord-x",
                                value: start_coord_x,
                                disabled: *working.read(),
                                max_val: maze.read().width() - 1,
                                min_val: 0,
                            }
                            label { for: "start-coord-y", "y:" },
                            NumInput {
                                id: "start-coord-y",
                                value: start_coord_y,
                                disabled: *working.read(),
                                max_val: maze.read().width() - 1,
                                min_val: 0,
                            }
                        }
                        label { for: "finish-coord-config", "Finishing Cell" },
                        div {
                            id: "finish-coord-config",
                            label { for: "finish-coord-x", "x:" },
                            NumInput {
                                id: "finish-coord-x",
                                value: finish_coord_x,
                                disabled: *working.read(),
                                max_val: maze.read().height() - 1,
                                min_val: 0,
                            }
                            label { for: "finish-coord-y", "y:" },
                            NumInput {
                                id: "finish-coord-y",
                                value: finish_coord_y,
                                disabled: *working.read(),
                                max_val: maze.read().height() - 1,
                                min_val: 0,
                            }
                        }
                    }
                    div {
                        id: "solver-speed-config",
                        label { for: "solver-speed-config", "Speed" },
                        NumSlider {
                            id: "generator-speed-slider",
                            value: solver_speed,
                            disabled: *working.read(),
                            max_val: 4,
                            min_val: 0,
                            step_val: 1,
                        }
                    }
                }
            }
            Button {
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

                        // let batch = *batch_size.read();

                        while solver_algo.read().status() != &SolverStatus::Done {
                            for _ in 0..*batch_size.read() {
                                if solver_algo.read().status() == &SolverStatus::Done {
                                    break;
                                }
                                solver_algo.write().find_solution(&mut maze);
                            }

                            if *solver_delay.read() > 0 {
                                TimeoutFuture::new(*solver_delay.read()).await;
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
}