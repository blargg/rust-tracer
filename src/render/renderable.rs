pub use super::material::Material;
use super::ray::Ray;
pub use super::shape::{DiffGeom, Shape};
use na::{Point3, Vector3};

/// This trait defines what the requirements to be renderable.
/// This is essentially a combination of the `Shape` and `Material` traits
pub trait Renderable: Shape + Material<NumTy = <Self as Shape>::NumTy> { }

pub struct ShapeMat<S, M> {
    shape: S,
    material: M,
}

impl<S, M> ShapeMat<S, M> {
    pub fn new(shape: S, material: M) -> ShapeMat<S, M> {
        ShapeMat { shape, material }
    }
}

impl<S: Shape, M> Shape for ShapeMat<S, M> {
    type NumTy = S::NumTy;

    fn intersection(&self, ray: &Ray<Self::NumTy>) -> Option<Self::NumTy> {
        self.shape.intersection(ray)
    }

    fn normal(&self, point: &Point3<Self::NumTy>) -> Vector3<Self::NumTy> {
        self.shape.normal(point)
    }
}

impl<S: Shape, M: Material<NumTy = <Self as Shape>::NumTy>> Material for ShapeMat<S, M> {
    type NumTy = <Self as Shape>::NumTy;
    type BSDF_fn = M::BSDF_fn;

    fn get_bsdf(&self, g: &DiffGeom<Self::NumTy>) -> Self::BSDF_fn {
        self.material.get_bsdf(g)
    }
}

impl<S, M> Renderable for ShapeMat<S, M>
where
    S: Shape,
    M: Material<NumTy = <Self as Shape>::NumTy>,
{}

#[cfg(test)]
mod tests {
    use super::super::material::{Lambert, UniformMaterial};
    use super::super::triangle::Triangle;
    use super::*;
    use crate::number::*;

    fn shapemat_is_renderable() {
        fn is_renderable<R: Renderable>() {};
        is_renderable::<ShapeMat<Triangle<f64>, UniformMaterial<Lambert<f64>>>>();
    }

    fn uniform_is_material() {
        fn is_material<M: Material>() {};
        is_material::<UniformMaterial<Lambert<f64>>>();
    }
}
