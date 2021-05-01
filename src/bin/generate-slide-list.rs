use glob::glob;
use maud::{html, Markup, PreEscaped, DOCTYPE};
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

fn header(page_title: &str) -> Markup {
    html! {
        head {
            meta charset="utf-8";
            title { (page_title) }
            meta name="viewport" content="width=device-width";
            link rel="stylesheet" href="stylesheet.css";
            script { (PreEscaped(javascript())) }
        }
    }
}

fn footer() -> Markup {
    html! {
        footer {
            a href="https://github.com/ollej/rusty-slider" { "Source Code" }
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

    fn name(&self) -> Cow<str> {
        match self.path.file_stem() {
            Some(name) => name.to_string_lossy(),
            None => Cow::from(""),
        }
    }
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

fn main() {
    let opt = CliOptions::from_args();

    let slides = Filename::files(&opt.path, "md");
    let themes = Filename::files(&opt.path, "json");

    let html = page(
        "Rusty Slider",
        html! {
            form {
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
                            (slide.name())
                        }
                    }
                }
            }
        },
    );
    println!("{}", html.into_string());
}
