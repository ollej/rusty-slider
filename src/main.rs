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

fn horizontal_position(width: f32, horizontal_offset: f32, align: &String) -> f32 {
    match align.as_str() {
        "left" => horizontal_offset,
        "right" => screen_width() - horizontal_offset - width,
        _ => screen_width() / 2. - width / 2.,
    }
}

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
    time: f32,
    automatic: f32,
    theme: Theme,
    font: Font,
    bold_font: Font,
    italic_font: Font,
    code_font: Font,
    background: Option<Texture2D>,
    code_blocks: CodeBlocks,
}

impl Slides {
    fn from_slides(
        slides: Vec<Slide>,
        theme: Theme,
        automatic: f32,
        font: Font,
        bold_font: Font,
        italic_font: Font,
        code_font: Font,
        background: Option<Texture2D>,
    ) -> Slides {
        let code_blocks = CodeBlocks::new(
            code_font,
            theme.code_font_size.to_owned(),
            theme.code_line_height.to_owned(),
            theme.code_background_color().to_owned(),
            theme.code_theme.to_owned(),
            theme.code_tab_width,
            theme.horizontal_offset,
            theme.align.to_owned(),
        );
        Slides {
            slides,
            active_slide: 0,
            automatic,
            time: 0.,
            theme,
            font,
            bold_font,
            italic_font,
            code_font,
            background,
            code_blocks,
        }
    }

    async fn load(
        slides_path: PathBuf,
        theme: Theme,
        automatic: u32,
        font: Font,
        bold_font: Font,
        italic_font: Font,
        code_font: Font,
        background: Option<Texture2D>,
    ) -> Self {
        let path = slides_path.as_path().to_str().unwrap().to_owned();
        let markdown = match load_string(&path).await {
            Ok(tokens) => tokens,
            Err(_) => {
                eprintln!("Couldn't parse markdown document: {}", path);
                std::process::exit(1);
            }
        };
        let tokens = markdown::tokenize(&markdown);
        let slides = Self::build_slides(tokens);
        Self::from_slides(
            slides,
            theme,
            automatic as f32,
            font,
            bold_font,
            italic_font,
            code_font,
            background,
        )
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
            self.time = 0.;
            self.active_slide += 1;
        }
    }

    fn prev(&mut self) {
        if self.active_slide > 0 {
            self.time = 0.;
            self.active_slide -= 1;
        }
    }

    fn draw(&mut self, delta: f32) {
        if self.automatic > 0. && self.time > self.automatic {
            self.next();
        } else {
            self.time += delta;
        }
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
        // TODO: Doesn't support codeblock
        // TODO: Title position not in middle vertically
        // TODO: code blocks should be changed to TextBox
        let text_boxes = self.blocks_to_text_boxes(&slide.content, None, TextBoxStyle::Standard);
        let mut new_position: f32 = start_position;
        for text_box in text_boxes {
            let hpos = self.horizontal_position(text_box.width_with_padding());
            new_position = text_box.draw(hpos, new_position);
        }
    }

    fn blocks_to_text_boxes(
        &self,
        blocks: &Vec<Block>,
        background_color: Option<Color>,
        style: TextBoxStyle,
    ) -> Vec<TextBox> {
        let mut text_boxes = vec![];
        let mut text_lines = vec![];
        for block in blocks.iter() {
            match block {
                Block::Header(spans, 1) => {
                    text_lines.push(TextLine::new(
                        self.theme.align.to_owned(),
                        self.spans_to_text_partials(
                            spans,
                            self.font,
                            self.theme.font_size_header_title,
                            self.theme.heading_color(),
                        ),
                    ));
                }
                Block::Header(spans, _size) => {
                    text_lines.push(TextLine::new(
                        self.theme.align.to_owned(),
                        self.spans_to_text_partials(
                            spans,
                            self.font,
                            self.theme.font_size_header_slides,
                            self.theme.heading_color(),
                        ),
                    ));
                }
                Block::Paragraph(spans) => {
                    text_lines.push(TextLine::new(
                        self.theme.align.to_owned(),
                        self.spans_to_text_partials(
                            spans,
                            self.font,
                            self.theme.font_size_text,
                            self.theme.text_color(),
                        ),
                    ));
                }
                Block::UnorderedList(items) => {
                    text_lines.extend(self.build_list_box(items, Some(&self.theme.bullet)));
                }
                Block::OrderedList(items, _) => {
                    text_lines.extend(self.build_list_box(items, None));
                }
                Block::Blockquote(blocks) => {
                    if text_lines.len() > 0 {
                        text_boxes.push(TextBox::new(
                            text_lines,
                            self.theme.vertical_offset,
                            background_color,
                            style,
                        ));
                    }
                    text_boxes.extend(self.blocks_to_text_boxes(
                        blocks,
                        Some(self.theme.blockquote_background_color()),
                        TextBoxStyle::Blockquote {
                            size: self.theme.font_size_header_title * 2,
                            font: self.font,
                            color: self.theme.text_color(),
                        },
                    ));
                    text_lines = Vec::new();
                }
                _ => (),
            }
        }
        if text_lines.len() > 0 {
            text_boxes.push(TextBox::new(
                text_lines,
                self.theme.vertical_offset,
                background_color,
                style,
            ));
        }
        text_boxes
    }

    fn spans_to_text_partials(
        &self,
        spans: &Vec<Span>,
        font: Font,
        font_size: u16,
        color: Color,
    ) -> Vec<TextPartial> {
        let mut partials = vec![];
        // TODO: Text with only newline should start new line
        for span in spans.iter() {
            match span {
                Span::Text(text) => partials.push(TextPartial::new(
                    &text,
                    font,
                    font_size,
                    color,
                    self.theme.line_height,
                )),
                Span::Code(text) => partials.push(TextPartial::new(
                    &text,
                    self.code_font,
                    font_size,
                    self.theme.text_color(), // TODO: Add code text color to theme
                    self.theme.line_height,
                )),
                Span::Emphasis(spans) => partials.extend(self.spans_to_text_partials(
                    spans,
                    self.italic_font,
                    font_size,
                    color,
                )),
                Span::Strong(spans) => partials.extend(self.spans_to_text_partials(
                    spans,
                    self.bold_font,
                    font_size,
                    color,
                )),
                _ => (),
            };
        }
        partials
    }

    fn build_list_box(&self, items: &Vec<ListItem>, bullet: Option<&String>) -> Vec<TextLine> {
        let mut lines: Vec<TextLine> = vec![];
        for (index, item) in items.iter().enumerate() {
            match item {
                ListItem::Simple(spans) => {
                    let mut partials = vec![];
                    partials.push(self.build_bullet_partial(index, bullet));
                    partials.extend(self.spans_to_text_partials(
                        spans,
                        self.font,
                        self.theme.font_size_text,
                        self.theme.text_color(),
                    ));
                    let text_line = TextLine::new("left".to_string(), partials);
                    lines.push(text_line);
                }
                _ => (),
            };
        }
        lines
    }

    fn build_bullet_partial(&self, index: usize, bullet: Option<&String>) -> TextPartial {
        let item_bullet = match bullet {
            Some(b) => b.to_owned(),
            None => format!("{}. ", index + 1),
        };
        TextPartial::new(
            &item_bullet,
            self.font,
            self.theme.font_size_text,
            self.theme.text_color(),
            self.theme.line_height,
        )
    }

    fn horizontal_position(&self, width: f32) -> f32 {
        horizontal_position(width, self.theme.horizontal_offset, &self.theme.align)
    }
}
#[derive(Copy, Clone)]
pub enum TextBoxStyle {
    Standard,
    Blockquote { size: u16, font: Font, color: Color },
    Code,
}

