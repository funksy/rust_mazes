use dioxus::prelude::*;
use crate::ui::components::Dropdown::Dropdown;
use crate::ui::components::NumInput::NumInput;

#[component]
pub fn SolverConfig(
    dropdown_options: Vec<(String, String)>,
    solver_algo_choice: Signal<String>,
    height: Signal<usize>,
    width: Signal<usize>,
    start_coord_x: Signal<usize>,
    start_coord_y: Signal<usize>,
    finish_coord_x: Signal<usize>,
    finish_coord_y: Signal<usize>,
    solver_delay: Signal<usize>,
    working: Signal<bool>,
) -> Element {

    rsx! {
        form {
            fieldset {
                id: "solver-algo-config",
                legend { "Solver Config" },

                Dropdown {
                    id: "solver-dropdown",
                    options: dropdown_options,
                    helper_text: "Maze Solver Algo".to_string(),
                    value: solver_algo_choice,
                    disabled: *working.read(),
                }
                div {
                    id: "start-finish-config",
                    div {
                        p { class: "coord-label", "Starting Cell" }
                        div {
                            id: "start-coord-config",
                            label { for: "start-coord-x", "x:" },
                            NumInput {
                                id: "start-coord-x",
                                value: start_coord_x,
                                disabled: *working.read(),
                                max_val: *width.read() - 1,
                                min_val: 0,
                            }
                            label { for: "start-coord-y", "y:" },
                            NumInput {
                                id: "start-coord-y",
                                value: start_coord_y,
                                disabled: *working.read(),
                                max_val: *height.read() - 1,
                                min_val: 0,
                            }
                        }
                    }
                    div {
                        label { for: "finish-coord-x", "Finishing Cell" },
                        div {
                            id: "finish-coord-config",
                            label { for: "finish-coord-x", "x:" },
                            NumInput {
                                id: "finish-coord-x",
                                value: finish_coord_x,
                                disabled: *working.read(),
                                max_val: *width.read() - 1,
                                min_val: 0,
                            }
                            label { for: "finish-coord-y", "y:" },
                            NumInput {
                                id: "finish-coord-y",
                                value: finish_coord_y,
                                disabled: *working.read(),
                                max_val: *height.read() - 1,
                                min_val: 0,
                            }
                        }
                    }
                }
                label { for: "solver-delay-config", "Render Delay (ms)" },
                NumInput {
                    id: "solver-delay-config",
                    value: solver_delay,
                    disabled: *working.read(),
                    max_val: 100,
                    min_val: 0,
                }
            }
        }
    }
}
