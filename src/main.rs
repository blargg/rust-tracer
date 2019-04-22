extern crate cgmath;
extern crate image;

mod render;

use image::{ImageBuffer, Rgb, Pixel};

fn main() {
    let mut img = ImageBuffer::from_fn(100, 100,
                                       |_, _| Rgb::from_channels(255u8, 0, 0, 255));
    img.put_pixel(10, 10, Rgb::from_channels(0, 255, 0, 255));
    img.save("./test.png").expect("Failed to save to file.");
}
