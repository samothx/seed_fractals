use std::cmp::Ordering;

use super::{
    canvas::Canvas,
    complex::Complex,
    fractal::Fractal,
    julia_set::JuliaSet,
    mandelbrot::Mandelbrot,
    stats::Stats,
    util::{get_f64_from_input, get_u32_from_input, set_f64_on_input, set_u32_on_input},
    FractalType, Model, MouseDrag, Msg, JULIA_DEFAULT_C, JULIA_DEFAULT_ITERATIONS, JULIA_DEFAULT_X,
    MANDELBROT_DEFAULT_C_MAX, MANDELBROT_DEFAULT_C_MIN, MANDELBROT_DEFAULT_ITERATIONS, STORAGE_KEY,
};
use seed::prelude::web_sys::{HtmlInputElement, HtmlSelectElement};
#[allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};

pub fn on_msg_start(model: &mut Model, orders: &mut impl Orders<Msg>) {
    if let Some(canvas) = model.canvas.as_ref() {
        canvas.clear_canvas(model);
    } else {
        let canvas = Canvas::new(model);
        canvas.clear_canvas(model);
        model.canvas = Some(canvas);
    }

    if model.config.view_stats {
        model.stats = Some(Stats::new());
        model.stats_text = String::new();
    }
    match model.config.active_config {
        FractalType::JuliaSet => {
            let mut fractal = JuliaSet::new(model);
            model
                .canvas
                .as_ref()
                .expect("unexpected missing canvas")
                .draw_results(fractal.calculate(model.stats.as_mut()));

            if let Some(stats) = model.stats.as_ref() {
                model.stats_text = stats.format_stats();
            }

            model.fractal = Some(Box::new(fractal));
        }
        FractalType::Mandelbrot => {
            let mut fractal = Mandelbrot::new(model);
            model
                .canvas
                .as_ref()
                .expect("unexpected missing canvas")
                .draw_results(fractal.calculate(model.stats.as_mut()));
            if let Some(stats) = model.stats.as_ref() {
                model.stats_text = stats.format_stats();
            }
            model.fractal = Some(Box::new(fractal));
        }
    }
    model.paused = false;

    orders.after_next_render(|_| Msg::Draw);
}

pub fn on_msg_clear(model: &mut Model) {
    if !model.paused {
        model.paused = true;
    }

    if model.config.view_stats {
        model.stats = Some(Stats::new());
        model.stats_text = String::new();
    }

    model.fractal = None;
    if model.canvas.is_none() {
        let canvas = Canvas::new(model);
        canvas.clear_canvas(model);
        model.canvas = Some(canvas);
    } else {
        model
            .canvas
            .as_ref()
            .expect("unexpected empty canvas")
            .clear_canvas(model);
    }
}

pub fn on_msg_type_changed(model: &mut Model) {
    let selected = window()
        .document()
        .expect("document not found in window")
        .get_element_by_id("type_select")
        .expect("type_select not found")
        .dyn_into::<HtmlSelectElement>()
        .expect("type_select is not a HtmlSelectElement")
        .value();

    model.config.active_config = match selected.as_str() {
        "type_mandelbrot" => FractalType::Mandelbrot,
        "type_julia_set" => FractalType::JuliaSet,
        _ => model.config.active_config,
    };
}

pub fn on_msg_stats_changed(model: &mut Model) {
    let stats_cb = window()
        .document()
        .expect("document not found")
        .get_element_by_id("stats_cb")
        .expect("stats checkbox not found")
        .dyn_into::<HtmlInputElement>()
        .expect("Failed to cast to HtmlInputElement");
    model.config.view_stats = stats_cb.checked();
    LocalStorage::insert(STORAGE_KEY, &model.config).expect("save data to LocalStorage");
    if model.config.view_stats {
        model.stats = Some(Stats::new());
        model.stats_text = String::new();
    } else {
        model.stats = None;
        model.stats_text = String::new();
    }
}

