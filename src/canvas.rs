use seed::canvas;
use crate::{Model, BACKGROUND_COLOR};
use seed::log;

use crate::fractal::Points;
use seed::prelude::web_sys::HtmlCanvasElement;
use seed::prelude::JsValue;

const COLOR_MAX: u32 = 0xFFFFFF;
const COLOR_MIN: u32 = 0xFFFFFF;

const START_HUE: u32 = 0;
const DEFAULT_SATURATION: f32 = 1.0;
const DEFAULT_LIGHTNESS: f32 = 0.5;


pub struct Canvas {
    canvas: HtmlCanvasElement,
    steps: u32,
    width: u32,
}


impl Canvas {
    pub fn new(model: &Model) -> Canvas {
        Canvas {
            canvas: canvas("canvas").unwrap(),
            steps: model.max_iterations,
            width: model.width,
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
        //  ctx.fill_rect(0.into(), 0.into(), model.width.into(), model.height.into());
        ctx.fill();
    }

    pub fn draw_results(&self, points: Points) {
        let mut x = points.x_start;
        let mut y = points.y_start;
        let ctx = seed::canvas_context_2d(&self.canvas);
        ctx.set_fill_style(&JsValue::from_str("FFFFFF"));
        ctx.fill();

        let mut last_color = "".to_string();
        points.values.iter().for_each(|value| {
            let color = if *value >= self.steps - 1 {
                BACKGROUND_COLOR.to_string()
            } else {
                Self::hue_to_rgb(f32::floor(*value as f32 * (360.0 / self.steps as f32)) as u32)
            };
            if color != last_color {
                log!(format!("draw_result: color: {} pos: {},{}", color, x, y));
                ctx.set_fill_style(&JsValue::from_str(color.as_str()));
                last_color = color;
            }
            ctx.fill_rect(x.into(), y.into(), 100.0, 100.0);

            x += 1;
            if x >= self.width {
                x = 0;
                y += 1;
            }
        });
    }

    fn hue_to_rgb(hue: u32) -> String {
        let safe_hue = if hue >= 360 {
            (hue % 360) as i32
        } else {
            hue as i32
        };

        const TMP: f32 = 2.0 * DEFAULT_LIGHTNESS - 1.0;
        const C: f32 = (1.0 - if TMP >= 0.0 { TMP } else { -TMP }) * DEFAULT_SATURATION;
        const M: f32 = DEFAULT_LIGHTNESS - C / 2.0;
        let x = C * (1.0 - i32::abs((safe_hue / 60) % 2 - 1) as f32);

        let (r, g, b) = match hue {
            0..=59 => (C, x, 0.0),
            60..=159 => (x, C, 0.0),
            120..=179 => (0.0, C, x),
            180..=239 => (0.0, x, C),
            240..=299 => (x, 0.0, C),
            300..=359 => (C, 0.0, x),
            _ => { panic!("invalid hue value"); }
        };

        let (r, g, b) = (
            f32::floor((r + M) * 255.0) as u32,
            f32::floor((g + M) * 255.0) as u32,
            f32::floor((b + M) * 255.0) as u32);

        format!("{:0>2X}{:0>2X}{:0>2X}", r % 0x100, g % 0x100, b % 0x100)
    }

    fn hsl_to_rgb(hue: u32, saturation: f32, lightness: f32) -> String {
        // see: https://www.rapidtables.com/convert/color/hsl-to-rgb.html

        assert!(saturation >= 0.0 && saturation <= 1.0);
        assert!(lightness >= 0.0 && lightness <= 1.0);

        let safe_hue = if hue >= 360 {
            (hue % 360) as i32
        } else {
            hue as i32
        };

        let c = (1.0 - f32::abs(2.0 * lightness - 1.0)) * saturation;
        let x = c * (1.0 - i32::abs((safe_hue / 60) % 2 - 1) as f32);
        let m = lightness - c / 2.0;
        let (r, g, b) = match hue {
            0..=59 => (c, x, 0.0),
            60..=159 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            300..=359 => (c, 0.0, x),
            _ => { panic!("invalid hue value"); }
        };

        let (r, g, b) = (
            f32::floor((r + m) * 255.0) as u32,
            f32::floor((g + m) * 255.0) as u32,
            f32::floor((b + m) * 255.0) as u32);

        format!("{:X}{:X}{:X}", r % 0x100, g % 0x100, b % 0x100)
    }
}
