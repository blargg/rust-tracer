extern crate num;

use super::ray;
use super::renderable::*;
use cgmath::{dot, BaseFloat, InnerSpace, Vector3};
use num::Zero;
use std::cmp::PartialOrd;

#[derive(Debug)]
pub struct Sphere<T> {
    center: Vector3<T>,
    radius: T,
}

impl<T: Zero + PartialOrd> Sphere<T> {
    pub fn new(center: Vector3<T>, radius: T) -> Sphere<T> {
        let r: T;
        if T::zero().lt(&radius) { r = radius; } else { r = T::zero(); }
        Sphere {
            center,
            radius: r,
        }
    }
}

impl<N: BaseFloat> Renderable for Sphere<N> {
    type NumTy = N;

    fn intersection(&self, ray: &ray::Ray<N>) -> Option<N> {
        let ray_offset = ray.origin - self.center;
        let a: N = ray.direction.magnitude2();
        let b: N = double(dot(ray_offset, ray.direction));
        let c: N = dot(ray_offset, ray_offset) - self.radius.powi(2);
        let discriminant = (b * b) - double(double(a * c));
        if discriminant < N::zero() {
            return None;
        } else {
            let disc_sq = discriminant.sqrt();
            let numerator = -b - disc_sq;
            if numerator > N::zero() {
                return Some(numerator / double(a));
            }

            let numerator = -b + disc_sq;
            if numerator > N::zero() {
                return Some(numerator / double(a));
            } else {
                return None;
            }
        }
    }
}

fn double<N: BaseFloat>(n: N) -> N {
    n + n
}

#[cfg(test)]
mod tests {
    use cgmath::{abs_diff_eq, vec3, InnerSpace};
    use proptest::prelude::*;

    use super::ray::tests::{arb_ray, st_vec3};
    use super::ray::Ray;
    use super::*;

    const DELTA: f32 = 0.001;

    pub fn arb_sphere<T>(
        c: impl Strategy<Value = T> + Clone,
        r: impl Strategy<Value = T> + Clone,
    ) -> impl Strategy<Value = Sphere<T>>
    where
        T: Arbitrary + Zero + PartialOrd,
    {
        (st_vec3(c.clone()), r.clone()).prop_map(|(cnt, rad)| Sphere::new( cnt, rad,))
    }

    #[test]
    fn intersection_test() {
        let s: Sphere<f32> = Sphere {
            center: vec3(0.0, 0.0, 0.0),
            radius: 5.0,
        };;
        let r: Ray<f32> = Ray {
            origin: vec3(0.0, 0.0, 0.0),
            direction: vec3(1.0, 0.0, 0.0),
        };

        let r2: Ray<f32> = Ray::new(vec3(100.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0));

        assert!(s.intersection(&r).is_some());
        assert!(s.intersection(&r2).is_none());
    }

    proptest! {
        #[test]
        fn intersection_detection(r in arb_ray(-100f32..100f32, -100f32..100f32),
                                  s in arb_sphere(-100f32..100f32, 0f32..100f32)) {
            prop_assume!(r.direction.magnitude() > DELTA);
            let intersects = s.intersection(&r).is_some();
            let closest = r.closest_point(s.center);
            let actual = (closest - s.center).magnitude() < s.radius + DELTA;

            prop_assert_eq!(intersects, actual);
        }

        #[test]
        fn intersection_on_surface(r in arb_ray(-100f32..100f32, -100f32..100f32),
                                   s in arb_sphere(-100f32..100f32, 0f32..100f32)) {
            prop_assume!(r.direction.magnitude() > DELTA);
            let intersection = s.intersection(&r).map(|t| r.at_time(t));

            match intersection {
                Some(point) => prop_assert!(abs_diff_eq!((point - s.center).magnitude(), s.radius, epsilon = DELTA)),
                None => (),
            }
        }
    }
}
