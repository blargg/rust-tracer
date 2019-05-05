use alga::general::*;

/// Trait defines a number of mathematical properties that are commonly needed.
pub trait GenFloat: RealField + ComplexField<RealField = Self> {}

impl GenFloat for f32 {}
impl GenFloat for f64 {}
