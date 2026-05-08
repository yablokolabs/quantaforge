pub use num_complex::Complex64;

pub const ZERO: Complex64 = Complex64::new(0.0, 0.0);
pub const ONE: Complex64 = Complex64::new(1.0, 0.0);
pub const I: Complex64 = Complex64::new(0.0, 1.0);

pub fn approx_eq(a: Complex64, b: Complex64, eps: f64) -> bool {
    (a - b).norm() < eps
}
