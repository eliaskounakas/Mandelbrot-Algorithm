use std::fmt;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl fmt::Display for Pixel {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} {} {}", self.r, self.g, self.b)
  }
}

pub struct Image {
  pub width: usize,
  pub height: usize,
  data: Vec<Pixel>,
}

impl Image {
  pub fn new(width: usize, height: usize) -> Image {
    let px = Pixel {
      r: 0,
      g: 0,
      b: 0
    };
    Image { width, height, data: vec![px; height * width]}
  }

  pub fn get(&self, x:usize, y:usize) -> Option<&Pixel> {
    if x < self.width && y < self.height {
      Some(&self.data[self.width * y + x])
    } else {
      None
    }
  }

  pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
    if x <= self.width && y <= self.height {
      Some(&mut self.data[self.width * y + x])
    } else {
      None
    }
  }

  pub fn get_mandelbrot_pixels(&self) -> usize {
    let mut sum = 0;
    let is_black_pixel = |px| px == &Pixel {
      r: 0,
      g: 0,
      b: 0
    };

    for px in self.data.iter() {
      if is_black_pixel(px) {
        sum += 1;
      }
    };

    sum
  }
}