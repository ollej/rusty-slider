extern crate markdown;
pub mod shaders;

use colorsys::Rgb;
use macroquad::prelude::*;
use macroquad::texture::Image;
use markdown::{Block, Span};
use nanoserde::DeJson;
use std::path::PathBuf;
use structopt::StructOpt;

use silicon::formatter::ImageFormatterBuilder;
use silicon::utils::init_syntect;
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;

struct Slide {
    content: Vec<Block>,
}

impl Slide {
    fn new(content: Vec<Block>) -> Slide {
        Slide { content }
    }
}

struct Slides {
    slides: Vec<Slide>,
    active_slide: usize,
    theme: Theme,
    font: Font,
    background: Option<Texture2D>,
}

impl Slides {
    fn from_slides(
        slides: Vec<Slide>,
        theme: Theme,
        font: Font,
        background: Option<Texture2D>,
    ) -> Slides {
        Slides {
            slides,
            active_slide: 0,
            theme,
            font,
            background,
        }
    }

    async fn load(
        slides_path: Option<PathBuf>,
        theme: Theme,
        font: Font,
        background: Option<Texture2D>,
    ) -> Self {
        let path = match slides_path {
            Some(p) => p.as_path().to_str().unwrap().to_owned(),
            None => "slides.md".to_string(),
        };
        let markdown = match load_string(&path).await {
            Ok(tokens) => tokens,
            Err(_) => {
                eprintln!("Couldn't parse markdown document: {}", path);
                std::process::exit(1);
            }
        };
        let tokens = markdown::tokenize(&markdown);
        let slides = Self::build_slides(tokens);
        Self::from_slides(slides, theme, font, background)
    }

    fn build_slides(tokens: Vec<Block>) -> Vec<Slide> {
        let mut slides: Vec<Slide> = vec![];
        let mut content: Vec<Block> = vec![];
        for block in tokens.iter() {
            debug!("{:?}", block);
            match block {
                Block::Hr => {
                    slides.push(Slide::new(content));
                    content = vec![];
                }
                _ => content.push(block.to_owned()),
            }
        }
        if content.len() > 0 {
            slides.push(Slide::new(content));
        }
        return slides;
    }

    fn next(&mut self) {
        if self.active_slide < (self.slides.len() - 1) {
            self.active_slide += 1;
        }
    }

    fn prev(&mut self) {
        if self.active_slide > 0 {
            self.active_slide -= 1;
        }
    }

    fn draw(&mut self) {
        clear_background(self.theme.background_color());
        self.draw_background(self.background);
        self.draw_slide(self.theme.vertical_offset);
    }

    fn draw_background(&self, background: Option<Texture2D>) {
        match background {
            Some(texture) => draw_texture_ex(
                texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            ),
            None => (),
        };
    }

    fn draw_slide(&self, start_position: f32) {
        let slide = &self.slides[self.active_slide];
        let mut new_position = start_position;
        for block in slide.content.iter() {
            new_position = match block {
                Block::Header(spans, size) => self.draw_header(spans, size - 1, new_position),
                Block::Paragraph(spans) => self.draw_paragraph(spans, new_position),
                Block::CodeBlock(language, code) => self.draw_code(language, code, new_position),
                Block::UnorderedList(_items) => 0.,
                _ => 0.,
            }
        }
    }

    fn draw_header(&self, spans: &Vec<Span>, header_size: usize, position: f32) -> f32 {
        let font_size = self.theme.font_size_header - (header_size as u16 * 2);
        let mut new_position = position;
        for span in spans.iter() {
            match span {
                Span::Text(text) => {
                    new_position =
                        self.draw_text(text, self.theme.heading_color(), font_size, new_position)
                }
                _ => (),
            }
        }
        new_position
    }

    fn draw_paragraph(&self, spans: &Vec<Span>, position: f32) -> f32 {
        let font_size = self.theme.font_size_text;
        let mut new_position = position;
        for span in spans.iter() {
            match span {
                Span::Text(text) => {
                    new_position =
                        self.draw_text(text, self.theme.text_color(), font_size, new_position)
                }
                _ => (),
            }
        }
        new_position
    }

    fn draw_text(&self, text: &String, color: Color, font_size: u16, position: f32) -> f32 {
        let text_params = TextParams {
            font: self.font,
            font_size: font_size,
            font_scale: 1.,
            color: color,
        };
        let dimensions = measure_text(text, Some(self.font), font_size, 1.);
        let hpos = screen_width() / 2. - dimensions.width / 2.;
        let vpos = position + font_size as f32 * self.theme.line_height;
        //debug!(
        //    "font_size: {}, position: {} hpos: {} vpos: {} height: {} offest_y: {} text: {}",
        //    font_size, position, hpos, vpos, dimensions.height, dimensions.offset_y, text
        //);
        draw_text_ex(text, hpos, vpos, text_params);
        vpos
    }

    fn draw_code(&self, language: &Option<String>, code: &String, position: f32) -> f32 {
        let code_texture = CodeImage::new(language, code).texture();
        let font_size = self.theme.font_size_text;
        let vpos = position + font_size as f32;
        draw_texture_ex(
            code_texture,
            screen_width() / 2. - code_texture.width() / 2.,
            vpos,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );
        vpos + code_texture.height()
    }
}

struct CodeImage {
    pub language: Option<String>,
    pub code: String,
}

impl CodeImage {
    fn new(language: &Option<String>, code: &String) -> CodeImage {
        CodeImage {
            language: language.to_owned(),
            code: code.to_owned(),
        }
    }

