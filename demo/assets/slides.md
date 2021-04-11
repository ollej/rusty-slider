# Rusty Slider

A small tool to display markdown files as a slideshow.

Use right key or left mouse button to go to next slide.

---

# Markdown

* Slides are written in markdown.
* Supports headers, paragraphs, unordered lists and code blocks.
* Text between horizontal lines will be a slide.

---

# Theme

Colors and fonts can be configured in a json file.

A background image can also be defined in the theme file.

---

# Rust + Macroquad

Developed with Rust and the macroquad game library.

---

# Cross-platform

Supports Windows, MacOS, Linux and web.

---

# Code

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

# Controls

Left/right keys switches between previous and next slide.

Escape quits the slideshow.

Space toggles the shader.

---

# Usage

Create a markdown file called slides.md in `assets` directory.

Optionally add a theme.json in `assets` directory.

---

# TODO

Add transitions.

Support image syntax to set backround image of slide.

