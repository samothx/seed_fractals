use std::cmp::Ordering;

#[allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};
use seed::prelude::web_sys::HtmlSelectElement;
use super::{
    canvas::Canvas,
    fractal::Fractal,
    julia_set::JuliaSet,
    mandelbrot::Mandelbrot,
    util::{get_f64_from_input, get_u32_from_input, set_f64_on_input, set_u32_on_input},
    FractalType, Model, MouseDrag, Msg, STORAGE_KEY,
};

pub fn on_msg_start(model: &mut Model, orders: &mut impl Orders<Msg>) {
    if model.canvas.is_none() {
        let canvas = Canvas::new(model);
        canvas.clear_canvas(model);
        model.canvas = Some(canvas);
    }
    log!("Message received: Start, creating fractal");

    match model.active_config {
        FractalType::JuliaSet => {
            let mut fractal = JuliaSet::new(model);
            model
                .canvas
                .as_ref()
                .expect("unexpected missing canvas")
                .draw_results(fractal.calculate());
            model.fractal = Some(Box::new(fractal));
        }
        FractalType::Mandelbrot => {
            let mut fractal = Mandelbrot::new(model);
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

pub fn on_msg_clear(model: &mut Model) {
    if !model.paused {
        model.paused = true;
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

    model.active_config = match selected.as_str() {
        "type_mandelbrot" => FractalType::Mandelbrot,
        "type_julia_set" => FractalType::JuliaSet,
        _ => model.active_config,
    };

}

pub fn on_msg_draw(model: &mut Model, orders: &mut impl Orders<Msg>) {
    if !model.paused {
        let fractal = model.fractal.as_mut().expect("unexpected missing fractal");
        model
            .canvas
            .as_ref()
            .expect("unexpected missing canvas")
            .draw_results(fractal.calculate());
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

pub fn on_msg_cancel_edit(model: &mut Model) {
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

pub fn on_msg_edit(model: &mut Model) {
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
        match model.active_config {
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
