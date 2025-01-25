use dioxus::prelude::*;

use crate::ui::components::NumInput::NumInput;

#[component]
pub fn DimensionConfig(height: Signal<usize>, width: Signal<usize>) -> Element {
    rsx! {
        form {
            fieldset {
                id: "dimension-config",
                legend { "Maze Config" },

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
            }
        }
    }
}