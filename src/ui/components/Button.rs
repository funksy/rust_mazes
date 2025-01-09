use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {

}

pub fn Button() -> Element {
    rsx! {
        button {
            "Generate maze"
        }
    }
}