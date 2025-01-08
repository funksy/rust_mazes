use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct DropdownProps {
    pub options: Vec<(String, String)>,
    pub helper_text: String,
    pub id: String,
}

pub fn Dropdown(props: DropdownProps) -> Element {
    let mut selected = use_signal(|| String::new());

    rsx!{
        select {
            id: "{props.id}",
            value: "{selected}",
            onchange: move |event| selected.set(event.value()),

            option { selected: "", disabled: true, value: "", "{props.helper_text}" },

            for option in props.options {
                option { key: "{option.0}", value: "{option.0}", label: "{option.1}" }
            }
        }
    }
}