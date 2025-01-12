use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    onclick: EventHandler<MouseEvent>,
    button_text: String,
    #[props(default = false)]
    disabled: bool,
}

pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            disabled: "{props.disabled}",
            onclick: move |evt| props.onclick.call(evt),
            "{props.button_text}",
        }
    }
}