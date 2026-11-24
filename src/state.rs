use std::rc::Rc;

use web_sys::ImageBitmap;

use crate::templates::RenderState;
use crate::theme::Theme;

#[derive(Clone)]
pub struct AppState {
    pub template_id: String,
    pub title: String,
    pub artist: String,
    pub header1: String,
    pub header2: String,
    pub caption: String,
    pub device: String,
    pub theme: Theme,
    pub album_art: Option<Rc<ImageBitmap>>,
    pub spectrum: Vec<f32>,
    pub elapsed: f64,
    pub duration: f64,
    pub playing: bool,
    pub theme_version: u32,
    pub art_version: u32,
    pub bg_version: u32,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            template_id: "android".into(),
            title: "Judul Lagu".into(),
            artist: "Nama Artis".into(),
            header1: "Sedang Diputar".into(),
            header2: "Playlist".into(),
            caption: String::new(),
            device: String::new(),
            theme: Theme::default(),
            album_art: None,
            spectrum: vec![],
            elapsed: 0.0,
            duration: 0.0,
            playing: false,
            theme_version: 0,
            art_version: 0,
            bg_version: 0,
        }
    }
}

impl AppState {
    pub fn static_key(&self) -> u64 {
        ((self.theme_version as u64) << 32)
            | ((self.art_version as u64) << 16)
            | (self.bg_version as u64)
    }

    pub fn render_state(&self) -> RenderState<'_> {
        RenderState {
            title: &self.title,
            artist: &self.artist,
            header1: &self.header1,
            header2: &self.header2,
            caption: &self.caption,
            device: &self.device,
            elapsed: self.elapsed,
            duration: self.duration,
            playing: self.playing,
            spectrum: &self.spectrum,
            album_art: self.album_art.as_deref(),
            theme: &self.theme,
        }
    }
}
