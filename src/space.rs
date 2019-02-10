use std::collections::HashMap;
use color_processing::Color;

use crate::get_colors;
use crate::constants::*;
use crate::region::Region;

/// The Space struct is a linear space subdivided in many regions
/// every region is a Region struct with the data inside this region
/// and a counter for frequency counting.
/// 
/// The idea is to insert data in a spacially dependent way.
pub struct Space {
    regions: Vec<Region>,
    num_regions: usize,
    region_size: usize,
}

impl Space {
    /// Returns the index of the color in the space.
    pub fn region_idx(&self, color: Color) -> usize {
        let laba = color.get_laba();
        let index_transform = self.region_size as f64;

        let l_idx = (index_transform * (laba.0 - L_START) / L_RANGE).floor() as usize;
        let a_idx = (index_transform * (laba.1 - A_START) / A_RANGE).floor() as usize;
        let b_idx = (index_transform * (laba.2 - B_START) / B_RANGE).floor() as usize;

        l_idx + a_idx * (index_transform as usize) + b_idx * (index_transform as usize) * (index_transform as usize)
    }

    /// Create a new space from a file, reading it's colors
    /// and saving them on the space sorted by frequency.
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

        for color in colors {
            let idx = space.region_idx(color);
            let key = (color.red, color.green, color.blue);

            *regions_counter[idx].entry(key).or_insert(0) += 1;
        }

        for region in regions_counter.iter().filter(|r| !r.is_empty()) {
            let mut new_region = Region::new();

            for (value, count) in region.iter() {
                new_region.data.push((Color::new_rgb(value.0, value.1, value.2), *count));
            }
            
            new_region.data.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            space.regions.push(new_region);
        }

        space.regions.sort_by(|a, b| {
            b.data[0].1.partial_cmp(&a.data[0].1).expect("Cannot compare empty region")
        });

        space
    }

    /// Get an iterator for all the non-empty regions of the space.
    pub fn regions_iter(&self) -> impl Iterator<Item = &Region> {
        self.regions.iter().filter(|r| !r.is_empty())
    }
}