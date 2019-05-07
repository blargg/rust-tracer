use super::ray;
use super::shape::*;
use alga::general::Ring;
use na::{Scalar, Vector3};

#[derive(Debug)]
pub struct Triangle<T: Scalar> {
    v1: Vector3<T>,
    v2: Vector3<T>,
    v3: Vector3<T>,
}

impl<T: Scalar> Triangle<T> {
    pub fn new(vertex1: Vector3<T>, vertex2: Vector3<T>, vertex3: Vector3<T>) -> Triangle<T> {
        Triangle {
            v1: vertex1,
            v2: vertex2,
            v3: vertex3,
        }
    }
}

impl<T: Scalar + Ring> Triangle<T> {
    // Returns true normal of the triangle.
    // This will ignore smoothing and bump mapping.
    fn true_normal(&self) -> Vector3<T> {
        let e1 = self.v2 - self.v1;
        let e2 = self.v3 - self.v1;
        e1.cross(&e2)
    }
}

impl Shape for Triangle<f64> {
    type NumTy = f64;
    fn intersection(&self, ray: &ray::Ray<f64>) -> Option<f64> {
        let e1: Vector3<f64> = self.v2 - self.v1;
        let e2: Vector3<f64> = self.v3 - self.v1;
        let s1 = ray.direction.cross(&e2);
        let divisor = s1.dot(&e1);
        // TODO can we use a better epsilon?
        if divisor.abs() < 0.000001 {
            return None;
        }

        let inv_div = 1.0 / divisor;
        let s: Vector3<f64> = ray.origin - self.v1;
        // b1 of the barycentric coordinates
        let b1: f64 = s1.dot(&s) * inv_div;

        if b1 < 0.0 || b1 > 1.0 {
            return None; // lies outside of the triangle
        }

        let s2 = s.cross(&e1);
        let b2 = ray.direction.dot(&s2) * inv_div;
        if b2 < 0.0 || b1 + b2 > 1.0 {
            return None;
        }

        let t = e2.dot(&s2) * inv_div;
        if t < 0.0 {
            None
        } else {
            Some(t)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ray::tests::{arb_ray, st_vec3};
    use super::ray::*;
    use super::*;
    use proptest::prelude::*;

    pub fn arb_tri<T: Scalar>(
        s: impl Strategy<Value = T> + Clone,
    ) -> impl Strategy<Value = Triangle<T>>
    where
        T: Arbitrary,
    {
        (st_vec3(s.clone()), st_vec3(s.clone()), st_vec3(s.clone()))
            .prop_map(|(v1, v2, v3)| Triangle::new(v1, v2, v3))
    }

    #[test]
    fn intersects_triangle() {
        let tri: Triangle<f64> = Triangle::new(
            Vector3::new(0.0, -1.0, 1.0),
            Vector3::new(0.0, -1.0, -1.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        let ray: Ray<f64> = Ray::new(Vector3::new(-1.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));

        let intersection = tri.intersection(&ray);
        match intersection {
            None => assert!(false),
            Some(t) => assert!((t - 1.0).abs() < 0.00001),
        }
    }

    proptest! {
        // Removing from actual test cases, until the sample rejection rate is improved
        // #[test]
        fn intersection_on_plane(tri in arb_tri(-1000f64..1000.0),
                                 ray in arb_ray(-1f64..1.0, -1f64..1.0)) {

            prop_assume!(ray.direction.magnitude() > 0.000001);
            let intersection = tri.intersection(&ray);

            match intersection {
                None => prop_assume!(false), // throw out non-intersecting cases
                Some (t) => {
                    let int_point = ray.at_time(t);
                    let n = tri.true_normal();
                    let d_intersection = n.dot(&int_point);
                    let d_actual = n.dot(&tri.v1);
                    prop_assert!((d_intersection - d_actual).abs() < 0.0001)
                },
            }
        }

        // prop, for any random point on the triange, a ray pointing to that should intersect
    }
}
