#![windows_subsystem = "windows"]

extern crate markdown;

use std::path::PathBuf;
use {clap::Parser, macroquad::prelude::*, quad_url::get_program_parameters};

use rusty_slider::prelude::*;

#[derive(Parser, Debug)]
#[command(
    name = "rusty-slider",
    about = "A small tool to display markdown files as a slideshow."
)]
struct CliOptions {
    /// Path to directory to load files from
    #[arg(short, long, default_value = "assets")]
    pub directory: PathBuf,
    /// Markdown files with slides text.
    #[arg(short, long, default_value = "rusty-slider.md")]
    pub slides: PathBuf,
    /// File with theme options.
    #[arg(short, long, default_value = "default-theme.json")]
    pub theme: PathBuf,
    /// Automatically switch slides every N seconds.
    #[arg(short, long, default_value = "0")]
    pub automatic: Duration,
    /// Switch transitions for every slide
    #[arg(long)]
    pub demo_transitions: bool,
    /// When taking screenshot, store PNG at this path
    #[arg(short = 'S', long, default_value = "screenshot.png")]
    pub screenshot: PathBuf,
    /// Enable executing code in code blocks
    #[arg(long)]
    pub enable_code_execution: bool,
}

impl CliOptions {
    fn slides_path(&self) -> PathBuf {
        let mut path = self.directory.clone();
        path.push(self.slides.clone());
        path
    }

    fn theme_path(&self) -> PathBuf {
        let mut path = self.directory.clone();
        path.push(self.theme.clone());
        path
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Slider".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let opt = CliOptions::parse_from(get_program_parameters().iter());

    let theme = Theme::load(opt.theme_path()).await;
    debug!(
        "background_color: {:?} text_color: {:?} heading_color{:?}",
        theme.background_color, theme.text_color, theme.heading_color,
    );
    let mut shader_activated = theme.shader;
    let mut slides = Slides::load(
        opt.slides_path(),
        theme,
        opt.automatic,
        opt.demo_transitions,
        opt.directory,
    )
    .await;
    let mut show_help = ShowHelp::new();
    let shader_material = load_material(crt::VERTEX, crt::FRAGMENT, Default::default()).unwrap();

    loop {
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_pressed(KeyCode::Q) | is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Left)
            || is_key_pressed(KeyCode::H)
            || is_mouse_button_pressed(MouseButton::Right)
        {
            slides.prev();
        }
        if is_key_pressed(KeyCode::Right)
            || is_key_pressed(KeyCode::L)
            || is_mouse_button_pressed(MouseButton::Left)
        {
            slides.next();
        }
        if is_key_pressed(KeyCode::Space) {
            shader_activated = !shader_activated;
        }
        if is_key_pressed(KeyCode::C) {
            slides.copy_codeblock();
        }
        match get_char_pressed() {
            Some('?') => show_help.toggle_show(),
            _ => (),
        }
        #[cfg(not(target_arch = "wasm32"))]
        if opt.enable_code_execution && is_key_pressed(KeyCode::Enter) {
            slides.run_code_block();
        }

        slides.update(get_frame_time());
        slides.draw();
        let texture = slides.texture();

        set_default_camera();
        clear_background(BLACK);
        if shader_activated {
            gl_use_material(shader_material);
        }
        draw_texture_ex(
            texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );
        if shader_activated {
            gl_use_default_material();
        }
        show_help.draw();

        if is_key_pressed(KeyCode::S) {
            get_screen_data().export_png(&opt.screenshot.to_string_lossy());
        }

        next_frame().await
    }
}
