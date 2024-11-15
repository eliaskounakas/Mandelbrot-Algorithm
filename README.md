### Usage

1. Clone the Repo and cd into folder
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
*max_iteraitons* is optional and set to 1024 by if omitted.

### Example

```bash
cargo run -- 525 300
```
will produce the following image:

![mandelbrot image](https://github.com/eliaskounakas/mandelbrot-algorithm/blob/main/preview_image.png)
