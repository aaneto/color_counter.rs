use clap::{App, Arg};

use color_counter::space::Space;

fn main() {
    let matches = App::new("Color Counter")
        .version("0.1.6")
        .author("Adilson Neto <almeidneto@gmail.com>")
        .about("Count and display most frequent color by color region.")
        .arg(
            Arg::with_name("filename")
                .required(true)
                .short("f")
                .long("filename")
                .help("The image file to open.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("num_regions")
                .short("r")
                .long("num_regions")
                .help("Number of regions to display")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("num_colors")
                .short("c")
                .long("num_colors")
                .help("Number of colors to display by region")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("region_size")
                .short("s")
                .long("region_size")
                .help("The size of the region in % of the color space.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("show_average")
                .short("a")
                .long("show_average")
                .help("Show average color for region?"),
        )
        .get_matches();

    let filename = matches.value_of("filename").expect("Error Processing Args");

    let num_regions: usize = matches
        .value_of("num_regions")
        .unwrap_or("10")
        .parse()
        .unwrap();

    let num_colors: usize = matches
        .value_of("num_colors")
        .unwrap_or("10")
        .parse()
        .unwrap();

    let region_size: f64 = matches
        .value_of("regions_size")
        .unwrap_or("0.1")
        .parse()
        .unwrap();

    let show_average: bool = matches.is_present("show_average");

    let space = Space::from_file(filename, region_size);

    for (i, region) in space.regions_iter().take(num_regions).enumerate() {
        println!("Region {}", i + 1);

        if show_average {
            println!("Average Color: {}", region.average_color().to_rgb_string());
        }

        for (i, (color, count)) in region.iter().take(num_colors).enumerate() {
            println!(
                "Color {}: {} Frequency: {}",
                i + 1,
                color.to_rgb_string(),
                count
            );
        }
        println!();
    }
}
