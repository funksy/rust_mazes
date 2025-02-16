use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures;

use crate::generator_algorithms::generator_helpers::{get_generator_algo, get_generator_options, GeneratorStatus};
use crate::structures::maze::Maze;
use crate::ui::components::{Button::Button, Dropdown::Dropdown, NumInput::NumInput};

#[component]
pub fn GeneratorConfigNew(maze: Signal<Maze>, generated: Signal<bool>, working: Signal<bool>) -> Element {
    let width: Signal<usize> = use_signal(|| maze.read().width());
    let height: Signal<usize> = use_signal(|| maze.read().height());

    let generator_algo_choice: Signal<String> = use_signal(|| "ellers".to_string());
    let mut generator_algo = use_signal(|| get_generator_algo(generator_algo_choice.read().as_str()));

    use_effect(move || {
        generator_algo_choice();
        generator_algo.set(get_generator_algo(generator_algo_choice.read().as_str()));
    });

    rsx!{
        div {
            id: "generator-config",
            class: "config-div",
            form {
                fieldset {
                    id: "generator-algo-config",
                    legend { "Generator Config" },
                    Dropdown {
                        id: "generator-dropdown",
                        options: get_generator_options(),
                        helper_text: "Maze Generator Algo".to_string(),
                        value: generator_algo_choice,
                        disabled: *working.read(),
                    }
                    div {
                        id: "height-config",
                        label { for: "height-input", "Height" },
                        NumInput {
                            id: "height-input",
                            value: height,
                            disabled: *working.read(),
                            max_val: 200,
                            min_val: 2,
                        }
                    }
                    div {
                        id: "width-config",
                        label { for: "width-input", "Width" },
                        NumInput {
                            id: "width-input",
                            value: width,
                            disabled: *working.read(),
                            max_val: 200,
                            min_val: 2,
                        }
                    }
                    label { for: "generator-delay-config", "Render Delay (ms)" }
                    NumInput {
                        id: "generator-delay-config",
                        value: use_signal(|| 20 as usize),
                        disabled: *working.read(),
                        max_val: 100,
                        min_val: 0,
                    }
                }
            }
            Button {
                button_text: "Generate maze".to_string(),
                disabled: *working.read(),
                onclick: move |_| {
                    generated.set(false);
                    working.set(true);

                    wasm_bindgen_futures::spawn_local(async move {
                            maze.set(Maze::new(*height.read(), *width.read()));
                            TimeoutFuture::new(200).await;

                            // let batch = *batch_size.read();

                            while generator_algo.read().status() != &GeneratorStatus::Done {
                                // for _ in 0..batch {
                                //     if generator_algo.read().status() == &GeneratorStatus::Done {
                                //         break;
                                //     }
                                generator_algo.write().create_maze(&mut maze);
                                // }

                                // if *generator_delay.read() > 0 {
                                //         TimeoutFuture::new(*generator_delay.read() as u32).await;
                                // }

                            }
                            if generator_algo.read().status() == &GeneratorStatus::Done {
                                generated.set(true);
                                working.set(false);
                            }
                        });

                    generated.set(true);
                }
            }
        }
    }
}