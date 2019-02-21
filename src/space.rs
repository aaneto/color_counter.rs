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
    region_to_vec: HashMap<usize, usize>,
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

        let mut space = Space {
            regions: Vec::new(),
            region_to_vec: HashMap::new(),
            region_size,
        };

        let mut regions_counter: HashMap<(u8, u8, u8), usize> = HashMap::new();

        // Count colors using a hashmap.
        for color in colors {
            let key = (color.red, color.green, color.blue);

            *regions_counter.entry(key).or_insert(0) += 1;
        }

        for (color, count) in regions_counter {
            let color = Color::new_rgb(color.0, color.1, color.2);

            let region_idx = space.region_idx(color);

            // Get a mutable reference to regions so
            // that it can be sent inside the closure below.
            let regions = &mut space.regions;

            // if region is not on the space, create a vec for it and
            // register the index of the vec in the region_to_vec hash.
            let regions_vec_idx = *space.region_to_vec.entry(region_idx).or_insert_with(|| {
                regions.push(Region::new());

                regions.len() - 1
            });

            space.regions[regions_vec_idx].push(color, count);
        }

        for region in space.regions.iter_mut() {
            region.sort_by_frequency();
        }

        // O(NUM_REGION * SIZE_REGION) on the worst case
        // But we can afford to be expensive, since there
        // are not many cases where all of the elements of
        // a region have the same count.
        space.regions.sort_by(|a, b| {
            let smaller_len = std::cmp::min(a.len(), b.len());
            let mut idx = 0;

            while b[idx].1 == a[idx].1 && idx < smaller_len - 1 {
                idx += 1;
            }

            b[idx].1.partial_cmp(&a[idx].1).unwrap()
        });

        space
    }

    /// Returns an iterator for the regions.
    pub fn regions_iter(&self) -> impl Iterator<Item = &Region> {
        self.regions.iter()
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
            let count = region[0].1;

            assert!(expected_data[i] == count);
        }
    }
}
