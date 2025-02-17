use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures;

use crate::generator_algorithms::generator_helpers::get_generator_algo;
use crate::solver_algorithms::solver_helpers::{get_solver_algo, get_solver_options, SolverStatus};
use crate::structures::cell::Coord;
use crate::structures::maze::Maze;
use crate::ui::components::{Dropdown::Dropdown, NumInput::NumInput, Button::Button, NumSlider::NumSlider};

#[component]
pub fn SolverConfig(maze: Signal<Maze>, generated: Signal<bool>, working: Signal<bool>) -> Element {
    let height: Memo<usize> = use_memo(move || { maze.read().height() });
    let width: Memo<usize> = use_memo(move || { maze.read().width() });

    let mut start_coord_x: Signal<usize> = use_signal(|| 0);
    let mut start_coord_y: Signal<usize> = use_signal(|| 0);
    let mut finish_coord_x: Signal<usize> = use_signal(|| maze.read().width() - 1);
    let mut finish_coord_y: Signal<usize> = use_signal(|| maze.read().height() - 1);
    let mut start_coord: Signal<Coord> = use_signal(|| { Coord{ x: start_coord_x(), y: start_coord_y() } });
    let mut finish_coord: Signal<Coord> = use_signal(|| { Coord{ x: finish_coord_x(), y: finish_coord_y() } });
    let mut solved: Signal<bool> = use_signal(|| false);

    let solver_algo_choice: Signal<String> = use_signal(|| "breadth_first_search".to_string());
    let mut solver_algo = use_signal(|| get_solver_algo(solver_algo_choice.read().as_str(), &start_coord(), &finish_coord()));

    let mut solver_speed: Signal<usize> = use_signal(|| 1);
    let mut solver_delay: Signal<u32> = use_signal(|| *solver_speed.read() as u32 * 10);
    let mut batch_size: Signal<usize> = use_signal(|| (maze.read().width() * maze.read().height()) / 50);

    use_effect(move || {
        solver_algo.set(get_solver_algo(solver_algo_choice.read().as_str(), &start_coord(), &finish_coord()));
    });

    use_effect(move || {
        finish_coord_x.set(*height.read() - 1);
        finish_coord_y.set(*width.read() - 1);
    });

    use_effect(move || {
        solver_delay.set(*solver_speed.read() as u32 * 10);
        if (*width.read() * *height.read() > 100) {
            batch_size.set((*width.read() * *height.read()) / 100);
        }
        else if (*width.read() * *height.read() > 10) {
            batch_size.set((*width.read() * *height.read()) / 10);
        }
        else {
            batch_size.set(1);
        }
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
                    start_coord.set(Coord{ x: *start_coord_x.read(), y: *start_coord_y.read() });
                    finish_coord.set(Coord{ x: *finish_coord_x.read(), y: *finish_coord_y.read() });

                    wasm_bindgen_futures::spawn_local(async move {
                        if solved() {
                            solver_algo.write().reset(&mut maze);
                            solver_algo.set(get_solver_algo(solver_algo_choice.read().as_str(), &start_coord(), &finish_coord()));
                            solved.set(false);
                        }

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