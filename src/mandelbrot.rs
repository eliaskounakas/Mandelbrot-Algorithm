use super::image::Image;
use super::complex::Complex;


pub fn generate_image(width: usize, height: usize, max_iterations: usize) -> Image {
  let mut img = Image::new(width, height);

  for x in 0..width-1 {
    for y in 0..height-1 {
      let cx = (x as f64/width as f64 - 0.75) * 3.5;
      let cy = (y as f64/height as f64 - 0.5) * 2.0;
      let c = Complex {re: cx, im: cy};

      if check_pixel(c, max_iterations) == None {
        let px = img.get_mut(x, y).unwrap();
        px.r = 0;
        px.b = 0;
        px.g = 0;
      };
    };
  };

  img
}


pub fn check_pixel(c: Complex, max_iterations: usize) -> Option<usize> {
  let mut z = c;
  let mut iterations = 0;

  while iterations < max_iterations {
    z = z * z + c;
    if z.mag() > 4.0 {
      break;
    };

    iterations += 1;
  };

  if iterations == max_iterations {
    None
  } else {
    Some(iterations)
  }
}
