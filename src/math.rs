//! An ultra-light private math library to make our short lives easier as we
//! implement super-complex noise stuff.

use num_traits::float::FloatCore;

pub(crate) mod interpolate;
pub(crate) mod s_curve;
pub(crate) mod vectors;

#[cfg(not(any(not(feature = "std"), target_os = "emscripten")))]
#[inline]
pub(crate) fn scale_shift(value: f64, n: f64) -> f64 {
    value.abs().mul_add(n, -1.0_f64)
}

#[cfg(any(not(feature = "std"), target_os = "emscripten"))]
#[inline]
pub(crate) fn scale_shift(value: f64, n: f64) -> f64 {
    (value.abs() * n) + -1.0_f64
}
