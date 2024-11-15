use std::ops::{Add, Mul};

#[derive(Copy, Clone)]
struct Complex {
  re: f64,
  im: f64,
}

impl Add for Complex {
  type Output = Complex;
  
  fn add(self, other: Complex) -> Complex {
    Complex {
      re: self.re + other.re,
      im: self.im + other.im,
    }
  }
}

impl Mul for Complex {
  type Output = Complex;
  
  fn mul(self, other: Complex) -> Complex {
    Complex {
      re: self.re * other.re - self.im * other.im,
      im: self.re * other.im + self.im * other.re,
    }
  }
}

impl Complex {
  fn mag(&self) -> f64 {
    self.re * self.re + self.im + self.im
  }
}