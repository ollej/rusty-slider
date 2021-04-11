extern crate markdown;
pub mod shaders;

use colorsys::Rgb;
use macroquad::prelude::*;
use markdown::{Block, ListItem, Span};
use nanoserde::DeJson;
use quad_url::get_program_parameters;
use std::collections::HashMap;
use std::path::PathBuf;
use structopt::StructOpt;

use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
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
    code_blocks: CodeBlocks,
}

impl Slides {
    fn from_slides(
        slides: Vec<Slide>,
        theme: Theme,
        font: Font,
        code_font: Font,
        background: Option<Texture2D>,
    ) -> Slides {
        let code_blocks = CodeBlocks::new(
            code_font,
            theme.code_font_size.to_owned(),
            theme.code_line_height.to_owned(),
            theme.code_background_color().to_owned(),
        );
        Slides {
            slides,
            active_slide: 0,
            theme,
            font,
            background,
            code_blocks,
        }
    }

    async fn load(
        slides_path: Option<PathBuf>,
        theme: Theme,
        font: Font,
        code_font: Font,
        background: Option<Texture2D>,
    ) -> Self {
        let path = match slides_path {
            Some(p) => p.as_path().to_str().unwrap().to_owned(),
            None => "assets/slides.md".to_string(),
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
        Self::from_slides(slides, theme, font, code_font, background)
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

    fn draw_slide(&mut self, start_position: f32) {
        let slide = &self.slides[self.active_slide];
        let mut new_position = start_position;
        for block in slide.content.iter() {
            new_position = match block {
                Block::Header(spans, size) => self.draw_header(spans, size - 1, new_position),
                Block::Paragraph(spans) => self.draw_paragraph(spans, new_position),
                Block::CodeBlock(language, code) => {
                    match self.code_blocks.get_code_block(
                        language.to_owned(),
                        code.to_owned(),
                        new_position,
                        self.active_slide,
                    ) {
                        Some(code_block) => code_block.draw(),
                        None => new_position,
                    }
                }
                Block::UnorderedList(items) => self.draw_list(items, new_position),
                _ => 0.,
            }
        }
    }

    fn draw_header(&self, spans: &Vec<Span>, header_size: usize, position: f32) -> f32 {
        let font_size = self.theme.font_size_header - (header_size as u16 * 2);
        let text = self.convert_spans(spans);
        self.draw_text(&text, self.theme.heading_color(), font_size, position, None)
    }

    fn draw_paragraph(&self, spans: &Vec<Span>, position: f32) -> f32 {
        let font_size = self.theme.font_size_text;
        let text = self.convert_spans(spans);
        self.draw_text(&text, self.theme.text_color(), font_size, position, None)
    }

    fn draw_list(&self, items: &Vec<ListItem>, position: f32) -> f32 {
        let mut max_width: f32 = 0.;
        let mut list: Vec<String> = vec![];
        for item in items.iter() {
            match item {
                ListItem::Simple(spans) => {
                    let text = format!("{} {}", self.theme.bullet, self.convert_spans(spans));
                    let dimensions =
                        measure_text(&text, Some(self.font), self.theme.font_size_text, 1.);
                    max_width = max_width.max(dimensions.width);
                    list.push(text);
                }
                _ => (),
            };
        }
        let mut new_position = position;
        let hpos = screen_width() / 2. - max_width / 2.;
        for text in list {
            new_position = self.draw_text(
                &text,
                self.theme.text_color(),
                self.theme.font_size_text,
                new_position,
                Some(hpos),
            );
        }
        new_position
    }

    fn draw_text(
        &self,
        text: &String,
        color: Color,
        font_size: u16,
        vposition: f32,
        hposition: Option<f32>,
    ) -> f32 {
        let text_params = TextParams {
            font: self.font,
            font_size: font_size,
            font_scale: 1.,
            color: color,
        };
        let dimensions = measure_text(text, Some(self.font), font_size, 1.);
        let hpos = match hposition {
            Some(pos) => pos,
            None => screen_width() / 2. - dimensions.width / 2.,
        };
        let vpos = vposition + font_size as f32 * self.theme.line_height;
        //debug!(
        //    "font_size: {}, position: {} hpos: {} vpos: {} height: {} offest_y: {} text: {}",
        //    font_size, position, hpos, vpos, dimensions.height, dimensions.offset_y, text
        //);
        draw_text_ex(text, hpos, vpos, text_params);
        vpos
    }

    fn convert_spans(&self, spans: &Vec<Span>) -> String {
        let mut line = "".to_string();
        for span in spans.iter() {
            line = match span {
                Span::Text(text) => format!("{} {}", line, text),
                Span::Code(text) => format!("{} '{}'", line, text),
                Span::Emphasis(spans) => format!("{}{}", line, self.convert_spans(spans)),
                Span::Strong(spans) => format!("{}{}", line, self.convert_spans(spans)),
                _ => line,
            };
        }
        line
    }
}

struct CodeBlocks {
    ps: SyntaxSet,
    ts: ThemeSet,
    font: Font,
    font_size: u16,
    line_height: f32,
    background_color: Color,
    blocks: HashMap<String, CodeBlock>,
}

impl CodeBlocks {
    const SYNTAX_THEME: &'static str = "Solarized (dark)";
    const TAB_SPACES: &'static str = "    ";

    fn new(font: Font, font_size: u16, line_height: f32, background_color: Color) -> Self {
        Self {
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
            blocks: HashMap::new(),
            font,
            font_size,
            line_height,
            background_color,
        }
    }

    fn get_code_block(
        &mut self,
        language: Option<String>,
        code: String,
        start_position: f32,
        slide_number: usize,
    ) -> Option<&CodeBlock> {
        let key = format!("{}-{}", slide_number, start_position);
        if !self.blocks.contains_key(&key) {
            let code_block = self.build_code_block(language, code, start_position);
            self.blocks.insert(key.to_owned(), code_block);
        }
        return self.blocks.get(&key);
    }

    fn build_code_block(
        &self,
        language: Option<String>,
        code: String,
        start_position: f32,
    ) -> CodeBlock {
        let code_box = self.build_code_box(language, code);
        CodeBlock {
            code_box,
            start_position: start_position,
            font: self.font,
            font_size: self.font_size,
            line_height: self.line_height,
            background_color: self.background_color,
        }
    }

    fn build_code_box(&self, language: Option<String>, code: String) -> CodeBox {
        let syntax = match language {
            Some(lang) => self.ps.find_syntax_by_token(&lang),
            None => self.ps.find_syntax_by_first_line(&code),
        }
        .unwrap_or_else(|| self.ps.find_syntax_plain_text());
        let theme = &self.ts.themes[Self::SYNTAX_THEME];
        let mut h = HighlightLines::new(syntax, &theme);
        let lines = LinesWithEndings::from(&code)
            .map(|line| h.highlight(line, &self.ps))
            .collect::<Vec<_>>();

        let mut partials = vec![];
        let mut code_width: f32 = 0.;
        let mut code_height: f32 = 0.;
        let line_height = self.font_size as f32 * self.line_height;
        for (lineno, tokens) in lines.iter().enumerate() {
            let mut line_width: f32 = 0.;
            code_height += line_height;
            for (style, text) in tokens {
                let text = text.trim_end_matches('\n').replace('\t', Self::TAB_SPACES);
                if text.is_empty() {
                    continue;
                }

                let dimensions = measure_text(&text, Some(self.font), self.font_size, 1.);
                let c = style.foreground;

                partials.push((
                    line_width,
                    line_height * (lineno + 1) as f32,
                    Color::from_rgba(c.r, c.g, c.b, c.a),
                    text.to_owned(),
                ));

                line_width += dimensions.width;
                code_width = code_width.max(line_width);
            }
        }

        CodeBox {
            width: code_width,
            height: code_height,
            partials,
        }
    }
}

struct CodeBlock {
    code_box: CodeBox,
    start_position: f32,
    font: Font,
    font_size: u16,
    line_height: f32,
    background_color: Color,
}

impl CodeBlock {
    fn draw(&self) -> f32 {
        let mut hpos = screen_width() / 2. - self.code_box.width_with_padding() / 2.;
        let mut vpos = self.start_position + CodeBox::BOX_MARGIN * 2.;
        let bottom_position = self.start_position + self.code_box.height_with_margin();
        draw_rectangle(
            hpos,
            vpos,
            self.code_box.width_with_padding(),
            self.code_box.height_with_padding(),
            self.background_color,
        );
        hpos += CodeBox::BOX_PADDING;
        vpos += CodeBox::BOX_PADDING - self.font_size as f32 * (self.line_height - 1.);
        for (x, y, color, text) in &self.code_box.partials {
            let text_params = TextParams {
                font: self.font,
                font_size: self.font_size,
                font_scale: 1.,
                color: color.to_owned(),
            };
            draw_text_ex(&text, hpos + x, vpos + y, text_params);
        }
        bottom_position
    }
}

struct CodeBox {
    width: f32,
    height: f32,
    partials: Vec<(f32, f32, Color, String)>,
}

impl CodeBox {
    const BOX_PADDING: f32 = 20.;
    const BOX_MARGIN: f32 = 20.;

    fn width_with_padding(&self) -> f32 {
        self.width + Self::BOX_PADDING * 2.
    }

    fn height_with_padding(&self) -> f32 {
        self.height + Self::BOX_PADDING * 2.
    }

    fn height_with_margin(&self) -> f32 {
        self.height + Self::BOX_PADDING * 2. + Self::BOX_MARGIN * 2.
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
    pub code_font: String,
    pub code_font_size: u16,
    pub code_line_height: f32,
    pub code_background_color: String,
    pub bullet: String,
    pub shader: bool,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            background_image: None,
            background_color: "#301934".to_string(),
            heading_color: "#b19cd9".to_string(),
            text_color: "#ffffff".to_string(),
            font: "assets/Amble-Regular.ttf".to_string(),
            font_size_header: 80,
            font_size_text: 40,
            vertical_offset: 20.0,
            line_height: 2.0,
            code_font: "assets/Hack-Regular.ttf".to_string(),
            code_font_size: 20,
            code_line_height: 1.2,
            code_background_color: "#002b36".to_string(),
            bullet: ".".to_string(),
            shader: true,
        }
    }
}

impl Theme {
    async fn load(theme_path: Option<PathBuf>) -> Self {
        let path = match theme_path {
            Some(p) => p.as_path().to_str().unwrap().to_owned(),
            None => "assets/theme.json".to_string(),
        };
        debug!("Theme path: {}", path);
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

    fn code_background_color(&self) -> Color {
        Self::from_hex(self.code_background_color.to_owned(), BLACK)
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
    /// Markdown files with slides text, defaults to assets/slides.md
    #[structopt(short, long, parse(from_os_str))]
    pub slides: Option<PathBuf>,
    /// File with theme options, defaults to assets/theme.json
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
    let opt = CliOptions::from_iter(get_program_parameters().iter());

    let theme = Theme::load(opt.theme).await;
    debug!(
        "background_color: {:?} text_color: {:?} heading_color{:?}",
        theme.background_color(),
        theme.text_color(),
        theme.heading_color(),
    );
    let text_font = load_ttf_font(&theme.font).await;
    debug!("code_font: {}", theme.code_font);
    let code_font = load_ttf_font(&theme.code_font).await;
    let background = match &theme.background_image {
        Some(path) => Some(load_texture(&path).await),
        None => None,
    };
    let mut shader_activated = theme.shader;
    let mut slides = Slides::load(opt.slides, theme, text_font, code_font, background).await;

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
