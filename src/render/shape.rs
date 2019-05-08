use na::{Point3, Scalar, Vector3};

use super::ray::Ray;

pub trait Shape {
    type NumTy: Scalar;
    /// Returns where the ray intersects the given shape, or None if no intersection exists.
    /// If there are multiple intersections, this should return the closest point
    ///
    /// The value returned is the scalar on the direction of the ray to reach to the intersection
    /// point. That is `intersection_point = ray.time_at(return_value)` is the actual point value
    /// of the intersection.
    fn intersection(&self, ray: &Ray<Self::NumTy>) -> Option<Self::NumTy>;

    /// Returns a normal for the shape on a given surface point.
    /// This does not need to be meaningful for points that cannot be intersection points.
    fn normal(&self, point: &Point3<Self::NumTy>) -> Vector3<Self::NumTy>;
}

pub struct DiffGeom<T: Scalar> {
    position: Point3<T>,
    normal: Vector3<T>,
}

impl<T: Scalar> DiffGeom<T> {
    pub fn new(position: Point3<T>, normal: Vector3<T>) -> DiffGeom<T> {
        DiffGeom { position, normal }
    }
}
