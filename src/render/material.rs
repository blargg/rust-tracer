extern crate cgmath;

use super::color::Rgb;
use super::shape::DiffGeom;
use cgmath::{BaseFloat, InnerSpace, Vector3};

// Defines the type of scattering functions used for lighting materials.
// Defines how light is reflected by the surface for queried light and view vectors
pub trait BSDF {
    type NumTy;
    /// Defines how light is reflected of the surface at an assumed point and orientation.
    /// Vectors will be oriented to the reflection space. That means this can assume the normal is
    /// unit vector in the positive Z direction <0, 0, 1>
    ///
    /// # Arguments
    /// * `view`: the vector of the view, a narrow slice of the light emited from a point in that
    /// direction.
    /// * `light`: the vector of the incoming light.
    ///
    /// # Returns
    /// Returns the ratio at which light will be reflected.
    fn bsdf(&self, view: &Vector3<Self::NumTy>, light: &Vector3<Self::NumTy>) -> Rgb<Self::NumTy>;
}

pub trait Material {
    // the number type to use
    type NumTy;
    // the BSDF to return
    type BSDF_fn;

    fn get_bsdf(&self, g: &DiffGeom<Self::NumTy>) -> Self::BSDF_fn;
}

#[derive(Clone)]
pub struct Lambert<T> {
    color: Rgb<T>,
}

impl<T> Lambert<T> {
    pub fn new(red: T, green: T, blue: T) -> Lambert<T> {
        Lambert {
            color: Rgb::new(red, green, blue),
        }
    }
}

impl<T: BaseFloat> BSDF for Lambert<T> {
    type NumTy = T;
    fn bsdf(&self, view: &Vector3<Self::NumTy>, light: &Vector3<Self::NumTy>) -> Rgb<Self::NumTy> {
        let cos = view.dot(*light) / (view.magnitude() * light.magnitude());
        self.color.clone() * cos
    }
}

pub struct UniformMaterial<T> {
    bsdf: T,
}

impl<T> UniformMaterial<T> {
    pub fn new(bsdf: T) -> UniformMaterial<T> {
        UniformMaterial { bsdf }
    }
}

impl<B: BSDF + Clone> Material for UniformMaterial<B> {
    type NumTy = B::NumTy;
    type BSDF_fn = B;

    fn get_bsdf(&self, _g: &DiffGeom<Self::NumTy>) -> Self::BSDF_fn {
        self.bsdf.clone()
    }
}
