use super::{Model, complex::Complex};
use seed::{log};
// use wasm_bindgen::prelude::web_sys;
use seed::prelude::web_sys;

const MAX_POINTS: usize = 1000;
const MAX_DURATION: f64 = 0.3;

pub struct Points {
    pub x_start: u32,
    pub y_start: u32,
    pub num_points: usize,
    pub values: [u32;MAX_POINTS],
}


pub struct JuliaSet {
    scale_real: f64,
    scale_imag: f64,
    offset: Complex,
    c: Complex,
    max: f64,
    x_curr: u32,
    width: u32,
    height: u32,
    y_curr: u32,
    iterations: u32,
    res: Points,
    done: bool
}


impl JuliaSet {
    pub fn new(model: &Model) -> JuliaSet {

        log!(format!("creating fractal with: x_max: {}, x_min: {}, c: {}",
            model.config.julia_set_cfg.x_max, model.config.julia_set_cfg.x_min , model.config.julia_set_cfg.c));

        let scale_real = (model.config.julia_set_cfg.x_max.real() - model.config.julia_set_cfg.x_min.real()) / model.width as f64;
        let scale_imag = (model.config.julia_set_cfg.x_max.imag() - model.config.julia_set_cfg.x_min.imag()) / model.height as f64;

        let c_norm = model.config.julia_set_cfg.c.norm();
        let mut max = c_norm;
        loop {
            let r_val = max * max - max;
            if r_val < c_norm {
                max += max / 2.0;
            } else {
                break;
            }
        }
        log!(format!("max: {}", max));

        JuliaSet {
            scale_real,
            scale_imag,
            offset: model.config.julia_set_cfg.x_min,
            c: model.config.julia_set_cfg.c,
            max,
            x_curr: 0,
            width: model.width,
            y_curr: 0,
            height: model.height,
            iterations: model.config.julia_set_cfg.max_iterations,
            res: Points{
                x_start: 0,
                y_start: 0,
                num_points: 0,
                values: [0;MAX_POINTS]
            },
            done: false
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    fn iterate(&self, x: &Complex) -> u32 {
        let mut curr = *x;
        if curr.square_length() >= self.max {
            0
        } else {
            // log!(format!("iterate: start: {}", curr));
            let mut last: Option<u32> = None;
            for idx in 1..=self.iterations {
                curr = curr * curr + self.c;
                if curr.square_length() >= self.max {
                    last = Some(idx);
                    break;
                }
            }

            // log!(format!("iterate: end:  {} norm: {} last: {:?}", curr, curr.square_length(), last));
            if let Some(last) = last {
                last
            } else {
                0
            }
        }
    }

    pub fn calculate<'a>(&'a mut self) -> &'a Points {
        let performance = web_sys::window().expect("Window not found")
            .performance()
            .expect("performance should be available");

        let start = performance.now();

        self.res.x_start = self.x_curr;
        self.res.y_start = self.y_curr;
        self.res.num_points = 0;

        let mut x = self.x_curr;
        let mut y = self.y_curr;
        let mut points_done : Option<usize> = None;
        for count in 0..self.res.values.len() {
            let calc = Complex::new(   x as f64 * self.scale_real + self.offset.real(),
                                                y as f64 * self.scale_imag + self.offset.imag());
            let curr = self.iterate(&calc);
            self.res.values[count] = curr;

            if x < self.width {
                x += 1;
            } else {
                x = 0;
                y += 1;
                if y >= self.height {
                    self.done = true;
                    points_done = Some(count + 1);
                    break;
                }
            }


            if count % 10 == 0 {
                if performance.now() - start >= MAX_DURATION {
                    points_done = Some(count + 1);
                    break;
                }
            }
        }
        if let Some(points) = points_done {
            self.res.num_points = points;
        } else {
            self.res.num_points = MAX_POINTS;
        }

        self.x_curr = x;
        self.y_curr = y;

        &self.res
    }
}
