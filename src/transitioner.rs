use crate::prelude::*;
use std::{
    fmt,
    path::{Path, PathBuf},
};
use {macroquad::prelude::*, nanoserde::DeJson};

#[derive(Copy, Clone, Debug, DeJson)]
#[allow(non_camel_case_types)]
pub enum Transitioning {
    bignoise,
    blobs,
    checkerboard,
    circleswipe,
    cubicnoise,
    curtains,
    diagonalleft,
    diagonalright,
    fade,
    fan,
    halftone,
    implode,
    lines,
    maze,
    mosaic,
    noise,
    plasma,
    radial,
    smoke,
    split,
    starburst,
    stripes,
    swipedown,
    swipeleft,
    swiperight,
    swipeup,
    swirl,
    triangles,
    vortex,
    waves,
    zebra,
}

impl Default for Transitioning {
    fn default() -> Self {
        Transitioning::split
    }
}

impl fmt::Display for Transitioning {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Transitioning {
    pub fn texture<P>(&self, directory: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        Self::with_dir(directory, self.filename())
    }

    fn filename(&self) -> String {
        format!("transitions/{}.png", self.to_string())
    }

    fn with_dir<P>(directory: P, path: String) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let mut dir = directory.as_ref().to_path_buf();
        dir.push(path);
        dir.as_path().to_owned()
    }
}

pub struct Transitioner {
    render_target: RenderTarget,
    transition: Transition,
    transition_progress: Duration,
    pub transitioning: bool,
}

impl Transitioner {
    const TRANSITIONING_TIME: Duration = 1.0;

    pub async fn load<P>(asset_dir: P, transitioning: Transitioning, fade: f32) -> Self
    where
        P: AsRef<Path>,
    {
        let texture_path = transitioning.texture(asset_dir);
        let transition_tex: Texture2D = load_texture(texture_path.to_str().unwrap()).await.unwrap();
        let transition = Transition::new(transition_tex, fade);
        let render_target = render_target(screen_width() as u32, screen_height() as u32);
        render_target.texture.set_filter(FilterMode::Linear);
        Self {
            render_target,
            transition,
            transition_progress: 0.,
            transitioning: false,
        }
    }

    pub fn start(&mut self) {
        self.transitioning = true;
    }

    pub fn update(&mut self, delta: Duration) {
        if !self.transitioning {
            return;
        }
        self.transition_progress += delta * 2.;
        if self.transition_progress > Self::TRANSITIONING_TIME {
            self.transition_progress = 0.;
            self.transitioning = false;
        }
    }

    pub fn draw(&mut self, from_texture: Texture2D, to_texture: Texture2D) {
        let scr_w = screen_width();
        let scr_h = screen_height();

        set_camera(&Camera2D {
            zoom: vec2(1. / scr_w * 2., -1. / scr_h * 2.),
            target: vec2(scr_w / 2., scr_h / 2.),
            render_target: Some(self.render_target),
            ..Default::default()
        });
        self.transition
            .draw(to_texture, from_texture, self.transition_progress);
    }

    pub fn texture(&self) -> Texture2D {
        self.render_target.texture
    }
}
