use crate::prelude::*;

use macroquad::prelude::*;
use regex::Regex;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Slide {
    pub draw_boxes: Vec<DrawBox>,
    pub code_block: Option<ExecutableCode>,
}

impl Slide {
    pub fn add_code_box(&mut self, draw_box: CodeBox) {
        self.draw_boxes.push(DrawBox::Code(draw_box));
    }

    pub fn add_text_box(&mut self, draw_box: TextBox) {
        self.draw_boxes.push(DrawBox::Text(draw_box));
    }
}

pub struct Slides {
    slides: Vec<Slide>,
    theme: Theme,
    code_box_builder: CodeBoxBuilder,
    background: Option<Texture2D>,
    automatic: Duration,
    active_slide: usize,
    time: Duration,
    render_target: RenderTarget,
}

impl Slides {
    fn from_slides(
        slides: Vec<Slide>,
        theme: Theme,
        code_box_builder: CodeBoxBuilder,
        background: Option<Texture2D>,
        automatic: Duration,
    ) -> Slides {
        Slides {
            slides,
            theme,
            code_box_builder,
            background,
            automatic,
            time: 0.,
            active_slide: 0,
            render_target: Self::render_target(),
        }
    }

    pub async fn load(slides_path: PathBuf, theme: Theme, automatic: Duration) -> Self {
        let path = slides_path.as_path().to_str().unwrap().to_owned();
        let markdown = match load_string(&path).await {
            Ok(text) => Self::sanitize_markdown(text),
            Err(_) => {
                eprintln!("Couldn't parse markdown document: {}", path);
                std::process::exit(1);
            }
        };
        debug!("Sanitized markdown:\n{}", markdown);

        let font_text = load_ttf_font(&theme.font)
            .await
            .expect("Couldn't load font");
        let font_bold = load_ttf_font(&theme.font_bold)
            .await
            .expect("Couldn't load font");
        let font_italic = load_ttf_font(&theme.font_italic)
            .await
            .expect("Couldn't load font");
        let font_code = load_ttf_font(&theme.font_code)
            .await
            .expect("Couldn't load font");
        let background = match &theme.background_image {
            Some(path) => Some(
                load_texture(path)
                    .await
                    .expect("Couldn't load background texture"),
            ),
            None => None,
        };

        let mut slides =
            MarkdownToSlides::new(theme.clone(), font_text, font_bold, font_italic, font_code)
                .parse(markdown);

        for slide in &mut slides.iter_mut() {
            for draw_box in &mut slide.draw_boxes.iter_mut() {
                draw_box.load_image().await;
            }
        }

        let code_box_builder =
            CodeBoxBuilder::new(theme.clone(), font_code, font_bold, font_italic);

        Self::from_slides(
            slides,
            theme.clone(),
            code_box_builder,
            background,
            automatic,
        )
    }

    fn render_target() -> RenderTarget {
        let render_target = render_target(screen_width() as u32, screen_height() as u32);
        render_target.texture.set_filter(FilterMode::Linear);
        render_target
    }

    pub fn texture(&self) -> Texture2D {
        self.render_target.texture
    }

    pub fn sanitize_markdown(text: String) -> String {
        let no_comments = Self::strip_comments(text);
        Self::strip_yaml_header(no_comments)
    }

    pub fn strip_comments(text: String) -> String {
        let re = Regex::new(r"(?sm)<!--.*?--\s*>").unwrap();
        re.replace_all(&text, "").to_string()
    }

    pub fn strip_yaml_header(text: String) -> String {
        let re =
            Regex::new(r"(?sm)^---(\r\n?|\n)((\w+?): (.+?)(\r\n?|\n))+?---(\r\n?|\n)").unwrap();
        re.replace_all(&text, "").to_string()
    }

    pub fn next(&mut self) {
        if self.active_slide < (self.slides.len() - 1) {
            self.time = 0.;
            self.active_slide += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.active_slide > 0 {
            self.time = 0.;
            self.active_slide -= 1;
        }
    }

    pub fn draw(&mut self, delta: Duration) {
        self.set_camera();
        if self.automatic > 0. && self.time > self.automatic {
            self.next();
        } else {
            self.time += delta;
        }
        clear_background(self.theme.background_color);
        self.draw_background(self.background);
        self.draw_slide();
    }

    /// set camera with following coordinate system:
    /// (0., 0)     .... (SCR_W, 0.)
    /// (0., SCR_H) .... (SCR_W, SCR_H)
    fn set_camera(&self) {
        let scr_w = screen_width();
        let scr_h = screen_height();

        set_camera(&Camera2D {
            zoom: vec2(1. / scr_w * 2., -1. / scr_h * 2.),
            target: vec2(scr_w / 2., scr_h / 2.),
            render_target: Some(self.render_target),
            ..Default::default()
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn run_code_block(&mut self) {
        let slide = self.slides.get_mut(self.active_slide).unwrap();
        if let Some(code_block) = &slide.code_block {
            let output = code_block.execute();
            let code_box = self.code_box_builder.build_draw_box(None, output);
            slide.add_code_box(code_box);
        }
    }

    fn draw_background(&self, background: Option<Texture2D>) {
        if let Some(texture) = background {
            draw_texture_ex(
                texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            )
        }
    }

    fn draw_slide(&mut self) {
        let slide = &self.slides[self.active_slide];
        let mut new_position: Vpos = 0.;
        for draw_box in slide.draw_boxes.iter() {
            let hpos = self.horizontal_position(draw_box.width_with_padding());
            new_position = draw_box.draw(hpos, new_position);
        }
    }

    fn horizontal_position(&self, width: Width) -> Hpos {
        match self.theme.align.as_str() {
            "left" => self.theme.horizontal_offset,
            "right" => screen_width() - self.theme.horizontal_offset - width,
            _ => screen_width() / 2. - width / 2.,
        }
    }
}
