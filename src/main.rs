extern crate cgmath;
extern crate image;

mod render;

use cgmath::*;
use render::camera::*;
use render::renderer::*;
use std::f64::consts::PI;
use std::path::Path;

fn main() {
    let scene: Scene<f64> =
        Scene::load(&Path::new("./example_resources/cube.obj")).expect("Could not load obj file");
    let cam = Camera::look_at(
        vec3(1.0, 2.0, -2.0),
        vec3(0.5, 0.5, 0.5),
        vec3(0.0, 1.0, 0.0),
        2.0,
        2.0,
        Rad(PI / 2.0),
    );
    let img = render(cam, &scene);
    img.save("./out/test.png").expect("Failed to save to file.");
}
