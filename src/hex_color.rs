use colorsys::Rgb;
use macroquad::prelude::{Color, WHITE};
use nanoserde::DeJson;

#[derive(DeJson)]
#[nserde(transparent)]
pub struct HexColor(String);

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
