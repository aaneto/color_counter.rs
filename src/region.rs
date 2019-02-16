//! The region is a fraction of the color space,
//! designed as a bucket to segregate colors into
//! different groups.

use color_processing::Color;

/// The struct itself is just a data placeholder,
/// it has meaning only in context.
#[derive(Default)]
pub struct Region {
    pub data: Vec<(Color, usize)>,
}

impl Region {
    /// Create new empty region.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Return the average color of this region.
    ///
    /// Note that this is not the average color of this region
    /// in space, but of the colors effectively contained in
    /// this region data.
    pub fn average_color(&self) -> Color {
        let sum_color = self.data.iter().fold((0.0, 0.0, 0.0), |sums, x| {
            let x = x.0.get_laba();

            (sums.0 + x.0, sums.1 + x.1, sums.2 + x.2)
        });

        let average_color = (
            sum_color.0 / self.data.len() as f64,
            sum_color.1 / self.data.len() as f64,
            sum_color.2 / self.data.len() as f64,
        );

        Color::new_lab(average_color.0, average_color.1, average_color.2)
    }

    /// Push a new color with count into data holder.
    pub fn push(&mut self, color: Color, count: usize) {
        self.data.push((color, count))
    }

    /// Wrapper for vec.is_empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