    fn texture(&self) -> Texture2D {
        let (ps, ts) = init_syntect();
        let syntax = match &self.language {
            Some(language) => ps.find_syntax_by_token(&language),
            None => ps.find_syntax_by_first_line(&self.code),
        }
        .unwrap_or_else(|| ps.find_syntax_plain_text());
        let theme = &ts.themes["Dracula"];
        let mut h = HighlightLines::new(syntax, &theme);
        let highlight = LinesWithEndings::from(&self.code)
            .map(|line| h.highlight(line, &ps))
            .collect::<Vec<_>>();
        let mut formatter = ImageFormatterBuilder::new()
            .font(vec![("Hack", 26.0)])
            .build()
            .unwrap();
        let image_buffer = formatter.format(&highlight, &theme).into_rgba8();
        let image = Image {
            width: image_buffer.width() as u16,
            height: image_buffer.height() as u16,
            bytes: image_buffer.into_raw(),
        };
        load_texture_from_image(&image)
    }
}

#[derive(Clone, DeJson)]
#[nserde(default)]
pub struct Theme {
    pub background_image: Option<String>,
    pub background_color: String,
    pub heading_color: String,
    pub text_color: String,
    pub font: String,
    pub font_size_header: u16,
    pub font_size_text: u16,
    pub vertical_offset: f32,
    pub line_height: f32,
    pub shader: bool,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            background_image: None,
            background_color: "#301934".to_string(),
            heading_color: "#b19cd9".to_string(),
            text_color: "#ffffff".to_string(),
            font: "Amble-Regular.ttf".to_string(),
            font_size_header: 80,
            font_size_text: 40,
            vertical_offset: 20.0,
            line_height: 2.0,
            shader: true,
        }
    }
}

impl Theme {
    async fn load(theme_path: Option<PathBuf>) -> Self {
        let path = match theme_path {
            Some(p) => p.as_path().to_str().unwrap().to_owned(),
            None => "theme.json".to_string(),
        };
        match load_string(&path).await {
            Ok(json) => match DeJson::deserialize_json(&json) {
                Ok(theme) => theme,
                Err(_) => {
                    eprintln!("Couldn't parse theme file: {}", path);
                    std::process::exit(2);
                }
            },
            Err(_) => Theme::default(),
        }
    }

    fn background_color(&self) -> Color {
        Self::from_hex(self.background_color.to_owned(), PURPLE)
    }

    fn heading_color(&self) -> Color {
        Self::from_hex(self.heading_color.to_owned(), WHITE)
    }

    fn text_color(&self) -> Color {
        Self::from_hex(self.text_color.to_owned(), WHITE)
    }

    fn from_hex(color: String, default: Color) -> Color {
        match Rgb::from_hex_str(&color) {
            Ok(rgb) => Color::new(
                rgb.red() as f32 / 255.,
                rgb.green() as f32 / 255.,
                rgb.blue() as f32 / 255.,
                1.,
            ),
            Err(_) => default,
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "rusty-slider",
    about = "A small tool to display markdown files as a slideshow."
)]
struct CliOptions {
    /// Markdown files with slides text, defaults to slides.md
    #[structopt(short, long, parse(from_os_str))]
    pub slides: Option<PathBuf>,
    /// File with theme options, defaults to theme.json
    #[structopt(short, long, parse(from_os_str))]
    pub theme: Option<PathBuf>,
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Slider".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let opt = CliOptions::from_args();
    let theme = Theme::load(opt.theme).await;
    debug!(
        "background_color: {:?} text_color: {:?} heading_color{:?}",
        theme.background_color(),
        theme.text_color(),
        theme.heading_color(),
    );
    let font = load_ttf_font(&theme.font).await;
    let background = match &theme.background_image {
        Some(path) => Some(load_texture(&path).await),
        None => None,
    };
    let mut shader_activated = theme.shader;
    let mut slides = Slides::load(opt.slides, theme, font, background).await;

    let render_target = render_target(screen_width() as u32, screen_height() as u32);
    set_texture_filter(render_target.texture, FilterMode::Linear);
    let shader_material = load_material(
        shaders::crt::VERTEX,
        shaders::crt::FRAGMENT,
        Default::default(),
    )
    .unwrap();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            return;
        }
        if is_key_pressed(KeyCode::Left) || is_mouse_button_pressed(MouseButton::Right) {
            slides.prev();
        }
        if is_key_pressed(KeyCode::Right) || is_mouse_button_pressed(MouseButton::Left) {
            slides.next();
        }
        if is_key_pressed(KeyCode::Space) {
            shader_activated = !shader_activated;
        }

        let scr_w = screen_width();
        let scr_h = screen_height();

        // build camera with following coordinate system:
        // (0., 0)     .... (SCR_W, 0.)
        // (0., SCR_H) .... (SCR_W, SCR_H)
        if shader_activated {
            set_camera(Camera2D {
                zoom: vec2(1. / scr_w * 2., -1. / scr_h * 2.),
                target: vec2(scr_w / 2., scr_h / 2.),
                render_target: Some(render_target),
                ..Default::default()
            });

            slides.draw();

            set_default_camera();

            clear_background(BLACK);
            gl_use_material(shader_material);
            draw_texture_ex(
                render_target.texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    flip_y: true,
                    ..Default::default()
                },
            );
            gl_use_default_material();
        } else {
            set_default_camera();
            slides.draw();
        }

        next_frame().await
    }
}
