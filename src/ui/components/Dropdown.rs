use dioxus::prelude::*;

#[component]
pub fn Dropdown(options: Vec<(String, String)>, helper_text: String, id: String, mut value: Signal<String>, disabled: bool) -> Element {
    rsx!{
        select {
            id: "{id}",
            value: "{value}",
            disabled: disabled,
            onchange: move |event| value.set(event.value()),

            option { selected: "", disabled: true, value: "", "{helper_text}" },

            for option in options {
                option { key: "{option.0}", value: "{option.0}", label: "{option.1}" }
            }
        }
    }
}