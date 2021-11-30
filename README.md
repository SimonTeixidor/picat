# picat - picture cat
Picat converts images to sixel graphics, and outputs them to stdout. The tool
achieves high quality output by making use of the
[libimagequant](https://pngquant.org/lib/) library for converting RGBA images
to the sixel palette.

Not all terminals support sixels. I have personally tested picat in xterm on
Xorg and foot on Wayland.

![screenshot of picat](scrot.png)

# Installation
Picat requires a rust toolchain. Installation is done using:
```
cargo install picat
```

# Usage
```
USAGE:
    picat [OPTIONS] [<FILE>...]

    If FILE is omitted, picat reads a single image from stdin.

OPTIONS:
    -w, --width <pixels>    Output image width in pixels
    -h, --help              Display this help page
```
