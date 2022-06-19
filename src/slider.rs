use crate::prelude::*;

use macroquad::prelude::*;
use markdown::{Block, ListItem, Span};
use regex::Regex;
use std::path::PathBuf;

#[derive(Clone)]
struct Slide {
    draw_boxes: Vec<Box<DrawBox>>,
    code_block: Option<ExecutableCode>,
}

impl Slide {
    pub fn add_draw_box(&mut self, draw_box: DrawBox) {
        self.draw_boxes.push(Box::new(draw_box));
    }
}

pub struct Slides {
    slides: Vec<Slide>,
    theme: Theme,
    code_box_builder: CodeBoxBuilder,
    background: Option<Texture2D>,
    automatic: Duration,
    active_slide: usize,
    time: Duration,
}

impl Slides {
    fn from_slides(
        slides: Vec<Slide>,
        theme: Theme,
        code_box_builder: CodeBoxBuilder,
        background: Option<Texture2D>,
        automatic: Duration,
    ) -> Slides {
        Slides {
            slides,
            theme,
            code_box_builder,
            background,
            automatic,
            time: 0.,
            active_slide: 0,
        }
    }

    pub async fn load(slides_path: PathBuf, theme: Theme, automatic: Duration) -> Self {
        let path = slides_path.as_path().to_str().unwrap().to_owned();
        let markdown = match load_string(&path).await {
            Ok(text) => Self::sanitize_markdown(text),
            Err(_) => {
                eprintln!("Couldn't parse markdown document: {}", path);
                std::process::exit(1);
            }
        };
        debug!("Sanitized markdown:\n{}", markdown);

        let font_text = load_ttf_font(&theme.font)
            .await
            .expect("Couldn't load font");
        let font_bold = load_ttf_font(&theme.font_bold)
            .await
            .expect("Couldn't load font");
        let font_italic = load_ttf_font(&theme.font_italic)
            .await
            .expect("Couldn't load font");
        let font_code = load_ttf_font(&theme.font_code)
            .await
            .expect("Couldn't load font");
        let background = match &theme.background_image {
            Some(path) => Some(
                load_texture(&path)
                    .await
                    .expect("Couldn't load background texture"),
            ),
            None => None,
        };

        let slides =
            MarkdownToSlides::new(theme.clone(), font_text, font_bold, font_italic, font_code)
                .parse(markdown);
        let code_box_builder =
            CodeBoxBuilder::new(theme.clone(), font_code, font_bold, font_italic);

        Self::from_slides(
            slides,
            theme.clone(),
            code_box_builder,
            background,
            automatic,
        )
    }

    pub fn sanitize_markdown(text: String) -> String {
        let no_comments = Self::strip_comments(text);
        Self::strip_yaml_header(no_comments)
    }

    pub fn strip_comments(text: String) -> String {
        let re = Regex::new(r"(?sm)<!--.*?--\s*>").unwrap();
        re.replace_all(&text, "").to_string()
    }

    pub fn strip_yaml_header(text: String) -> String {
        let re =
            Regex::new(r"(?sm)^---(\r\n?|\n)((\w+?): (.+?)(\r\n?|\n))+?---(\r\n?|\n)").unwrap();
        re.replace_all(&text, "").to_string()
    }