impl TextBoxStyle {
    fn draw(&self, hpos: f32, vpos: f32, text_box: &TextBox) {
        match self {
            TextBoxStyle::Blockquote { size, font, color } => {
                self.draw_blockquote(hpos, vpos, text_box, *size, *font, *color)
            }
            _ => (),
        }
    }

    fn draw_blockquote(
        &self,
        hpos: f32,
        vpos: f32,
        text_box: &TextBox,
        size: u16,
        font: Font,
        color: Color,
    ) {
        let text_params = TextParams {
            font: font,
            font_size: size,
            font_scale: 1.,
            color: color,
        };
        let dimensions = measure_text("“", Some(font), size, 1.);
        draw_text_ex(
            "“",
            hpos - dimensions.width,
            vpos + size as f32,
            text_params,
        );
        draw_text_ex(
            "„",
            hpos + text_box.width_with_padding(),
            vpos + text_box.height_with_margin(),
            text_params,
        );
    }
}

struct TextBox {
    width: f32,
    height: f32,
    margin: f32,
    padding: f32,
    offset_y: f32,
    background_color: Option<Color>,
    style: TextBoxStyle,
    lines: Vec<TextLine>,
}

impl TextBox {
    const BOX_PADDING: f32 = 20.;

    fn new(
        lines: Vec<TextLine>,
        margin: f32,
        background_color: Option<Color>,
        style: TextBoxStyle,
    ) -> Self {
        let mut width: f32 = 0.;
        let mut height: f32 = 0.;
        let mut offset_y: f32 = 0.;
        for line in lines.iter() {
            width = width.max(line.width);
            offset_y = offset_y.max(line.offset_y);
            height += line.height;
        }
        Self {
            width,
            height,
            margin,
            padding: Self::BOX_PADDING,
            offset_y,
            background_color,
            style,
            lines,
        }
    }

