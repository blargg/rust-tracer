extern crate cgmath;

use cgmath::Vector3;
use super::color::*;

pub struct PointLight<T> {
    position: Vector3<T>,
    color: Rgb<T>,
}