    pub fn next(&mut self) {
        if self.active_slide < (self.slides.len() - 1) {
            self.time = 0.;
            self.active_slide += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.active_slide > 0 {
            self.time = 0.;
            self.active_slide -= 1;
        }
    }

    pub fn draw(&mut self, delta: Duration) {
        if self.automatic > 0. && self.time > self.automatic {
            self.next();
        } else {
            self.time += delta;
        }
        clear_background(self.theme.background_color);
        self.draw_background(self.background);
        self.draw_slide();
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn run_code_block(&mut self) {
        let slide = self.slides.get_mut(self.active_slide).unwrap();
        if let Some(code_block) = &slide.code_block {
            let output = code_block.execute();
            let code_box = self
                .code_box_builder
                .build_draw_box(None, output.to_owned());
            slide.add_draw_box(code_box);
        }
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
        for draw_box in slide.draw_boxes.iter() {
            let hpos = self.horizontal_position(draw_box.width_with_padding());
            new_position = draw_box.draw(hpos, new_position);
        }
    }

    fn horizontal_position(&self, width: Width) -> Hpos {
        match self.theme.align.as_str() {
            "left" => self.theme.horizontal_offset,
            "right" => screen_width() - self.theme.horizontal_offset - width,
            _ => screen_width() / 2. - width / 2.,
        }
    }
}

pub struct MarkdownToSlides {
    theme: Theme,
    font_text: Font,
    font_bold: Font,
    font_italic: Font,
    font_code: Font,
    code_box_builder: CodeBoxBuilder,
}

impl MarkdownToSlides {
    fn new(
        theme: Theme,
        font_text: Font,
        font_bold: Font,
        font_italic: Font,
        font_code: Font,
    ) -> Self {
        let code_box_builder =
            CodeBoxBuilder::new(theme.clone(), font_code, font_bold, font_italic);
        Self {
            theme,
            code_box_builder,
            font_text,
            font_bold,
            font_italic,
            font_code,
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
        if !blocks.is_empty() {
            slides.push(blocks);
        }
        slides
    }

    fn build_slides(&self, slide_blocks: Vec<Vec<Block>>) -> Vec<Slide> {
        let mut slides = vec![];
        for blocks in slide_blocks.iter() {
            slides.push(self.build_slide(blocks));
        }
        slides
    }

    fn build_slide(&self, blocks: &[Block]) -> Slide {
        Slide {
            draw_boxes: self.blocks_to_draw_boxes(blocks, None, DrawBoxStyle::Standard),
            code_block: self.find_first_code_block(blocks),
        }
    }

    fn find_first_code_block(&self, blocks: &[Block]) -> Option<ExecutableCode> {
        for block in blocks.iter() {
            if let Block::CodeBlock(Some(language), code) = block {
                if let Some(cb) = ExecutableCode::from(language, code) {
                    return Some(cb);
                }
            }
        }
        None
    }

    fn blocks_to_draw_boxes(
        &self,
        blocks: &[Block],
        background_color: Option<Color>,
        style: DrawBoxStyle,
    ) -> Vec<Box<DrawBox>> {
        let mut draw_boxes = vec![];
        let mut text_lines = vec![];
        for block in blocks.iter() {
            match block {
                Block::Header(spans, 1) => {
                    if !text_lines.is_empty() {
                        draw_boxes.push(Box::new(DrawBox::new(
                            text_lines,
                            self.theme.vertical_offset,
                            background_color,
                            style,
                        )));
                        text_lines = Vec::new();
                    }
                    draw_boxes.push(Box::new(DrawBox::new(
                        vec![TextLine::new(
                            self.theme.align.to_owned(),
                            self.spans_to_text_partials(
                                spans,
                                self.font_text,
                                self.theme.font_size_header_title,
                                self.theme.heading_color,
                            ),
                        )],
                        self.theme.vertical_offset,
                        background_color,
                        DrawBoxStyle::Title,
                    )));
                }
                Block::Header(spans, _size) => {
                    text_lines.push(TextLine::new(
                        self.theme.align.to_owned(),
                        self.spans_to_text_partials(
                            spans,
                            self.font_text,
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
                            self.font_text,
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
                    if !text_lines.is_empty() {
                        draw_boxes.push(Box::new(DrawBox::new(
                            text_lines,
                            self.theme.vertical_offset,
                            background_color,
                            style,
                        )));
                        text_lines = Vec::new();
                    }
                    draw_boxes.extend(self.blocks_to_draw_boxes(
                        blocks,
                        Some(self.theme.blockquote_background_color),
                        DrawBoxStyle::Blockquote {
                            size: self.theme.font_size_header_title * 2,
                            font: self.font_text,
                            color: self.theme.text_color,
                        },
                    ));
                }
                Block::CodeBlock(language, code) => {
                    if !text_lines.is_empty() {
                        draw_boxes.push(Box::new(DrawBox::new(
                            text_lines,
                            self.theme.vertical_offset,
                            background_color,
                            style,
                        )));
                        text_lines = Vec::new();
                    }
                    draw_boxes.push(Box::new(
                        self.code_box_builder
                            .build_draw_box(language.to_owned(), code.to_owned()),
                    ));
                }

                _ => (),
            }
        }
        if !text_lines.is_empty() {
            draw_boxes.push(Box::new(DrawBox::new(
                text_lines,
                self.theme.vertical_offset,
                background_color,
                style,
            )));
        }
        draw_boxes
    }

    fn spans_to_text_partials(
        &self,
        spans: &[Span],
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
                    self.font_code,
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

    fn build_list_box(&self, items: &[ListItem], bullet: Option<&String>) -> Vec<TextLine> {
        let mut lines: Vec<TextLine> = vec![];
        for (index, item) in items.iter().enumerate() {
            if let ListItem::Simple(spans) = item {
                let mut partials = vec![self.build_bullet_partial(index, bullet)];
                partials.extend(self.spans_to_text_partials(
                    spans,
                    self.font_text,
                    self.theme.font_size_text,
                    self.theme.text_color,
                ));
                let text_line = TextLine::new("left".to_string(), partials);
                lines.push(text_line);
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
            self.font_text,
            self.theme.font_size_text,
            self.theme.text_color,
            self.theme.line_height,
        )
    }
}

#[derive(Copy, Clone)]
pub enum DrawBoxStyle {
    Standard,
    Title,
    Blockquote {
        size: FontSize,
        font: Font,
        color: Color,
    },
    Code,
}

impl DrawBoxStyle {
    fn draw(&self, hpos: Hpos, vpos: Vpos, draw_box: &DrawBox) {
        if let DrawBoxStyle::Blockquote { size, font, color } = self {
            self.draw_blockquote(hpos, vpos, draw_box, *size, *font, *color)
        }
    }

    fn top_position(&self, vpos: Vpos, draw_box: &DrawBox) -> Vpos {
        match self {
            DrawBoxStyle::Title => {
                screen_height() / 2.
                    - draw_box.height / 2.
                    - draw_box.margin
                    - draw_box.padding
                    - draw_box.offset_y
            }
            _ => vpos,
        }
    }

    fn draw_blockquote(
        &self,
        hpos: Hpos,
        vpos: Vpos,
        draw_box: &DrawBox,
        font_size: FontSize,
        font: Font,
        color: Color,
    ) {
        let text_params = TextParams {
            font,
            font_size,
            font_scale: 1.,
            color,
            font_scale_aspect: 1.,
        };
        let dimensions = measure_text("“", Some(font), font_size, 1.);
        draw_text_ex(
            "“",
            hpos - dimensions.width,
            vpos + font_size as Vpos,
            text_params,
        );
        draw_text_ex(
            "„",
            hpos + draw_box.width_with_padding(),
            vpos + draw_box.height_with_margin(),
            text_params,
        );
    }
}

trait Draw {
    fn draw(&self, hpos: Hpos, vpos: Vpos) -> Vpos;

    fn draw_background(&self, hpos: Hpos, vpos: Vpos) {
        if let Some(color) = self.background_color() {
            draw_rectangle(
                hpos,
                vpos,
                self.width_with_padding(),
                self.height_with_padding(),
                color,
            );
        }
    }

    fn background_color(&self) -> Option<Color>;

    fn width(&self) -> Width;

    fn width_with_padding(&self) -> Width;

    fn height_with_padding(&self) -> Height;

    fn height_with_margin(&self) -> Height;
}

#[derive(Clone)]
pub struct ImageBox {
    width: Width,
    height: Height,
    margin: Height,
    padding: f32,
    offset_y: Vpos,
    background_color: Option<Color>,
    image: Texture2D,
}

impl ImageBox {
    const BOX_PADDING: f32 = 20.;
}

impl Draw for ImageBox {
    fn draw(&self, hpos: Hpos, vpos: Vpos) -> Vpos {
        vpos + self.height_with_margin()
    }

    fn background_color(&self) -> Option<Color> {
        self.background_color
    }

    fn width(&self) -> Width {
        self.width
    }

    fn width_with_padding(&self) -> Width {
        self.width() + self.padding * 2.
    }

    fn height_with_padding(&self) -> Height {
        self.height + self.padding * 2.
    }

    fn height_with_margin(&self) -> Height {
        self.height_with_padding() + self.margin
    }
}

#[derive(Clone)]
pub struct DrawBox {
    width: Width,
    height: Height,
    margin: Height,
    padding: f32,
    offset_y: Vpos,
    background_color: Option<Color>,
    style: DrawBoxStyle,
    lines: Vec<TextLine>,
}

impl DrawBox {
    const BOX_PADDING: f32 = 20.;

    pub fn new(
        lines: Vec<TextLine>,
        margin: Height,
        background_color: Option<Color>,
        style: DrawBoxStyle,
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
}

impl Draw for DrawBox {
    fn draw(&self, hpos: Hpos, vpos: Vpos) -> Vpos {
        let vpos = self.style.top_position(vpos, self);
        self.draw_background(hpos, vpos + self.margin + self.offset_y);
        self.style.draw(hpos, vpos, self);
        let inner_hpos = hpos + self.padding;
        let mut new_position = vpos + self.padding + self.margin;
        for line in self.lines.iter() {
            let line_hpos = match line.align.as_str() {
                "left" => inner_hpos,
                "right" => inner_hpos + self.width() - line.width,
                _ => inner_hpos + self.width() / 2. - line.width / 2.,
            };
            new_position = line.draw(line_hpos, new_position);
        }
        vpos + self.height_with_margin()
    }

    fn draw_background(&self, hpos: Hpos, vpos: Vpos) {
        if let Some(color) = self.background_color {
            draw_rectangle(
                hpos,
                vpos,
                self.width_with_padding(),
                self.height_with_padding(),
                color,
            );
        }
    }

    fn background_color(&self) -> Option<Color> {
        self.background_color
    }

    fn width(&self) -> Width {
        self.width
    }

    fn width_with_padding(&self) -> Width {
        self.width() + self.padding * 2.
    }

    fn height_with_padding(&self) -> Height {
        self.height + self.padding * 2.
    }

    fn height_with_margin(&self) -> Height {
        self.height_with_padding() + self.margin
    }
}

#[derive(Clone)]
pub struct TextLine {
    width: Width,
    height: Height,
    offset_y: Vpos,
    align: String,
    partials: Vec<TextPartial>,
}

impl TextLine {
    pub fn new(align: String, partials: Vec<TextPartial>) -> Self {
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

#[derive(Clone)]
pub struct TextPartial {
    width: Width,
    height: Height,
    color: Color,
    font: Font,
    font_size: FontSize,
    offset_y: Vpos,
    text: String,
}

impl TextPartial {
    pub fn new(
        text: &str,
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
                font_scale_aspect: 1.,
            },
        );
        hpos + self.width
    }
}
