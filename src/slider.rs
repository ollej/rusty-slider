use crate::prelude::*;

use macroquad::prelude::*;
use nanoserde::DeJson;
use regex::Regex;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone, Debug, DeJson)]
#[allow(non_camel_case_types)]
pub enum DrawAlignment {
    left,
    right,
    center,
}

impl Default for DrawAlignment {
    fn default() -> Self {
        DrawAlignment::left
    }
}

#[derive(Clone)]
pub struct Slide {
    pub draw_boxes: Vec<DrawBox>,
    pub code_block: Option<ExecutableCode>,
    align: DrawAlignment,
    horizontal_offset: Hpos,
    background_texture: Option<Texture2D>,
    background_path: Option<String>,
}

impl Slide {
    pub fn new(
        draw_boxes: Vec<DrawBox>,
        code_block: Option<ExecutableCode>,
        align: DrawAlignment,
        horizontal_offset: Hpos,
        background_path: Option<String>,
    ) -> Self {
        Self {
            draw_boxes,
            code_block,
            align,
            horizontal_offset,
            background_texture: None,
            background_path,
        }
    }

    pub fn empty(align: DrawAlignment, horizontal_offset: Hpos) -> Self {
        Self {
            draw_boxes: vec![],
            code_block: None,
            align,
            horizontal_offset,
            background_texture: None,
            background_path: None,
        }
    }

    pub fn draw(&self, default_background: Option<Texture2D>) {
        self.draw_background(default_background);
        let mut new_position: Vpos = 0.;
        for draw_box in self.draw_boxes.iter() {
            let hpos = self.horizontal_position(draw_box.width_with_padding());
            new_position = draw_box.draw(hpos, new_position);
        }
    }

    pub async fn load_images(&mut self) {
        self.load_background_image().await;
        for draw_box in &mut self.draw_boxes.iter_mut() {
            draw_box.load_image().await;
        }
    }

    /// Ignores missing backgrounds.
    async fn load_background_image(&mut self) {
        if let Some(path) = &self.background_path {
            self.background_texture = load_texture(path).await.ok()
        }
    }

    pub fn add_code_box(&mut self, draw_box: CodeBox) {
        self.draw_boxes.push(DrawBox::Code(draw_box));
    }

    pub fn add_text_box(&mut self, draw_box: TextBox) {
        self.draw_boxes.push(DrawBox::Text(draw_box));
    }

    fn draw_background(&self, default_background: Option<Texture2D>) {
        if let Some(texture) = self.background_texture.or(default_background) {
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

    fn horizontal_position(&self, width: Width) -> Hpos {
        match self.align {
            DrawAlignment::left => self.horizontal_offset,
            DrawAlignment::right => screen_width() - self.horizontal_offset - width,
            DrawAlignment::center => screen_width() / 2. - width / 2.,
        }
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
    pub last_texture: Option<Texture2D>,
    transitioner: Option<Transitioner>,
}

impl Slides {
    fn from_slides(
        slides: Vec<Slide>,
        theme: Theme,
        code_box_builder: CodeBoxBuilder,
        background: Option<Texture2D>,
        automatic: Duration,
        transitioner: Option<Transitioner>,
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
            last_texture: None,
            transitioner,
        }
    }

    pub async fn load<P>(
        slides_path: PathBuf,
        theme: Theme,
        automatic: Duration,
        assets_dir: P,
    ) -> Self
    where
        P: AsRef<Path>,
    {
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

        // Load images for all slides
        for slide in &mut slides.iter_mut() {
            slide.load_images().await;
        }

        let code_box_builder =
            CodeBoxBuilder::new(theme.clone(), font_code, font_bold, font_italic);

        let transitioner = match theme.transition {
            Some(transition) => Some(Transitioner::load(assets_dir, transition, 0.1).await),
            None => None,
        };

        Self::from_slides(
            slides,
            theme.clone(),
            code_box_builder,
            background,
            automatic,
            transitioner,
        )
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
            self.update_last_texture();
            self.start_transition();
        }
    }

    pub fn prev(&mut self) {
        if self.active_slide > 0 {
            self.time = 0.;
            self.active_slide -= 1;
            self.update_last_texture();
            self.start_transition();
        }
    }

    fn start_transition(&mut self) {
        if let Some(transitioner) = &mut self.transitioner {
            transitioner.start();
        }
    }

    pub fn update(&mut self, delta: Duration) {
        if self.automatic > 0. && self.time > self.automatic {
            self.next();
        } else {
            self.time += delta;
        }
        if let Some(transitioner) = &mut self.transitioner {
            transitioner.update(delta);
        }
    }

    pub fn draw(&self) {
        self.set_camera();
        clear_background(self.theme.background_color);
        self.draw_slide();
    }

    fn draw_slide(&self) {
        if let Some(slide) = self.slides.get(self.active_slide) {
            slide.draw(self.background);
        }
    }

    pub fn texture(&mut self) -> Texture2D {
        if let Some(transitioner) = &mut self.transitioner {
            if let Some(last_texture) = self.last_texture {
                if transitioner.transitioning {
                    transitioner.draw(last_texture, self.render_target.texture);
                    return transitioner.texture();
                }
            }
        }
        self.render_target.texture
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

    fn update_last_texture(&mut self) {
        self.last_texture = Some(Texture2D::from_image(&self.texture().get_texture_data()));
    }

    fn render_target() -> RenderTarget {
        let render_target = render_target(screen_width() as u32, screen_height() as u32);
        render_target.texture.set_filter(FilterMode::Linear);
        render_target
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
}
