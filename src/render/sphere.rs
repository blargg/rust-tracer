use super::ray;
use super::shape::*;
use crate::number::*;
use na::{Scalar, Vector3, Point3};
use num::Zero;
use std::cmp::PartialOrd;

#[derive(Debug)]
pub struct Sphere<T: Scalar> {
    center: Point3<T>,
    radius: T,
}

impl<T: Scalar + Zero + PartialOrd> Sphere<T> {
    pub fn new(center: Point3<T>, radius: T) -> Sphere<T> {
        let r: T;
        if T::zero().lt(&radius) {
            r = radius;
        } else {
            r = T::zero();
        }
        Sphere { center:center, radius: r }
    }
}

impl<N: GenFloat> Shape for Sphere<N> {
    type NumTy = N;

    fn intersection(&self, ray: &ray::Ray<N>) -> Option<N> {
        let ray_offset: Vector3<N> = ray.origin - self.center;
        let a: N = ray.direction.norm_squared();
        let b: N = double(ray_offset.dot(&ray.direction));
        let c: N = ray_offset.dot(&ray_offset) - self.radius.powi(2);
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

    fn normal(&self, point: &Point3<Self::NumTy>) -> Vector3<Self::NumTy> {
        point - self.center
    }
}

fn double<N: std::ops::Add + Copy>(n: N) -> N::Output {
    n + n
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::ray::tests::*;
    use super::ray::Ray;
    use super::*;
    use approx::abs_diff_eq;

    const DELTA: f32 = 0.001;

    pub fn arb_sphere<T: Scalar>(
        c: impl Strategy<Value = T> + Clone,
        r: impl Strategy<Value = T> + Clone,
    ) -> impl Strategy<Value = Sphere<T>>
    where
        T: Arbitrary + Zero + PartialOrd,
    {
        (arb_point(c.clone()), r.clone()).prop_map(|(cnt, rad)| Sphere::new(cnt, rad))
    }

    #[test]
    fn intersection_test() {
        let s: Sphere<f32> = Sphere {
            center: Point3::origin(),
            radius: 5.0,
        };;
        let r: Ray<f32> = Ray {
            origin: Point3::origin(),
            direction: Vector3::new(1.0, 0.0, 0.0),
        };

        let r2: Ray<f32> = Ray::new(Point3::new(100.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));

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
