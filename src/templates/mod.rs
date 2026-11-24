use web_sys::{CanvasRenderingContext2d, ImageBitmap};

use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Portrait,
    Landscape,
}

impl Orientation {
    pub fn aspect(&self) -> f64 {
        match self {
            Self::Portrait => 9.0 / 16.0,
            Self::Landscape => 16.0 / 9.0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FieldId {
    Title,
    Artist,
    Device,
    Header1,
    Header2,
    Caption,
}

pub struct RenderState<'a> {
    pub title: &'a str,
    pub artist: &'a str,
    pub header1: &'a str,
    pub header2: &'a str,
    pub caption: &'a str,
    pub device: &'a str,
    pub elapsed: f64,
    pub duration: f64,
    pub playing: bool,
    pub spectrum: &'a [f32],
    pub album_art: Option<&'a ImageBitmap>,
    pub theme: &'a Theme,
}

pub trait MusicTemplate: Send + Sync + 'static {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn orientation(&self) -> Orientation;
    fn visible_fields(&self) -> &'static [FieldId];
    fn render_static(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64, s: &RenderState);
    fn render_dynamic(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64, s: &RenderState);

    fn render(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64, s: &RenderState) {
        self.render_static(ctx, w, h, s);
        self.render_dynamic(ctx, w, h, s);
    }

    fn render_preview(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64) {
        let theme = Theme::default();
        let spectrum = vec![
            0.62f32, 0.81, 0.44, 0.92, 0.53, 0.71, 0.40, 0.66, 0.48, 0.75,
        ];
        let s = RenderState {
            title: "Judul Lagu",
            artist: "Nama Artis",
            header1: "Sedang Diputar",
            header2: "Playlist",
            caption: "Mendengarkan sekarang",
            device: "",
            elapsed: 42.0,
            duration: 180.0,
            playing: true,
            spectrum: &spectrum,
            album_art: None,
            theme: &theme,
        };
        self.render(ctx, w, h, &s);
    }
}

pub struct TemplateRegistry {
    templates: Vec<Box<dyn MusicTemplate>>,
}

impl TemplateRegistry {
    pub fn new() -> Self {
        let templates: Vec<Box<dyn MusicTemplate>> = vec![
            Box::new(android::AndroidTemplate),
            Box::new(ios_glass::IosGlassTemplate),
            Box::new(ig::IgTemplate),
        ];

        #[cfg(debug_assertions)]
        {
            let mut ids: Vec<&str> = templates.iter().map(|t| t.id()).collect();
            ids.sort_unstable();
            for w in ids.windows(2) {
                if w[0] == w[1] {
                    panic!("Duplicate template id: {}", w[0]);
                }
            }
        }

        Self { templates }
    }

    pub fn get(&self, id: &str) -> Option<&dyn MusicTemplate> {
        self.templates.iter().find(|t| t.id() == id).map(|b| b.as_ref())
    }

    pub fn all(&self) -> &[Box<dyn MusicTemplate>] {
        &self.templates
    }
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::new()
    }
}

pub mod android;
pub mod ig;
pub mod ios_glass;
