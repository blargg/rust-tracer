use cgmath::{Vector3, dot};
use super::ray;
use super::renderable::*;

pub struct Sphere<T> {
    center: Vector3<T>,
    radius: T,
}

impl Renderable for Sphere<f32> {
    fn intersection(self, ray: ray::Ray<f32>) -> bool {
        let ray_offset = ray.origin - self.center;
        let a: f32 = dot(ray.direction, ray.direction); // TODO test with magnitude2
        let b = 2.0 * dot(ray_offset, ray.direction);
        let c = dot(ray_offset, ray_offset) - self.radius.powi(2);
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            false
        }
        else {
            true
        }
    }
}