    fn draw(&self, hpos: f32, vpos: f32) -> f32 {
        self.draw_background(hpos, vpos + self.margin + self.offset_y);
        self.draw_style(hpos, vpos);
        let inner_hpos = hpos + self.padding;
        let mut new_position = vpos + self.padding + self.margin;
        for line in self.lines.iter() {
            let line_hpos = match line.align.as_str() {
                "left" => inner_hpos,
                "right" => inner_hpos + self.width - line.width,
                _ => inner_hpos + self.width / 2. - line.width / 2.,
            };
            new_position = line.draw(line_hpos, new_position);
        }
        vpos + self.height_with_margin()
    }

    fn draw_background(&self, hpos: f32, vpos: f32) {
        match self.background_color {
            Some(color) => draw_rectangle(
                hpos,
                vpos,
                self.width_with_padding(),
                self.height_with_padding(),
                color,
            ),
            None => (),
        }
    }

    fn draw_style(&self, hpos: f32, vpos: f32) {
        self.style.draw(hpos, vpos, self);
    }

    fn width_with_padding(&self) -> f32 {
        self.width + self.padding * 2.
    }

    fn height_with_padding(&self) -> f32 {
        self.height + self.padding * 2.
    }

    fn height_with_margin(&self) -> f32 {
        self.height_with_padding() + self.margin
    }
}

struct TextLine {
    width: f32,
    height: f32,
    offset_y: f32,
    align: String,
    partials: Vec<TextPartial>,
}

impl TextLine {
    fn new(align: String, partials: Vec<TextPartial>) -> Self {
        let mut width: f32 = 0.;
        let mut height: f32 = 0.;
        let mut offset_y: f32 = 0.;
        for partial in &partials {
            width += partial.width;
            height = height.max(partial.height);
            offset_y = offset_y.max(partial.offset_y);
        }
        Self {
            width,
            height,
            offset_y,
            align,
            partials,
        }
    }

    fn draw(&self, start_hpos: f32, vpos: f32) -> f32 {
        let mut hpos = start_hpos;
        for partial in &self.partials {
            hpos = partial.draw(hpos, vpos);
        }
        vpos + self.height
    }
}

struct TextPartial {
    width: f32,
    height: f32,
    color: Color,
    font: Font,
    font_size: u16,
    offset_y: f32,
    text: String,
}

impl TextPartial {
    fn new(text: &String, font: Font, font_size: u16, color: Color, line_height: f32) -> Self {
        let dimensions = measure_text(text, Some(font), font_size, 1.);
        Self {
            width: dimensions.width,
            height: font_size as f32 * line_height,
            color,
            font,
            font_size,
            offset_y: dimensions.offset_y,
            text: text.to_owned(),
        }
    }

    fn draw(&self, hpos: f32, vpos: f32) -> f32 {
        draw_text_ex(
            &self.text,
            hpos,
            vpos + self.height,
            TextParams {
                font: self.font,
                font_size: self.font_size,
                font_scale: 1.,
                color: self.color,
            },
        );
        hpos + self.width
    }
}

struct CodeBlocks {
    ps: SyntaxSet,
    ts: ThemeSet,
    font: Font,
    font_size: u16,
    line_height: f32,
    background_color: Color,
    tab_spaces: String,
    theme: String,
    horizontal_offset: f32,
    align: String,
    blocks: HashMap<String, CodeBlock>,
}

impl CodeBlocks {
    fn new(
        font: Font,
        font_size: u16,
        line_height: f32,
        background_color: Color,
        theme: String,
        tab_width: u8,
        horizontal_offset: f32,
        align: String,
    ) -> Self {
        Self {
            ts: ThemeSet::load_defaults(),
            ps: SyntaxSet::load_defaults_newlines(),
            blocks: HashMap::new(),
            font,
            font_size,
            line_height,
            background_color,
            tab_spaces: " ".repeat(tab_width as usize),
            theme,
            horizontal_offset,
            align,
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
            horizontal_offset: self.horizontal_offset,
            align: self.align.to_owned(),
        }
    }

