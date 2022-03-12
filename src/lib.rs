#![allow(clippy::wildcard_imports)]
#![allow(dead_code)]

use seed::{prelude::*, *};

mod complex;

mod fractal;
use fractal::Fractal;

mod canvas;
use canvas::Canvas;


const DEFAULT_XY: f64 = 1.5;
const DEFAULT_C: (f64, f64) = (-0.4, 0.6);
const DEFAULT_WIDTH: u32 = 1024;
const DEFAULT_HEIGHT: u32 = 600;
const DEFAULT_ITERATIONS: u32 = 400;
const ENTER_KEY: &str = "Enter";
const BACKGROUND_COLOR: &str = "#000000";
const MAX_DURATION: f64 = 0.1;

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(|_| Msg::Clear);
    Model {
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        max_iterations: DEFAULT_ITERATIONS,
        c_real: DEFAULT_C.0,
        c_imag: DEFAULT_C.1,
        x_max: DEFAULT_XY,
        x_min: -DEFAULT_XY,
        y_max: DEFAULT_XY,
        y_min: -DEFAULT_XY,
        background_color: BACKGROUND_COLOR.to_string(),
        canvas: None,
        fractal: None,
        paused: true,
        max_duration: MAX_DURATION
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    width: u32,
    height: u32,
    max_iterations: u32,
    x_max: f64,
    x_min: f64,
    y_max: f64,
    y_min: f64,
    c_real: f64,
    c_imag: f64,
    background_color: String,
    canvas: Option<Canvas>,
    fractal: Option<Fractal>,
    paused: bool,
    max_duration: f64
}

// ------ ------
//    Update
// ------ ------


#[derive(Clone)]
enum Msg {
    // HeightChanged(usize),
    // WidthChanged(usize),
    MaxXChanged(String),
    MinXChanged(String),
    MaxYChanged(String),
    MinYChanged(String),
    CRealChanged(String),
    CImagChanged(String),
    IterationsChanged(String),
    Start,
    Pause,
    Clear,
    Draw,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        // Msg::HeightChanged(height) => { log!("Message received: HeightChanged to {}",height); },
        // Msg::WidthChanged(width)  => { log!("Message received: WidthChanged to {}",width); },
        Msg::MaxXChanged(x_max) => {
            log!("Message received: MaxXChanged to {}",x_max);
            let x_max = x_max.parse::<f64>().unwrap_or(model.x_max);
            if x_max != model.x_max && x_max >= model.x_min {
                model.x_max = x_max;
                orders.after_next_render(|_| Msg::Clear);
            }
        }
        Msg::MinXChanged(x_min) => {
            log!("Message received: MinXChanged to {}",x_min);
            let x_min = x_min.parse::<f64>().unwrap_or(model.x_min);
            if x_min != model.x_min && x_min <= model.x_max {
                model.x_min = x_min;
                orders.after_next_render(|_| Msg::Clear);
            }
        }
        Msg::MaxYChanged(y_max) => {
            log!("Message received: MaxYChanged to {}",y_max);
            let y_max = y_max.parse::<f64>().unwrap_or(model.y_max);
            if y_max != model.y_max && y_max >= model.y_min {
                model.y_max = y_max;
                orders.after_next_render(|_| Msg::Clear);
            }
        }
        Msg::MinYChanged(y_min) => {
            log!("Message received: MinYChanged to {}",y_min);
            let y_min = y_min.parse::<f64>().unwrap_or(model.y_min);
            if y_min != model.y_min && y_min <= model.y_max {
                model.y_min = y_min;
                orders.after_next_render(|_| Msg::Clear);
            }
        }
        Msg::CRealChanged(c_real) => {
            log!("Message received: CRealChanged to {}",c_real);
            model.c_real = c_real.parse::<f64>().unwrap_or(model.c_real);
            orders.after_next_render(|_| Msg::Clear);
        }
        Msg::CImagChanged(c_imag) => {
            log!("Message received: CImagChanged to {}",c_imag);
            model.c_imag = c_imag.parse::<f64>().unwrap_or(model.c_imag);
            orders.after_next_render(|_| Msg::Clear);
        }
        Msg::IterationsChanged(iterations) => {
            log!("Message received: IterationsChanged to {}",iterations);
            model.max_iterations = iterations.parse::<u32>().unwrap_or(model.max_iterations);
            orders.after_next_render(|_| Msg::Clear);
        }
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
            }
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
        view_inputs(model),
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

fn view_inputs(model: &Model) -> Vec<Node<Msg>> {
    vec![
        div![
            C!["input_cntr"],
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
                        At::Value => {model.max_iterations.to_string()},
                    },
                    input_ev(Ev::Input, Msg::IterationsChanged),
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
                        At::Value => {model.c_real.to_string()},
                    },
                    input_ev(Ev::Input, Msg::CRealChanged),
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
                        At::Value => {model.c_imag.to_string()},
                    },
                    input_ev(Ev::Input, Msg::CImagChanged),
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
                        At::Value => {model.x_max.to_string()},
                    },
                    input_ev(Ev::Input, Msg::MaxXChanged),
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
                        At::Value => {model.x_min.to_string()},
                    },
                    input_ev(Ev::Input, Msg::MinXChanged),
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
                        At::Value => {model.y_max.to_string()},
                    },
                    input_ev(Ev::Input, Msg::MaxYChanged),
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
                        At::Value => {model.y_min.to_string()},
                    },
                    input_ev(Ev::Input, Msg::MinYChanged),
                ]
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
