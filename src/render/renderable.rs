use super::material::Material;
use super::ray::Ray;
use super::shape::{DiffGeom, Shape};
use na::{Scalar, Vector3};

/// This trait defines what the requirements to be renderable.
/// This is essentially a combination of the `Shape` and `Material` traits
// TODO reimplement to establish connection to shape and material
pub trait Renderable {
    type NumTy: Scalar;
    type BSDF_fn;
    fn intersection(&self, ray: &Ray<Self::NumTy>) -> Option<Self::NumTy>;
    fn normal(&self, point: &Vector3<Self::NumTy>) -> Vector3<Self::NumTy>;
    fn get_bsdf(&self, g: &DiffGeom<Self::NumTy>) -> Self::BSDF_fn;
}

pub struct ShapeMat<S, M> {
    shape: S,
    material: M,
}

impl<S, M> ShapeMat<S, M> {
    pub fn new(shape: S, material: M) -> ShapeMat<S, M> {
        ShapeMat { shape, material }
    }
}

impl<S: Shape, M: Material<NumTy = S::NumTy>> Renderable for ShapeMat<S, M> {
    type NumTy = S::NumTy;
    type BSDF_fn = M::BSDF_fn;

    fn intersection(&self, ray: &Ray<Self::NumTy>) -> Option<Self::NumTy> {
        self.shape.intersection(ray)
    }

    fn normal(&self, point: &Vector3<Self::NumTy>) -> Vector3<Self::NumTy> {
        self.shape.normal(point)
    }

    fn get_bsdf(&self, g: &DiffGeom<Self::NumTy>) -> Self::BSDF_fn {
        self.material.get_bsdf(g)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::triangle::Triangle;
    use crate::number::*;
    use super::super::material::{Lambert, UniformMaterial};

    fn shapemat_is_renderable() {
        fn is_renderable<R: Renderable>() { };
        is_renderable::<ShapeMat<Triangle<f64>, UniformMaterial<Lambert<f64>>>>();
    }

    // fn shapemat_ref_is_renderable() {
    //     fn is_renderable<R: Renderable>() { };
    //     is_renderable::<&ShapeMat<Triangle<f64>, UniformMaterial<Lambert<f64>>>>();
    // }

    fn uniform_is_material() {
        fn is_material<M: Material>() { };
        is_material::<UniformMaterial<Lambert<f64>>>();
    }
}
