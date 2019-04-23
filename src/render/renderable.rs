use super::ray::Ray;

pub trait Renderable {
    type NumTy;
    fn intersection(&self, ray: &Ray<Self::NumTy>) -> bool;
}
