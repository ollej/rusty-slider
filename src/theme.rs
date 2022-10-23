use crate::prelude::*;
use macroquad::prelude::{debug, load_string, Color, WHITE};
use nanoserde::DeJson;
use std::path::PathBuf;

#[derive(Clone, DeJson)]
#[nserde(default)]
pub struct Theme {
    pub background_image: Option<String>,
    #[nserde(proxy = "HexColor")]
    pub background_color: Color,
    #[nserde(proxy = "HexColor")]
    pub heading_color: Color,
    #[nserde(proxy = "HexColor")]
    pub text_color: Color,
    pub align: String,
    pub font: String,
    pub font_bold: String,
    pub font_italic: String,
    pub font_size_header_title: FontSize,
    pub font_size_header_slides: FontSize,
    pub font_size_text: FontSize,
    pub vertical_offset: Vpos,
    pub horizontal_offset: Hpos,
    pub line_height: Height,
    #[nserde(proxy = "HexColor")]
    pub blockquote_background_color: Color,
    pub blockquote_padding: f32,
    pub blockquote_left_quote: String,
    pub blockquote_right_quote: String,
    pub font_code: String,
    pub font_code_size: FontSize,
    pub code_line_height: Height,
    #[nserde(proxy = "HexColor")]
    pub code_background_color: Color,
    pub code_theme: String,
    pub code_tab_width: usize,
    pub bullet: String,
    pub shader: bool,
    pub transition: Transitioning,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            background_image: None,
            background_color: Color::from_rgba(48, 25, 52, 255),
            heading_color: Color::from_rgba(177, 156, 217, 255),
            text_color: WHITE,
            align: "center".to_string(),
            font: "assets/Amble-Regular.ttf".to_string(),
            font_bold: "assets/Amble-Bold.ttf".to_string(),
            font_italic: "assets/Amble-Italic.ttf".to_string(),
            font_size_header_title: 100,
            font_size_header_slides: 80,
            font_size_text: 40,
            vertical_offset: 20.0,
            horizontal_offset: 20.0,
            line_height: 2.0,
            blockquote_background_color: Color::from_rgba(51, 51, 51, 255),
            blockquote_padding: 20.,
            blockquote_left_quote: "“".to_string(),
            blockquote_right_quote: "„".to_string(),
            font_code: "assets/Hack-Regular.ttf".to_string(),
            font_code_size: 20,
            code_line_height: 1.2,
            code_background_color: Color::from_rgba(0, 43, 54, 255),
            code_theme: "Solarized (dark)".to_string(),
            code_tab_width: 4,
            bullet: "• ".to_string(),
            shader: true,
            transition: Transitioning::Swipe,
        }
    }
}

impl Theme {
    pub async fn load(theme_path: PathBuf) -> Self {
        let path = theme_path.as_path().to_str().unwrap().to_owned();
        debug!("Theme path: {}", path);
        match load_string(&path).await {
            Ok(json) => match DeJson::deserialize_json(&json) {
                Ok(theme) => theme,
                Err(_) => {
                    eprintln!("Couldn't parse theme file: {}", path);
                    std::process::exit(2);
                }
            },
            Err(_) => Theme::default(),
        }
    }
}
