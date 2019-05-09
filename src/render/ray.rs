use crate::vector::*;
use na::*;

#[derive(Debug)]
pub struct Ray<T: Scalar> {
    pub origin: Point3<T>,
    pub direction: UnitVec3<T>,
}

impl<T: Scalar> Ray<T> {
    pub fn new(origin: Point3<T>, direction: UnitVec3<T>) -> Ray<T> {
        Ray { origin, direction }
    }
}

impl<T: RealField> Ray<T> {
    pub fn new_normalize(origin: Point3<T>, direction: Vector3<T>) -> Ray<T> {
        Ray {
            origin,
            direction: Unit::new_normalize(direction),
        }
    }

    pub fn closest_point(&self, p: Point3<T>) -> Point3<T> {
        let p_trans = p - self.origin;
        let mag: T = self.direction.dot(&p_trans) / self.direction.magnitude_squared();
        let mag: T = mag.max(T::zero());
        self.origin + (self.direction.into_inner() * mag)
    }

    // returns the point in the ray traveling time * the direction vector from the orign
    pub fn at_time(&self, time: T) -> Point3<T> {
        self.origin + self.direction.into_inner() * time
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

    pub fn arb_point<N>(st: impl Strategy<Value = N> + Clone) -> impl Strategy<Value = Point3<N>>
    where
        N: Arbitrary + Scalar,
    {
        st_vec3(st).prop_map(Point3::from)
    }

    pub fn arb_ray<T: RealField>(
        orig_st: impl Strategy<Value = T> + Clone,
        dir_st: impl Strategy<Value = T> + Clone,
    ) -> impl Strategy<Value = Ray<T>>
    where
        T: Arbitrary,
    {
        (arb_point(orig_st), st_vec3(dir_st)).prop_map(|(o, d)| Ray::new_normalize(o, d))
    }

    proptest! {
        #[test]
        fn closest_origin(point in arb_point(-1000f64..1000f64)) {
            let v0 = Point3::new(0.0, 0.0, 0.0);
            let r: Ray<f64> = Ray::new_normalize(v0, (point - v0));
            let closest = r.closest_point(point);
            let diff = (point-closest).magnitude();
            prop_assert!(diff < DELTA, "diff = {}", diff);
        }

        #[test]
        fn closest_in_front(orig in arb_point(-1000f64..1000f64),
                            point in arb_point(-1000f64..1000f64),
                           ) {
            // point lies on the line that contains r: Ray
            let r: Ray<f64> = Ray::new_normalize(orig, point - orig);
            let closest = r.closest_point(point);
            prop_assert!((point - closest).magnitude() < DELTA);
        }

        #[test]
        fn closest_behind(orig in arb_point(-1000f64..1000f64),
                          point in arb_point(-1000f64..1000f64),
                          scale in -1000f64..0f64) {
            // closest point must be at the start of the ray
            let r: Ray<f64> = Ray::new_normalize(orig, (point - orig) * scale);
            let closest = r.closest_point(point);
            prop_assert!((r.origin - closest).magnitude() < DELTA);
        }
    }
}
