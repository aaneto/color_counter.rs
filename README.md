# Color Test

A rust project for getting colors off an image.

The code works by dividing the color into a cube on the CIELAB color space. Later those "regions"
are filled with colors, which are ordered by frequency on that region, the code then returns those
regions sorted by their most frequent color, and the colors on the regions are sorted by frequency also.