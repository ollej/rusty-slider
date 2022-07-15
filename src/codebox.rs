use crate::prelude::*;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct CodeBox {
    width: Width,
    height: Height,
    margin: Height,
    background_color: Option<Color>,
    textbox: TextBox,
}

impl CodeBox {
    const TITLE_BAR_HEIGHT: f32 = 30.;
    const CHROME_CORNER_RADIUS: f32 = 8.;
    const CHROME_CIRCLE_DISTANCE: f32 = 10.;
    const CHROME_CIRCLE_RADIUS: f32 = 8.;
    const DEFAULT_TITLE_BAR_COLOR: Color = Color::new(0.965, 0.961, 0.961, 1.0);
    const CHROME_COLOR_RED: Color = Color::new(0.996, 0.373, 0.345, 1.);
    const CHROME_OUTLINE_RED: Color = Color::new(0.863, 0.227, 0.216, 1.);
    const CHROME_COLOR_YELLOW: Color = Color::new(0.996, 0.737, 0.173, 1.);
    const CHROME_OUTLINE_YELLOW: Color = Color::new(0.863, 0.592, 0.110, 1.);
    const CHROME_COLOR_GREEN: Color = Color::new(0.157, 0.784, 0.251, 1.);
    const CHROME_OUTLINE_GREEN: Color = Color::new(0.106, 0.639, 0.153, 1.);

    pub fn new(textbox: TextBox, margin: Height, background_color: Option<Color>) -> Self {
        Self {
            width: textbox.width_with_padding(),
            height: textbox.height_with_margin(),
            margin,
            background_color,
            textbox,
        }
    }

    pub fn draw(&self, hpos: Hpos, vpos: Vpos) -> Vpos {
        let vpos = vpos + self.margin;
        self.draw_rounded_rectangle(
            hpos,
            vpos,
            self.width + Self::CHROME_CORNER_RADIUS,
            self.height + Self::TITLE_BAR_HEIGHT + Self::CHROME_CORNER_RADIUS,
            Self::CHROME_CORNER_RADIUS,
            self.background_color
                .unwrap_or(Self::DEFAULT_TITLE_BAR_COLOR),
        );
        self.draw_outlined_circle(
            hpos + Self::CHROME_CIRCLE_DISTANCE + Self::CHROME_CIRCLE_RADIUS,
            vpos,
            Self::CHROME_COLOR_RED,
            Self::CHROME_OUTLINE_RED,
        );
        self.draw_outlined_circle(
            hpos + Self::CHROME_CIRCLE_DISTANCE * 2. + Self::CHROME_CIRCLE_RADIUS * 3.,
            vpos,
            Self::CHROME_COLOR_YELLOW,
            Self::CHROME_OUTLINE_YELLOW,
        );
        self.draw_outlined_circle(
            hpos + Self::CHROME_CIRCLE_DISTANCE * 3. + Self::CHROME_CIRCLE_RADIUS * 5.,
            vpos,
            Self::CHROME_COLOR_GREEN,
            Self::CHROME_OUTLINE_GREEN,
        );
        let new_vpos = self.textbox.draw(hpos, vpos + Self::TITLE_BAR_HEIGHT);
        new_vpos + self.margin
    }

    fn draw_rounded_rectangle(
        &self,
        hpos: Hpos,
        vpos: Vpos,
        width: Width,
        height: Height,
        corner_radius: f32,
        color: Color,
    ) {
        draw_circle(
            hpos + corner_radius,
            vpos + corner_radius,
            corner_radius,
            color,
        );
        draw_circle(
            hpos + width - corner_radius,
            vpos + corner_radius,
            corner_radius,
            color,
        );
        draw_rectangle(
            hpos,
            vpos + corner_radius,
            width,
            height - corner_radius * 2.,
            color,
        );
        draw_rectangle(
            hpos + corner_radius,
            vpos,
            width - corner_radius * 2.,
            height,
            color,
        );
        draw_circle(
            hpos + corner_radius,
            vpos + height - corner_radius,
            corner_radius,
            color,
        );
        draw_circle(
            hpos + width - corner_radius,
            vpos + height - corner_radius,
            corner_radius,
            color,
        );
    }

    fn draw_outlined_circle(&self, hpos: Hpos, vpos: Vpos, color: Color, outline_color: Color) {
        draw_circle(
            hpos,
            vpos + Self::TITLE_BAR_HEIGHT / 2.,
            Self::CHROME_CIRCLE_RADIUS,
            color,
        );
        draw_circle_lines(
            hpos,
            vpos + Self::TITLE_BAR_HEIGHT / 2.,
            Self::CHROME_CIRCLE_RADIUS,
            1.,
            outline_color,
        );
    }

    pub fn width(&self) -> Width {
        self.width
    }

    pub fn width_with_padding(&self) -> Width {
        self.width + Self::CHROME_CORNER_RADIUS * 2.
    }

    pub fn height(&self) -> Height {
        self.height
    }

    pub fn height_with_padding(&self) -> Width {
        self.height + Self::CHROME_CORNER_RADIUS
    }

    pub fn height_with_margin(&self) -> Height {
        self.height() + self.margin
    }
}
