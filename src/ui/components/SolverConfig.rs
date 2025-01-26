use dioxus::prelude::*;
use crate::ui::components::Dropdown::Dropdown;
use crate::ui::components::NumInput::NumInput;

#[component]
pub fn SolverConfig(
    dropdown_options: Vec<(String, String)>,
    solver_algo_choice: Signal<String>,
    height: Signal<usize>,
    width: Signal<usize>,
    starting_coord_x: Signal<usize>,
    starting_coord_y: Signal<usize>,
    finishing_coord_x: Signal<usize>,
    finishing_coord_y: Signal<usize>,
    solved: bool,
    solver_delay: Signal<usize>,
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
                    disabled: solved,
                }
                div {
                    id: "start-finish-config",
                    div {
                        p { class: "coord-label", "Starting Cell" }
                        div {
                            id: "starting-coord-config",
                            label { for: "starting-coord-x", "x:" },
                            NumInput {
                                id: "starting-coord-x",
                                value: starting_coord_x,
                                max_val: *width.read() - 1,
                                min_val: 0,
                            }
                            label { for: "starting-coord-y", "y:" },
                            NumInput {
                                id: "starting-coord-y",
                                value: starting_coord_y,
                                max_val: *height.read() - 1,
                                min_val: 0,
                            }
                        }
                    }
                    div {
                        label { for: "finishing-coord-x", "Finishing Cell" },
                        div {
                            id: "finishing-coord-config",
                            label { for: "finishing-coord-x", "x:" },
                            NumInput {
                                id: "finishing-coord-x",
                                value: finishing_coord_x,
                                max_val: *width.read() - 1,
                                min_val: 0,
                            }
                            label { for: "finishing-coord-y", "y:" },
                            NumInput {
                                id: "finishing-coord-y",
                                value: finishing_coord_y,
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
                    max_val: 100,
                    min_val: 0,
                }
            }
        }
    }
}
