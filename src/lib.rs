#![allow(dead_code)]

// use std::cmp::Ordering;

#[allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};

use serde::{Deserialize, Serialize};

mod complex;
use complex::Complex;

mod fractal;
use fractal::Fractal;

mod julia_set;
mod mandelbrot;

pub mod util;
// use util::{get_f64_from_input, get_u32_from_input};

mod canvas;

mod views;
use views::view;

mod event_handler;
use event_handler::{
    on_msg_cancel_edit, on_msg_draw, on_msg_edit, on_msg_mouse_down, on_msg_mouse_move,
    on_msg_mouse_up, on_msg_save_edit, on_msg_start, on_msg_clear, on_msg_type_changed,
    on_msg_reset_area, on_msg_reset_params, on_msg_zoom_out_area
};

use canvas::Canvas;
use web_sys::{ImageData};

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
    edit_mode: bool
}

#[derive(Serialize, Deserialize)]
struct Config {
    active_config: FractalType,
    julia_set_cfg: JuliaSetCfg,
    mandelbrot_cfg: MandelbrotCfg,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            active_config: FractalType::Mandelbrot,
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
        Self {
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
        Self {
            max_iterations: 400,
            c_max: Complex::new(MANDELBROT_DEFAULT_C_MAX.0, MANDELBROT_DEFAULT_C_MAX.1),
            c_min: Complex::new(MANDELBROT_DEFAULT_C_MIN.0, MANDELBROT_DEFAULT_C_MIN.1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
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
    Stop,
    Clear,
    TypeChanged,
    Edit,
    SaveEdit,
    CancelEdit,
    Draw,
    ResetParams,
    ResetArea,
    ZoomOutArea,
    MouseDown(web_sys::MouseEvent),
    MouseMove(web_sys::MouseEvent),
    MouseUp(Option<web_sys::MouseEvent>),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Start => {
            log!("Message received: Start");
            on_msg_start(model, orders);
        }
        Msg::Stop => {
            log!("Message received: Stop");
            model.paused = true;
        }
        Msg::Clear => {
            log!("Message received: Clear");
            on_msg_clear(model);            
        }
        Msg::TypeChanged => {
            log!("Message received: TypeChanged");
            on_msg_type_changed(model);
        }

        Msg::Edit => {
            log!("Message received: Edit");
            on_msg_edit(model);
        }

        Msg::SaveEdit => {
            log!("Message received: SaveEdit");
            on_msg_save_edit(model, orders);
        }
        Msg::CancelEdit => {
            log!("Message received: SaveEdit");
            on_msg_cancel_edit(model);
        }
        Msg::ResetParams => {
            log!("Message received: ResetParams");
            on_msg_reset_params(model);
        },
        Msg::ResetArea => {
            log!("Message received: ResetArea");
            on_msg_reset_area(model);
        },
        Msg::ZoomOutArea => {
            log!("Message received: ZoomOutArea");
            on_msg_zoom_out_area(model);
        },
    
        Msg::Draw => {
            // log!("Message received: Draw");
            on_msg_draw(model, orders);
        }
        Msg::MouseDown(ev) => {
            log!("Message received: MouseDown");
            on_msg_mouse_down(model, &ev);
        }
        Msg::MouseMove(ev) => {
            log!("Message received: MouseMove");
            on_msg_mouse_move(model, &ev, orders);
        }
        Msg::MouseUp(ev) => {
            log!("Message received: MouseUp");
            on_msg_mouse_up(model, ev);
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
#[allow(clippy::unused_unit)]
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
