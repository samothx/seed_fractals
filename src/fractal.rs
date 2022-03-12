use super::{Model, complex::Complex};
use seed::{log};

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
    y_curr: u32,
    height: u32,
    iterations: u32,
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
            iterations: model.max_iterations
        }
    }

    fn iterate(&self, x: f64, y: f64) -> u32 {
        let mut curr = Complex::new(x,y);
        if curr.square_length() >= self.max {
            0
        } else {
            log!(format!("iterate: start: {}", curr));
            let mut last: Option<u32> = None;
            for idx in 1..=self.iterations {
                curr = curr * curr + self.c;
                if curr.square_length() >= self.max {
                    last = Some(idx);
                    break;
                }
            }

            log!(format!("iterate: end:  {} norm: {} last: {:?}", curr, curr.square_length(), last));
            if let Some(last) = last {
                last
            } else {
                0
            }
        }
    }

    pub fn calculate(&mut self, num_points: u32) -> Points {
        let mut res = Points {
            x_start: self.x_curr,
            y_start: self.y_curr,
            values: Vec::with_capacity(num_points as usize),
        };

        let mut x = self.x_curr;
        let mut y = self.y_curr;
        for _ in 0..num_points {
            let x_calc = x as f64 * self.x_scale + self.x_offset;
            let y_calc = y as f64 * self.y_scale + self.y_offset;
            res.values.push(self.iterate(x_calc, y_calc));
            if x < self.width {
                x += 1;
            } else {
                x = 0;
                y += 1;
            }
        }
        self.x_curr = x;
        self.y_curr = y;
        log!(format!("Fractal::calculate: res ({},{}) {:?}", res.x_start, res.y_start, res.values));
        res
    }
}
