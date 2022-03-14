#![allow(clippy::wildcard_imports)]
#![allow(dead_code)]

use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

mod complex;

use complex::Complex;

mod julia_set;

use julia_set::JuliaSet;

mod util;

use util::{get_u32_from_input, get_f64_from_input};

mod canvas;

use canvas::Canvas;
use web_sys::{ImageData, HtmlSelectElement};
use crate::util::{set_f64_on_input, set_u32_on_input};


const DEFAULT_XY: f64 = 1.5;
const DEFAULT_C: (f64, f64) = (-0.4, 0.6);
const DEFAULT_WIDTH: u32 = 1024;
const DEFAULT_HEIGHT: u32 = 600;
const DEFAULT_ITERATIONS: u32 = 400;
const ENTER_KEY: &str = "Enter";
const BACKGROUND_COLOR: &str = "#000000";
const STORAGE_KEY: &str = "seed_fractals_v1";

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Clear);
    Model {
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        config: LocalStorage::get(STORAGE_KEY).unwrap_or_default(),
        background_color: BACKGROUND_COLOR.to_string(),
        canvas: None,
        fractal: None,
        mouse_drag: None,
        paused: true,
        edit_mode: false,
        active_config: FractalType::JuliaSet
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
    fractal: Option<JuliaSet>,
    mouse_drag: Option<MouseDrag>,
    paused: bool,
    edit_mode: bool,
    active_config: FractalType
}

#[derive(Serialize, Deserialize)]
struct Config {
    julia_set_cfg: JuliaSetCfg,
    mandelbrot_cfg: MandelbrotCfg
}

