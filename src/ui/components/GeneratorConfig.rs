use dioxus::prelude::*;

use crate::ui::components::Dropdown::Dropdown;

#[component]
pub fn GeneratorConfig(dropdown_options: Vec<(String, String)>) -> Element {
    rsx! {
        form {
            fieldset {
                id: "generator-algo-config",
                legend { "Generator Config" },

                Dropdown {
                    id: "generator-dropdown",
                    options: dropdown_options,
                    helper_text: "Maze Generator Algo".to_string(),
                }
            }
        }
    }
}