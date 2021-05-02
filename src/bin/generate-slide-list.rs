use glob::glob;
use macroquad::prelude::*;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use rusty_slider::slider::{Slides, Theme};
use std::borrow::Cow;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "generate-slide-list",
    about = "Generates an html file with links to Rusty Slider slideshows."
)]
struct CliOptions {
    /// Path to directory with slides
    #[structopt(short, long, parse(from_os_str), default_value = "./assets/")]
    pub path: PathBuf,
    /// Path to where html file will be saved
    #[structopt(
        short,
        long,
        parse(from_os_str),
        default_value = "./assets/slides.html"
    )]
    pub output: PathBuf,
}

fn stylesheet() -> &'static str {
    r#"
    @font-face{
        font-family: 'Amble';
        src: url('assets/Amble.woff') format('woff');
    }
    body {
        font-family: 'Amble'
    }
    h1 {
        text-align: center;
        font-size: 3em;
    }
    ul {
        display: flex;
        flex-wrap: wrap;
        justify-content: flex-start;
        list-style: none;
        margin: 0;
        padding: 0;
    }
    a {
        color: black;
        text-decoration: none;
    }
    img {
        max-width: 20rem;
    }
    figcaption {
        text-align: center;
    }
    footer {
        text-align: center;
        font-size: small;
    }
    "#
}

fn javascript() -> &'static str {
    r#"
    function change_theme(el) {
        let url = new URL(el.href);
        let query = new URLSearchParams(url.search);
        const theme = document.getElementById("theme").selectedOptions[0].value;
        query.set("theme", theme);
        url.search = query;
        el.href = url;
    }
    "#
}

fn header(page_title: &str) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            title { (page_title) }
            meta name="viewport" content="width=device-width";
            style { (PreEscaped(stylesheet())) }
            script { (PreEscaped(javascript())) }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer {
            p {
                a href="https://github.com/ollej/rusty-slider" { "Copyright 2021 Olle Wreede" }
            }
        }
    }
}

pub fn page(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            (header(title))
            body {
                h1 { (title) }
                (content)
                (footer())
            }
        }
    }
}

struct Filename {
    path: PathBuf,
}

impl Filename {
    fn href(&self, theme: Option<&Filename>) -> String {
        format!(
            "index.html?slides={}&theme={}",
            self.path(),
            match theme {
                Some(t) => t.path(),
                None => "",
            }
        )
    }

    fn files(path: &PathBuf, extension: &str) -> Vec<Self> {
        glob(&format!("{}/*.{}", path.to_string_lossy(), extension))
            .expect("Couldn't read files")
            .filter(|entry| entry.is_ok())
            .map(|entry| Filename {
                path: entry.unwrap(),
            })
            .collect()
    }

    fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }

    fn png_path(&self) -> PathBuf {
        let mut path = self.path.to_owned();
        path.set_extension("png");
        path
    }

    fn name(&self) -> Cow<str> {
        match self.path.file_stem() {
            Some(name) => name.to_string_lossy(),
            None => Cow::from(""),
        }
    }
}

#[macroquad::main("generate-slide-list")]
async fn main() {
    let opt = CliOptions::from_args();

    let slides = Filename::files(&opt.path, "md");
    let themes = Filename::files(&opt.path, "json");

    let render_target = render_target(screen_width() as u32, screen_height() as u32);
    render_target.texture.set_filter(FilterMode::Linear);
    let theme = Theme::load(PathBuf::from("assets/theme.json")).await;
    let scr_w = screen_width();
    let scr_h = screen_height();
    for slide in slides.iter() {
        set_camera(&Camera2D {
            zoom: vec2(1. / scr_w * 2., -1. / scr_h * 2.),
            target: vec2(scr_w / 2., scr_h / 2.),
            render_target: Some(render_target),
            ..Default::default()
        });
        let mut slideshow = Slides::load(slide.path.to_owned(), theme.clone(), 0.0).await;
        slideshow.draw(0.);
        set_default_camera();
        render_target
            .texture
            .get_texture_data()
            .export_png(&slide.png_path().to_string_lossy());
    }

    let html = page(
        "Rusty Slider example slideshows",
        html! {
            form {
                label for="theme" { "Theme:" }
                select#theme {
                    @for theme in &themes {
                        option value=(theme.path()) {
                            (theme.name())
                        }
                    }
                }
            }
            ul {
                @for slide in &slides {
                    li {
                        a href=(PreEscaped(slide.href(themes.first()))) onclick="change_theme(this)" {
                            figure {
                                img src=(slide.png_path().to_string_lossy()) title=(slide.name());
                                figcaption { (slide.name()) }
                            }
                        }
                    }
                }
            }
        },
    );
    println!("{}", html.into_string());
}
