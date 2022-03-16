#[allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};

use super::{FractalType, Model, Msg};

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["outer_cntr"],
        IF![model.config.active_config == FractalType::Mandelbrot => h1!["Mandelbrot Set"]],
        IF![model.config.active_config == FractalType::JuliaSet => h1!["Julia Set"]],
        div![
            C!["inner_cntr"],
            view_buttons(model),
            div![
                C!["fractal_container"],
                view_julia_set_cfg_editor(),
                view_mandelbrot_cfg_editor(),
                div![
                    C!["canvas_cntr"],
                    canvas![
                        C!["canvas"],
                        id!("canvas"),
                        attrs! {
                            At::Width => model.width.to_string(),
                            At::Height => model.height.to_string()
                        },
                        "Your browser does not support the canvas tag.",
                        IF!(model.edit_mode =>
                                ev(Ev::MouseDown, |event| {
                                    let mouse_event: web_sys::MouseEvent = event.unchecked_into();
                                    Msg::MouseDown(mouse_event)})
                        ),
                        IF!(model.mouse_drag.is_some() =>
                            vec![
                                ev(Ev::MouseMove, |event| {
                                    let mouse_event: web_sys::MouseEvent = event.unchecked_into();
                                    Msg::MouseMove(mouse_event)
                                }),
                                ev(Ev::MouseUp, |event| {
                                    let mouse_event: web_sys::MouseEvent = event.unchecked_into();
                                    Msg::MouseUp(Some(mouse_event))
                                })
                            ]
                        ),
                    ]
                ]
            ]
        ]
    ]
}

fn view_buttons(model: &Model) -> Vec<Node<Msg>> {
    vec![div![
        C!["button_cntr"],
        button![
            C!["menu_button"],
            id!("start"),
            ev(Ev::Click, |_| Msg::Start),
            IF!(!model.paused =>  attrs!{At::Disabled => "true" } ),
            "Start"
        ],
        button![
            C!["menu_button"],
            id!("pause"),
            ev(Ev::Click, |_| Msg::Stop),
            IF!(model.paused =>  attrs!{At::Disabled => "true" } ),
            "Stop"
        ],
        button![
            C!["menu_button"],
            id!("clear"),
            ev(Ev::Click, |_| Msg::Clear),
            "Clear"
        ],
        button![
            C!["menu_button"],
            id!("edit"),
            ev(Ev::Click, |_| Msg::Edit),
            IF!(model.edit_mode =>  attrs!{At::Disabled => "true" } ),
            "Edit"
        ],
        label![
            C!["type_select_label"],
            attrs! { At::For => "type_select"},
            "Select Type"
        ],
        select![
            C!["type_select"],
            id!("type_select"),
            attrs! {At::Name => "type_select" },
            IF![model.config.active_config == FractalType::Mandelbrot => attrs!{At::Value => "type_mandelbrot"}],
            IF![model.config.active_config == FractalType::JuliaSet => attrs!{At::Value => "type_julia_set"}],
            option![attrs! {At::Value => "type_mandelbrot" }, "Mandelbrot Set"],
            option![attrs! {At::Value => "type_julia_set" }, "Julia Set"],
            IF!(!model.paused =>  attrs!{At::Disabled => "true" } ),
            IF!(model.edit_mode =>  attrs!{At::Disabled => "true" } ),
            ev(Ev::Change, |_| Msg::TypeChanged),
        ],
        div![
            C!["cb_stats_cntr"],
            label![
                C!["type_select_label"],
                attrs! { At::For => "stats_cb"},
                "View Stats"
            ],
            input![
                C!["stats_cb"],
                id!("stats_cb"),
                attrs! {
                    At::Name => "stats_cb",
                    At::Type => "checkbox",
                },
                IF!(!model.paused => attrs!{ At::Disabled => "" }),
                IF!(model.config.view_stats => attrs!{ At::Checked => "1" }),
                ev(Ev::Change, |_| Msg::StatsChanged),
            ]
        ],
        div![
            IF![model.config.view_stats => C!["stats_cntr_visible"]],
            IF![!model.config.view_stats => C!["stats_cntr_hidden"]],
            textarea![
                C!["stats_text"],
                attrs! {
                   // At::Value => model.stats_text,
                   At::ReadOnly => "true",
                   At::Rows => "4",
                   At::Placeholder => "No Stats yet"
                },
                model.stats_text.as_str()
            ],
        ]
    ]]
}

