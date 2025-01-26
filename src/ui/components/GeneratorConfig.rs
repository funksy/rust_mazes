use dioxus::prelude::*;

use crate::ui::components::Dropdown::Dropdown;
use crate::ui::components::NumInput::NumInput;

#[component]
pub fn GeneratorConfig(
    dropdown_options: Vec<(String, String)>,
    height: Signal<usize>, width: Signal<usize>,
    generator_algo_choice: Signal<String>,
    disabled: bool,
    generator_delay: Signal<usize>
) -> Element {
    rsx! {
        form {
            fieldset {
                id: "generator-algo-config",
                legend { "Generator Config" },

                Dropdown {
                    id: "generator-dropdown",
                    options: dropdown_options,
                    helper_text: "Maze Generator Algo".to_string(),
                    value: generator_algo_choice,
                    disabled: disabled,
                }
                div {
                    id: "height-config",
                    label { for: "height-input", "Height:" },
                    NumInput {
                        id: "height-input",
                        value: height,
                        max_val: 200,
                        min_val: 2,
                    }
                }

                div {
                    id: "width-config",
                    label { for: "width-input", "Width:" },
                    NumInput {
                        id: "width-input",
                        value: width,
                        max_val: 200,
                        min_val: 2,
                    }
                }
                label { for: "generator-delay-config", "Render Delay (ms)" },
                NumInput {
                    id: "generator-delay-config",
                    value: generator_delay,
                    max_val: 100,
                    min_val: 0,
                }
            }
        }
    }
}