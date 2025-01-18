use dioxus::prelude::*;

#[component]
pub fn Dropdown(options: Vec<(String, String)>, helper_text: String, id: String,) -> Element {
    let mut selected = use_signal(|| String::new());

    rsx!{
        select {
            id: "{id}",
            value: "{selected}",
            onchange: move |event| selected.set(event.value()),

            option { selected: "", disabled: true, value: "", "{helper_text}" },

            for option in options {
                option { key: "{option.0}", value: "{option.0}", label: "{option.1}" }
            }
        }
    }
}