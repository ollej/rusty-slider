use macroquad::{
    color::{Color, colors::WHITE},
    shapes::draw_rectangle,
    text::draw_text,
    window::{screen_height, screen_width},
};

pub struct ShowHelp {
    pub showing: bool,
}

impl Default for ShowHelp {
    fn default() -> Self {
        Self::new()
    }
}

impl ShowHelp {
    const BACKGROUND_COLOR: Color = Color::new(0.1, 0.1, 0.1, 0.5);
    const FONT_COLOR: Color = WHITE;
    const MARGIN: f32 = 60.;
    const FONT_SIZE: f32 = 70.;
    const LINE_OFFSET: f32 = 10.;
    const HELP_TEXT: &'static str = include_str!("helptext.txt");

    pub fn new() -> Self {
        Self { showing: false }
    }

    pub fn draw(&self) {
        if !self.showing {
            return;
        }
        draw_rectangle(
            Self::MARGIN,
            Self::MARGIN,
            screen_width() - Self::MARGIN * 2.,
            screen_height() - Self::MARGIN * 2.,
            Self::BACKGROUND_COLOR,
        );

        let mut offset_y = Self::MARGIN + Self::FONT_SIZE + Self::LINE_OFFSET;
        for line in Self::HELP_TEXT.split('\n') {
            offset_y = self.draw_line(Self::MARGIN * 2., offset_y, line);
        }
    }

    pub fn toggle_show(&mut self) {
        self.showing = !self.showing
    }

    fn draw_line(&self, offset_x: f32, offset_y: f32, text: &str) -> f32 {
        draw_text(text, offset_x, offset_y, Self::FONT_SIZE, Self::FONT_COLOR);
        offset_y + Self::FONT_SIZE + Self::LINE_OFFSET
    }
}
