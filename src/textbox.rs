use crate::prelude::*;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct TextBox {
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

    pub fn new(
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

    pub fn draw(&self, hpos: Hpos, vpos: Vpos) -> Vpos {
        let vpos = self.style.top_position(vpos, self);
        self.draw_background(hpos, vpos + self.margin);
        self.style.draw(hpos, vpos, self);
        let inner_hpos = hpos + self.padding;
        let mut new_position = vpos + self.padding + self.margin;
        for line in self.lines.iter() {
            let line_hpos = match line.align {
                DrawAlignment::left => inner_hpos,
                DrawAlignment::right => inner_hpos + self.width() - line.width,
                DrawAlignment::center => inner_hpos + self.width() / 2. - line.width / 2.,
            };
            new_position = line.draw(line_hpos, new_position, self.offset_y);
        }
        vpos + self.height_with_margin()
    }

    pub fn draw_background(&self, hpos: Hpos, vpos: Vpos) {
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

    pub fn background_color(&self) -> Option<Color> {
        self.background_color
    }

    pub fn width(&self) -> Width {
        self.width
    }

    pub fn width_with_padding(&self) -> Width {
        self.width() + self.padding * 2.
    }

    pub fn height(&self) -> Height {
        self.height
    }

    pub fn height_with_padding(&self) -> Height {
        self.height() + self.padding * 2.
    }

    pub fn height_with_margin(&self) -> Height {
        self.height_with_padding() + self.margin
    }
}

#[derive(Clone)]
pub struct TextLine {
    width: Width,
    height: Height,
    offset_y: Vpos,
    align: DrawAlignment,
    partials: Vec<TextPartial>,
}

impl TextLine {
    pub fn new(align: DrawAlignment, partials: Vec<TextPartial>) -> Self {
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

    fn draw(&self, start_hpos: Hpos, vpos: Vpos, offset_y: Vpos) -> Vpos {
        let mut hpos = start_hpos;
        for partial in &self.partials {
            hpos = partial.draw(hpos, vpos, offset_y);
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

    fn draw(&self, hpos: Hpos, vpos: Vpos, offset_y: Vpos) -> Vpos {
        draw_text_ex(
            &self.text,
            hpos,
            vpos + offset_y,
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
    fn draw(&self, hpos: Hpos, vpos: Vpos, draw_box: &TextBox) {
        if let TextBoxStyle::Blockquote { size, font, color } = self {
            self.draw_blockquote(hpos, vpos, draw_box, *size, *font, *color)
        }
    }

    fn top_position(&self, vpos: Vpos, draw_box: &TextBox) -> Vpos {
        match self {
            TextBoxStyle::Title => {
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
        draw_box: &TextBox,
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
