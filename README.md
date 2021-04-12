# Rusty Slider
[![Cross-compile](https://github.com/ollej/rusty-slider/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/ollej/rusty-slider/actions/workflows/rust.yml)
[![Release](https://github.com/ollej/rusty-slider/actions/workflows/release.yml/badge.svg?event=release)](https://github.com/ollej/rusty-slider/actions/workflows/release.yml)

A small tool to display markdown files as a slideshow.

![Screenshot](https://ollej.github.io/rusty-slider/assets/screenshot.png)

## Demo

Try out Rusty Slider online:

* [Default theme](https://ollej.github.io/rusty-slider/demo/).
* [Rusty theme](https://ollej.github.io/rusty-slider/demo/?theme=assets/rusty.json).

## Download

Rusty Slider is available for multiple platforms, such as Windows, 
Linux, and MacOS. Download the latest binary build from github:

[https://github.com/ollej/rusty-slider/releases/](https://github.com/ollej/rusty-slider/releases/)

## Usage

The file `slides.md` will be read and split into slides on
horizontal lines: `---`

At the moment, the markdown supported is headers, paragraphs,
code blocks, and simple lists. Emphasis and strong will be ignored.

Heading level 1 can be used as title page, as it will render in the
middle of the slide and can have a larger font size set by the theme
option `font_size_header_title`.

Use left and right arrow keys or left and right mouse button to move
back and forth between slides.

Use flag `automatic` to automatically switch slide every N seconds.

## Theme

Create a file called `assets/theme.json` to modify default display values.

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
    "font_size_header_title": 100,
    "font_size_header_slides": 80,
    "font_size_text": 40,
    "vertical_offset": 20.0,
    "line_height": 2.0,
    "code_font": "assets/Hack-Regular.ttf",
    "code_font_size": 20,
    "code_line_height": 1.2,
    "code_background_color": "#002b36",
    "code_theme": "Solarized (dark)",
    "code_tab_width": 2,
    "bullet": "•",
    "shader": true
}
```

## Command line options

The command line options can also be used as URL arguments to the
web demo.

```
rusty-slider 0.4.0
A small tool to display markdown files as a slideshow.

USAGE:
rusty-slider [OPTIONS]

FLAGS:
-h, --help       Prints help information
-V, --version    Prints version information

OPTIONS:
-a, --automatic <automatic>    Automatically switch slides every N seconds [default: 0]
-s, --slides <slides>          Markdown files with slides text [default: assets/slides.md]
-t, --theme <theme>            File with theme options [default: assets/theme.json]
```

## License

Copyright 2021 Olle Wreede, released under the MIT License.

Fonts are copyrighted by their respective designers.
