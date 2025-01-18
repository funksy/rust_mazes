use dioxus::prelude::*;
use crate::cell::Coord;
use crate::ui::components::Dropdown::Dropdown;
use crate::ui::components::NumInput::NumInput;

#[component]
pub fn SolverConfig(
    dropdown_options: Vec<(String, String)>,
    height: Signal<usize>,
    width: Signal<usize>,
    starting_coord: Signal<Coord>,
    finishing_coord: Signal<Coord>
) -> Element {
    let mut starting_coord_x: Signal<usize> = use_signal(|| 0);
    let mut starting_coord_y: Signal<usize> = use_signal(|| 0);
    let mut finishing_coord_x: Signal<usize> = use_signal(|| width - 1);
    let mut finishing_coord_y: Signal<usize> = use_signal(|| height - 1);

    use_effect(move || {
        starting_coord_x();
        starting_coord_y();
        finishing_coord_x();
        finishing_coord_y();

        starting_coord.set(Coord { x: *starting_coord_x.read(), y: *starting_coord_y.read() });
        finishing_coord.set(Coord { x: *finishing_coord_x.read(), y: *finishing_coord_y.read() });
    });

    rsx! {
        form {
            fieldset {
                id: "solver-algo-config",
                legend { "Solver Config" },

                Dropdown {
                    id: "solver-dropdown",
                    options: dropdown_options,
                    helper_text: "Maze Solver Algo".to_string(),
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
                                max_val: *width.read(),
                                min_val: 0,
                            }
                            label { for: "starting-coord-y", "y:" },
                            NumInput {
                                id: "starting-coord-y",
                                value: starting_coord_y,
                                max_val: *height.read(),
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
                                max_val: *width.read(),
                                min_val: 0,
                            }
                            label { for: "finishing-coord-y", "y:" },
                            NumInput {
                                id: "finishing-coord-y",
                                value: finishing_coord_y,
                                max_val: *height.read(),
                                min_val: 0,
                            }
                        }
                    }
                }
            }
        }
    }
}
