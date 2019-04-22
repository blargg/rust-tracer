use super::ray::Ray;

pub trait Renderable {
    fn intersection(&self, ray: Ray<f32>) -> bool;
}
