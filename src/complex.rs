use std::ops::{MulAssign, Mul, AddAssign, Add};
use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Complex {
        Complex {
            real,
            imag,
        }
    }

    #[inline]
    pub fn real(&self) -> f64 { self.real }
    #[inline]
    pub fn imag(&self) -> f64 { self.imag }
    #[inline]
    pub fn set_real(&mut self, real: f64)  { self.real = real; }
    #[inline]
    pub fn set_imag(&mut self, imag: f64)  { self.imag = imag; }
    #[inline]
    pub fn square_length(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }
    #[inline]
    pub fn norm(&self) -> f64 {
        f64::sqrt(self.square_length())
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        self.real = self.real + other.real;
        self.imag = self.imag + other.imag;
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, other: Self) {
        let real = self.real * other.real - self.imag * other.imag;
        let imag = self.real * other.imag + self.imag * other.real;
        self.real = real;
        self.imag = imag;
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"({}+{}i)", self.real, self.imag)
    }
}
