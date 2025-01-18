use dioxus::prelude::*;

#[component]
pub fn NumInput(id: String, mut value: Signal<usize>, max_val: usize, min_val: usize) -> Element {
    rsx! {
        input {
            id: "{id}",
            type: "number",
            max: max_val,
            min: min_val,
            value: value,
            onchange: move |evt| {
                if let Ok(num) = evt.value().parse::<usize>()  {
                    if num > max_val {
                        evt.prevent_default();
                        value.set(max_val);
                    } else if num < min_val {
                        value.set(min_val);
                    } else {
                        value.set(num);
                    }
                }
            }
        }
    }
}