//! This module define some useful constants.
//!
//! The constants serve the purpose of easing the
//! division of the color space.
//!
//! Note that these values are aggressively rounded.
//! These coordinates are all based on the CIELAB color
//! space.

/// The starting value of the L coordinate.
pub const L_START: f64 = 0.0;
/// The range of the L coordinate.
pub const L_RANGE: f64 = 100.0;

/// The starting value of the A coordinate.
pub const A_START: f64 = -90.0;
/// The range of the A coordinate.
pub const A_RANGE: f64 = 190.0;

/// The starting value of the B coordinate.
pub const B_START: f64 = -110.0;
/// The range of the B coordinate.
pub const B_RANGE: f64 = 210.0;
