use super::ray::Ray;

trait Renderable {
    fn intersection(self, ray: Ray<f32>) -> Bool;
}
