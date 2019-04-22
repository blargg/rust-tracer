extern crate cgmath;

use cgmath::{Vector3, BaseFloat, InnerSpace};

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
    pub fn closestPoint(&self, p: Vector3<T>) -> Vector3<T> {
        let p_trans = p - self.origin;
        p_trans.project_on(self.direction) + self.origin
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use cgmath::vec3;
    use super::*;

    const delta: f64 = 0.00001;

    fn st_vec3<T>(s: impl Strategy<Value=T> + Clone) -> impl Strategy<Value = Vector3<T>>
        where T: Arbitrary
    {
        (s.clone(), s.clone(), s.clone()).prop_map(|(x, y, z)| vec3(x, y, z))
    }

    proptest! {
        #[test]
        fn closestOrig(point in st_vec3(-1000f64..1000f64)) {
            let v0 = vec3(0.0, 0.0, 0.0);
            let r: Ray<f64> = Ray::new(v0, point);
            let closest = r.closestPoint(point);
            let diff = (point-closest).magnitude();
            prop_assert!(diff < delta, "diff = {}", diff);
        }

        #[test]
        fn closestOnLine(orig in st_vec3(-1000f64..1000f64),
                         point in st_vec3(-1000f64..1000f64),
                         scale in -100f64..100f64) {
            // point lies on the line that contains r: Ray
            let r: Ray<f64> = Ray::new(orig, (point - orig) * scale);
            let closest = r.closestPoint(point);
            prop_assert!((point - closest).magnitude() < delta);
        }
    }
}
