# Rusty Slider
[![Cross-compile](https://github.com/ollej/rusty-slider/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/ollej/rusty-slider/actions/workflows/rust.yml)

A small tool to display markdown files as a slideshow.

![Screenshot](https://ollej.github.io/rusty-slider/assets/screenshot.png)

## Demo

Try out Rusty Slider online:

* [Example slideshows](https://ollej.github.io/rusty-slider/demo/example-slideshows.html).

## Download

Rusty Slider is available for multiple platforms, such as Windows, 
Linux, MacOS, and the web. Download the latest binary build from github:

[https://github.com/ollej/rusty-slider/releases/](https://github.com/ollej/rusty-slider/releases/)

## Usage

The file `assets/rusty-slider.md` will be read and split into slides on
horizontal lines: `---`

At the moment, the markdown supported is headers, paragraphs,
code blocks, blockquotes, and simple lists. Emphasis and strong are
supported if the theme has italic and bold fonts.

Heading level 1 can be used as title page, as it will render in the
middle of the slide and can have a larger font size set by the theme
option `font_size_header_title`.

You may use html comments (`<!-- ... -->`) in the markdown for anything you
don't want to be shown.

### Shortcuts

Use left and right arrow keys or left and right mouse button to move
back and forth between slides.

The `S` key saves the current slide as a PNG on disk.

Use the key `Q` or `Escape` to exit the slideshow.

### Command line options

Use flag `--automatic` when starting the application to automatically switch
slide every N seconds.

### Run code blocks

When the command line flag `--enable-code-execution` is used, it is possible
to run shell code in code blocks and show the result.

Code blocks marked with `bash`, `sh`, or `python` can be executed by pressing
the `enter` key.  The output will be added in a new code block at the bottom
of the slide.

This feature only works when running locally on a machine that can run shell
scripts. Be careful when using this as there is no checks done on the shell
script.

Only the first bash code block on a slide can be executed.

## Theme

Create a file called `assets/default-theme.json` to modify default display values.

If you make your own theme file, and want to share it, I'd be happy
to add it to the release.

### Example theme.json

```json
{
    "background_image": "assets/background.png",
    "background_color": "#753204",
    "heading_color": "#8f4d22",
    "text_color": "#cccccc",
    "align": "right",
    "font": "assets/Amble-Regular.ttf",
    "font_bold": "assets/Amble-Bold.ttf",
    "font_italic": "assets/Amble-Italic.ttf",
    "font_size_header_title": 100,
    "font_size_header_slides": 80,
    "font_size_text": 40,
    "vertical_offset": 20.0,
    "horizontal_offset": 100.0,
    "line_height": 2.0,
    "blockquote_background_color": "#333333",
    "blockquote_padding": 20.0,
    "blockquote_left_quote": "“",
    "blockquote_right_quote": "„",
    "code_font": "assets/Hack-Regular.ttf",
    "code_font_size": 20,
    "code_line_height": 1.2,
    "code_background_color": "#002b36",
    "code_theme": "Solarized (dark)",
    "code_tab_width": 2,
    "bullet": "• ",
    "shader": true
}
```

## Command line options

The command line options can also be used as URL arguments to the
web demo.

```
rusty-slider 0.11.0
A small tool to display markdown files as a slideshow.

USAGE:
    rusty-slider [OPTIONS]

FLAGS:
        --enable-code-execution    Enable executing code in code blocks
    -h, --help                     Prints help information
    -V, --version                  Prints version information

OPTIONS:
    -a, --automatic <automatic>      Automatically switch slides every N seconds [default: 0]
    -S, --screenshot <screenshot>    When taking screenshot, store PNG at this path [default: screenshot.png]
    -s, --slides <slides>            Markdown files with slides text [default: assets/rusty-slider.md]
    -t, --theme <theme>              File with theme options [default: assets/default-theme.json]
```

## License

Copyright 2021 Olle Wreede, released under the MIT License.

Fonts are copyrighted by their respective designers.
