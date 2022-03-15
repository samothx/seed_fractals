#![allow(clippy::wildcard_imports)]
#![allow(dead_code)]

use std::cmp::Ordering;

use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};

mod complex;
use complex::Complex;

mod fractal;
use fractal::Fractal;

mod mandelbrot;
use mandelbrot::Mandelbrot;

mod julia_set;
use julia_set::JuliaSet;

pub mod util;
use util::{get_f64_from_input, get_u32_from_input};

mod canvas;

mod views;
use views::view;

use crate::util::{set_f64_on_input, set_u32_on_input};
use canvas::Canvas;
use web_sys::{HtmlSelectElement, ImageData};

const JULIA_DEFAULT_X: (f64, f64) = (1.5, 1.0);
const JULIA_DEFAULT_C: (f64, f64) = (-0.8, 0.156);
const JULIA_DEFAULT_ITERATIONS: u32 = 400;

const MANDELBROT_DEFAULT_C_MAX: (f64, f64) = (0.47, 1.12);
const MANDELBROT_DEFAULT_C_MIN: (f64, f64) = (-2.00, -1.12);
const MANDELBROT_DEFAULT_ITERATIONS: u32 = 400;

const DEFAULT_WIDTH: u32 = 1024;
const DEFAULT_HEIGHT: u32 = 800;

const ENTER_KEY: &str = "Enter";
const BACKGROUND_COLOR: &str = "#000000";
const STORAGE_KEY: &str = "seed_fractals_v1";

const MAX_DURATION: f64 = 0.3;

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
        active_config: FractalType::JuliaSet,
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
    fractal: Option<Box<dyn Fractal>>,
    mouse_drag: Option<MouseDrag>,
    paused: bool,
    edit_mode: bool,
    active_config: FractalType,
}

