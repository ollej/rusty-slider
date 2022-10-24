use {
    glob::glob,
    std::{
        borrow::Cow,
        error::Error,
        fs::File,
        io::prelude::*,
        path::{Path, PathBuf},
        process::Command,
    },
    structopt::StructOpt,
};

use convert_case::{Case, Casing};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use tempfile::tempdir;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "generate-slide-list",
    about = "Generates an html file with links to Rusty Slider slideshows."
)]
struct CliOptions {
    /// Path to directory with slides
    #[structopt(short, long, parse(from_os_str), default_value = "./assets/")]
    pub path: PathBuf,
    /// Path to where html file and screenshots will be saved
    #[structopt(short, long, parse(from_os_str), default_value = "./demo/")]
    pub output: PathBuf,
    /// Regenerate screenshots
    #[structopt(short, long)]
    pub screenshots: bool,
}

fn stylesheet() -> &'static str {
    r#"
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
    @media screen and (min-width: 35.5em) {
      img.thumbnail {
          max-width: 17.5rem;
      }
    }
    img.thumbnail {
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
    r##"
    function url_for(href, slideshow, theme) {
        let url = new URL(href);
        let query = new URLSearchParams(url.search);
        query.set("theme", theme + ".json");
        query.set("slides", slideshow + ".md");
        url.search = query;
        return url;
    }
    function choose_theme(el) {
        const theme = el.dataset.theme;
        update_slideshow_thumbnails(theme);
        update_slideshow_href(theme);
        setTimeout(function () {
            document.getElementById("slideshows-heading").scrollIntoView({
              behavior: "smooth"
            });
        }, 1);
        return false;
    }
    function update_slideshow_href(theme) {
        const links = document.querySelectorAll("#slideshows a");
        for (var link of links) {
            const url = url_for(link.href, link.getAttribute("data-slideshow"), theme);
            console.log("update href", theme, link.dataset.slideshow, url);
            link.setAttribute("href", url);
        }
    }
    function update_slideshow_thumbnails(theme) {
        const thumbnails = document.getElementsByClassName("slideshow");
        for (var thumbnail of thumbnails) {
            const path = `assets/${theme}-${thumbnail.dataset.slideshow}.png`;
            thumbnail.setAttribute("src", path);
        }
    }
    "##
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
                        h1 class="title" { (title) }
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

fn generate_html(slides: Files, themes: Files) -> PreEscaped<String> {
    page(
        "rusty slider",
        html! {
            h2 { "Themes" }
            p { "These are the themes included with Rusty Slider. You can also create your own." }
            ul id="themes" class="thumbnails" {
                @for theme in &themes {
                    li {
                        a href="#" onclick="choose_theme(this)" data-theme=(theme.name()) {
                            figure {
                                img class="thumbnail theme" src=(theme.theme_thumbnail_path("assets/"));
                                figcaption { (theme.display_name()) }
                            }
                        }
                    }
                }
            }
            h2 id="slideshows-heading" { "Slideshows" }
            p { "Click on a presentation to run it with the selected theme." }
            ul id="slideshows" class="thumbnails" {
                @for slide in &slides {
                    li {
                        a href=(PreEscaped(slide.href("default-theme.json"))) data-slideshow=(slide.name()) {
                            figure {
                                img class="thumbnail slideshow" src=(slide.default_thumbnail_path()) data-slideshow=(slide.name());
                                figcaption { (slide.display_name()) }
                            }
                        }
                    }
                }
            }
        },
    )
}

type Files = Vec<Filename>;

#[derive(Debug, Clone)]
struct Filename {
    path: PathBuf,
}

impl Filename {
    fn href(&self, theme: &str) -> String {
        format!("index.html?slides={}&theme={}", self.filename(), theme)
    }

    fn files(path: &Path, extension: &str) -> Files {
        let mut files: Vec<PathBuf> = glob(&format!("{}/*.{}", path.to_string_lossy(), extension))
            .expect("Couldn't read files")
            .filter_map(Result::ok)
            .collect();
        files.sort();
        files.into_iter().map(|path| Filename { path }).collect()
    }

    fn filename(&self) -> String {
        self.path
            .as_path()
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

    fn default_thumbnail_path(&self) -> String {
        let thumbnail_path = self.png_path();
        let filename = thumbnail_path.file_name().unwrap().to_string_lossy();
        format!("assets/default-theme-{}", filename)
    }

    fn theme_thumbnail_path(&self, directory: &str) -> String {
        format!(
            "{}thumbnail-{}",
            directory,
            self.png_path().file_name().unwrap().to_string_lossy()
        )
    }

    fn name(&self) -> Cow<str> {
        match self.path.file_stem() {
            Some(name) => name.to_string_lossy(),
            None => Cow::from(""),
        }
    }

    fn display_name(&self) -> String {
        self.name().to_string().to_case(Case::Title)
    }
}

fn take_screenshot(slideshow: String, theme: String, filename: PathBuf) {
    //eprintln!(
    //    "RUSTFLAGS=--cfg one_screenshot cargo run -- --slides {} --theme {}",
    //    slideshow, theme
    //);
    Command::new("cargo")
        .args(&["run", "--", "--slides", &slideshow, "--theme", &theme])
        .env("RUSTFLAGS", "--cfg one_screenshot")
        .current_dir("./")
        .output()
        .unwrap();
    std::fs::copy("./screenshot.png", filename).unwrap();
}

fn build_path(output_path: &PathBuf, filename: String) -> PathBuf {
    let mut screenshot_path = output_path.clone();
    screenshot_path.push("assets");
    screenshot_path.push(filename);
    screenshot_path
}

fn generate_screenshots(slides: Files, themes: Files, output_path: &PathBuf) {
    for theme in themes.iter() {
        generate_theme_slideshow(theme, output_path);
        for slide in slides.iter() {
            let screenshot_path = build_path(
                output_path,
                format!("{}-{}.png", theme.name(), slide.name()),
            );
            take_screenshot(slide.filename(), theme.filename(), screenshot_path);
        }
    }
}

fn generate_theme_slideshow(theme: &Filename, output_path: &PathBuf) {
    let temp_dir = tempdir().expect("Failed creating tempdir");
    let theme_filename = format!("{}.md", theme.name());
    let slideshow_path = temp_dir.path().join(theme_filename);
    let mut theme_file = File::create(&slideshow_path).expect("Failed creating theme slideshow");
    writeln!(theme_file, "# {}", theme.display_name()).expect("Couldn't write theme slideshow");
    let theme_slideshow_thumbnail = build_path(output_path, theme.theme_thumbnail_path(""));
    take_screenshot(
        slideshow_path.to_string_lossy().to_string(),
        theme.filename(),
        theme_slideshow_thumbnail,
    );
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = CliOptions::from_args();

    let slides = Filename::files(&opt.path, "md");
    let themes = Filename::files(&opt.path, "json");

    if opt.screenshots {
        generate_screenshots(slides.clone(), themes.clone(), &opt.output);
    }

    let html = generate_html(slides, themes);

    let mut output_path = opt.output;
    output_path.push("example-slideshows.html");
    File::create(output_path)?.write_all(html.into_string().as_bytes())?;

    Ok(())
}
