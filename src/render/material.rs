extern crate cgmath;

use cgmath::{Vector3};

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
    fn bsdf(&self, view: &Vector3<Self::NumTy>, light: &Vector3<Self::NumTy>) -> Self::NumTy;
}
