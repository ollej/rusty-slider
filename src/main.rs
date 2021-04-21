extern crate markdown;
pub mod shaders;

use colorsys::Rgb;
use macroquad::prelude::*;
use markdown::{Block, ListItem, Span};
use nanoserde::DeJson;
use quad_url::get_program_parameters;
use std::path::PathBuf;
use structopt::StructOpt;

use syntect::easy::HighlightLines;
use syntect::highlighting::FontStyle;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

type Vpos = f32;
type Hpos = f32;
type Duration = f32;
type Width = f32;
type Height = f32;
type FontSize = u16;

struct Slide {
    text_boxes: Vec<TextBox>,
}

struct Slides {
    slides: Vec<Slide>,
    active_slide: usize,
    time: Duration,
    automatic: Duration,
    background_color: Color,
    background: Option<Texture2D>,
    horizontal_offset: Hpos,
    align: String,
}

impl Slides {
    fn from_markdown(
        markdown: String,
        theme: Theme,
        automatic: Duration,
        font: Font,
        font_bold: Font,
        font_italic: Font,
        code_font: Font,
        background: Option<Texture2D>,
    ) -> Slides {
        let background_color = theme.background_color;
        let horizontal_offset = theme.horizontal_offset;
        let align = theme.align.to_owned();
        let slides =
            MarkdownToSlides::new(theme, font, font_bold, font_italic, code_font).parse(markdown);

        Slides {
            slides,
            active_slide: 0,
            automatic,
            time: 0.,
            background_color,
            background,
            horizontal_offset,
            align,
        }
    }

