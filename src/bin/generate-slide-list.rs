use glob::glob;
use macroquad::rand::gen_range;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use std::{
    borrow::Cow,
    collections::HashMap,
    error::Error,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
    process::Command,
};
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
        default_value = "./demo/example-slideshows.html"
    )]
    pub output: PathBuf,
    /// Regenerate screenshots
    #[structopt(short, long)]
    pub screenshots: bool,
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
    .theme-chooser {
        text-align: center;
    }
    .theme-chooser label {
        margin-right: 0.5rem;
    }
    ul.thumbnails {
        display: flex;
        flex-wrap: wrap;
        justify-content: flex-start;
        list-style: none;
        margin: 0;
        padding: 0;
    }
    img.thumbnail {
        max-width: 17.5rem;
        border: solid thin black;
    }
    ul.thumbnails figure {
        margin: 1rem;
    }
    figcaption {
        text-align: center;
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
            link rel="stylesheet" href="https://ollej.github.io/rusty-slider/assets/css/style.css";
            style { (PreEscaped(stylesheet())) }
            script { (PreEscaped(javascript())) }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer {
            div class="owner" {
                p {
                    a href="https://github.com/ollej" class="avatar" {
                        img src="https://github.com/ollej.png" width="48" height="48";
                    }
                    a href="https://github.com/ollej" { "ollej" }
                    " maintains "
                    a href="https://github.com/ollej/rusty-slider" { "Rusty Slider" }
                }
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
                div class="wrapper" {
                    header {
                        h1 { (title) }
                    }
                    div id="container" {
                        p class="tagline" {
                            "A list of example slideshows made with Rusty Slider."
                        }
                        div id="main" role="main" {
                            div class="download-bar" {
                                div class="inner" {
                                    a href="https://github.com/ollej/rusty-slider" class="code" {
                                        "View rusty-slider on GitHub"
                                    }
                                }
                                span class="blc" {}
                                span class="trc" {}
                            }
                            article class="markdown-body" {
                                (content)
                            }
                        }
                    }
                    (footer())
                }
            }
        }
    }
}

type Files = HashMap<String, Filename>;

#[derive(Debug, Clone)]
struct Filename {
    path: PathBuf,
}

impl Filename {
    fn href(&self, theme: Option<&Filename>) -> String {
        format!(
            "index.html?slides={}&theme={}",
            self.filename(),
            match theme {
                Some(t) => t.filename(),
                None => "".to_string(),
            }
        )
    }

    fn files(path: &Path, extension: &str) -> Files {
        glob(&format!("{}/*.{}", path.to_string_lossy(), extension))
            .expect("Couldn't read files")
            .filter(|entry| entry.is_ok())
            .map(|entry| {
                let file = Filename {
                    path: entry.unwrap(),
                };
                (file.basename(), file)
            })
            .collect()
    }

    fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }

    fn filename(&self) -> String {
        self.path
            .as_path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into()
    }

    fn basename(&self) -> String {
        self.path
            .as_path()
            .with_extension("")
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into()
    }
    fn png_path(&self) -> PathBuf {
        let mut path = self.path.to_owned();
        path.set_extension("png");
        path
    }

    fn with_extension(&self, extension: &str) -> PathBuf {
        let mut path = self.path.to_owned();
        path.set_extension(extension);
        path
    }

    fn name(&self) -> Cow<str> {
        match self.path.file_stem() {
            Some(name) => name.to_string_lossy(),
            None => Cow::from(""),
        }
    }
}

fn take_screenshot(slideshow: String, theme: String, filename: String) {
    Command::new("cargo")
        .args(&["run", "--", "--slides", &slideshow, "--theme", &theme])
        .env("RUSTFLAGS", "--cfg one_screenshot")
        .current_dir("./")
        .output()
        .unwrap();
    std::fs::copy("./screenshot.png", filename).unwrap();
}

fn generate_screenshots(slides: Files, themes: Files) {
    for (_, slide) in slides.iter() {
        let theme_path = if let Some(filename) = themes.get(&slide.basename()) {
            filename.filename()
        } else {
            let mut keys = themes.keys();
            let random = gen_range(0, keys.len());
            keys.nth(random)
                .map(|k| themes.get(k).map(|t| t.filename()))
                .flatten()
                .unwrap_or_else(|| "default-theme.json".to_string())
        };
        take_screenshot(
            slide.filename(),
            theme_path,
            slide.png_path().to_string_lossy().into(),
        );
    }
}

fn generate_html(slides: Files, themes: Files) -> PreEscaped<String> {
    page(
        "Rusty Slider Example Slideshows",
        html! {
            form class="theme-chooser" {
                label for="theme" { "Choose theme to view slideshow with: " }
                select #theme {
                    @for (_, theme) in &themes {
                        option value=(theme.filename()) {
                            (theme.name())
                        }
                    }
                }
            }
            ul class="thumbnails" {
                @for (_, slide) in &slides {
                    li {
                        a href=(PreEscaped(slide.href(themes.get(&slide.basename())))) onclick="change_theme(this)" {
                            figure {
                                img class="thumbnail" src=(slide.png_path().to_string_lossy()) title=(slide.name());
                                figcaption { (slide.name()) }
                            }
                        }
                    }
                }
            }
        },
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = CliOptions::from_args();

    let slides = Filename::files(&opt.path, "md");
    let themes = Filename::files(&opt.path, "json");

    if opt.screenshots {
        generate_screenshots(slides.clone(), themes.clone());
    }

    let html = generate_html(slides, themes);

    File::create(opt.output)?.write_all(html.into_string().as_bytes())?;

    Ok(())
}
