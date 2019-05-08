use na::{Scalar, Vector3};

use super::color::*;

pub struct PointLight<T: Scalar> {
    pub position: Vector3<T>,
    pub color: Rgb<T>,
}
