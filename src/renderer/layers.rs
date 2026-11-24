use std::cell::RefCell;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::templates::{MusicTemplate, RenderState};

pub struct LayeredRenderer {
    pub width: u32,
    pub height: u32,
    static_canvas: HtmlCanvasElement,
    static_ctx: CanvasRenderingContext2d,
    dynamic_canvas: HtmlCanvasElement,
    dynamic_ctx: CanvasRenderingContext2d,
    cached_key: Option<u64>,
}

pub fn get_2d(canvas: &HtmlCanvasElement) -> CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|c| c.dyn_into::<CanvasRenderingContext2d>().ok())
        .expect("2d context unavailable")
}

fn make_canvas(w: u32, h: u32) -> HtmlCanvasElement {
    let doc = web_sys::window().unwrap().document().unwrap();
    let c = doc
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();
    c.set_width(w);
    c.set_height(h);
    c
}

impl LayeredRenderer {
    pub fn new(width: u32, height: u32) -> Self {
        let static_canvas = make_canvas(width, height);
        let dynamic_canvas = make_canvas(width, height);
        let static_ctx = get_2d(&static_canvas);
        let dynamic_ctx = get_2d(&dynamic_canvas);
        Self {
            width,
            height,
            static_canvas,
            static_ctx,
            dynamic_canvas,
            dynamic_ctx,
            cached_key: None,
        }
    }

    pub fn ensure_static(&mut self, template: &dyn MusicTemplate, state: &RenderState, key: u64) {
        if self.cached_key == Some(key) {
            return;
        }
        self.static_ctx
            .clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        template.render_static(&self.static_ctx, self.width as f64, self.height as f64, state);
        self.cached_key = Some(key);
    }

    pub fn render_frame(
        &mut self,
        target: &CanvasRenderingContext2d,
        template: &dyn MusicTemplate,
        state: &RenderState,
        key: u64,
    ) -> Result<(), JsValue> {
        self.ensure_static(template, state, key);

        self.dynamic_ctx
            .clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        template.render_dynamic(&self.dynamic_ctx, self.width as f64, self.height as f64, state);

        target.clear_rect(0.0, 0.0, self.width as f64, self.height as f64);
        target.draw_image_with_html_canvas_element_and_dw_and_dh(
            &self.static_canvas,
            0.0,
            0.0,
            self.width as f64,
            self.height as f64,
        )?;
        target.draw_image_with_html_canvas_element_and_dw_and_dh(
            &self.dynamic_canvas,
            0.0,
            0.0,
            self.width as f64,
            self.height as f64,
        )?;
        Ok(())
    }

    pub fn invalidate(&mut self) {
        self.cached_key = None;
    }
}

pub type SharedRenderer = std::rc::Rc<RefCell<LayeredRenderer>>;
