
use cgmath::{Vector3, vec3, dot};
use cgmath::InnerSpace;
use super::ray;
use super::renderable::*;

pub struct Sphere<T> {
    center: Vector3<T>,
    radius: T,
}

impl Renderable for Sphere<f32> {
    fn intersection(&self, ray: ray::Ray<f32>) -> bool {
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

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use cgmath::abs_diff_eq;

    use super::*;
    use super::ray::Ray;

    #[test]
    fn intersection_test() {
        let s: Sphere<f32> = Sphere{center: vec3(0.0, 0.0 ,0.0), radius: 5.0};;
        let r: Ray<f32> = Ray{origin: vec3(0.0,0.0,0.0), direction: vec3(1.0,0.0,0.0)};

        let r2: Ray<f32> = Ray::new(vec3(100.0, 0.0, 0.0), vec3(-1.0, 0.0, 0.0));

        assert!(s.intersection(r));
        //assert!(!(s.intersection(r2)));
    }
}
