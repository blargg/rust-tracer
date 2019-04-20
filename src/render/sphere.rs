use cgmath::Vector3;
use super::ray;
use super::renderable::*;

pub struct Sphere<T> {
    center: Vector3<T>,
    radius: T,
}

impl Renderable for Sphere<f32> {
    fn intersection(self, ray: ray::Ray<f32>) -> bool {
        false
    }
}
