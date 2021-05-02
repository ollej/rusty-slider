extern crate markdown;
pub mod shaders;
pub mod slider;

use macroquad::prelude::*;
use quad_url::get_program_parameters;
use slider::{Duration, Slides, Theme};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "rusty-slider",
    about = "A small tool to display markdown files as a slideshow."
)]
struct CliOptions {
    /// Markdown files with slides text.
    #[structopt(short, long, parse(from_os_str), default_value = "assets/slides.md")]
    pub slides: PathBuf,
    /// File with theme options.
    #[structopt(short, long, parse(from_os_str), default_value = "assets/theme.json")]
    pub theme: PathBuf,
    /// Automatically switch slides every N seconds.
    #[structopt(short, long, default_value = "0")]
    pub automatic: Duration,
    /// Take screenshot of first slide and store as PNG at path
    #[structopt(short = "S", long)]
    pub screenshot: Option<PathBuf>,
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
    let opt = CliOptions::from_iter(get_program_parameters().iter());

    let theme = Theme::load(opt.theme).await;
    debug!(
        "background_color: {:?} text_color: {:?} heading_color{:?}",
        theme.background_color, theme.text_color, theme.heading_color,
    );
    let mut shader_activated = theme.shader;
    let mut slides = Slides::load(opt.slides, theme, opt.automatic).await;

    let render_target = render_target(screen_width() as u32, screen_height() as u32);
    render_target.texture.set_filter(FilterMode::Linear);
    let shader_material = load_material(
        shaders::crt::VERTEX,
        shaders::crt::FRAGMENT,
        Default::default(),
    )
    .unwrap();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            return;
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

        let scr_w = screen_width();
        let scr_h = screen_height();

        // build camera with following coordinate system:
        // (0., 0)     .... (SCR_W, 0.)
        // (0., SCR_H) .... (SCR_W, SCR_H)
        set_camera(&Camera2D {
            zoom: vec2(1. / scr_w * 2., -1. / scr_h * 2.),
            target: vec2(scr_w / 2., scr_h / 2.),
            render_target: Some(render_target),
            ..Default::default()
        });

        slides.draw(get_frame_time());

        set_default_camera();

        clear_background(BLACK);
        if shader_activated {
            gl_use_material(shader_material);
        }
        draw_texture_ex(
            render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(scr_w, scr_h)),
                flip_y: true,
                ..Default::default()
            },
        );
        if shader_activated {
            gl_use_default_material();
        }

        if let Some(path) = opt.screenshot {
            render_target
                .texture
                .get_texture_data()
                .export_png(path.into_os_string().to_str().expect("Incorrect path"));
            return;
        }

        next_frame().await
    }
}