#[allow(clippy::too_many_lines)]
fn view_julia_set_cfg_editor() -> Node<Msg> {
    div![
        C!["edit_cntr_hidden"],
        id!("julia_edit_cntr"),
        div![
            C!["input_cntr"],
            div![
                C!["input_inner"],
                label![
                    C!["input_label"],
                    attrs! { At::For => "julia_iterations"},
                    "Iterations"
                ],
                input![
                    C!["input"],
                    id!("julia_iterations"),
                    attrs! {
                        At::Name => "julia_iterations",
                        At::Type => "number",
                        At::Min =>"100",
                        At::Max =>"1000",
                        // At::Value => {model.max_iterations.to_string()},
                    },
                ],
            ],
            div![
                C!["input_inner"],
                label![
                    C!["input_label"],
                    attrs! { At::For => "julia_c_real"},
                    "C Real"
                ],
                input![
                    C!["input"],
                    id!("julia_c_real"),
                    attrs! {
                        At::Name => "julia_c_real",
                        At::Type => "number",
                        At::Step => "0.0000001"
                        //At::Value => {model.c_real.to_string()},
                    },
                ],
            ],
            div![
                C!["input_inner"],
                label![
                    C!["input_label"],
                    attrs! { At::For => "julia_c_imag"},
                    "C Imag."
                ],
                input![
                    C!["input"],
                    id!("julia_c_imag"),
                    attrs! {
                        At::Name => "julia_c_imag",
                        At::Type => "number",
                        At::Step => "0.0000001"
                        //At::Value => {model.c_imag.to_string()},
                    },
                ],
            ],
            button![
                C!["editor_button"],
                id!("julia_reset_params"),
                ev(Ev::Click, |_| Msg::ResetParams),
                "Reset to Default",
                ev(Ev::Click, |_| Msg::ResetParams),
            ]
        ],
        div![
            C!["input_cntr"],
            div![
                C!["area_cntr"],
                div![
                    C!["input_inner"],
                    label![
                        C!["input_label"],
                        attrs! { At::For => "julia_max_real"},
                        "Max. Real"
                    ],
                    input![
                        C!["input"],
                        id!("julia_max_real"),
                        attrs! {
                            At::Name => "julia_max_real",
                            At::Type => "number",
                            At::Step => "0.01",
                            //At::Value => {model.x_max.to_string()},
                        },
                    ]
                ],
                div![
                    C!["input_inner"],
                    label![
                        C!["input_label"],
                        attrs! { At::For => "julia_min_real"},
                        "Min. Real"
                    ],
                    input![
                        C!["input"],
                        id!("julia_min_real"),
                        attrs! {
                            At::Name => "julia_min_real",
                            At::Type => "number",
                            At::Step => "0.01",
                            //At::Value => {model.x_min.to_string()},
                        },
                    ]
                ],
            ],
            div![
                C!["area_cntr"],
                div![
                    C!["input_inner"],
                    label![
                        C!["input_label"],
                        attrs! { At::For => "julia_max_imag"},
                        "Max. Imag."
                    ],
                    input![
                        C!["input"],
                        id!("julia_max_imag"),
                        attrs! {
                            At::Name => "julia_max_imag",
                            At::Type => "number",
                            At::Step => "0.01",
                            //At::Value => {model.y_max.to_string()},
                        },
                    ]
                ],
                div![
                    C!["input_inner"],
                    label![
                        C!["input_label"],
                        attrs! { At::For => "julia_min_imag"},
                        "Min. Imag."
                    ],
                    input![
                        C!["input"],
                        id!("julia_min_imag"),
                        attrs! {
                            At::Name => "julia_min_imag",
                            At::Type => "number",
                            At::Step => "0.01",
                            //At::Value => {model.y_min.to_string()},
                        },
                    ]
                ],
            ],
            div![
                C!["area_cntr"],
                button![
                    C!["editor_button"],
                    id!("julia_reset_params"),
                    ev(Ev::Click, |_| Msg::ResetParams),
                    "Reset to Default",
                    ev(Ev::Click, |_| Msg::ResetArea),
                ],
                button![
                    C!["editor_button"],
                    id!("julia_reset_params"),
                    ev(Ev::Click, |_| Msg::ResetParams),
                    "Zoom Out",
                    ev(Ev::Click, |_| Msg::ZoomOutArea),
                ]
            ]
        ],
        div![
            C!["edit_button_cntr"],
            button![
                C!["editor_button"],
                id!("julia_save"),
                ev(Ev::Click, |_| Msg::SaveEdit),
                "Save"
            ],
            button![
                C!["editor_button"],
                id!("julia_cancel"),
                ev(Ev::Click, |_| Msg::CancelEdit),
                "Cancel"
            ]
        ]
    ]
}

