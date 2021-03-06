use na::{Point3, RealField, Scalar, Vector3};

pub struct Plane<T: Scalar> {
    normal: Vector3<T>,
    dist: T,
}

impl<T: Scalar> Plane<T> {
    pub fn new(normal: Vector3<T>, dist: T) -> Plane<T> {
        Plane { normal, dist }
    }
}

impl<T: RealField> Plane<T> {
    pub fn new_at_point(position: Point3<T>, normal: Vector3<T>) -> Plane<T> {
        Plane::new(normal, -normal.dot(&(position - Point3::origin())))
    }

    pub fn distance_to(&self, point: Point3<T>) -> T {
        let t = (-self.dist + -self.normal.dot(&(point - Point3::origin())))
            / self.normal.magnitude_squared();
        (self.normal * t).magnitude()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::abs_diff_eq;

    fn vec3<T: Scalar>(x: T, y: T, z: T) -> Vector3<T> {
        Vector3::new(x, y, z)
    }

    #[test]
    fn distance_to_test() {
        let plane: Plane<f64> = Plane::new(vec3(1.0, 0.0, 0.0), 0.0);

        let dist = plane.distance_to(Point3::new(100.0, 992.3, 59.0));
        assert!(abs_diff_eq!(dist, 100.0));

        let plane: Plane<f64> = Plane::new(vec3(1.0, 0.0, 0.0), -10.0);
        let dist = plane.distance_to(Point3::new(100.0, 992.3, 59.0));
        assert!(abs_diff_eq!(dist, 90.0));

        let plane: Plane<f64> = Plane::new(vec3(1.0, 1.0, 0.0), 0.0);
        let dist = plane.distance_to(Point3::new(-1.0, 1.0, 0.0));
        assert!(abs_diff_eq!(dist, 0.0));
    }

    #[test]
    fn new_at_point_test() {
        let pos = Point3::new(100.0, 50.0, 25.2);
        let plane: Plane<f64> = Plane::new_at_point(pos, vec3(1.0, 0.0, 0.0));

        assert!(plane.distance_to(pos) < 0.0001);
        assert!(plane.distance_to(Point3::new(100.0, 20.0, 88.0)) < 0.0001);
    }
}
