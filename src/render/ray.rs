extern crate cgmath;

use cgmath::Vector3;

pub struct Ray<T> {
    pub origin: Vector3<T>,
    pub direction: Vector3<T>,
}

impl<T> Ray<T> {
    pub const fn new(orig: Vector3<T>, dir: Vector3<T>) -> Ray<T> {
        Ray { origin: orig, direction: dir }
    }
}
