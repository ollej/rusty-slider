use crate::prelude::*;
use clap::Args;
use macroquad::prelude::{Color, WHITE, debug, load_string};
use nanoserde::DeJson;
use std::path::PathBuf;

#[derive(Args, Debug, Clone)]
pub struct ThemeOptions {
    /// Background image
    #[arg(long, value_name = "image path", value_hint = clap::ValueHint::DirPath)]
    pub background_image: Option<String>,
    /// Background color
    #[arg(long, value_name = "hex color")]
    pub background_color: Option<String>,
    /// Heading color
    #[arg(long, value_name = "hex color")]
    pub heading_color: Option<String>,
    /// Text color
    #[arg(long, value_name = "hex color")]
    pub text_color: Option<String>,
    /// Draw alignment
    #[arg(long, value_enum, value_name = "alignment")]
    pub align: Option<DrawAlignment>,
    /// Font for normal text
    #[arg(long, value_name = "font path", value_hint = clap::ValueHint::DirPath)]
    pub font: Option<String>,
    /// Font for bold text
    #[arg(long, value_name = "hex color", value_hint = clap::ValueHint::DirPath)]
    pub font_bold: Option<String>,
    /// Font for italic text
    #[arg(long, value_name = "hex color", value_hint = clap::ValueHint::DirPath)]
    pub font_italic: Option<String>,
    /// Font size for header title
    #[arg(long, value_name = "number")]
    pub font_size_header_title: Option<FontSize>,
    /// Font size for header on slides
    #[arg(long, value_name = "number")]
    pub font_size_header_slides: Option<FontSize>,
    /// Font size for normal text
    #[arg(long, value_name = "number")]
    pub font_size_text: Option<FontSize>,
    /// Vertical offset
    #[arg(long, value_name = "number")]
    pub vertical_offset: Option<Vpos>,
    /// Horizontal offset
    #[arg(long, value_name = "number")]
    pub horizontal_offset: Option<Hpos>,
    /// Line height of text
    #[arg(long, value_name = "number")]
    pub line_height: Option<Height>,
    /// Background color of blockquotes
    #[arg(long, value_name = "hex color")]
    pub blockquote_background_color: Option<String>,
    /// Padding of blockquote box
    #[arg(long, value_name = "number")]
    pub blockquote_padding: Option<f32>,
    /// Character to use for opening quotation mark on blockquotes
    #[arg(long, value_name = "string")]
    pub blockquote_left_quote: Option<String>,
    /// Character to use for closing quotation mark on blockquotes
    #[arg(long, value_name = "string")]
    pub blockquote_right_quote: Option<String>,
    /// Font for code blocks
    #[arg(long, value_name = "hex color", value_hint = clap::ValueHint::DirPath)]
    pub font_code: Option<String>,
    /// Font size for code blocks
    #[arg(long, value_name = "number")]
    pub font_code_size: Option<FontSize>,
    /// Line height for code blocks
    #[arg(long, value_name = "number")]
    pub code_line_height: Option<f32>,
    /// Background color of code blocks
    #[arg(long, value_name = "hex color")]
    pub code_background_color: Option<String>,
    /// Code block color highlighting theme
    #[arg(long, value_name = "string")]
    pub code_theme: Option<String>,
    /// Tab width in code blocks
    #[arg(long, value_name = "number")]
    pub code_tab_width: Option<usize>,
    /// Character to use for bullet points
    #[arg(long, value_name = "string")]
    pub bullet: Option<String>,
    /// Whether to activate shader
    #[arg(long, value_name = "boolean")]
    pub shader: Option<bool>,
    /// Slide transition to use when changing slides
    #[arg(long, value_enum, value_name = "transition name")]
    pub transition: Option<Transitioning>,
}

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
    pub align: DrawAlignment,
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
    pub transition: Option<Transitioning>,
}

impl Default for Theme {
    fn default() -> Theme {
        Theme {
            background_image: None,
            background_color: Color::from_rgba(48, 25, 52, 255),
            heading_color: Color::from_rgba(177, 156, 217, 255),
            text_color: WHITE,
            align: DrawAlignment::center,
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
            transition: Some(Transitioning::swiperight),
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

    pub fn with_overrides(&self, values: ThemeOptions) -> Self {
        Self {
            background_image: values
                .background_image
                .or_else(|| self.background_image.clone()),
            background_color: values
                .background_color
                .map(|c| HexColor::from(c).into())
                .unwrap_or_else(|| self.background_color),
            heading_color: values
                .heading_color
                .map(|c| HexColor::from(c).into())
                .unwrap_or_else(|| self.heading_color),
            text_color: values
                .text_color
                .map(|c| HexColor::from(c).into())
                .unwrap_or_else(|| self.text_color),
            align: values.align.unwrap_or_else(|| self.align),
            font: values.font.unwrap_or_else(|| self.font.clone()),
            font_bold: values.font_bold.unwrap_or_else(|| self.font_bold.clone()),
            font_italic: values
                .font_italic
                .unwrap_or_else(|| self.font_italic.clone()),
            font_size_header_title: values
                .font_size_header_title
                .unwrap_or_else(|| self.font_size_header_title),
            font_size_header_slides: values
                .font_size_header_slides
                .unwrap_or_else(|| self.font_size_header_title),
            font_size_text: values.font_size_text.unwrap_or_else(|| self.font_size_text),
            vertical_offset: values
                .vertical_offset
                .unwrap_or_else(|| self.vertical_offset),
            horizontal_offset: values
                .horizontal_offset
                .unwrap_or_else(|| self.horizontal_offset),
            line_height: values.line_height.unwrap_or_else(|| self.line_height),
            blockquote_background_color: values
                .blockquote_background_color
                .map(|c| HexColor::from(c).into())
                .unwrap_or_else(|| self.blockquote_background_color),
            blockquote_padding: values
                .blockquote_padding
                .unwrap_or_else(|| self.blockquote_padding),
            blockquote_left_quote: values
                .blockquote_left_quote
                .unwrap_or_else(|| self.blockquote_left_quote.clone()),
            blockquote_right_quote: values
                .blockquote_right_quote
                .unwrap_or_else(|| self.blockquote_right_quote.clone()),
            font_code: values.font_code.unwrap_or_else(|| self.font_code.clone()),
            font_code_size: values.font_code_size.unwrap_or_else(|| self.font_code_size),
            code_line_height: values
                .code_line_height
                .unwrap_or_else(|| self.code_line_height),
            code_background_color: values
                .code_background_color
                .map(|c| HexColor::from(c).into())
                .unwrap_or_else(|| self.code_background_color),
            code_theme: values.code_theme.unwrap_or_else(|| self.code_theme.clone()),
            code_tab_width: values.code_tab_width.unwrap_or_else(|| self.code_tab_width),
            bullet: values.bullet.unwrap_or_else(|| self.bullet.clone()),
            shader: values.shader.unwrap_or_else(|| self.shader),
            transition: values.transition.or_else(|| self.transition),
        }
    }
}
