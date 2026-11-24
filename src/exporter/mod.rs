pub mod mediarecorder;

use wasm_bindgen::prelude::*;
use web_sys::Blob;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ExportFormat {
    Mp4,
    Webm,
}

#[derive(Clone)]
pub struct ExportOptions {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub format: ExportFormat,
    pub video_bps: u32,
}

impl ExportOptions {
    pub fn from_resolution(res: u32) -> Self {
        let height = res * 16 / 9;
        Self {
            width: res,
            height,
            fps: 30,
            format: ExportFormat::Webm,
            video_bps: res * height * 30 / 8,
        }
    }
}

pub struct ExportProgress {
    pub frame: u64,
    pub total_frames: u64,
    pub phase: &'static str,
}

pub type ProgressCb = Box<dyn FnMut(ExportProgress) + 'static>;

pub async fn export(
    template_id: String,
    state: crate::state::AppState,
    audio_bytes: Vec<u8>,
    opts: ExportOptions,
    on_progress: ProgressCb,
) -> Result<Blob, JsValue> {
    mediarecorder::export(template_id, state, audio_bytes, opts, on_progress).await
}

pub(crate) fn pick_mime() -> &'static str {
    let candidates = [
        "video/mp4;codecs=avc1.42001f,mp4a.40.2",
        "video/mp4;codecs=h264,aac",
        "video/mp4",
        "video/webm;codecs=vp9,opus",
        "video/webm;codecs=vp8,opus",
        "video/webm",
    ];
    for m in candidates {
        if web_sys::MediaRecorder::is_type_supported(m) {
            return m;
        }
    }
    "video/webm"
}

pub(crate) fn make_canvas(w: u32, h: u32) -> web_sys::HtmlCanvasElement {
    use wasm_bindgen::JsCast;
    let doc = web_sys::window().unwrap().document().unwrap();
    let c = doc
        .create_element("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    c.set_width(w);
    c.set_height(h);
    c
}

pub(crate) async fn wait_frame() {
    use wasm_bindgen::JsCast;
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let cb = Closure::once(move || {
            resolve.call0(&JsValue::UNDEFINED).ok();
        });
        web_sys::window()
            .unwrap()
            .request_animation_frame(cb.as_ref().unchecked_ref())
            .ok();
        cb.forget();
    });
    wasm_bindgen_futures::JsFuture::from(promise).await.ok();
}
