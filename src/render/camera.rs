use super::ray::Ray;
use na::{Point3, RealField, Rotation3, Scalar, Vector3};

#[derive(Debug)]
pub struct Camera<T: Scalar> {
    // position of the camera
    position: Point3<T>,
    // (where it is looking at)
    orientation: Rotation3<T>,
    width: T,
    height: T,
    fov: T,
}

impl<T: Scalar> Camera<T> {
    pub fn new(
        position: Point3<T>,
        orientation: Rotation3<T>,
        width: T,
        height: T,
        fov: T,
    ) -> Camera<T> {
        Camera {
            position,
            orientation,
            width,
            height,
            fov,
        }
    }
}

impl<T: RealField> Camera<T> {
    pub fn look_at(
        position: Point3<T>,
        at_point: Point3<T>,
        up: Vector3<T>,
        width: T,
        height: T,
        fov: T,
    ) -> Camera<T> {
        let view_direction = at_point - position;
        let orientation = Rotation3::look_at_lh(&view_direction, &up).inverse();
        Camera::new(position, orientation, width, height, fov)
    }
}

// generalize this to other floats
impl Camera<f64> {
    // returns a ray at the given coordinates on the camera
    // x and y: [0, 1] are percents of the way across the camera
    pub fn ray_at(&self, x: f64, y: f64) -> Ray<f64> {
        // point on the unit screen at
        // calculate the focal point behind the sceen
        // draw a ray at the screen with the angle
        // orient and translate the ray
        let off_set: Vector3<f64> = self.orientation.transform_vector(&Vector3::new(
            (x - 0.5) * self.width,
            (y - 0.5) * self.height,
            0.0,
        ));
        let point = self.position + off_set;

        // focal_point lies behind the camera plane, used to determine the ray direction.
        let half_fov = self.fov / 2.0;
        let focal_distance = self.width / (2.0 * half_fov.tan());
        let focal_point = self.position
            + self
                .orientation
                .transform_vector(&Vector3::new(0.0, 0.0, -1.0))
                * focal_distance;
        Ray::new_normalize(point, point - focal_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::plane::*;
    use crate::render::ray::tests::*;
    use approx::abs_diff_eq;
    use proptest::prelude::*;
    use std::f64::consts::PI;

    #[test]
    fn center_point_at_position() {
        let position = Point3::new(0.0, 0.0, 0.0);
        let cam: Camera<f64> = Camera::look_at(
            position,
            Point3::new(0.0, 0.0, 1.0),
            Vector3::new(0.0, 1.0, 0.0),
            1.0,
            1.0,
            std::f64::consts::PI / 2.0,
        );
        let mid_ray = cam.ray_at(0.5, 0.5);

        assert!(abs_diff_eq!(mid_ray.origin, position));
    }

    prop_compose! {
        fn arb_basis3()
            (v_init in st_vec3(-1.0f64..1.0f64),
             rot in 0.0..1.0f64) -> Rotation3<f64>
        {
            let v = if v_init.magnitude() < 0.000001 {
                Vector3::x()
            } else {
                v_init
            };
            Rotation3::from_scaled_axis(v.normalize() * 2.0 * PI * rot)
        }
    }

    prop_compose! {

        fn arb_camera()
            (pos in arb_point(-100.0f64..100.0f64),
             orientation in arb_basis3(),
             width in 1.0f64..100.0,
             height in 1.0f64..100.0) -> Camera<f64>
        {
            Camera::new(pos, orientation, width, height, PI / 2.0)
        }
    }

    proptest! {
        #[test]
        fn looks_at_point(position in arb_point(-100.0f64..100.0),
                          target in arb_point(-100.0f64..100.0)) {
            prop_assume!((position - target).magnitude_squared() > 0.0001);
            let cam = Camera::look_at(position, target, Vector3::y(), 1.0, 1.0, PI/2.0);
            let center_ray = cam.ray_at(0.5, 0.5);
            let closest = center_ray.closest_point(target);
            println!("cam = {:?}", cam);
            println!("ray = {:?}", center_ray);
            prop_assert!((closest - target).magnitude() < 0.0001);
        }

        #[test]
        fn ray_origin_on_view_plane(cam in arb_camera(),
                                    x in 0.0f64..1.0,
                                    y in 0.0f64..1.0) {
            let ray = cam.ray_at(x, y);
            let view_plane = Plane::new_at_point(cam.position, cam.orientation.transform_vector(&(Vector3::z() * -1.0)));

            prop_assert!(view_plane.distance_to(ray.origin) < 0.00001);
        }

        #[test]
        fn center_ray_views_direction(cam in arb_camera()) {
            let ray = cam.ray_at(0.5, 0.5);
            let view_vector = cam.orientation.transform_vector(&Vector3::z());
            let theta = ray.direction.angle(&view_vector);

            prop_assert!(theta < 0.00001, "Failed: ray.dir = {:?}, view_vector = {:?}, theta = {}", ray.direction, view_vector, theta);
        }

        #[test]
        fn camera_width(cam in arb_camera(),
                        y in 0.0f64..1.0f64) {
            let left_most = cam.ray_at(0.0, y);
            let right_most = cam.ray_at(1.0, y);
            let calc_width_sq = (left_most.origin - right_most.origin).magnitude_squared();
            let actual_width_sq = cam.width.powi(2);
            prop_assert!(abs_diff_eq!(calc_width_sq, actual_width_sq, epsilon=0.00001),
            "calculated = {}, actual = {}", calc_width_sq, actual_width_sq);
        }

        #[test]
        fn camera_height(cam in arb_camera(),
                         x in 0.0f64..1.0f64) {
            let top_most = cam.ray_at(x, 1.0);
            let bottom_most = cam.ray_at(x, 0.0);
            prop_assert!(abs_diff_eq!(
                    (top_most.origin - bottom_most.origin).magnitude_squared(),
                    cam.height.powi(2),
                    epsilon=0.000001));
        }

        #[test]
        fn horizontal_view_angle_is_fov(cam in arb_camera()) {
            let left_most = cam.ray_at(0.0, 0.5);
            let right_most = cam.ray_at(1.0, 0.5);

            let theta = left_most.direction.angle(&right_most.direction);
            prop_assert!(abs_diff_eq!(theta, cam.fov, epsilon=0.00001), "theta = {:?}", theta);
        }
    }
}
