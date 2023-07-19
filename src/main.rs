#![windows_subsystem = "windows"]

extern crate markdown;

use {clap::Parser, macroquad::prelude::*, quad_url::get_program_parameters};

use rusty_slider::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty Slider".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let options = AppOptions::parse_from(get_program_parameters().iter());

    let theme = Theme::load(options.theme_path()).await;
    debug!(
        "background_color: {:?} text_color: {:?} heading_color{:?}",
        theme.background_color, theme.text_color, theme.heading_color,
    );
    let mut shader_activated = theme.shader;
    let mut slides = Slides::load(options.clone(), theme).await;
    let mut show_help = ShowHelp::new();
    let shader_material = load_material(
        ShaderSource {
            glsl_vertex: Some(crt::VERTEX),
            glsl_fragment: Some(crt::FRAGMENT),
            metal_shader: None,
        },
        Default::default(),
    )
    .unwrap();

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
        if is_key_pressed(KeyCode::Up)
            || is_key_pressed(KeyCode::K)
            || is_key_pressed(KeyCode::Home)
        {
            slides.first();
        }
        if is_key_pressed(KeyCode::Down)
            || is_key_pressed(KeyCode::J)
            || is_key_pressed(KeyCode::End)
        {
            slides.last();
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
        if options.enable_code_execution && is_key_pressed(KeyCode::Enter) {
            slides.run_code_block();
        }

        slides.update(get_frame_time());
        slides.draw();
        let texture = slides.texture();

        set_camera(&Camera2D::from_display_rect(Rect::new(
            0.,
            0.,
            screen_width(),
            screen_height(),
        )));
        clear_background(BLACK);
        if shader_activated {
            gl_use_material(&shader_material);
        }
        draw_texture_ex(
            &texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        if shader_activated {
            gl_use_default_material();
        }
        show_help.draw();

        if is_key_pressed(KeyCode::S) {
            get_screen_data().export_png(&options.screenshot.to_string_lossy());
        }

        next_frame().await
    }
}
