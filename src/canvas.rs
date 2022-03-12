use seed::canvas;
use crate::Model;
use seed::{prelude::*, *};
use web_sys::HtmlCanvasElement;

pub struct Canvas {
    canvas: HtmlCanvasElement
}

impl Canvas {
    pub fn new() -> Canvas {
        Canvas {
            canvas: canvas("canvas").unwrap()
        }
    }

    pub fn clear_canvas(&self, model: &Model) {
        log!("Clear Canvas");
        if model.height != self.canvas.height() {
            self.canvas.set_height(model.height);
        }
        if model.width != self.canvas.width() {
            self.canvas.set_width(model.width);
        }

        let ctx = seed::canvas_context_2d(&self.canvas);
        ctx.set_fill_style(&JsValue::from_str(model.background_color.as_str()));
        ctx.fill_rect(0.into(), 0.into(), model.width.into(), model.height.into());
    }
}
