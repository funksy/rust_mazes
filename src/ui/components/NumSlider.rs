use dioxus::prelude::*;

#[component]
pub fn NumSlider(id: String, mut value: Signal<usize>, disabled: bool, max_val: usize, min_val: usize, step_val: usize) -> Element {
    rsx! {
        input {
            id: "{id}",
            type: "range",
            disabled: disabled,
            max: max_val,
            min: min_val,
            step: step_val,
            value: value,
            onchange: move |evt| {
                if let Ok(num) = evt.value().parse::<usize>() {
                    value.set(num);
                }
            }
        }
    }
}