pub fn on_msg_draw(model: &mut Model, orders: &mut impl Orders<Msg>) {
    if !model.paused {
        let fractal = model.fractal.as_mut().expect("unexpected missing fractal");
        model
            .canvas
            .as_ref()
            .expect("unexpected missing canvas")
            .draw_results(fractal.calculate(model.stats.as_mut()));
        if let Some(stats) = model.stats.as_ref() {
            model.stats_text = stats.format_stats();
        }

        if fractal.is_done() {
            model.paused = true;
        } else {
            orders.after_next_render(|_| Msg::Draw);
        }
    }
}

pub fn on_msg_save_edit(model: &mut Model, orders: &mut impl Orders<Msg>) {
    model.edit_mode = false;
    let document = window().document().expect("document not found");
    match model.config.active_config {
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

    adjust_height_to_ratio(model);

    // TODO: save to local storage
    orders.after_next_render(|_| Msg::Clear);
}

pub fn on_msg_cancel_edit(model: &mut Model) {
    model.edit_mode = false;
    match model.config.active_config {
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

pub fn on_msg_edit(model: &mut Model) {
    model.edit_mode = true;
    set_editor_fields_params(model);
    set_editor_fields_area(model);
}

pub fn on_msg_reset_params(model: &mut Model) {
    match model.config.active_config {
        FractalType::JuliaSet => {
            model.config.julia_set_cfg.max_iterations = JULIA_DEFAULT_ITERATIONS;
            model.config.julia_set_cfg.c = Complex::new(JULIA_DEFAULT_C.0, JULIA_DEFAULT_C.1);
        }
        FractalType::Mandelbrot => {
            model.config.mandelbrot_cfg.max_iterations = MANDELBROT_DEFAULT_ITERATIONS;
        }
    }
    set_editor_fields_params(model);
}

pub fn on_msg_reset_area(model: &mut Model) {
    match model.config.active_config {
        FractalType::JuliaSet => {
            model.config.julia_set_cfg.x_max = Complex::new(JULIA_DEFAULT_X.0, JULIA_DEFAULT_X.1);
            model.config.julia_set_cfg.x_min = Complex::new(-JULIA_DEFAULT_X.0, -JULIA_DEFAULT_X.1);
        }
        FractalType::Mandelbrot => {
            model.config.mandelbrot_cfg.c_max =
                Complex::new(MANDELBROT_DEFAULT_C_MAX.0, MANDELBROT_DEFAULT_C_MAX.1);
            model.config.mandelbrot_cfg.c_min =
                Complex::new(MANDELBROT_DEFAULT_C_MIN.0, MANDELBROT_DEFAULT_C_MIN.1);
        }
    }
    set_editor_fields_area(model);
}

pub fn on_msg_zoom_out_area(model: &mut Model) {
    match model.config.active_config {
        FractalType::JuliaSet => {
            let increment = Complex::new(
                (model.config.julia_set_cfg.x_max.real() - model.config.julia_set_cfg.x_min.real()).abs() / 2.0,
                (model.config.julia_set_cfg.x_max.imag() - model.config.julia_set_cfg.x_min.imag()).abs() / 2.0
            );

            model.config.julia_set_cfg.x_max += increment;
            model.config.julia_set_cfg.x_min -= increment;
        }
        FractalType::Mandelbrot => {
            let increment = Complex::new(
                (model.config.mandelbrot_cfg.c_max.real() - model.config.mandelbrot_cfg.c_min.real()).abs() / 2.0,
                (model.config.mandelbrot_cfg.c_max.imag() - model.config.mandelbrot_cfg.c_min.imag()).abs() / 2.0
            );

            model.config.mandelbrot_cfg.c_max += increment;
            model.config.mandelbrot_cfg.c_min -= increment;
        }
    }

    set_editor_fields_area(model);
}

pub fn on_msg_mouse_down(model: &mut Model, ev: &web_sys::MouseEvent) {
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

pub fn on_msg_mouse_move(
    model: &mut Model,
    ev: &web_sys::MouseEvent,
    orders: &mut impl Orders<Msg>,
) {
    if let Some(mouse_drag) = model.mouse_drag.as_mut() {
        let canvas = model.canvas.as_ref().expect("unexpected missing canvas");

        if let Some(image_data) = mouse_drag.image_data.as_ref() {
            canvas.undraw(image_data);
        }

        if let Some(canvas_coords) = canvas.viewport_to_canvas_coords(ev.client_x(), ev.client_y())
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
pub fn on_msg_mouse_up(model: &mut Model, ev: Option<web_sys::MouseEvent>) {
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
        match model.config.active_config {
            FractalType::JuliaSet => {
                let x_scale = (model.config.julia_set_cfg.x_max.real()
                    - model.config.julia_set_cfg.x_min.real())
                    / f64::from(model.width);
                set_f64_on_input(
                    "julia_max_real",
                    f64::from(x_end).mul_add(x_scale, model.config.julia_set_cfg.x_min.real()),
                );
                set_f64_on_input(
                    "julia_min_real",
                    f64::from(x_start).mul_add(x_scale, model.config.julia_set_cfg.x_min.real()),
                );
                let x_scale = (model.config.julia_set_cfg.x_max.imag()
                    - model.config.julia_set_cfg.x_min.imag())
                    / f64::from(model.height);
                set_f64_on_input(
                    "julia_max_imag",
                    f64::from(y_end).mul_add(x_scale, model.config.julia_set_cfg.x_min.imag()),
                );
                set_f64_on_input(
                    "julia_min_imag",
                    f64::from(y_start).mul_add(x_scale, model.config.julia_set_cfg.x_min.imag()),
                );
            }
            FractalType::Mandelbrot => {
                let x_scale = (model.config.mandelbrot_cfg.c_max.real()
                    - model.config.mandelbrot_cfg.c_min.real())
                    / f64::from(model.width);
                set_f64_on_input(
                    "mandelbrot_max_real",
                    f64::from(x_end).mul_add(x_scale, model.config.mandelbrot_cfg.c_min.real()),
                );
                set_f64_on_input(
                    "mandelbrot_min_real",
                    f64::from(x_start).mul_add(x_scale, model.config.mandelbrot_cfg.c_min.real()),
                );
                let x_scale = (model.config.mandelbrot_cfg.c_max.imag()
                    - model.config.mandelbrot_cfg.c_min.imag())
                    / f64::from(model.height);
                set_f64_on_input(
                    "mandelbrot_max_imag",
                    f64::from(y_end).mul_add(x_scale, model.config.mandelbrot_cfg.c_min.imag()),
                );
                set_f64_on_input(
                    "mandelbrot_min_imag",
                    f64::from(y_start).mul_add(x_scale, model.config.mandelbrot_cfg.c_min.imag()),
                );
            }
        }

        mouse_drag.image_data = None;
        model.mouse_drag = None;
    }
}

fn adjust_height_to_ratio(model: &mut Model) {
    let dim = match model.config.active_config {
        FractalType::JuliaSet => {
            model.config.julia_set_cfg.x_max - model.config.julia_set_cfg.x_min
        }
        FractalType::Mandelbrot => {
            model.config.mandelbrot_cfg.c_max - model.config.mandelbrot_cfg.c_min
        }
    };
    model.height = (f64::from(model.width) * dim.imag() / dim.real()) as u32;
}

fn set_editor_fields_params(model: &Model) {
    match model.config.active_config {
        FractalType::JuliaSet => {
            set_u32_on_input(
                "julia_iterations",
                model.config.julia_set_cfg.max_iterations,
            );
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
                model.config.mandelbrot_cfg.max_iterations,
            );
            window()
                .document()
                .expect("document not found")
                .get_element_by_id("mandelbrot_edit_cntr")
                .expect("edit_cntr not found")
                .set_class_name("edit_cntr_visible");
        }
    }
}

fn set_editor_fields_area(model: &Model) {
    match model.config.active_config {
        FractalType::JuliaSet => {
            set_f64_on_input("julia_max_real", model.config.julia_set_cfg.x_max.real());
            set_f64_on_input("julia_min_real", model.config.julia_set_cfg.x_min.real());
            set_f64_on_input("julia_max_imag", model.config.julia_set_cfg.x_max.imag());
            set_f64_on_input("julia_min_imag", model.config.julia_set_cfg.x_min.imag());

            window()
                .document()
                .expect("document not found")
                .get_element_by_id("julia_edit_cntr")
                .expect("edit_cntr not found")
                .set_class_name("edit_cntr_visible");
        }
        FractalType::Mandelbrot => {
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
}