    fn build_code_box(&self, language: Option<String>, code: String) -> CodeBox {
        let syntax = match language {
            Some(lang) => self.ps.find_syntax_by_token(&lang),
            None => self.ps.find_syntax_by_first_line(&code),
        }
        .unwrap_or_else(|| self.ps.find_syntax_plain_text());
        let theme = &self.ts.themes[&self.theme];
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
                let text = text.trim_end_matches('\n').replace('\t', &self.tab_spaces);
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
    horizontal_offset: f32,
    align: String,
}

impl CodeBlock {
    fn draw(&self) -> f32 {
        let mut hpos = self.horizontal_position();
        let mut vpos = self.start_position + CodeBox::BOX_MARGIN * 2.;
        draw_rectangle(
            hpos,
            vpos,
            self.width(),
            self.height(),
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
        self.start_position + self.code_box.height_with_margin()
    }

    fn horizontal_position(&self) -> f32 {
        horizontal_position(self.width(), self.horizontal_offset, &self.align)
    }

    fn width(&self) -> f32 {
        self.code_box.width_with_padding()
    }

    fn height(&self) -> f32 {
        self.code_box.height_with_padding()
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
        self.height_with_padding() + Self::BOX_MARGIN * 2.
    }
}

#[derive(Clone, DeJson)]
#[nserde(default)]
pub struct Theme {
    pub background_image: Option<String>,
    pub background_color: String,
    pub heading_color: String,
    pub text_color: String,
    pub align: String,
    pub font: String,
    pub font_bold: String,
    pub font_italic: String,
    pub font_size_header_title: u16,
    pub font_size_header_slides: u16,
    pub font_size_text: u16,
    pub vertical_offset: f32,
    pub horizontal_offset: f32,
    pub line_height: f32,
    pub blockquote_background_color: String,
    pub blockquote_padding: f32,
    pub blockquote_left_quote: String,
    pub blockquote_right_quote: String,
    pub code_font: String,
    pub code_font_size: u16,
    pub code_line_height: f32,
    pub code_background_color: String,
    pub code_theme: String,
    pub code_tab_width: u8,
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
            align: "center".to_string(),
            font: "assets/Amble-Regular.ttf".to_string(),
            font_bold: "assets/Amble-Bold.ttf".to_string(),
            font_italic: "assets/Amble-Italic.ttf".to_string(),
            font_size_header_title: 100,
            font_size_header_slides: 80,
            font_size_text: 40,
            vertical_offset: 20.0,
            horizontal_offset: 20.0,
            line_height: 2.0,
            blockquote_background_color: "#333333".to_string(),
            blockquote_padding: 20.,
            blockquote_left_quote: "“".to_string(),
            blockquote_right_quote: "„".to_string(),
            code_font: "assets/Hack-Regular.ttf".to_string(),
            code_font_size: 20,
            code_line_height: 1.2,
            code_background_color: "#002b36".to_string(),
            code_theme: "Solarized (dark)".to_string(),
            code_tab_width: 4,
            bullet: "• ".to_string(),
            shader: true,
        }
    }
}

impl Theme {
    async fn load(theme_path: PathBuf) -> Self {
        let path = theme_path.as_path().to_str().unwrap().to_owned();
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

    fn blockquote_background_color(&self) -> Color {
        Self::from_hex(self.blockquote_background_color.to_owned(), BLACK)
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
    /// Markdown files with slides text.
    #[structopt(short, long, parse(from_os_str), default_value = "assets/slides.md")]
    pub slides: PathBuf,
    /// File with theme options.
    #[structopt(short, long, parse(from_os_str), default_value = "assets/theme.json")]
    pub theme: PathBuf,
    /// Automatically switch slides every N seconds.
    #[structopt(short, long, default_value = "0")]
    pub automatic: u32,
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
    let bold_font = load_ttf_font(&theme.font_bold).await;
    let italic_font = load_ttf_font(&theme.font_italic).await;
    let code_font = load_ttf_font(&theme.code_font).await;
    let background = match &theme.background_image {
        Some(path) => Some(load_texture(&path).await),
        None => None,
    };
    let mut shader_activated = theme.shader;
    let mut slides = Slides::load(
        opt.slides,
        theme,
        opt.automatic,
        text_font,
        bold_font,
        italic_font,
        code_font,
        background,
    )
    .await;

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
        if is_key_pressed(KeyCode::Left)
            || is_key_pressed(KeyCode::H)
            || is_mouse_button_pressed(MouseButton::Right)
        {
            slides.prev();
        }
        if is_key_pressed(KeyCode::Right)
            || is_key_pressed(KeyCode::L)
            || is_mouse_button_pressed(MouseButton::Left)
        {
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

            slides.draw(get_frame_time());

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
            slides.draw(get_frame_time());
        }

        next_frame().await
    }
}
