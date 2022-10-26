use crate::prelude::*;
use macroquad::prelude::*;
use markdown::{Block, ListItem, Span};
use std::mem::discriminant;

pub struct MarkdownToSlides {
    theme: Theme,
    font_text: Font,
    font_bold: Font,
    font_italic: Font,
    font_code: Font,
    code_box_builder: CodeBoxBuilder,
}

impl MarkdownToSlides {
    pub fn new(
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

    pub fn parse(&self, markdown: String) -> Vec<Slide> {
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
        Slide::new(
            self.blocks_to_draw_boxes(blocks, None, TextBoxStyle::Standard),
            self.find_first_code_block(blocks),
        )
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
        style: TextBoxStyle,
    ) -> Vec<DrawBox> {
        let mut draw_boxes = vec![];
        let mut text_lines = vec![];
        for block in blocks.iter() {
            match block {
                Block::Header(spans, 1) => {
                    if !text_lines.is_empty() {
                        draw_boxes.push(DrawBox::Text(TextBox::new(
                            text_lines,
                            self.theme.vertical_offset,
                            background_color,
                            style,
                        )));
                        text_lines = Vec::new();
                    }
                    draw_boxes.push(DrawBox::Text(TextBox::new(
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
                        TextBoxStyle::Title,
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
                Block::Paragraph(spans) if self.is_image(spans) => {
                    if !text_lines.is_empty() {
                        draw_boxes.push(DrawBox::Text(TextBox::new(
                            text_lines,
                            self.theme.vertical_offset,
                            background_color,
                            style,
                        )));
                        text_lines = Vec::new();
                    }
                    if let Some(Span::Image(_title, path, _)) = spans.first() {
                        draw_boxes.push(DrawBox::Image(ImageBox::new(path, 0., None)));
                    }
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
                        draw_boxes.push(DrawBox::Text(TextBox::new(
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
                        TextBoxStyle::Blockquote {
                            size: self.theme.font_size_header_title * 2,
                            font: self.font_text,
                            color: self.theme.text_color,
                        },
                    ));
                }
                Block::CodeBlock(language, code) => {
                    if !text_lines.is_empty() {
                        draw_boxes.push(DrawBox::Text(TextBox::new(
                            text_lines,
                            self.theme.vertical_offset,
                            background_color,
                            style,
                        )));
                        text_lines = Vec::new();
                    }
                    draw_boxes.push(DrawBox::Code(
                        self.code_box_builder
                            .build_draw_box(language.to_owned(), code.to_owned()),
                    ));
                }

                _ => (),
            }
        }
        if !text_lines.is_empty() {
            draw_boxes.push(DrawBox::Text(TextBox::new(
                text_lines,
                self.theme.vertical_offset,
                background_color,
                style,
            )));
        }
        draw_boxes
    }

    fn is_image(&self, spans: &[Span]) -> bool {
        if let Some(span) = spans.first() {
            return discriminant(span)
                == discriminant(&Span::Image("".to_string(), "".to_string(), None));
        }
        false
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
                    text,
                    font,
                    font_size,
                    color,
                    self.theme.line_height,
                )),
                Span::Code(text) => partials.push(TextPartial::new(
                    text,
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
