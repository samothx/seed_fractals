#![allow(clippy::wildcard_imports)]
#![allow(dead_code)]

use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

mod complex;
use complex::Complex;

mod fractal;

mod util;

use fractal::Fractal;

mod canvas;

use canvas::Canvas;
use crate::util::{get_u32_from_input, get_f64_from_input};


const DEFAULT_XY: f64 = 1.5;
const DEFAULT_C: (f64, f64) = (-0.4, 0.6);
const DEFAULT_WIDTH: u32 = 1024;
const DEFAULT_HEIGHT: u32 = 600;
const DEFAULT_ITERATIONS: u32 = 400;
const ENTER_KEY: &str = "Enter";
const BACKGROUND_COLOR: &str = "#000000";
const STORAGE_KEY: &str = "seed_fractals";

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Clear);
    Model {
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        config:  LocalStorage::get(STORAGE_KEY).unwrap_or_default(),
        background_color: BACKGROUND_COLOR.to_string(),
        canvas: None,
        fractal: None,
        paused: true,
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    width: u32,
    height: u32,
    config: Config,
    background_color: String,
    canvas: Option<Canvas>,
    fractal: Option<Fractal>,
    paused: bool,
}

#[derive(Serialize, Deserialize)]
struct Config {
    max_iterations: u32,
    x_max: f64,
    x_min: f64,
    y_max: f64,
    y_min: f64,
    c_real: f64,
    c_imag: f64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_iterations: DEFAULT_ITERATIONS,
            x_max: DEFAULT_XY,
            x_min: -DEFAULT_XY,
            y_max: DEFAULT_XY,
            y_min: -DEFAULT_XY,
            c_real: DEFAULT_C.0,
            c_imag: DEFAULT_C.1,
        }
    }
}

// ------ ------
//    Update
// ------ ------


#[derive(Clone)]
enum Msg {
    Start,
    Pause,
    Clear,
    Edit,
    SaveEdit,
    CancelEdit,
    Draw,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Start => {
            log!("Message received: Start");
            if model.canvas.is_none() {
                let canvas = Canvas::new(&model);
                canvas.clear_canvas(&model);
                model.canvas = Some(canvas);
            }
            log!("Message received: Start, creating fractal");
            let mut fractal = Fractal::new(&model);
            model.canvas.as_ref().expect("unexpected missing canvas")
                .draw_results(fractal.calculate());
            model.fractal = Some(fractal);
            model.paused = false;
            orders.after_next_render(|_| Msg::Draw);
        }
        Msg::Pause => {
            log!("Message received: Pause");
            model.paused = true;
        }
        Msg::Clear => {
            log!("Message received: Clear");
            if !model.paused {
                model.paused = true;
            }
            model.fractal = None;
            if model.canvas.is_none() {
                let canvas = Canvas::new(&model);
                canvas.clear_canvas(&model);
                model.canvas = Some(canvas);
            } else {
                model.canvas.as_ref().expect("unexpected empty canvas").clear_canvas(&model);
            }
        }

        Msg::Edit => {
            log!("Message received: Edit");
            let document = window().document().expect("document not found");

            let _ = document.get_element_by_id("iterations").expect("iterations not found")
                .set_attribute(At::Value.as_str(), &model.config.max_iterations.to_string());

            let _ = document.get_element_by_id("max_x").expect("max_x not found")
                .set_attribute(At::Value.as_str(), &model.config.x_max.to_string());

            let _ = document.get_element_by_id("min_x").expect("min_x not found")
                .set_attribute(At::Value.as_str(), &model.config.x_min.to_string());

            let _ = document.get_element_by_id("max_y").expect("max_y not found")
                .set_attribute(At::Value.as_str(), &model.config.y_max.to_string());

            let _ = document.get_element_by_id("min_y").expect("min_y not found")
                .set_attribute(At::Value.as_str(), &model.config.y_min.to_string());

            let _ = document.get_element_by_id("c_real").expect("c_real not found")
                .set_attribute(At::Value.as_str(), &model.config.c_real.to_string());

            let _ = document.get_element_by_id("c_imag").expect("c_imag not found")
                .set_attribute(At::Value.as_str(), &model.config.c_imag.to_string());

            document.get_element_by_id("edit_cntr").expect("edit_cntr not found")
                .set_class_name("edit_cntr_visible");
        }
        Msg::SaveEdit => {
            log!("Message received: SaveEdit");
            let document = window().document().expect("document not found");

            if let Some(value) = get_u32_from_input("iterations") {
                model.config.max_iterations = value;
            }

            if let Some(value) = get_f64_from_input("max_x") {
                model.config.x_max = value;
            }

            if let Some(value) = get_f64_from_input("min_x") {
                model.config.x_min = value;
            }

            if let Some(value) = get_f64_from_input("max_y") {
                model.config.y_max = value;
            }

            if let Some(value) = get_f64_from_input("min_y") {
                model.config.y_min = value;
            }

            if let Some(value) = get_f64_from_input("c_real") {
                model.config.c_real = value;
            }

            if let Some(value) = get_f64_from_input("c_imag") {
                model.config.c_imag = value;
            }

            LocalStorage::insert(STORAGE_KEY, &model.config).expect("save data to LocalStorage");

            log!(format!("Save: saved values x_max: {}, x_min: {}, y_max: {}, y_min: {}, c: {}",
                model.config.x_max, model.config.x_min, model.config.y_max, model.config.y_min,
                Complex::new(model.config.c_real, model.config.c_imag)));
            document.get_element_by_id("edit_cntr").expect("edit_cntr not found")
                .set_class_name("edit_cntr_hidden");
            // TODO: save to local storage
            orders.after_next_render(|_| Msg::Clear);
        }
        Msg::CancelEdit => {
            log!("Message received: SaveEdit");
            window().document().expect("document not found")
                .get_element_by_id("edit_cntr").expect("edit_cntr not found")
                .set_class_name("edit_cntr_hidden");
        }

