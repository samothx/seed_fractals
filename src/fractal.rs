use super::{Model, complex::Complex};
use seed::{log};
// use wasm_bindgen::prelude::web_sys;
use seed::prelude::web_sys;

const MAX_POINTS: usize = 1000;

pub struct Points {
    pub x_start: u32,
    pub y_start: u32,
    pub values: Vec<u32>,
}


pub struct Fractal {
    x_scale: f64,
    y_scale: f64,
    x_offset: f64,
    y_offset: f64,
    c: Complex,
    max: f64,
    x_curr: u32,
    width: u32,
    height: u32,
    y_curr: u32,
    iterations: u32,
    max_duration: f64,
    done: bool
}


impl Fractal {
    pub fn new(model: &Model) -> Fractal {
        log!("creating fractal");
        let x_scale = (model.x_max - model.x_min) / model.width as f64;
        let y_scale = (model.y_max - model.y_min) / model.height as f64;
        Fractal {
            x_scale,
            y_scale,
            x_offset: model.x_min,
            y_offset: model.y_min,
            c: Complex::new(model.c_real, model.c_imag),
            max: model.c_real * model.c_real + model.c_imag * model.c_imag,
            x_curr: 0,
            width: model.width,
            y_curr: 0,
            height: model.height,
            iterations: model.max_iterations,
            max_duration: model.max_duration,
            done: false
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    fn iterate(&self, x: f64, y: f64) -> u32 {
        let mut curr = Complex::new(x,y);
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

    pub fn calculate(&mut self) -> Points {
        let performance = web_sys::window().expect("Window not found")
            .performance()
            .expect("performance should be available");

        let start = performance.now();
        let mut res = Points {
            x_start: self.x_curr,
            y_start: self.y_curr,
            values: Vec::with_capacity(MAX_POINTS),
        };

        let mut x = self.x_curr;
        let mut y = self.y_curr;
        let mut points = 0;

        loop {
            let x_calc = x as f64 * self.x_scale + self.x_offset;
            let y_calc = y as f64 * self.y_scale + self.y_offset;
            let curr = self.iterate(x_calc, y_calc);
            res.values.push(curr);
            points += 1;
            
            if x < self.width {
                x += 1;
            } else {
                x = 0;
                y += 1;
                if y >= self.height {
                    self.done = true;
                    break;
                }
            }


            if points % 10 == 0 {
                if points >= MAX_POINTS {
                    break;
                }

                // log!(format!("Fractal::calculate: check time: iterations: {}, elapsed: {}", num_iterations - last_check, performance.now() - start));
                if performance.now() - start >= self.max_duration {
                    break;
                }
            }
        }

        self.x_curr = x;
        self.y_curr = y;

        res
    }
}
