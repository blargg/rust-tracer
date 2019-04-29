extern crate cgmath;

use super::ray::Ray;
use cgmath::*;

#[derive(Debug)]
struct Camera<T> {
    // position of the camera
    position: Vector3<T>,
    // (where it is looking at)
    orientation: Basis3<T>,
    width: T,
    height: T,
    fov: Rad<T>,
}

impl<T> Camera<T> {
    fn new(
        position: Vector3<T>,
        orientation: Basis3<T>,
        width: T,
        height: T,
        fov: Rad<T>,
    ) -> Camera<T> {
        Camera {
            position: position,
            orientation: orientation,
            width: width,
            height: height,
            fov: fov,
        }
    }
}

impl<T: BaseFloat> Camera<T> {
    fn look_at(
        position: Vector3<T>,
        at_point: Vector3<T>,
        up: Vector3<T>,
        width: T,
        height: T,
        fov: Rad<T>,
    ) -> Camera<T> {
        let view_direction = at_point - position;
        let orientation = Basis3::look_at(view_direction, up);
        Camera::new(position, orientation, width, height, fov)
    }
}

// generalize this to other floats
impl Camera<f64> {
    // returns a ray at the given coordinates on the camera
    // x and y: [0, 1] are percents of the way across the camera
    fn ray_at(&self, x: f64, y: f64) -> Ray<f64> {
        // point on the unit screen at
        // calculate the focal point behind the sceen
        // draw a ray at the screen with the angle
        // orient and translate the ray
        let off_set: Vector3<f64> = self.orientation.rotate_vector(vec3(
            (x - 0.5) * self.width,
            (y - 0.5) * self.height,
            0.0,
        ));
        let point = off_set + self.position;

        // focal_point lies behind the camera plane, used to determine the ray direction.
        let half_fov = self.fov / 2.0;
        let focal_distance = self.width / (2.0 * half_fov.tan());
        let focal_point =
            self.orientation.rotate_vector(vec3(0.0, 0.0, -1.0)) * focal_distance + self.position;
        Ray::new(point, point - focal_point)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::plane::*;
    use crate::render::ray::tests::{arb_ray, st_vec3};
    use proptest::prelude::*;
    use std::f64::consts::PI;

    #[test]
    fn center_point_at_position() {
        let position = vec3(0.0, 0.0, 0.0);
        let cam: Camera<f64> = Camera::look_at(
            position,
            vec3(0.0, 0.0, 1.0),
            vec3(0.0, 1.0, 0.0),
            1.0,
            1.0,
            Rad(std::f64::consts::PI / 2.0),
        );
        let mid_ray = cam.ray_at(0.5, 0.5);

        assert!(abs_diff_eq!(mid_ray.origin, position));
    }

    prop_compose! {
        fn arb_basis3()
            (v_init in st_vec3(-1.0f64..1.0f64),
             rot in 0.0..1.0f64) -> Basis3<f64>
        {
            let v = if v_init.magnitude() < 0.000001 {
                Vector3::unit_x()
            } else {
                v_init
            };
            let un_normalized: Matrix3<f64> = Matrix3::from_axis_angle(v, Rad(2.0 * PI) * rot);
            Basis3::look_at(un_normalized * Vector3::unit_x(), un_normalized * Vector3::unit_y())
        }
    }

    prop_compose! {

        fn arb_camera()
            (pos in st_vec3(-100.0f64..100.0f64),
             orientation in arb_basis3(),
             width in 1.0f64..100.0,
             height in 1.0f64..100.0) -> Camera<f64>
        {
            Camera::new(pos, orientation, width, height, Rad(PI / 2.0))
        }
    }

    proptest! {
        #[test]
        fn ray_origin_on_view_plane(cam in arb_camera(),
                                    x in 0.0f64..1.0,
                                    y in 0.0f64..1.0) {
            let ray = cam.ray_at(x, y);
            let view_plane = Plane::new_at_point(cam.position, cam.orientation.rotate_vector(Vector3::unit_z() * -1.0));

            prop_assert!(view_plane.distance_to(ray.origin) < 0.00001);
        }

        #[test]
        fn center_ray_views_direction(cam in arb_camera()) {
            let ray = cam.ray_at(0.5, 0.5);
            let view_vector = cam.orientation.rotate_vector(Vector3::unit_z());
            let Rad(theta) = ray.direction.angle(view_vector);

            prop_assert!(theta < 0.00001, "Failed: ray.dir = {:?}, view_vector = {:?}, theta = {}", ray.direction, view_vector, theta);
        }

        #[test]
        fn camera_width(cam in arb_camera(),
                        y in 0.0f64..1.0f64) {
            let left_most = cam.ray_at(0.0, y);
            let right_most = cam.ray_at(1.0, y);
            let calc_width_sq = (left_most.origin - right_most.origin).magnitude2();
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
                    (top_most.origin - bottom_most.origin).magnitude2(),
                    cam.height.powi(2),
                    epsilon=0.000001));
        }

        #[test]
        fn horizontal_view_angle_is_fov(cam in arb_camera()) {
            let left_most = cam.ray_at(0.0, 0.5);
            let right_most = cam.ray_at(1.0, 0.5);

            let theta = left_most.direction.angle(right_most.direction);
            prop_assert!(abs_diff_eq!(theta, cam.fov, epsilon=0.00001), "theta = {:?}", theta);
        }
    }
}
