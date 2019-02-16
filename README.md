# Color Counter

A rust project for getting the color distribution of one image.

The code works by dividing the colors into a cube on the CIELAB color space. Later those "regions"
are filled with colors, which are ordered by frequency on that region, the code then returns those
regions sorted by their most frequent color, and the colors on the regions are sorted by frequency also.
