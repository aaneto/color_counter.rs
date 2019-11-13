//! The color counter crate is a CLI tool
//! to open an image and count it's pixel colors
//! and divide this count by color region.
//!
//! The Space is the color space in its entirety,
//! the Region is a piece of the color space.
//!
//! There is also a constants module, to define the boundaries
//! of the color space, this is used to divide the color space
//! and put every color on its 'bucket'.
#![deny(missing_docs)]

use color_processing::Color;
use image::GenericImageView;

/// Read an image file and store all pixels
/// as Color's on a Vec.
fn colors_from_file(filepath: &str) -> Vec<Color> {
    let image = image::open(filepath).unwrap();

    image
        .pixels()
        .map(|(_x, _y, color)| Color::new_rgba(color[0], color[1], color[2], color[3]))
        .collect()
}

/// Read an byte array of a image file and store
/// all pixels in a Vec as Color's.
pub fn colors_from_bytes(bytes: Vec<u8>) -> Vec<Color> {
    let img = image::load_from_memory(&bytes).expect("Cannot load image from bytes.");

    img.pixels()
        .map(|(_x, _y, color)| Color::new_rgba(color[0], color[1], color[2], color[3]))
        .collect()
}

pub mod constants;
pub mod region;
pub mod space;

pub use color_processing;
pub use image;

/// Prelude module with usable reexports and common modules.
pub mod prelude {
    pub use crate::region::Region;
    pub use crate::space::Space;
    pub use color_processing::Color;
    pub use image::GenericImageView;
}
