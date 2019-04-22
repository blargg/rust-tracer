extern crate cgmath;

use cgmath::{dot, Vector3, BaseFloat, InnerSpace};

#[derive(Debug)]
pub struct Ray<T> {
    pub origin: Vector3<T>,
    pub direction: Vector3<T>,
}

impl<T> Ray<T> {
    pub const fn new(orig: Vector3<T>, dir: Vector3<T>) -> Ray<T> {
        Ray { origin: orig, direction: dir }
    }
}

impl<T: BaseFloat> Ray<T> {
    pub fn closest_point(&self, p: Vector3<T>) -> Vector3<T> {
        let p_trans = p - self.origin;
        let mag: T = dot(self.direction, p_trans) / self.direction.magnitude2();
        let mag: T = mag.max(T::zero());
        self.origin + (self.direction * mag)
    }
}

#[cfg(test)]
pub mod tests {
    use proptest::prelude::*;
    use cgmath::vec3;
    use super::*;

    const DELTA: f64 = 0.00001;

    pub fn st_vec3<T>(s: impl Strategy<Value=T> + Clone) -> impl Strategy<Value = Vector3<T>>
        where T: Arbitrary
    {
        (s.clone(), s.clone(), s.clone()).prop_map(|(x, y, z)| vec3(x, y, z))
    }

    pub fn arb_ray<T>(orig_st: impl Strategy<Value=T> + Clone, dir_st: impl Strategy<Value=T> + Clone)
                     -> impl Strategy<Value = Ray<T>>
        where T: Arbitrary
    {
        (st_vec3(orig_st), st_vec3(dir_st)).prop_map(|(o, d)| Ray::new(o, d))
    }

    proptest! {
        #[test]
        fn closest_origin(point in st_vec3(-1000f64..1000f64)) {
            let v0 = vec3(0.0, 0.0, 0.0);
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
