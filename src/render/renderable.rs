use super::material::Material;
use super::ray::Ray;
use super::shape::{DiffGeom, Shape};

/// This trait defines what the requirements to be renderable.
/// This is essentially a combination of the `Shape` and `Material` traits
pub trait Renderable {
    type NumTy;
    type BSDF_fn;
    fn intersection(&self, ray: &Ray<Self::NumTy>) -> Option<Self::NumTy>;
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

    fn get_bsdf(&self, g: &DiffGeom<Self::NumTy>) -> Self::BSDF_fn {
        self.material.get_bsdf(g)
    }
}