impl Default for Config {
    fn default() -> Self {
        Config {
            julia_set_cfg: JuliaSetCfg::default(),
            mandelbrot_cfg: MandelbrotCfg::default()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct JuliaSetCfg {
    max_iterations: u32,
    x_max: Complex,
    x_min: Complex,
    c: Complex,
}

impl Default for JuliaSetCfg {
    fn default() -> Self {
        JuliaSetCfg{
            max_iterations: 400,
            x_max: Complex::new(1.5,1.0),
            x_min: Complex::new(-1.5,-1.0),
            c: Complex::new(-0.8,0.156)
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MandelbrotCfg {
    max_iterations: u32,
    c_max: Complex,
    c_min: Complex,
}

impl Default for MandelbrotCfg {
    fn default() -> Self {
        MandelbrotCfg{
            max_iterations: 400,
            c_max: Complex::new(0.47, 1.12),
            c_min: Complex::new(-2.00, -1.12)
        }
    }
}

#[derive(Debug, Copy, Clone,PartialEq)]
enum FractalType {
    Mandelbrot,
    JuliaSet
}

struct MouseDrag {
    start: (u32, u32),
    curr: (u32, u32),
    image_data: Option<ImageData>,
}


// ------ ------
//    Update
// ------ ------


#[derive(Clone)]
enum Msg {
    Start,
    Pause,
    Clear,
    TypeChanged,
    EditJulaSet,
    SaveEditJuliaSet,
    CancelEditJuliaSet,
    Draw,
    MouseDown(web_sys::MouseEvent),
    MouseMove(web_sys::MouseEvent),
    MouseUp(Option<web_sys::MouseEvent>),
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
            let mut fractal = JuliaSet::new(&model);
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
        },
        Msg::TypeChanged => {
            log!("Message received: TypeChanged");
            let selected = window().document().expect("document not found in window")
                .get_element_by_id("type_select").expect("type_select not found")
                .dyn_into::<HtmlSelectElement>().expect("type_select is not a HtmlSelectElement")
                .value();

            model.active_config = match selected.as_str() {
                "type_mandelbrot" => FractalType::Mandelbrot,
                "type_julia_set" => FractalType::JuliaSet,
                _ => model.active_config
            };
        },

        Msg::EditJulaSet => {
            log!("Message received: Edit");
            model.edit_mode = true;
            set_u32_on_input("iterations",model.config.julia_set_cfg.max_iterations);
            set_f64_on_input("max_real", model.config.julia_set_cfg.x_max.real());
            set_f64_on_input("min_real", model.config.julia_set_cfg.x_min.real());
            set_f64_on_input("max_imag", model.config.julia_set_cfg.x_max.imag());
            set_f64_on_input("min_imag", model.config.julia_set_cfg.x_min.imag());
            set_f64_on_input("c_real", model.config.julia_set_cfg.c.real());
            set_f64_on_input("c_imag", model.config.julia_set_cfg.c.imag());

            window().document().expect("document not found").get_element_by_id("edit_cntr").expect("edit_cntr not found")
                .set_class_name("edit_cntr_visible");
        },

        Msg::SaveEditJuliaSet => {
            log!("Message received: SaveEdit");
            model.edit_mode = false;
            let document = window().document().expect("document not found");

            if let Some(value) = get_u32_from_input("iterations") {
                model.config.julia_set_cfg.max_iterations = value;
            }

            if let Some(value) = get_f64_from_input("max_real") {
                model.config.julia_set_cfg.x_max.set_real(value);
            }

            if let Some(value) = get_f64_from_input("min_real") {
                model.config.julia_set_cfg.x_min.set_real(value);
            }

            if let Some(value) = get_f64_from_input("max_imag") {
                model.config.julia_set_cfg.x_max.set_imag( value);
            }

            if let Some(value) = get_f64_from_input("min_imag") {
                model.config.julia_set_cfg.x_min.set_imag(value);
            }

            if let Some(value) = get_f64_from_input("c_real") {
                model.config.julia_set_cfg.c.set_real(value);
            }

            if let Some(value) = get_f64_from_input("c_imag") {
                model.config.julia_set_cfg.c.set_imag(value);
            }

            LocalStorage::insert(STORAGE_KEY, &model.config).expect("save data to LocalStorage");

            document.get_element_by_id("edit_cntr").expect("edit_cntr not found")
                .set_class_name("edit_cntr_hidden");
            // TODO: save to local storage
            orders.after_next_render(|_| Msg::Clear);
        }
        Msg::CancelEditJuliaSet => {
            log!("Message received: SaveEdit");
            model.edit_mode = false;
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
        Msg::MouseDown(ev) => {
            log!("Message received: MouseDown");
            if let Some(canvas_coords) = model.canvas.as_ref().expect("unexpected missing canvas")
                .viewport_to_canvas_coords(ev.client_x(), ev.client_y()) {
                model.mouse_drag = Some(MouseDrag {
                    start: canvas_coords,
                    curr: canvas_coords,
                    image_data: None,
                });
            }
        }
        Msg::MouseMove(ev) => {
            log!("Message received: MouseMove");
            if let Some(mouse_drag) = model.mouse_drag.as_mut() {
                let canvas = model.canvas.as_ref().expect("unexpected missing canvas");

                if let Some(image_data) = mouse_drag.image_data.as_ref() {
                    canvas.undraw(image_data);
                }

                if let Some(canvas_coords) = canvas.viewport_to_canvas_coords(ev.client_x(), ev.client_y()) {
                    mouse_drag.curr = canvas_coords;
                    mouse_drag.image_data = Some(canvas.draw_frame(
                        mouse_drag.start.0, mouse_drag.start.1,
                        mouse_drag.curr.0, mouse_drag.curr.1));
                } else {
                    mouse_drag.image_data = None;
                    orders.after_next_render(|_| Msg::MouseUp(None));
                }
            }
        }
        Msg::MouseUp(ev) => {
            log!("Message received: MouseUp");
            if let Some(mouse_drag) = model.mouse_drag.as_mut() {
                let canvas = model.canvas.as_ref().expect("unexpected missing canvas");
                if let Some(image_data) = mouse_drag.image_data.as_ref() {
                    canvas.undraw(image_data);
                }
                if let Some(mouse_ev) = ev {
                    if let Some(canvas_coords) = canvas.viewport_to_canvas_coords(mouse_ev.client_x(), mouse_ev.client_y()) {
                        mouse_drag.curr = canvas_coords;
                    }
                }

                let (x_start, x_end) = if mouse_drag.curr.0 > mouse_drag.start.0 {
                    (mouse_drag.start.0, mouse_drag.curr.0)
                } else if mouse_drag.curr.0 < mouse_drag.start.0 {
                    (mouse_drag.curr.0, mouse_drag.start.0)
                } else {
                    model.mouse_drag = None;
                    return;
                };

                let (y_start, y_end) = if mouse_drag.curr.1 > mouse_drag.start.1 {
                    (mouse_drag.start.1, mouse_drag.curr.1)
                } else if mouse_drag.curr.1 < mouse_drag.start.1 {
                    (mouse_drag.curr.1, mouse_drag.start.1)
                } else {
                    model.mouse_drag = None;
                    return;
                };

                log!("setting new values");
                match model.active_config {
                    FractalType::JuliaSet => {
                        let x_scale = (model.config.julia_set_cfg.x_max.real() - model.config.julia_set_cfg.x_min.real()) / model.width as f64;
                        set_f64_on_input("max_real", x_end as f64 * x_scale + model.config.julia_set_cfg.x_max.real());
                        set_f64_on_input("min_real", x_start as f64 * x_scale + model.config.julia_set_cfg.x_min.real());
                        let x_scale = (model.config.julia_set_cfg.x_max.imag() - model.config.julia_set_cfg.x_min.imag()) / model.height as f64;
                        set_f64_on_input("max_imag", y_end as f64 * x_scale + model.config.julia_set_cfg.x_max.imag());
                        set_f64_on_input("min_imag", y_start as f64 * x_scale + model.config.julia_set_cfg.x_min.imag());
                    },
                    FractalType::Mandelbrot => {
                        /*
                        let x_scale = (model.config.mandelbrot_cfg.x_max.real() - model.config.mandelbrot_cfg.x_min.real()) / model.width as f64;
                        set_f64_on_input("max_real", x_end * x_scale + model.config.mandelbrot_cfg.x_max.real());
                        set_f64_on_input("min_real", x_start * x_scale + model.config.mandelbrot_cfg.x_min.real());
                        let x_scale = (model.config.julia_set_cfg.x_max.imag() - model.config.mandelbrot_cfg.x_min.imag()) / model.height as f64;
                        set_f64_on_input("max_imag", y_end * x_scale + model.config.mandelbrot_cfg.x_max.imag());
                        set_f64_on_input("min_imag", y_start * x_scale + model.config.mandelbrot_cfg.x_min.imag());
                        */
                    }
                }

                mouse_drag.image_data = None;
                model.mouse_drag = None;
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
        IF![model.active_config == FractalType::Mandelbrot => h1!["Mandelbrot Set"]],
        IF![model.active_config == FractalType::JuliaSet => h1!["Julia Set"]],
        view_buttons(model),
        view_julia_set_cfg_editor(),
        div![
            C!["canvas_cntr"],
            canvas![
                C!["canvas"],
                id!("canvas"),
                attrs!{
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
                ev(Ev::Click, |_| Msg::EditJulaSet),
                "Edit"
            ],
            label![
                C!["input_label"],
                attrs! { At::For => "type_select"}, "Select Type"],

            select![
                C!["type_select"],
                id!("type_select"),
                attrs!{At::Name => "type_select" },
                IF![model.active_config == FractalType::Mandelbrot => attrs!{At::Value => "type_mandelbrot"}],
                IF![model.active_config == FractalType::JuliaSet => attrs!{At::Value => "type_julia_set"}],
                option![
                    attrs!{At::Value => "type_mandelbrot" },
                    "Mandelbrot Set"
                ],
                option![
                    attrs!{At::Value => "type_julia_set" },
                    "Julia Set"
                ],
                ev(Ev::Change, |_| Msg::TypeChanged),
            ]
        ]
    ]
}

fn view_julia_set_cfg_editor() -> Node<Msg> {
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
                    attrs! { At::For => "c_imag"}, "C Imag."],
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
                    attrs! { At::For => "max_real"}, "Max. Real"],
                input![
                    C!["input"],
                    id!("max_real"),
                    attrs! {
                        At::Name => "max_real",
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
                    attrs! { At::For => "min_real"}, "Min. Real"],
                input![
                    C!["input"],
                    id!("min_real"),
                    attrs! {
                        At::Name => "min_real",
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
                    attrs! { At::For => "max_imag"}, "Max. Imag."],
                input![
                    C!["input"],
                    id!("max_imag"),
                    attrs! {
                        At::Name => "max_imag",
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
                    attrs! { At::For => "min_imag"}, "Min. Imag."],
                input![
                    C!["input"],
                    id!("min_imag"),
                    attrs! {
                        At::Name => "min_imag",
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
                ev(Ev::Click, |_| Msg::SaveEditJuliaSet),
                "Save"
            ],
            button![
                C!["button"],
                id!("cancel"),
                ev(Ev::Click, |_| Msg::CancelEditJuliaSet),
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
