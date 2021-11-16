use core::ops::{Add, Mul, Sub};

/// Performs linear interpolation between two values.
#[cfg(not(any(not(feature = "std"), target_os = "emscripten")))]
#[inline]
pub(crate) fn linear<T>(a: T, b: T, x: T) -> T
where
    T: num_traits::MulAdd<Output = T> + Sub<Output = T> + Copy,
{
    x.mul_add(b - a, a)
}

/// Performs linear interpolation between two values.
#[cfg(any(not(feature = "std"), target_os = "emscripten"))]
#[inline]
pub(crate) fn linear<T>(a: T, b: T, x: T) -> T
where
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    (x * (b - a)) + a
}

/// Performs cubic interpolation between two values bound between two other
/// values.
///
/// - n0 - The value before the first value.
/// - n1 - The first value.
/// - n2 - The second value.
/// - n3 - The value after the second value.
/// - alpha - The alpha value.
///
/// The alpha value should range from 0.0 to 1.0. If the alpha value is
/// 0.0, this function returns _n1_. If the alpha value is 1.0, this
/// function returns _n2_.
#[inline]
pub(crate) fn cubic<T>(n0: T, n1: T, n2: T, n3: T, alpha: T) -> T
where
    T: Add<Output = T> + Mul<Output = T> + Sub<Output = T> + Copy,
{
    let p = (n3 - n2) - (n0 - n1);
    let q = (n0 - n1) - p;
    let r = n2 - n0;
    let s = n1;
    p * alpha * alpha * alpha + q * alpha * alpha + r * alpha + s
}
