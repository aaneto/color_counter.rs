use image::GenericImageView;
use color_processing::Color;

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

pub mod tests;
pub mod region;
pub mod space;
pub mod constants;