#[allow(clippy::too_many_lines)]
fn view_mandelbrot_cfg_editor() -> Node<Msg> {
    div![
        C!["edit_cntr_hidden"],
        id!("mandelbrot_edit_cntr"),
        div![
            C!["input_cntr"],
            div![
                C!["input_inner"],
                label![
                    C!["input_label"],
                    attrs! { At::For => "mandelbrot_iterations"},
                    "Iterations"
                ],
                input![
                    C!["input"],
                    id!("mandelbrot_iterations"),
                    attrs! {
                        At::Name => "mandelbrot_iterations",
                        At::Type => "number",
                        At::Min =>"100",
                        At::Max =>"1000",
                        // At::Value => {model.max_iterations.to_string()},
                    },
                ],
            ],
            button![
                C!["editor_button"],
                id!("julia_reset_params"),
                ev(Ev::Click, |_| Msg::ResetParams),
                "Reset to Default",
                ev(Ev::Click, |_| Msg::ResetParams),
            ]
        ],
        div![
            C!["input_cntr"],
            div![
                C!["area_cntr"],
                div![
                    C!["input_inner"],
                    label![
                        C!["input_label"],
                        attrs! { At::For => "mandelbrot_max_real"},
                        "Max. Real"
                    ],
                    input![
                        C!["input"],
                        id!("mandelbrot_max_real"),
                        attrs! {
                            At::Name => "mandelbrot_max_real",
                            At::Type => "number",
                            At::Step => "0.01",
                            //At::Value => {model.x_max.to_string()},
                        },
                    ]
                ],
                div![
                    C!["input_inner"],
                    label![
                        C!["input_label"],
                        attrs! { At::For => "mandelbrot_min_real"},
                        "Min. Real"
                    ],
                    input![
                        C!["input"],
                        id!("mandelbrot_min_real"),
                        attrs! {
                            At::Name => "mandelbrot_min_real",
                            At::Type => "number",
                            At::Step => "0.01",
                            //At::Value => {model.x_min.to_string()},
                        },
                    ]
                ],
            ],
            div![
                C!["area_cntr"],
                div![
                    C!["input_inner"],
                    label![
                        C!["input_label"],
                        attrs! { At::For => "mandelbrot_max_imag"},
                        "Max. Imag."
                    ],
                    input![
                        C!["input"],
                        id!("mandelbrot_max_imag"),
                        attrs! {
                            At::Name => "mandelbrot_max_imag",
                            At::Type => "number",
                            At::Step => "0.01",
                            //At::Value => {model.y_max.to_string()},
                        },
                    ]
                ],
                div![
                    C!["input_inner"],
                    label![
                        C!["input_label"],
                        attrs! { At::For => "mandelbrot_min_imag"},
                        "Min. Imag."
                    ],
                    input![
                        C!["input"],
                        id!("mandelbrot_min_imag"),
                        attrs! {
                            At::Name => "mandelbrot_min_imag",
                            At::Type => "number",
                            At::Step => "0.01",
                            //At::Value => {model.y_min.to_string()},
                        },
                    ]
                ],
            ],
            div![
                C!["area_cntr"],
                button![
                    C!["editor_button"],
                    id!("julia_reset_params"),
                    ev(Ev::Click, |_| Msg::ResetParams),
                    "Reset to Default",
                    ev(Ev::Click, |_| Msg::ResetArea),
                ],
                button![
                    C!["editor_button"],
                    id!("julia_reset_params"),
                    ev(Ev::Click, |_| Msg::ResetParams),
                    "Zoom Out",
                    ev(Ev::Click, |_| Msg::ZoomOutArea),
                ]
            ]
        ],
        div![
            C!["edit_button_cntr"],
            button![
                C!["editor_button"],
                id!("mandelbrot_save"),
                ev(Ev::Click, |_| Msg::SaveEdit),
                "Save"
            ],
            button![
                C!["editor_button"],
                id!("mandelbrot_cancel"),
                ev(Ev::Click, |_| Msg::CancelEdit),
                "Cancel"
            ]
        ]
    ]
}
