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
        draw_rectangle(
            hpos,
            vpos,
            self.width,
            self.height + Self::TITLE_BAR_HEIGHT,
            self.background_color
                .unwrap_or(Self::DEFAULT_TITLE_BAR_COLOR),
        );
        draw_circle(
            hpos + Self::CHROME_CIRCLE_DISTANCE + Self::CHROME_CIRCLE_RADIUS,
            vpos + Self::TITLE_BAR_HEIGHT / 2.,
            Self::CHROME_CIRCLE_RADIUS,
            Self::CHROME_COLOR_RED,
        );
        draw_circle_lines(
            hpos + Self::CHROME_CIRCLE_DISTANCE + Self::CHROME_CIRCLE_RADIUS,
            vpos + Self::TITLE_BAR_HEIGHT / 2.,
            Self::CHROME_CIRCLE_RADIUS,
            1.,
            Self::CHROME_OUTLINE_RED,
        );
        draw_circle(
            hpos + Self::CHROME_CIRCLE_DISTANCE * 2. + Self::CHROME_CIRCLE_RADIUS * 3.,
            vpos + Self::TITLE_BAR_HEIGHT / 2.,
            Self::CHROME_CIRCLE_RADIUS,
            Self::CHROME_COLOR_YELLOW,
        );
        draw_circle_lines(
            hpos + Self::CHROME_CIRCLE_DISTANCE * 2. + Self::CHROME_CIRCLE_RADIUS * 3.,
            vpos + Self::TITLE_BAR_HEIGHT / 2.,
            Self::CHROME_CIRCLE_RADIUS,
            1.,
            Self::CHROME_OUTLINE_YELLOW,
        );
        draw_circle(
            hpos + Self::CHROME_CIRCLE_DISTANCE * 3. + Self::CHROME_CIRCLE_RADIUS * 5.,
            vpos + Self::TITLE_BAR_HEIGHT / 2.,
            Self::CHROME_CIRCLE_RADIUS,
            Self::CHROME_COLOR_GREEN,
        );
        draw_circle_lines(
            hpos + Self::CHROME_CIRCLE_DISTANCE * 3. + Self::CHROME_CIRCLE_RADIUS * 5.,
            vpos + Self::TITLE_BAR_HEIGHT / 2.,
            Self::CHROME_CIRCLE_RADIUS,
            1.,
            Self::CHROME_OUTLINE_GREEN,
        );
        let new_vpos = self.textbox.draw(hpos, vpos + Self::TITLE_BAR_HEIGHT);
        new_vpos + self.margin
    }

    pub fn width(&self) -> Width {
        self.width
    }

    pub fn width_with_padding(&self) -> Width {
        self.width
    }

    pub fn height(&self) -> Height {
        self.height
    }

    pub fn height_with_margin(&self) -> Height {
        self.height() + self.margin
    }
}
