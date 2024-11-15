use std::fmt;
#[derive(Debug, Copy, Clone, PartialEq)]
struct Pixel {
  r: u8,
  g: u8,
  b: u8,
}

impl fmt::Display for Pixel {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} {} {}", self.r, self.g, self.b)
  }
}

struct Image {
  width: usize,
  height: usize,
  data: Vec<Pixel>,
}

impl Image {
  pub fn new(width: usize, height: usize) -> Image {
    let px = Pixel {
      r: 255,
      g: 255,
      b: 255
    };
    Image { width, height, data: vec![px; height * width]}
  }

  pub fn get(&self, x:usize, y:usize) -> Option<&Pixel> {
    if x <= self.width && y <= self.height {
      let location = &self.data[self.width * y + x];
      Some(location)
    } else {
      None
    }
  }

  pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
    if x <= self.width && y <= self.height {
      let location = &mut self.data[self.width * y + x];
      Some(location)
    } else {
      None
    }
  }

  pub fn get_mandelbrot_pixels(&self) -> usize {
    1
  }
}