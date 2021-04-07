# Rusty Slider

A small tool to display markdown files as a slideshow.

## Demo

Try out [Rusty Slider online](https://ollej.github.io/rusty-slider/demo/).

## Usage

The file `slides.md` will be read and split into slides on
horizontal lines: `---`

Use left and right arrow keys or left and right mouse button to move
back and forth between slides.

## Theme

Create a file called `theme.json` to modify default display values.

### Example theme.json

```json
{
    "background_image": "background.png",
    "background_color": "#753204",
    "heading_color": "#8f4d22",
    "text_color": "#cccccc",
    "font": "Amble-Regular.ttf",
    "font_size_header": 80,
    "font_size_text": 40,
    "vertical_offset": 20.0,
    "line_height": 2.0,
    "shader": true
}
```

## License

Copyright 2021 Olle Wreede, released under the MIT License.
