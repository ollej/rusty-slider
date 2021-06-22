# Rusty Slider

---

## About

A small tool to display markdown files as a slideshow.

Use right key or left mouse button to go to next slide.

---

## Markdown

* Slides are written in markdown.
* Supports headers, paragraphs, lists, blockquotes and code blocks.
* Text between horizontal lines will be a slide.

---

## Theme

Colors and fonts can be configured in a json file.

A background image can also be defined in the theme file.

---

## Rust + Macroquad

Developed with Rust and the macroquad game library.

---

## Cross-platform

Supports Windows, MacOS, Linux and web.

---

## Code

Code blocks will be rendered with syntax highlighting.

```rust
let shader_material = load_material(
	shaders::crt::VERTEX,
	shaders::crt::FRAGMENT,
	Default::default(),
)
.unwrap();
```

---

## Run code!

Execute `bash` code blocks by pressing `enter`.

```bash
echo "Hello, World!"
```

---

## Blockquotes

> Blockquotes renders with background color
>
> *And fancy quotes.*

---

## Controls

Left/right keys switches between previous and next slide.

Escape quits the slideshow.

Space toggles the shader.

---

## Usage

1. Create a markdown file called `slides.md` in `assets` directory.
1. Optionally add a `theme.json` in `assets` directory.

---

## Possible improvements

Add transitions.

Support image syntax to set backround image of slide.

---

## License

**Copyright 2021 Olle Wreede**

Released under the MIT license.