#[derive(Serialize, Deserialize)]
struct Config {
    julia_set_cfg: JuliaSetCfg,
    mandelbrot_cfg: MandelbrotCfg,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            julia_set_cfg: JuliaSetCfg::default(),
            mandelbrot_cfg: MandelbrotCfg::default(),
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
        JuliaSetCfg {
            max_iterations: JULIA_DEFAULT_ITERATIONS,
            x_max: Complex::new(JULIA_DEFAULT_X.0, JULIA_DEFAULT_X.1),
            x_min: Complex::new(-JULIA_DEFAULT_X.0, -JULIA_DEFAULT_X.1),
            c: Complex::new(JULIA_DEFAULT_C.0, JULIA_DEFAULT_C.1),
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
        MandelbrotCfg {
            max_iterations: 400,
            c_max: Complex::new(MANDELBROT_DEFAULT_C_MAX.0, MANDELBROT_DEFAULT_C_MAX.1),
            c_min: Complex::new(MANDELBROT_DEFAULT_C_MIN.0, MANDELBROT_DEFAULT_C_MIN.1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum FractalType {
    Mandelbrot,
    JuliaSet,
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
pub enum Msg {
    Start,
    Pause,
    Clear,
    TypeChanged,
    Edit,
    SaveEdit,
    CancelEdit,
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

            match model.active_config {
                FractalType::JuliaSet => {
                    let mut fractal = JuliaSet::new(&model);
                    model
                        .canvas
                        .as_ref()
                        .expect("unexpected missing canvas")
                        .draw_results(fractal.calculate());
                    model.fractal = Some(Box::new(fractal));
                }
                FractalType::Mandelbrot => {
                    let mut fractal = Mandelbrot::new(&model);
                    model
                        .canvas
                        .as_ref()
                        .expect("unexpected missing canvas")
                        .draw_results(fractal.calculate());
                    model.fractal = Some(Box::new(fractal));
                }
            }
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
                model
                    .canvas
                    .as_ref()
                    .expect("unexpected empty canvas")
                    .clear_canvas(&model);
            }
        }
        Msg::TypeChanged => {
            log!("Message received: TypeChanged");
            let selected = window()
                .document()
                .expect("document not found in window")
                .get_element_by_id("type_select")
                .expect("type_select not found")
                .dyn_into::<HtmlSelectElement>()
                .expect("type_select is not a HtmlSelectElement")
                .value();

            model.active_config = match selected.as_str() {
                "type_mandelbrot" => FractalType::Mandelbrot,
                "type_julia_set" => FractalType::JuliaSet,
                _ => model.active_config,
            };
        }

        Msg::Edit => {
            log!("Message received: Edit");
            match model.active_config {
                FractalType::JuliaSet => {
                    set_u32_on_input(
                        "julia_iterations",
                        model.config.julia_set_cfg.max_iterations,
                    );
                    set_f64_on_input("julia_max_real", model.config.julia_set_cfg.x_max.real());
                    set_f64_on_input("julia_min_real", model.config.julia_set_cfg.x_min.real());
                    set_f64_on_input("julia_max_imag", model.config.julia_set_cfg.x_max.imag());
                    set_f64_on_input("julia_min_imag", model.config.julia_set_cfg.x_min.imag());
                    set_f64_on_input("julia_c_real", model.config.julia_set_cfg.c.real());
                    set_f64_on_input("julia_c_imag", model.config.julia_set_cfg.c.imag());

                    window()
                        .document()
                        .expect("document not found")
                        .get_element_by_id("julia_edit_cntr")
                        .expect("edit_cntr not found")
                        .set_class_name("edit_cntr_visible");
                }
                FractalType::Mandelbrot => {
                    set_u32_on_input(
                        "mandelbrot_iterations",
                        model.config.julia_set_cfg.max_iterations,
                    );
                    set_f64_on_input(
                        "mandelbrot_max_real",
                        model.config.mandelbrot_cfg.c_max.real(),
                    );
                    set_f64_on_input(
                        "mandelbrot_min_real",
                        model.config.mandelbrot_cfg.c_min.real(),
                    );
                    set_f64_on_input(
                        "mandelbrot_max_imag",
                        model.config.mandelbrot_cfg.c_max.imag(),
                    );
                    set_f64_on_input(
                        "mandelbrot_min_imag",
                        model.config.mandelbrot_cfg.c_min.imag(),
                    );

                    window()
                        .document()
                        .expect("document not found")
                        .get_element_by_id("mandelbrot_edit_cntr")
                        .expect("edit_cntr not found")
                        .set_class_name("edit_cntr_visible");
                }
            }
            model.edit_mode = true;
        }

        Msg::SaveEdit => {
            log!("Message received: SaveEdit");
            model.edit_mode = false;
            let document = window().document().expect("document not found");
            match model.active_config {
                FractalType::JuliaSet => {
                    if let Some(value) = get_u32_from_input("julia_iterations") {
                        model.config.julia_set_cfg.max_iterations = value;
                    }

                    if let Some(value) = get_f64_from_input("julia_max_real") {
                        model.config.julia_set_cfg.x_max.set_real(value);
                    }

                    if let Some(value) = get_f64_from_input("julia_min_real") {
                        model.config.julia_set_cfg.x_min.set_real(value);
                    }

                    if let Some(value) = get_f64_from_input("julia_max_imag") {
                        model.config.julia_set_cfg.x_max.set_imag(value);
                    }

                    if let Some(value) = get_f64_from_input("julia_min_imag") {
                        model.config.julia_set_cfg.x_min.set_imag(value);
                    }

                    if let Some(value) = get_f64_from_input("julia_c_real") {
                        model.config.julia_set_cfg.c.set_real(value);
                    }

                    if let Some(value) = get_f64_from_input("julia_c_imag") {
                        model.config.julia_set_cfg.c.set_imag(value);
                    }

                    document
                        .get_element_by_id("julia_edit_cntr")
                        .expect("edit_cntr not found")
                        .set_class_name("edit_cntr_hidden");
                }
                FractalType::Mandelbrot => {
                    if let Some(value) = get_u32_from_input("mandelbrot_iterations") {
                        model.config.mandelbrot_cfg.max_iterations = value;
                    }

                    if let Some(value) = get_f64_from_input("mandelbrot_max_real") {
                        model.config.mandelbrot_cfg.c_max.set_real(value);
                    }

                    if let Some(value) = get_f64_from_input("mandelbrot_min_real") {
                        model.config.mandelbrot_cfg.c_min.set_real(value);
                    }

                    if let Some(value) = get_f64_from_input("mandelbrot_max_imag") {
                        model.config.mandelbrot_cfg.c_max.set_imag(value);
                    }

                    if let Some(value) = get_f64_from_input("mandelbrot_min_imag") {
                        model.config.mandelbrot_cfg.c_min.set_imag(value);
                    }

                    document
                        .get_element_by_id("mandelbrot_edit_cntr")
                        .expect("edit_cntr not found")
                        .set_class_name("edit_cntr_hidden");
                }
            }
            LocalStorage::insert(STORAGE_KEY, &model.config).expect("save data to LocalStorage");

            // TODO: save to local storage
            orders.after_next_render(|_| Msg::Clear);
        }
        Msg::CancelEdit => {
            log!("Message received: SaveEdit");
            model.edit_mode = false;
            match model.active_config {
                FractalType::JuliaSet => {
                    window()
                        .document()
                        .expect("document not found")
                        .get_element_by_id("julia_edit_cntr")
                        .expect("edit_cntr not found")
                        .set_class_name("edit_cntr_hidden");
                }
                FractalType::Mandelbrot => {
                    window()
                        .document()
                        .expect("document not found")
                        .get_element_by_id("mandelbrot_edit_cntr")
                        .expect("edit_cntr not found")
                        .set_class_name("edit_cntr_hidden");
                }
            }
        }
        Msg::Draw => {
            // log!("Message received: Draw");
            if !model.paused {
                let fractal = model.fractal.as_mut().expect("unexpected missing fractal");
                model
                    .canvas
                    .as_ref()
                    .expect("unexpected missing canvas")
                    .draw_results(fractal.calculate());
                if !fractal.is_done() {
                    orders.after_next_render(|_| Msg::Draw);
                } else {
                    model.paused = true;
                }
            }
        }
        Msg::MouseDown(ev) => {
            log!("Message received: MouseDown");
            if let Some(canvas_coords) = model
                .canvas
                .as_ref()
                .expect("unexpected missing canvas")
                .viewport_to_canvas_coords(ev.client_x(), ev.client_y())
            {
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

                if let Some(canvas_coords) =
                    canvas.viewport_to_canvas_coords(ev.client_x(), ev.client_y())
                {
                    mouse_drag.curr = canvas_coords;
                    mouse_drag.image_data = Some(canvas.draw_frame(
                        mouse_drag.start.0,
                        mouse_drag.start.1,
                        mouse_drag.curr.0,
                        mouse_drag.curr.1,
                    ));
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
                    if let Some(canvas_coords) =
                        canvas.viewport_to_canvas_coords(mouse_ev.client_x(), mouse_ev.client_y())
                    {
                        mouse_drag.curr = canvas_coords;
                    }
                }

                let (x_start, x_end) = match mouse_drag.curr.0.cmp(&mouse_drag.start.0) {
                    Ordering::Greater => (mouse_drag.start.0, mouse_drag.curr.0),
                    Ordering::Less => (mouse_drag.curr.0, mouse_drag.start.0),
                    Ordering::Equal => {
                        model.mouse_drag = None;
                        return;
                    }
                };

                let (y_start, y_end) = match mouse_drag.curr.1.cmp(&mouse_drag.start.1) {
                    Ordering::Greater => (mouse_drag.start.1, mouse_drag.curr.1),
                    Ordering::Less => (mouse_drag.curr.1, mouse_drag.start.1),
                    Ordering::Equal => {
                        model.mouse_drag = None;
                        return;
                    }
                };

                log!(format!(
                    "setting new values, canvas coordinates: ({},{}), ({},{})",
                    x_start, y_start, x_end, y_end
                ));
                match model.active_config {
                    FractalType::JuliaSet => {
                        let x_scale = (model.config.julia_set_cfg.x_max.real()
                            - model.config.julia_set_cfg.x_min.real())
                            / f64::from(model.width);
                        set_f64_on_input(
                            "julia_max_real",
                            f64::from(x_end)
                                .mul_add(x_scale, model.config.julia_set_cfg.x_min.real()),
                        );
                        set_f64_on_input(
                            "julia_min_real",
                            f64::from(x_start)
                                .mul_add(x_scale, model.config.julia_set_cfg.x_min.real()),
                        );
                        let x_scale = (model.config.julia_set_cfg.x_max.imag()
                            - model.config.julia_set_cfg.x_min.imag())
                            / f64::from(model.height);
                        set_f64_on_input(
                            "julia_max_imag",
                            f64::from(y_end)
                                .mul_add(x_scale, model.config.julia_set_cfg.x_min.imag()),
                        );
                        set_f64_on_input(
                            "julia_min_imag",
                            f64::from(y_start)
                                .mul_add(x_scale, model.config.julia_set_cfg.x_min.imag()),
                        );
                    }
                    FractalType::Mandelbrot => {
                        let x_scale = (model.config.mandelbrot_cfg.c_max.real()
                            - model.config.mandelbrot_cfg.c_min.real())
                            / f64::from(model.width);
                        set_f64_on_input(
                            "mandelbrot_max_real",
                            f64::from(x_end)
                                .mul_add(x_scale, model.config.mandelbrot_cfg.c_min.real()),
                        );
                        set_f64_on_input(
                            "mandelbrot_min_real",
                            f64::from(x_start)
                                .mul_add(x_scale, model.config.mandelbrot_cfg.c_min.real()),
                        );
                        let x_scale = (model.config.mandelbrot_cfg.c_max.imag()
                            - model.config.mandelbrot_cfg.c_min.imag())
                            / f64::from(model.height);
                        set_f64_on_input(
                            "mandelbrot_max_imag",
                            f64::from(y_end)
                                .mul_add(x_scale, model.config.mandelbrot_cfg.c_min.imag()),
                        );
                        set_f64_on_input(
                            "mandelbrot_min_imag",
                            f64::from(y_start)
                                .mul_add(x_scale, model.config.mandelbrot_cfg.c_min.imag()),
                        );
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

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
