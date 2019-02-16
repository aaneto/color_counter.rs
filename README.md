# Color Counter

A rust project for getting the color distribution of one image.

The code works by dividing the colors into a cube on the CIELAB color space. Later those "regions"
are filled with colors, which are ordered by frequency on that region, the code then returns those
regions sorted by their most frequent color, and the colors on the regions are sorted by frequency also.

## Using

Divide the color space in regions of 10% in each dimension (1000 regions in total).
Display the first 2 regions sorted by their most frequent color, and print the
2 most frequent colors of those regions. Use the file forest.jpg.

```bash

cargo run -- -r 2 -c 2 -s 0.1 -f forest.jpg

Region 1
Color 1: rgb(142, 146, 113) Frequency: 48
Color 2: rgb(138, 142, 109) Frequency: 36

Region 2
Color 1: rgb(145, 149, 116) Frequency: 48
Color 2: rgb(148, 152, 119) Frequency: 46
```

To see the options:
```bash
cargo run -- --help
```

# Todo
- Adjust algorithm for populating the regions (what is the probability of EVERY region having a color?)
- Better error handling
- Better tests
- Add documentation