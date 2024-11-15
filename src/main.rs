use client::save_to_file;
use mandelbrot::generate_image;

mod image;
mod complex;
mod mandelbrot;
mod client;

fn main() {
    let input: (usize, usize, usize);

    match client::parse_args() {
        Ok(res) => {
            input = res;
        }
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            std::process::exit(1);
        }
    }

    let width = input.0;
    let height = input.1;
    let max_iterations = input.2;

    println!("Generating Mandelbrot for {}x{} image (max_iterations: {})", width, height, max_iterations);

    // call mandelbrot::generate_image(width, height, max_iterations) and save the result to an image
    let mandelbrot_img = &generate_image(width, height, max_iterations);
    save_to_file(mandelbrot_img, "mandelbrot.ppm");

    // call the get_mandelbrot_pixels() method on the image struct and save the result in mandelbrot_pixel_count
    let mandelbrot_pixel_count = mandelbrot_img.get_mandelbrot_pixels();

    // if the above line worked you should be able to uncomment this line
    println!("Pixels in the set: {}", mandelbrot_pixel_count);
    
    // uncomment and call after you implement the mandelbrot functions, and handle the possible error
    // client::save_to_file(&image, "mandelbrot.ppm");
}
