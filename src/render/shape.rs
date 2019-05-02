extern crate cgmath;

use super::ray::Ray;
use cgmath::*;

pub trait Shape {
    type NumTy;
    fn intersection(&self, ray: &Ray<Self::NumTy>) -> Option<Self::NumTy>;
}

pub struct DiffGeom<T> {
    position: Vector3<T>,
    normal: Vector3<T>,
}
