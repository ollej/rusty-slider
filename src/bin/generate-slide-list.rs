use glob::glob;
use maud::{html, Markup, PreEscaped, DOCTYPE};
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
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
                a href="https://github.com/ollej/rusty-slider" { "Copyright 2022 Olle Wreede" }
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

#[derive(Debug)]
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

    fn files(path: &Path, extension: &str) -> HashMap<String, Self> {
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

fn main() {
    let opt = CliOptions::from_args();

    let slides = Filename::files(&opt.path, "md");
    let themes = Filename::files(&opt.path, "json");

    for (_, slide) in slides.iter() {
        let theme_path = if let Some(filename) = themes.get(&slide.basename()) {
            filename.filename()
        } else {
            "default-theme.json".to_string()
        };
        take_screenshot(
            slide.filename(),
            theme_path,
            slide.png_path().to_string_lossy().into(),
        );
    }

    let html = page(
        "Rusty Slider example slideshows",
        html! {
            form {
                label for="theme" { "Theme:" }
                select #theme {
                    @for (_, theme) in &themes {
                        option value=(theme.filename()) {
                            (theme.name())
                        }
                    }
                }
            }
            ul {
                @for (_, slide) in &slides {
                    li {
                        a href=(PreEscaped(slide.href(themes.get(&slide.basename())))) onclick="change_theme(this)" {
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
