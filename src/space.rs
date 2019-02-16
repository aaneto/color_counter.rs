//! The Space is the color space itself, containing many
//! colors in their respective regions, the space is
//! responsible for holding and sorting this data so that
//! frequent colors are easy to extract.

use color_processing::Color;
use std::collections::HashMap;

use crate::constants::*;
use crate::get_colors;
use crate::region::Region;

/// The Space struct is a linear space subdivided
/// in many regions intended to segregate colors
/// spatially.
pub struct Space {
    regions: Vec<Region>,
    num_regions: usize,
    region_size: usize,
}

impl Space {
    /// Returns the index of the region that this color belongs to.
    pub fn region_idx(&self, color: Color) -> usize {
        let laba = color.get_laba();
        let index_transform = self.region_size as f64;

        let l_idx = (index_transform * (laba.0 - L_START) / L_RANGE).floor() as usize;
        let a_idx = (index_transform * (laba.1 - A_START) / A_RANGE).floor() as usize;
        let b_idx = (index_transform * (laba.2 - B_START) / B_RANGE).floor() as usize;

        l_idx
            + a_idx * (index_transform as usize)
            + b_idx * (index_transform as usize) * (index_transform as usize)
    }

    /// Create a new sorted space from a image file.
    pub fn from_file(filepath: &str, region_percentage: f64) -> Self {
        let colors = get_colors(filepath);
        let region_size = (1.0 / region_percentage) as usize;
        let num_regions = region_size * region_size * region_size;

        let mut space = Space {
            regions: Vec::new(),
            num_regions,
            region_size,
        };

        let mut regions_counter: Vec<HashMap<(u8, u8, u8), usize>> = Vec::new();

        for _ in 0..space.num_regions {
            regions_counter.push(HashMap::new());
        }

        // Count colors using a hashmap.
        for color in colors {
            let idx = space.region_idx(color);
            let key = (color.red, color.green, color.blue);

            *regions_counter[idx].entry(key).or_insert(0) += 1;
        }

        // Collect this hashmap into many regions
        for region in regions_counter.iter().filter(|r| !r.is_empty()) {
            let mut new_region = Region::new();

            for (value, count) in region.iter() {
                new_region.push(Color::new_rgb(value.0, value.1, value.2), *count);
            }

            new_region
                .data
                .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            space.regions.push(new_region);
        }

        // sort regions using their most frequent color as a key.
        space.regions.sort_by(|a, b| {
            b.data[0]
                .1
                .partial_cmp(&a.data[0].1)
                .expect("Cannot compare empty region")
        });

        space
    }

    /// Get an iterator for all the non-empty regions of the space.
    pub fn regions_iter(&self) -> impl Iterator<Item = &Region> {
        self.regions.iter().filter(|r| !r.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::Space;

    #[test]
    /// Test correct color frequency detection for sample image
    fn test_color_freq() {
        let space = Space::from_file("forest.jpg", 0.1);

        let expected_data = vec![48, 48, 31, 28, 27, 25, 25, 20, 18, 17];

        for (i, region) in space.regions_iter().take(10).enumerate() {
            let count = region.data[0].1;
            // Only count is tested because there may be a slight variation
            // on the actual rgb data.
            assert!(expected_data[i] == count);
        }
    }
}
