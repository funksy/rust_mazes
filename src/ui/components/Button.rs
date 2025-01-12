use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    // onclick: EventHandler<MouseEvent>,
    button_text: String,
}

pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            // onclick: move |evt| props.onclick.call(evt),
            "{props.button_text}",
        }
    }
}