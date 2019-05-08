extern crate alga;
extern crate image;
extern crate nalgebra as na;
extern crate num;

mod number;
mod render;

use na::{Point3, Vector3};
use render::camera::*;
use render::renderer::*;
use render::scene::*;
use std::f64::consts::PI;
use std::path::Path;

fn main() {
    let scene: Scene<f64> =
        Scene::load(&Path::new("./example_resources/cube.obj")).expect("Could not load obj file");
    let cam = Camera::look_at(
        Point3::new(1.0, 2.0, -2.0),
        Point3::new(0.5, 0.5, 0.5),
        Vector3::new(0.0, 1.0, 0.0),
        2.0,
        2.0,
        PI / 2.0,
    );
    let img = render(cam, &scene);
    img.save("./out/test.png").expect("Failed to save to file.");
}
