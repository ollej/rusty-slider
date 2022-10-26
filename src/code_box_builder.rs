use crate::prelude::*;
use macroquad::prelude::{Color, Font};
use syntect::easy::HighlightLines;
use syntect::highlighting::FontStyle;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct CodeBoxBuilder {
    ps: SyntaxSet,
    ts: ThemeSet,
    font_text: Font,
    font_bold: Font,
    font_italic: Font,
    font_size: FontSize,
    line_height: Height,
    background_color: Color,
    tab_spaces: String,
    highlighting_theme: String,
    margin: Height,
}

impl CodeBoxBuilder {
    pub fn new(theme: Theme, font_text: Font, font_bold: Font, font_italic: Font) -> Self {
        Self {
            ts: ThemeSet::load_defaults(),
            ps: SyntaxSet::load_defaults_newlines(),
            font_text,
            font_bold,
            font_italic,
            font_size: theme.font_code_size.to_owned(),
            line_height: theme.code_line_height.to_owned(),
            background_color: theme.code_background_color.to_owned(),
            tab_spaces: " ".repeat(theme.code_tab_width),
            highlighting_theme: theme.code_theme.to_owned(),
            margin: theme.vertical_offset.to_owned(),
        }
    }

    pub fn build_draw_box(&self, language: Option<String>, code: String) -> CodeBox {
        CodeBox::new(
            TextBox::new(
                self.build_text_lines(language, code),
                0.,
                Some(self.background_color),
                TextBoxStyle::Code,
            ),
            self.margin,
            Some(self.background_color),
        )
    }

    fn build_text_lines(&self, language: Option<String>, code: String) -> Vec<TextLine> {
        let syntax = match language {
            Some(lang) => self.ps.find_syntax_by_token(&lang),
            None => self.ps.find_syntax_by_first_line(&code),
        }
        .unwrap_or_else(|| self.ps.find_syntax_plain_text());
        let theme = &self.ts.themes[&self.highlighting_theme];
        let mut h = HighlightLines::new(syntax, theme);
        let lines = LinesWithEndings::from(&code)
            .map(|line| h.highlight_line(line, &self.ps))
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        let mut text_lines = vec![];
        let mut partials = vec![];
        for tokens in lines.iter() {
            for (style, text) in tokens {
                let text = text.trim_end_matches('\n').replace('\t', &self.tab_spaces);
                if text.is_empty() {
                    continue;
                }

                let c = style.foreground;
                let font_style = match style.font_style {
                    FontStyle::BOLD => self.font_bold,
                    FontStyle::ITALIC => self.font_italic,
                    _ => self.font_text,
                };

                partials.push(TextPartial::new(
                    &text,
                    font_style,
                    self.font_size,
                    Color::from_rgba(c.r, c.g, c.b, c.a),
                    self.line_height,
                ));
            }
            text_lines.push(TextLine::new(DrawAlignment::left, partials));
            partials = Vec::new();
        }

        text_lines
    }
}
