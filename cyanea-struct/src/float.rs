//! `f64` methods that need `std` on stable Rust.
//!
//! Without the `std` feature, importing [`FloatExt`] gives `f64` these methods
//! backed by `libm`. With `std`, the inherent methods are used and this module
//! is empty, so std builds compute exactly what they did before.

#[cfg(not(feature = "std"))]
#[allow(dead_code)] // unused when another crate in the build links std
pub(crate) trait FloatExt {
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, x: Self) -> Self;
}

#[cfg(not(feature = "std"))]
impl FloatExt for f64 {
    fn sqrt(self) -> f64 {
        libm::sqrt(self)
    }
    fn sin(self) -> f64 {
        libm::sin(self)
    }
    fn cos(self) -> f64 {
        libm::cos(self)
    }
    fn acos(self) -> f64 {
        libm::acos(self)
    }
    fn atan(self) -> f64 {
        libm::atan(self)
    }
    fn atan2(self, x: f64) -> f64 {
        libm::atan2(self, x)
    }
}
