use na::{Scalar, Vector3};

use super::ray::Ray;

pub trait Shape {
    type NumTy: Scalar;
    fn intersection(&self, ray: &Ray<Self::NumTy>) -> Option<Self::NumTy>;
}

pub struct DiffGeom<T: Scalar> {
    position: Vector3<T>,
    normal: Vector3<T>,
}
