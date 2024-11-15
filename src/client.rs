use std::{env, fs,};
use crate::image::Image;
use std::num::ParseIntError;

pub fn parse_args() -> Result<(usize, usize, usize), ParseIntError> {

  let args: Vec<String> = env::args().collect();
  
  if args.len() < 3 || args.len() > 4 {
    println!("Usage: {} <width> <height> <max_iterations>", args[0]);
    std::process::exit(1);
  };

  let width = args[1].parse::<usize>()?;
  let height = args[2].parse::<usize>()?;
  let mut max_iterations = 1024;      

  if args.len() == 4 {
    max_iterations = args[3].parse::<usize>()?;      
  }

  Ok((width, height, max_iterations))
}

pub fn save_to_file(image: &Image, filename: &str) {
    let mut s = String::new();
    s.push_str(&format!("P3\n{} {}\n255\n", image.width, image.height));
    
    for y in 0..image.height {
        for x in 0..image.width {
            let pixel = image.get(x, y).unwrap();
            s.push_str(&format!("{}\n", pixel));
        }
    }

    fs::write(filename, s).expect("Error writing to disk!");
}
