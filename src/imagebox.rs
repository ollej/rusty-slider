use crate::prelude::*;
use macroquad::prelude::*;

#[derive(Clone)]
pub struct ImageBox {
    margin: Height,
    padding: f32,
    background_color: Option<Color>,
    path: String,
    image: Option<Texture2D>,
}

impl ImageBox {
    const BOX_PADDING: f32 = 20.;

    pub fn new(path: &str, margin: Height, background_color: Option<Color>) -> Self {
        Self {
            margin,
            padding: Self::BOX_PADDING,
            background_color,
            path: path.to_string(),
            image: None,
        }
    }

    pub fn draw(&self, hpos: Hpos, vpos: Vpos) -> Vpos {
        //debug!(
        //    "Image draw hpos:{} vpos:{} width:{} height: {}",
        //    hpos,
        //    vpos,
        //    self.width(),
        //    self.height()
        //);
        if let Some(image) = self.image.clone() {
            draw_texture_ex(
                &image,
                hpos,
                vpos + self.padding + self.margin,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(self.width(), self.height())),
                    ..Default::default()
                },
            );
        }
        vpos + self.height_with_margin()
    }

    pub fn draw_background(&self, hpos: Hpos, vpos: Vpos) {
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

    pub fn path(&self) -> Option<String> {
        Some(self.path.clone())
    }

    pub fn set_image(&mut self, image: Texture2D) {
        self.image = Some(image);
    }

    fn background_color(&self) -> Option<Color> {
        self.background_color
    }

    pub fn width(&self) -> Width {
        match self.image.clone() {
            Some(image) => image.width(),
            None => 0.,
        }
    }

    pub fn width_with_padding(&self) -> Width {
        self.width() + self.padding * 2.
    }

    pub fn height(&self) -> Height {
        match self.image.clone() {
            Some(image) => image.height(),
            None => 0.,
        }
    }

    pub fn height_with_padding(&self) -> Height {
        self.height() + self.padding * 2.
    }

    fn height_with_margin(&self) -> Height {
        self.height_with_padding() + self.margin
    }
}
