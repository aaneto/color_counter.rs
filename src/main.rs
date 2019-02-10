use colortest::space::Space;

fn main() {
    let space = Space::from_file("forest.jpg", 0.1);
    

    for region in space.regions_iter().take(10) {
        let color = region.data[0].0;
        let count = region.data[0].1;
        
        println!("Color: {} Frequency: {}", color.to_rgb_string(), count);
    }
}
