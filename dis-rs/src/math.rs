pub use self::fns::*;

#[cfg(feature = "std")]
mod fns {
    /// Arccosine (f64)
    ///
    /// Computes the inverse cosine (arc cosine) of the input value.
    /// Arguments must be in the range -1 to 1.
    /// Returns values in radians, in the range of 0 to pi.
    #[inline]
    pub fn acos(x: f64) -> f64 {
        x.acos()
    }

    /// Arcsine (f64)
    ///
    /// Computes the inverse sine (arc sine) of the argument `x`.
    /// Arguments to asin must be in the range -1 to 1.
    /// Returns values in radians, in the range of -pi/2 to pi/2.
    #[inline]
    pub fn asin(x: f64) -> f64 {
        x.asin()
    }

    /// Arctangent of y/x (f64)
    ///
    /// Computes the inverse tangent (arc tangent) of `y/x`.
    /// Produces the correct result even for angles near pi/2 or -pi/2 (that is, when `x` is near 0).
    /// Returns a value in radians, in the range of -pi to pi.
    #[inline]
    pub fn atan2(y: f64, x: f64) -> f64 {
        y.atan2(x)
    }

    /// The cosine of `x` (f64).
    ///
    /// `x` is specified in radians.
    #[inline]
    pub fn cos(x: f64) -> f64 {
        x.cos()
    }

    /// Round `x` to the nearest integer, breaking ties away from zero.
    #[inline]
    pub fn round(x: f64) -> f64 {
        x.round()
    }

    /// The sine of `x` (f64).
    ///
    /// `x` is specified in radians.
    #[inline]
    pub fn sin(x: f64) -> f64 {
        x.sin()
    }

    /// The square root of `x` (f64).
    #[inline]
    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }
}

#[cfg(not(feature = "std"))]
mod fns {
    pub use libm::{acos, asin, atan2, cos, round, sin, sqrt};
}
