#[cfg(test)]
mod tests {
    use crate::space::Space;

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