        Msg::Draw => {
            // log!("Message received: Draw");
            if !model.paused {
                model.canvas.as_ref().expect("unexpected missing canvas")
                    .draw_results(model.fractal.as_mut().expect("unexpectted missing fractal")
                        .calculate());
                if !model.fractal.as_ref().expect("unexpectted missing fractal").is_done() {
                    orders.after_next_render(|_| Msg::Draw);
                } else {
                    model.paused = true;
                }
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["outer_cntr"],
        h1!["Julia Sets"],
        view_buttons(model),
        view_editor(),
        div![
            C!["canvas_cntr"],
            canvas![
                C!["canvas"],
                id!("canvas"),
                attrs!{
                    At::Width => model.width.to_string(),
                    At::Height => model.height.to_string()
                },
                "Your browser does not support the canvas tag."
            ]
        ]
    ]
}

fn view_buttons(model: &Model) -> Vec<Node<Msg>> {
    vec![
        div![
            C!["button_cntr"],
            button![
                C!["button"],
                id!("start"),
                ev(Ev::Click, |_| Msg::Start),
                IF!(!model.paused =>  attrs!{At::Disabled => "true" } ),
                "Start"
            ],
            button![
                C!["button"],
                id!("pause"),
                ev(Ev::Click, |_| Msg::Pause),
                IF!(model.paused =>  attrs!{At::Disabled => "true" } ),
                "Pause"
            ],
            button![
                C!["button"],
                id!("clear"),
                ev(Ev::Click, |_| Msg::Clear),
                "Clear"
            ],
            button![
                C!["button"],
                id!("edit"),
                ev(Ev::Click, |_| Msg::Edit),
                "Edit"
            ],
        ]
    ]
}

fn view_editor() -> Node<Msg> {
    div![
        C!["edit_cntr_hidden"],
        id!("edit_cntr"),
        div![
            C!["input_cntr"],
            div![
                C!["input_inner"],
                label![
                    C!["input_label"],
                    attrs! { At::For => "iterations"}, "Iterations"],
                input![
                    C!["input"],
                    id!("iterations"),
                    attrs! {
                        At::Name => "iterations",
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
                    attrs! { At::For => "c_real"}, "C Real"],
                input![
                    C!["input"],
                    id!("c_real"),
                    attrs! {
                        At::Name => "c_real",
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
                    attrs! { At::For => "c_imag"}, "C Imaginary"],
                input![
                    C!["input"],
                    id!("c_imag"),
                    attrs! {
                        At::Name => "c_imag",
                        At::Type => "number",
                        At::Step => "0.0000001"
                        //At::Value => {model.c_imag.to_string()},
                    },
                ],
            ],
    ],
    div![
            C!["input_cntr"],
            div![
                C!["input_inner"],
                label![
                    C!["input_label"],
                    attrs! { At::For => "max_x"}, "Max. X"],
                input![
                    C!["input"],
                    id!("max_x"),
                    attrs! {
                        At::Name => "max_x",
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
                    attrs! { At::For => "min_x"}, "Min. X"],
                input![
                    C!["input"],
                    id!("min_x"),
                    attrs! {
                        At::Name => "min_x",
                        At::Type => "number",
                        At::Step => "0.01",
                        //At::Value => {model.x_min.to_string()},
                    },
                ]
            ],
            div![
                C!["input_inner"],

                label![
                    C!["input_label"],
                    attrs! { At::For => "max_y"}, "Max. X"],
                input![
                    C!["input"],
                    id!("max_y"),
                    attrs! {
                        At::Name => "max_y",
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
                    attrs! { At::For => "min_y"}, "Min. Y"],
                input![
                    C!["input"],
                    id!("min_y"),
                    attrs! {
                        At::Name => "min_y",
                        At::Type => "number",
                        At::Step => "0.01",
                        //At::Value => {model.y_min.to_string()},
                    },
                ]
            ],
        ],
        div![
            C!["button_cntr"],
            button![
                C!["button"],
                id!("save"),
                ev(Ev::Click, |_| Msg::SaveEdit),
                "Save"
            ],
            button![
                C!["button"],
                id!("cancel"),
                ev(Ev::Click, |_| Msg::CancelEdit),
                "Cancel"
            ],
        ]
    ]
}



// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
