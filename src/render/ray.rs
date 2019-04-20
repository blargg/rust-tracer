extern crate cgmath;

use cgmath::Vector3;

pub struct Ray<T> {
    pub origin: Vector3<T>,
    pub direction: Vector3<T>,
}

pub fn new<T> (orig: Vector3<T>, dir: Vector3<T>) -> Ray<T> {
    Ray { origin: orig, direction: dir }
}
