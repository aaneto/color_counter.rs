# Color Counter

A project for getting the color distribution of one image.

The current version works by dividing the CIELAB color space linearly and creating "regions" of colors sorted by frequency.

## Using
This command will divide the color space in 10 regions (10% of the dimension each), with 1000 regions in total, display the first 2 regions sorted by their most frequent color, and print the 2 most frequent colors of those regions, using the file forest.jpg.

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

## Future Work
After tinkering around with this projects, I decided to call it done, all
the improvements that can be made are related to testing and error handling.

While doing this I realised that there is a whole thing about this kind of color
extraction, such as color quantization, since this crate is called color counter,
I believe it's not meant to use those many more efficient algorithms.

So, if I ever come with different implementations, it will probably be made into
a different crate (color quantize, maybe?).
