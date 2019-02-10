use color_processing::Color;

/// The region struct is just a data holder
/// for the elements of specified color region
#[derive(Default)]
pub struct Region {
    pub data: Vec<(Color, usize)>,
}

impl Region {
    pub fn new() -> Self {
        Self {
            data: Vec::new()
        }
    }

    /// Return the average color of the colors
    /// on this color region.
    pub fn average_color(&self) -> Color {
        let sum_color = self.data.iter().fold((0.0, 0.0, 0.0), |sums, x| {
            let x = x.0.get_laba();

            (sums.0 + x.0, sums.1 + x.1, sums.2 + x.2)
        });

        let average_color = (
            sum_color.0 / self.data.len() as f64,
            sum_color.1 / self.data.len() as f64,
            sum_color.2 / self.data.len() as f64
        );

        Color::new_lab(average_color.0, average_color.1, average_color.2)
    }

    pub fn push(&mut self, color: Color) {
        self.data.push((color, 1))
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}