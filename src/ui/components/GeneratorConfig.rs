use dioxus::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures;

use crate::generator_algorithms::generator_helpers::{get_generator_algo, get_generator_options, GeneratorStatus};
use crate::structures::maze::Maze;
use crate::ui::components::{Button::Button, Dropdown::Dropdown, NumInput::NumInput, NumSlider::NumSlider};

#[component]
pub fn GeneratorConfig(maze: Signal<Maze>, generated: Signal<bool>, working: Signal<bool>) -> Element {
    let width: Signal<usize> = use_signal(|| maze.read().width());
    let height: Signal<usize> = use_signal(|| maze.read().height());

    let generator_algo_choice: Signal<String> = use_signal(|| "ellers".to_string());
    let mut generator_algo = use_signal(|| get_generator_algo(generator_algo_choice.read().as_str()));

    let mut generator_speed: Signal<usize> = use_signal(|| 1);
    let mut generator_delay: Signal<u32> = use_signal(|| *generator_speed.read() as u32 * 10);
    let mut batch_size: Signal<usize> = use_signal(|| (*width.read() * *height.read()) / 10);

    use_effect(move || {
        generator_delay.set(*generator_speed.read() as u32 * 10);
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
                            max_val: 50,
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
                            max_val: 50,
                            min_val: 2,
                        }
                    }
                    div {
                        id: "generator-speed-config",
                        label { for: "generator-speed-slider", "Speed"}
                        NumSlider {
                            id: "generator-speed-slider",
                            value: generator_speed,
                            disabled: *working.read(),
                            max_val: 4,
                            min_val: 0,
                            step_val: 1,
                        }
                    }
                }
            }
            Button {
                button_text: "Generate maze".to_string(),
                disabled: *working.read(),
                onclick: move |_| {
                    generated.set(false);
                    working.set(true);
                    generator_algo.set(get_generator_algo(generator_algo_choice.read().as_str()));

                    wasm_bindgen_futures::spawn_local(async move {
                            maze.set(Maze::new(*height.read(), *width.read()));
                            TimeoutFuture::new(200).await;

                            while generator_algo.read().status() != &GeneratorStatus::Done {
                                for _ in 0..*batch_size.read() {
                                    if generator_algo.read().status() == &GeneratorStatus::Done {
                                        break;
                                    }
                                generator_algo.write().create_maze(&mut maze);
                                }

                                if *generator_delay.read() > 0 {
                                        TimeoutFuture::new(*generator_delay.read()).await;
                                }

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