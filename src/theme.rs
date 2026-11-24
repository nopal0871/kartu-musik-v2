use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::ImageBitmap;

#[derive(Clone)]
pub struct Theme {
    pub bg_top: String,
    pub bg_bottom: String,
    pub dim: f64,
    pub blur: f64,
    pub background: Option<Rc<ImageBitmap>>,
    pub background_version: u32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg_top: "#4a3a52".into(),
            bg_bottom: "#10131a".into(),
            dim: 0.2,
            blur: 0.6,
            background: None,
            background_version: 0,
        }
    }
}

impl Theme {
    pub fn invalidate(&mut self) {
        self.background_version = self.background_version.wrapping_add(1);
    }

    pub fn apply_css_vars(&self) {
        let Some(doc) = web_sys::window().and_then(|w| w.document()) else { return };
        let Some(root) = doc.document_element() else { return };
        let Some(html) = root.dyn_ref::<web_sys::HtmlElement>() else { return };
        let style = html.style();
        style.set_property("--bg-top", &self.bg_top).ok();
        style.set_property("--bg-bottom", &self.bg_bottom).ok();
    }
}
