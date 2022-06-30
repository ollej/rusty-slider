use crate::prelude::*;
use macroquad::prelude::*;

#[derive(Clone)]
pub enum DrawBox {
    Image(ImageBox),
    Text(TextBox),
}

impl DrawBox {
    pub async fn load_image(&mut self) {
        match self {
            DrawBox::Image(draw_box) => {
                if let Some(path) = draw_box.path() {
                    if let Ok(texture) = load_texture(&path).await {
                        draw_box.set_image(texture);
                        debug!("Image loaded: {}", path);
                    } else {
                        error!("Couldn't load image file: {}", path);
                    }
                }
            }
            DrawBox::Text(_) => (),
        }
    }

    pub fn draw(&self, hpos: Hpos, vpos: Vpos) -> Vpos {
        match self {
            DrawBox::Image(image_box) => image_box.draw(hpos, vpos),
            DrawBox::Text(text_box) => text_box.draw(hpos, vpos),
        }
    }

    pub fn width_with_padding(&self) -> Width {
        match self {
            DrawBox::Image(image_box) => image_box.width_with_padding(),
            DrawBox::Text(text_box) => text_box.width_with_padding(),
        }
    }
}
