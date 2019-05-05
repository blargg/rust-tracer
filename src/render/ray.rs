extern crate alga;
extern crate nalgebra as na;

use crate::number::*;
use na::*;

#[derive(Debug)]
pub struct Ray<T: Scalar> {
    pub origin: Vector3<T>,
    pub direction: Vector3<T>,
}

impl<T: Scalar> Ray<T> {
    pub fn new(orig: Vector3<T>, dir: Vector3<T>) -> Ray<T> {
        Ray {
            origin: orig,
            direction: dir,
        }
    }
}

impl<T: GenFloat> Ray<T> {
    pub fn closest_point(&self, p: Vector3<T>) -> Vector3<T> {
        let p_trans = p - self.origin;
        let mag: T = self.direction.dot(&p_trans) / self.direction.magnitude_squared();
        let mag: T = mag.max(T::zero());
        self.origin + (self.direction * mag)
    }

    // returns the point in the ray traveling time * the direction vector from the orign
    pub fn at_time(&self, time: T) -> Vector3<T> {
        self.origin + self.direction * time
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use na::{ArrayStorage, U1, U3};
    use proptest::prelude::*;

    const DELTA: f64 = 0.00001;

    pub fn st_vec3<T>(
        s: impl Strategy<Value = T> + Clone,
    ) -> impl Strategy<Value = Matrix<T, U3, U1, ArrayStorage<T, U3, U1>>>
    where
        T: Arbitrary + Scalar,
    {
        (s.clone(), s.clone(), s.clone()).prop_map(|(x, y, z)| Vector3::new(x, y, z))
    }

    pub fn arb_ray<T: Scalar>(
        orig_st: impl Strategy<Value = T> + Clone,
        dir_st: impl Strategy<Value = T> + Clone,
    ) -> impl Strategy<Value = Ray<T>>
    where
        T: Arbitrary,
    {
        (st_vec3(orig_st), st_vec3(dir_st)).prop_map(|(o, d)| Ray::new(o, d))
    }

    proptest! {
        #[test]
        fn closest_origin(point in st_vec3(-1000f64..1000f64)) {
            let v0 = Vector3::new(0.0, 0.0, 0.0);
            let r: Ray<f64> = Ray::new(v0, point);
            let closest = r.closest_point(point);
            let diff = (point-closest).magnitude();
            prop_assert!(diff < DELTA, "diff = {}", diff);
        }

        #[test]
        fn closest_in_front(orig in st_vec3(-1000f64..1000f64),
                            point in st_vec3(-1000f64..1000f64),
                            scale in 0f64..100f64) {
            // point lies on the line that contains r: Ray
            let r: Ray<f64> = Ray::new(orig, (point - orig) * scale);
            let closest = r.closest_point(point);
            prop_assert!((point - closest).magnitude() < DELTA);
        }

        #[test]
        fn closest_behind(orig in st_vec3(-1000f64..1000f64),
                          point in st_vec3(-1000f64..1000f64),
                          scale in -1000f64..0f64) {
            // closest point must be at the start of the ray
            let r: Ray<f64> = Ray::new(orig, (point - orig) * scale);
            let closest = r.closest_point(point);
            prop_assert!((r.origin - closest).magnitude() < DELTA);
        }
    }
}
