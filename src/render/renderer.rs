use super::camera::Camera;
use super::color;
use super::material::BSDF;
use super::ray::Ray;
use super::renderable::Renderable;
use super::scene::*;
use super::shape::DiffGeom;
use image::{ImageBuffer, Pixel, Rgb};
use num::ToPrimitive;

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
    spectrum_to_pixel_color(radiance(ray, scene))
}

/// Calculates the radiance of the light moving into the ray origin from the scene.
fn radiance(ray: Ray<f64>, scene: &Scene<f64>) -> color::Rgb<f64> {
    let intersection = scene.intersects_renderable(&ray);
    match intersection {
        None => color::Rgb::new(0.0, 0.0, 0.0),
        Some((renderable, t)) => {
            let isct_pt = ray.at_time(t);
            let norm = renderable.normal(&isct_pt);
            let diff_geom = DiffGeom::new(isct_pt, norm);
            let bsdf = renderable.get_bsdf(&diff_geom);
            // TODO iterate over all lights
            let reflect = bsdf.bsdf(
                &(ray.direction.into_inner() * -1.0),
                &norm,
                &(scene.lights[0].position - isct_pt),
            );
            scene.lights[0].color.clone() * reflect
        }
    }
}

/// Converts a light spectrum to a pixel color.
fn spectrum_to_pixel_color(spec: color::Rgb<f64>) -> image::Rgb<u8> {
    Rgb::from_channels(
        clamp_255(spec.red),
        clamp_255(spec.green),
        clamp_255(spec.blue),
        255u8,
    )
}

/// Clamps on the range 0.0 to 1.0, then converts to u8.
/// 0.0 -> 0u8
/// 1.0 -> 255u8
fn clamp_255(x: f64) -> u8 {
    (x.min(1.0).max(0.0) * 255.0).to_u8().unwrap_or(0u8)
}

// fn lighting(r: Renderable, view: Vector3<f64>, light: Vecter3<f64>) -> Rgb<f64> {
//
// }
