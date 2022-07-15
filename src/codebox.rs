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
    pub fn new(textbox: TextBox, margin: Height, background_color: Option<Color>) -> Self {
        Self {
            width: textbox.width(),
            height: textbox.height(),
            margin,
            background_color,
            textbox,
        }
    }

    pub fn draw(&self, hpos: Hpos, vpos: Vpos) -> Vpos {
        self.textbox.draw(hpos, vpos)
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
