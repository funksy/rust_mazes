use dioxus::prelude::*;

#[component]
pub fn NumInput(mut value: Signal<usize>, max_val: usize) -> Element {
    rsx! {
        input {
            type: "number",
            placeholder: "",
            max: max_val,
            min: 2,
            value: value,
            onchange: move |evt| {
                if let Ok(num) = evt.value().parse::<usize>()  {
                    if num > max_val {
                        evt.prevent_default();
                        value.set(max_val);
                    } else if num < 2 {
                        value.set(2);
                    } else {
                        value.set(num);
                    }
                }
            }
        }
    }
}