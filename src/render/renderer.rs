use super::camera::Camera;
use super::ray::Ray;
use super::renderable::Renderable;
use super::scene::*;
use image::{ImageBuffer, Pixel, Rgb};
use std::borrow::Borrow;

pub fn render(cam: Camera<f64>, scene: &Scene<f64>) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let img_height = 100;
    let img_width = 100;

    ImageBuffer::from_fn(img_height, img_width, |x, y| {
        render_ray(
            cam.ray_at(
                f64::from(x) / f64::from(img_width),
                f64::from(y) / f64::from(img_height),
            ),
            &scene,
        )
    })
}

fn render_ray(ray: Ray<f64>, scene: &Scene<f64>) -> Rgb<u8> {
    let objs: &Vec<_> = scene.objects.borrow();
    for render_obj in objs {
        if render_obj.intersection(&ray).is_some() {
            return Rgb::from_channels(255u8, 0, 0, 255);
        }
    }
    Rgb::from_channels(0u8, 0, 0, 255)
}
