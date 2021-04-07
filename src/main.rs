extern crate markdown;
use colorsys::Rgb;
use macroquad::prelude::*;
use markdown::{Block, Span};
use nanoserde::DeJson;

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
}

impl Slides {
    const HEADER_SIZE: u16 = 80;
    const TEXT_SIZE: u16 = 40;
    const START_POSITION: f32 = 20.;
    const LINE_HEIGHT: f32 = 1.2;

    fn from_slides(slides: Vec<Slide>, theme: Theme, font: Font) -> Slides {
        Slides {
            slides,
            active_slide: 0,
            theme,
            font,
        }
    }

    async fn load(path: String, theme: Theme, font: Font) -> Self {
        let markdown = load_string(&path).await.unwrap();
        let tokens = markdown::tokenize(&markdown);
        let slides = Self::build_slides(tokens);
        Self::from_slides(slides, theme, font)
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
        self.draw_slide(Self::START_POSITION);
    }

    fn draw_slide(&self, start_position: f32) {
        let slide = &self.slides[self.active_slide];
        let mut new_position = start_position;
        for block in slide.content.iter() {
            new_position = match block {
                Block::Header(spans, size) => self.draw_header(spans, *size, new_position),
                Block::Paragraph(spans) => self.draw_paragraph(spans, new_position),
                Block::UnorderedList(_items) => 0.,
                _ => 0.,
            }
        }
    }

    fn draw_header(&self, spans: &Vec<Span>, size: usize, position: f32) -> f32 {
        let size = Self::HEADER_SIZE - (size as u16 * 2);
        let new_position = position + size as f32 * Self::LINE_HEIGHT;
        for span in spans.iter() {
            match span {
                Span::Text(text) => {
                    self.draw_text(text, self.theme.heading_color(), size, new_position)
                }
                _ => (),
            }
        }
        return new_position;
    }

    fn draw_paragraph(&self, spans: &Vec<Span>, position: f32) -> f32 {
        let size = Self::TEXT_SIZE;
        let new_position = position + size as f32 * Self::LINE_HEIGHT;
        for span in spans.iter() {
            match span {
                Span::Text(text) => {
                    self.draw_text(text, self.theme.text_color(), size, new_position)
                }
                _ => (),
            }
        }
        return new_position;
    }

    fn draw_text(&self, text: &String, color: Color, font_size: u16, position: f32) {
        let text_params = TextParams {
            font: self.font,
            font_size: font_size,
            font_scale: 1.,
            color: color,
        };
        //debug!("pos: {} text: {}", position, text);
        let dimensions = measure_text(text, Some(self.font), font_size, 1.);
        let hpos = screen_width() / 2. - dimensions.width / 2.;
        draw_text_ex(text, hpos, position, text_params);
    }
}

#[derive(Clone, DeJson)]
#[nserde(default)]
pub struct Theme {
    pub background_color: String,
    pub heading_color: String,
    pub text_color: String,
    pub font: String,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            background_color: "#301934".to_string(),
            heading_color: "#b19cd9".to_string(),
            text_color: "#ffffff".to_string(),
            font: "Amble-Regular.ttf".to_string(),
        }
    }
}

impl Theme {
    async fn load() -> Self {
        match load_string("theme.json").await {
            Ok(json) => DeJson::deserialize_json(&json).unwrap(),
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

fn window_conf() -> Conf {
    Conf {
        window_title: "Rmdslider".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let theme = Theme::load().await;
    debug!(
        "background_color: {:?} text_color: {:?} heading_color{:?}",
        theme.background_color(),
        theme.text_color(),
        theme.heading_color(),
    );
    let font = load_ttf_font(&theme.font).await;
    let mut slides = Slides::load("slides.md".to_string(), theme, font).await;

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

        slides.draw();

        next_frame().await
    }
}
