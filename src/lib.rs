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

use image::GenericImageView;
use color_processing::Color;

/// Open an image in a path 'filepath' and
/// store all it's pixels in a Vec of Colors.
/// 
/// Effectively this just converts the reading of an
/// image by the image crate to a Color in the color_processing
/// crate format.
pub fn get_colors(filepath: &str) -> Vec<Color> {
    let image = image::open(filepath).unwrap();

    image.pixels().map(|pixel| {
        let data = pixel.2.data;
        
        Color::new_rgba(
            data[0],
            data[1],
            data[2],
            data[3]
        )
    }).collect()
}

pub mod region;
pub mod space;
pub mod constants;