use na::{Point3, Scalar};

use super::color::*;

pub struct PointLight<T: Scalar> {
    pub position: Point3<T>,
    pub color: Rgb<T>,
}