    async fn load(
        slides_path: PathBuf,
        theme: Theme,
        automatic: Duration,
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
        Self::from_markdown(
            markdown,
            theme,
            automatic,
            font,
            bold_font,
            italic_font,
            code_font,
            background,
        )
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

    fn draw(&mut self, delta: Duration) {
        if self.automatic > 0. && self.time > self.automatic {
            self.next();
        } else {
            self.time += delta;
        }
        clear_background(self.background_color);
        self.draw_background(self.background);
        self.draw_slide();
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

    fn draw_slide(&mut self) {
        let slide = &self.slides[self.active_slide];
        let mut new_position: Vpos = 0.;
        for text_box in slide.text_boxes.iter() {
            let hpos = self.horizontal_position(text_box.width_with_padding());
            new_position = text_box.draw(hpos, new_position);
        }
    }

    fn horizontal_position(&self, width: Width) -> Hpos {
        match self.align.as_str() {
            "left" => self.horizontal_offset,
            "right" => screen_width() - self.horizontal_offset - width,
            _ => screen_width() / 2. - width / 2.,
        }
    }
}

struct MarkdownToSlides {
    theme: Theme,
    font: Font,
    font_bold: Font,
    font_italic: Font,
    code_font: Font,
    code_box_builder: CodeBoxBuilder,
}

impl MarkdownToSlides {
    fn new(theme: Theme, font: Font, font_bold: Font, font_italic: Font, code_font: Font) -> Self {
        let code_box_builder = CodeBoxBuilder::new(
            code_font,
            font_bold,
            font_italic,
            theme.code_font_size.to_owned(),
            theme.code_line_height.to_owned(),
            theme.code_background_color.to_owned(),
            theme.code_theme.to_owned(),
            theme.code_tab_width,
            theme.vertical_offset,
        );
        Self {
            theme,
            font,
            font_bold,
            font_italic,
            code_font,
            code_box_builder,
        }
    }

    fn parse(&self, markdown: String) -> Vec<Slide> {
        let tokens = markdown::tokenize(&markdown);
        let slide_blocks = self.split_tokens_into_slides(tokens);
        self.build_slides(slide_blocks)
    }

    fn split_tokens_into_slides(&self, tokens: Vec<Block>) -> Vec<Vec<Block>> {
        let mut slides: Vec<Vec<Block>> = vec![];
        let mut blocks: Vec<Block> = vec![];
        for block in tokens.iter() {
            debug!("{:?}", block);
            match block {
                Block::Hr => {
                    slides.push(blocks);
                    blocks = vec![];
                }
                _ => blocks.push(block.to_owned()),
            }
        }
        if blocks.len() > 0 {
            slides.push(blocks);
        }
        return slides;
    }

    fn build_slides(&self, slide_blocks: Vec<Vec<Block>>) -> Vec<Slide> {
        let mut slides = vec![];
        for blocks in slide_blocks.iter() {
            slides.push(Slide {
                text_boxes: self.build_slide_boxes(blocks),
            });
        }
        slides
    }

    fn build_slide_boxes(&self, blocks: &Vec<Block>) -> Vec<TextBox> {
        self.blocks_to_text_boxes(blocks, None, TextBoxStyle::Standard)
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
                    if text_lines.len() > 0 {
                        text_boxes.push(TextBox::new(
                            text_lines,
                            self.theme.vertical_offset,
                            background_color,
                            style,
                        ));
                        text_lines = Vec::new();
                    }
                    text_boxes.push(TextBox::new(
                        vec![TextLine::new(
                            self.theme.align.to_owned(),
                            self.spans_to_text_partials(
                                spans,
                                self.font,
                                self.theme.font_size_header_title,
                                self.theme.heading_color,
                            ),
                        )],
                        self.theme.vertical_offset,
                        background_color,
                        TextBoxStyle::Title,
                    ));
                }
                Block::Header(spans, _size) => {
                    text_lines.push(TextLine::new(
                        self.theme.align.to_owned(),
                        self.spans_to_text_partials(
                            spans,
                            self.font,
                            self.theme.font_size_header_slides,
                            self.theme.heading_color,
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
                            self.theme.text_color,
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
                        text_lines = Vec::new();
                    }
                    text_boxes.extend(self.blocks_to_text_boxes(
                        blocks,
                        Some(self.theme.blockquote_background_color),
                        TextBoxStyle::Blockquote {
                            size: self.theme.font_size_header_title * 2,
                            font: self.font,
                            color: self.theme.text_color,
                        },
                    ));
                }
                Block::CodeBlock(language, code) => {
                    if text_lines.len() > 0 {
                        text_boxes.push(TextBox::new(
                            text_lines,
                            self.theme.vertical_offset,
                            background_color,
                            style,
                        ));
                        text_lines = Vec::new();
                    }
                    text_boxes.push(
                        self.code_box_builder
                            .build_text_box(language.to_owned(), code.to_owned()),
                    );
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
        font_size: FontSize,
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
                    self.theme.text_color, // TODO: Add code text color to theme
                    self.theme.line_height,
                )),
                Span::Emphasis(spans) => partials.extend(self.spans_to_text_partials(
                    spans,
                    self.font_italic,
                    font_size,
                    color,
                )),
                Span::Strong(spans) => partials.extend(self.spans_to_text_partials(
                    spans,
                    self.font_bold,
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
                        self.theme.text_color,
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
            self.theme.text_color,
            self.theme.line_height,
        )
    }
}

#[derive(Copy, Clone)]
pub enum TextBoxStyle {
    Standard,
    Title,
    Blockquote {
        size: FontSize,
        font: Font,
        color: Color,
    },
    Code,
}

impl TextBoxStyle {
    fn draw(&self, hpos: Hpos, vpos: Vpos, text_box: &TextBox) {
        match self {
            TextBoxStyle::Blockquote { size, font, color } => {
                self.draw_blockquote(hpos, vpos, text_box, *size, *font, *color)
            }
            _ => (),
        }
    }

    fn top_position(&self, vpos: Vpos, text_box: &TextBox) -> Vpos {
        match self {
            TextBoxStyle::Title => {
                screen_height() / 2.
                    - text_box.height / 2.
                    - text_box.margin
                    - text_box.padding
                    - text_box.offset_y
            }
            _ => vpos,
        }
    }

    fn draw_blockquote(
        &self,
        hpos: Hpos,
        vpos: Vpos,
        text_box: &TextBox,
        size: FontSize,
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
            vpos + size as Vpos,
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
    width: Width,
    height: Height,
    margin: Height,
    padding: f32,
    offset_y: Vpos,
    background_color: Option<Color>,
    style: TextBoxStyle,
    lines: Vec<TextLine>,
}

impl TextBox {
    const BOX_PADDING: f32 = 20.;

    fn new(
        lines: Vec<TextLine>,
        margin: Height,
        background_color: Option<Color>,
        style: TextBoxStyle,
    ) -> Self {
        let mut width: Width = 0.;
        let mut height: Height = 0.;
        let mut offset_y: Vpos = 0.;
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

    fn draw(&self, hpos: Hpos, vpos: Vpos) -> Vpos {
        let vpos = self.style.top_position(vpos, self);
        self.draw_background(hpos, vpos + self.margin + self.offset_y);
        self.style.draw(hpos, vpos, self);
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

    fn draw_background(&self, hpos: Hpos, vpos: Vpos) {
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

    fn width_with_padding(&self) -> Width {
        self.width + self.padding * 2.
    }

    fn height_with_padding(&self) -> Height {
        self.height + self.padding * 2.
    }

    fn height_with_margin(&self) -> Height {
        self.height_with_padding() + self.margin
    }
}

struct TextLine {
    width: Width,
    height: Height,
    offset_y: Vpos,
    align: String,
    partials: Vec<TextPartial>,
}

impl TextLine {
    fn new(align: String, partials: Vec<TextPartial>) -> Self {
        let mut width: Width = 0.;
        let mut height: Height = 0.;
        let mut offset_y: Vpos = 0.;
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

    fn draw(&self, start_hpos: Hpos, vpos: Vpos) -> Vpos {
        let mut hpos = start_hpos;
        for partial in &self.partials {
            hpos = partial.draw(hpos, vpos);
        }
        vpos + self.height
    }
}

struct TextPartial {
    width: Width,
    height: Height,
    color: Color,
    font: Font,
    font_size: FontSize,
    offset_y: Vpos,
    text: String,
}

impl TextPartial {
    fn new(
        text: &String,
        font: Font,
        font_size: FontSize,
        color: Color,
        line_height: Height,
    ) -> Self {
        let dimensions = measure_text(text, Some(font), font_size, 1.);
        Self {
            width: dimensions.width,
            height: font_size as Height * line_height,
            color,
            font,
            font_size,
            offset_y: dimensions.offset_y,
            text: text.to_owned(),
        }
    }

    fn draw(&self, hpos: Hpos, vpos: Vpos) -> Vpos {
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

struct CodeBoxBuilder {
    ps: SyntaxSet,
    ts: ThemeSet,
    font: Font,
    font_bold: Font,
    font_italic: Font,
    font_size: FontSize,
    line_height: Height,
    background_color: Color,
    tab_spaces: String,
    highlighting_theme: String,
    margin: Height,
}

impl CodeBoxBuilder {
    fn new(
        font: Font,
        font_bold: Font,
        font_italic: Font,
        font_size: FontSize,
        line_height: Height,
        background_color: Color,
        highlighting_theme: String,
        tab_width: usize,
        margin: Height,
    ) -> Self {
        Self {
            ts: ThemeSet::load_defaults(),
            ps: SyntaxSet::load_defaults_newlines(),
            font,
            font_bold,
            font_italic,
            font_size,
            line_height,
            background_color,
            tab_spaces: " ".repeat(tab_width),
            highlighting_theme,
            margin,
        }
    }

    fn build_text_box(&self, language: Option<String>, code: String) -> TextBox {
        TextBox::new(
            self.build_text_lines(language, code),
            self.margin,
            Some(self.background_color),
            TextBoxStyle::Code,
        )
    }

    fn build_text_lines(&self, language: Option<String>, code: String) -> Vec<TextLine> {
        let syntax = match language {
            Some(lang) => self.ps.find_syntax_by_token(&lang),
            None => self.ps.find_syntax_by_first_line(&code),
        }
        .unwrap_or_else(|| self.ps.find_syntax_plain_text());
        let theme = &self.ts.themes[&self.highlighting_theme];
        let mut h = HighlightLines::new(syntax, &theme);
        let lines = LinesWithEndings::from(&code)
            .map(|line| h.highlight(line, &self.ps))
            .collect::<Vec<_>>();

        let mut text_lines = vec![];
        let mut partials = vec![];
        for tokens in lines.iter() {
            for (style, text) in tokens {
                let text = text.trim_end_matches('\n').replace('\t', &self.tab_spaces);
                if text.is_empty() {
                    continue;
                }

                let c = style.foreground;
                let font = match style.font_style {
                    FontStyle::BOLD => self.font_bold,
                    FontStyle::ITALIC => self.font_italic,
                    _ => self.font,
                };

                partials.push(TextPartial::new(
                    &text,
                    font,
                    self.font_size,
                    Color::from_rgba(c.r, c.g, c.b, c.a),
                    self.line_height,
                ));
            }
            text_lines.push(TextLine::new("left".to_owned(), partials));
            partials = Vec::new();
        }

        text_lines
    }
}

#[derive(DeJson)]
#[nserde(transparent)]
struct HexColor(String);
impl HexColor {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl From<&HexColor> for Color {
    fn from(color: &HexColor) -> Color {
        match Rgb::from_hex_str(color.as_str()) {
            Ok(rgb) => Color::new(
                rgb.red() as f32 / 255.,
                rgb.green() as f32 / 255.,
                rgb.blue() as f32 / 255.,
                1.,
            ),
            Err(_) => WHITE,
        }
    }
}

#[derive(Clone, DeJson)]
#[nserde(default)]
pub struct Theme {
    pub background_image: Option<String>,
    #[nserde(proxy = "HexColor")]
    pub background_color: Color,
    #[nserde(proxy = "HexColor")]
    pub heading_color: Color,
    #[nserde(proxy = "HexColor")]
    pub text_color: Color,
    pub align: String,
    pub font: String,
    pub font_bold: String,
    pub font_italic: String,
    pub font_size_header_title: FontSize,
    pub font_size_header_slides: FontSize,
    pub font_size_text: FontSize,
    pub vertical_offset: Vpos,
    pub horizontal_offset: Hpos,
    pub line_height: Height,
    #[nserde(proxy = "HexColor")]
    pub blockquote_background_color: Color,
    pub blockquote_padding: f32,
    pub blockquote_left_quote: String,
    pub blockquote_right_quote: String,
    pub code_font: String,
    pub code_font_size: FontSize,
    pub code_line_height: Height,
    #[nserde(proxy = "HexColor")]
    pub code_background_color: Color,
    pub code_theme: String,
    pub code_tab_width: usize,
    pub bullet: String,
    pub shader: bool,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            background_image: None,
            background_color: Color::from_rgba(48, 25, 52, 255),
            heading_color: Color::from_rgba(177, 156, 217, 255),
            text_color: WHITE,
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
            blockquote_background_color: Color::from_rgba(51, 51, 51, 255),
            blockquote_padding: 20.,
            blockquote_left_quote: "“".to_string(),
            blockquote_right_quote: "„".to_string(),
            code_font: "assets/Hack-Regular.ttf".to_string(),
            code_font_size: 20,
            code_line_height: 1.2,
            code_background_color: Color::from_rgba(0, 43, 54, 255),
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
    pub automatic: Duration,
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
        theme.background_color, theme.text_color, theme.heading_color,
    );
    let text_font = load_ttf_font(&theme.font).await;
    let bold_font = load_ttf_font(&theme.font_bold).await;
    let italic_font = load_ttf_font(&theme.font_italic).await;
    let code_font = load_ttf_font(&theme.code_font).await;
    let background = match &theme.background_image {
        Some(path) => Some(
            load_texture(&path)
                .await
                .expect("Couldn't load background texture"),
        ),
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
    render_target.texture.set_filter(FilterMode::Linear);
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
            set_camera(&Camera2D {
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
