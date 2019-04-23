extern crate cgmath;
extern crate image;

mod render;

use image::{ImageBuffer, Rgb, Pixel};
use render::renderer::*;

fn main() {
    let img = render();
    img.save("./test.png").expect("Failed to save to file.");
}
