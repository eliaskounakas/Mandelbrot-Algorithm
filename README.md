### Usage

1. Clone the repo and cd into folder
```bash
 git clone https://github.com/eliaskounakas/mandelbrot-generator.git
```
```bash
 cd mandelbrot-generator
```
2. Run the generator
```bash
 cargo run -- <width> <height> <max_iterations>
```
*max_iterations* is optional and set to 1024 if omitted.

### Example

```bash
cargo run -- 525 300
```
will produce the following image (increase the pixel count for higher res):

![mandelbrot image](https://github.com/eliaskounakas/mandelbrot-algorithm/blob/main/preview_image.png)
