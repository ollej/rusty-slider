# Rusty Slider
[![Cross-compile](https://github.com/ollej/rusty-slider/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/ollej/rusty-slider/actions/workflows/rust.yml)
[![Release](https://github.com/ollej/rusty-slider/actions/workflows/release.yml/badge.svg?event=release)](https://github.com/ollej/rusty-slider/actions/workflows/release.yml)

A small tool to display markdown files as a slideshow.

![Screenshot](https://ollej.github.io/rusty-slider/assets/screenshot.png)

## Demo

Try out [Rusty Slider online](https://ollej.github.io/rusty-slider/demo/).

## Download

Rusty Slider is available for multiple platforms, such as Windows, 
Linuxi, and MacOS. Download the latest binary build from github:

[https://github.com/ollej/rusty-slider/releases/](https://github.com/ollej/rusty-slider/releases/)

## Usage

The file `slides.md` will be read and split into slides on
horizontal lines: `---`

At the moment, the only markdown supported is headers, paragraphs and
code blocks.

Use left and right arrow keys or left and right mouse button to move
back and forth between slides.

## Theme

Create a file called `theme.json` to modify default display values.

If you make your own theme file, and want to share it, I'd be happy
to add it to the release.

### Example theme.json

```json
{
    "background_image": "assets/background.png",
    "background_color": "#753204",
    "heading_color": "#8f4d22",
    "text_color": "#cccccc",
    "font": "assets/Amble-Regular.ttf",
    "font_size_header": 80,
    "font_size_text": 40,
    "vertical_offset": 20.0,
    "line_height": 2.0,
    "code_font": "assets/Hack-Regular.ttf",
    "code_font_size": 20,
    "code_line_height": 1.2,
    "code_background_color": "#002b36",
    "bullet": "•",
    "shader": true
}
```

## Command line options

Use `--slides` to set path to a markdown file with slides.

Use `--theme` to set path to a theme json file.

```
rusty-slider 0.1.0
A small tool to display markdown files as a slideshow.

USAGE:
    rusty-slider [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --slides <slides>    Markdown files with slides text, defaults to slides.md
    -t, --theme <theme>      File with theme options, defaults to theme.json
```

## License

Copyright 2021 Olle Wreede, released under the MIT License.

Fonts are copyrighted by their respective designers.
