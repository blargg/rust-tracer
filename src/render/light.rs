extern crate nalgebra as na;
use na::{Scalar, Vector3};

use super::color::*;

pub struct PointLight<T: Scalar> {
    position: Vector3<T>,
    color: Rgb<T>,
}
