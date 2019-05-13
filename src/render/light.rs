use na::{Point3, Scalar};

use super::spectrum::*;

pub struct PointLight<T: Scalar> {
    pub position: Point3<T>,
    pub color: Spec<T>,
}
