use glob::glob;
use maud::{html, Markup, DOCTYPE};
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
    pub slides: PathBuf,
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

struct Slide {
    path: PathBuf,
}

impl Slide {
    fn href(&self) -> String {
        format!("index.html?slides={}", self.path.to_str().unwrap())
    }

    fn name(&self) -> Cow<str> {
        match self.path.file_stem() {
            Some(name) => name.to_string_lossy(),
            None => Cow::from(""),
        }
    }
}

fn main() {
    let opt = CliOptions::from_args();
    let path = format!(
        "{}/*.md",
        opt.slides
            .into_os_string()
            .to_str()
            .expect("Incorrect slides path")
    );
    let mut slides = vec![];
    for entry in glob(&path).expect("Couldn't read slides") {
        match entry {
            Ok(path) => slides.push(Slide { path }),
            Err(e) => println!("{:?}", e),
        }
    }

    let html = page(
        "Rusty Slider",
        html! {
            ul {
                @for slide in &slides {
                    li {
                        a href=(slide.href()) {
                            (slide.name())
                        }
                    }
                }
            }
        },
    );
    println!("{}", html.into_string());
}
