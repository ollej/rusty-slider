use std::path::PathBuf;

use clap::{command, Parser};

use crate::prelude::Duration;

#[derive(Parser, Debug, Clone)]
#[command(
    name = "rusty-slider",
    about = "A small tool to display markdown files as a slideshow."
)]
pub struct AppOptions {
    /// Path to directory to load slideshow files from
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
    /// Path to directory where application files are loaded from
    #[arg(short = 'A', long, default_value = "assets")]
    pub assets: PathBuf,
    /// Slide number to start at
    #[arg(short = 'n', long, default_value = "1", value_parser = clap::value_parser!(u32).range(1..))]
    pub number: u32,
}

impl AppOptions {
    pub fn slides_path(&self) -> PathBuf {
        let mut path = self.directory.clone();
        path.push(self.slides.clone());
        path
    }

    pub fn theme_path(&self) -> PathBuf {
        let mut path = self.directory.clone();
        path.push(self.theme.clone());
        path
    }
}
