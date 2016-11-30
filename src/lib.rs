//!BASIC VECTO	R2D IMPLEMETATION FOR USE WITHIN VARIOUS CODINGGAME PROJECTS, ETC TODO: DOCUMENT
//!TODO: DOCUMENT THIS STUFF; also add more tests
pub mod vec2d;
pub use vec2d::Vec2D;
#[cfg(test)]
mod tests;

/// should perform modula arithmtic with wrapping (negatives wrap around to positive)
/// #Examples
/// ```
///		use phys2d::sane_mod;
/// 	assert_eq!(sane_mod(5f64, 4f64), 1f64);
/// 	assert_eq!(sane_mod(-6f64, 4f64), 2f64);
/// ```
pub fn sane_mod(a: f64, b: f64) -> f64 {
    a - (a/b).floor() * b
}
