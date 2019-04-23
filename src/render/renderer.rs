extern crate cgmath;
extern crate image;

use cgmath::{vec3};
use super::sphere::Sphere;
use super::renderable::Renderable;
use super::ray::Ray;
use image::{ImageBuffer, Rgb, Pixel};
use std::borrow::Borrow;

pub struct Scene<T> {
    objects: Vec<Sphere<T>>
}

pub fn render() -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let img_height = 100;
    let img_width = 100;
    let scene_height = 100.0;
    let scene_width = 100.0;

    let s = Sphere::new(vec3(50.0f32, 50.0, 100.0), 25.0f32);
    let sc = Scene {objects: vec![s]};

    ImageBuffer::from_fn(img_height, img_width,
                         |x, y| render_ray(generate_ray(x,y), &sc))
}

fn generate_ray(x: u32, y: u32) -> Ray<f32> {
    let origin = vec3(x as f32, y as f32, 0.0);
    let direction = vec3(0.0, 0.0, 1.0);
    Ray::new(origin, direction)
}

fn render_ray(ray: Ray<f32>, scene: &Scene<f32>) -> Rgb<u8> {
    let objs: &Vec<_> = scene.objects.borrow();
    for sphere in objs {
        if sphere.intersection(&ray).is_some() {
            return Rgb::from_channels(255u8, 0, 0, 255);
        }
    }
    return Rgb::from_channels(0u8, 0, 0, 255);
}
