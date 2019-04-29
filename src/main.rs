extern crate cgmath;
extern crate image;

mod render;

use render::renderer::*;
use std::path::Path;

fn main() {
    let scene: Scene<f64> =
        Scene::load(&Path::new("./example_resources/cube.obj")).expect("Could not load obj file");
    let img = render(&scene);
    img.save("./out/test.png").expect("Failed to save to file.");
}
