use dioxus::prelude::*;

#[component]
pub fn Button(onclick: EventHandler<MouseEvent>, button_text: String, disabled: bool) -> Element {
    rsx! {
        button {
            disabled: "{disabled}",
            onclick: move |evt| onclick.call(evt),
            "{button_text}",
        }
    }
}