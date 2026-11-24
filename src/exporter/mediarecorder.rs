use std::cell::RefCell;
use std::rc::Rc;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::*;

use super::{make_canvas, pick_mime, wait_frame, ExportOptions, ExportProgress, ProgressCb};
use crate::renderer::{get_2d, LayeredRenderer};
use crate::templates::TemplateRegistry;

#[wasm_bindgen(module = "/js/media-recorder-bridge.js")]
extern "C" {
    fn createRecorder(
        stream: &MediaStream,
        mime: &str,
        bps: u32,
    ) -> MediaRecorder;
}

pub async fn export(
    template_id: String,
    mut state: crate::state::AppState,
    audio_bytes: Vec<u8>,
    opts: ExportOptions,
    mut on_progress: ProgressCb,
) -> Result<Blob, JsValue> {
    on_progress(ExportProgress {
        frame: 0,
        total_frames: 0,
        phase: "audio",
    });

    let audio_ctx = AudioContext::new()?;
    let arr = Uint8Array::from(audio_bytes.as_slice());
    let decoded = JsFuture::from(audio_ctx.decode_audio_data(&arr.buffer())?).await?;
    let audio_buffer: AudioBuffer = decoded.dyn_into()?;
    let duration = audio_buffer.duration();
    let total_frames = (duration * opts.fps as f64).ceil() as u64;

    let canvas = make_canvas(opts.width, opts.height);
    let ctx = get_2d(&canvas);
    let mut renderer = LayeredRenderer::new(opts.width, opts.height);
    let registry = TemplateRegistry::new();
    let tpl = registry
        .get(&template_id)
        .ok_or_else(|| JsValue::from_str("template tidak ditemukan"))?;

    let actx = AudioContext::new()?;
    let dest = actx.create_media_stream_destination()?;
    let src = actx.create_buffer_source()?;
    src.set_buffer(Some(&audio_buffer));
    src.connect_with_audio_node(&dest)?;
    let audio_stream = dest.stream();

    let stream = canvas.capture_stream_with_frame_rate(opts.fps as f64)?;
    let audio_tracks = audio_stream.get_audio_tracks();
    for i in 0..audio_tracks.length() {
        if let Ok(t) = audio_tracks.get(i).dyn_into::<MediaStreamTrack>() {
            stream.add_track(&t);
        }
    }

    let mime = pick_mime();
    let recorder = createRecorder(&stream, mime, opts.video_bps);

    let chunks: Rc<RefCell<Vec<Blob>>> = Rc::new(RefCell::new(Vec::new()));
    let chunks_cb = chunks.clone();
    let on_data = Closure::<dyn FnMut(BlobEvent)>::new(move |e: BlobEvent| {
        let data = e.data();
        if data.size() > 0.0 {
            chunks_cb.borrow_mut().push(data);
        }
    });
    recorder.set_ondataavailable(Some(on_data.as_ref().unchecked_ref()));
    on_data.forget();

    let stop_promise = js_sys::Promise::new(&mut |resolve, _| {
        let rec_clone = recorder.clone();
        let cb = Closure::once(move || {
            resolve.call0(&JsValue::UNDEFINED).ok();
        });
        rec_clone.set_onstop(Some(cb.as_ref().unchecked_ref()));
        cb.forget();
    });

    actx.resume().ok();
    src.start()?;
    recorder.start()?;

    let start_time = actx.current_time();
    let mut frame_idx: u64 = 0;

    while (actx.current_time() - start_time) < duration {
        let now = actx.current_time() - start_time;
        state.elapsed = now;
        state.playing = true;
        let rs = state.render_state();
        let key = state.static_key();
        renderer.render_frame(&ctx, tpl, &rs, key)?;

        frame_idx += 1;
        if frame_idx % 30 == 0 {
            on_progress(ExportProgress {
                frame: frame_idx,
                total_frames,
                phase: "video",
            });
        }
        wait_frame().await;
    }

    on_progress(ExportProgress {
        frame: total_frames,
        total_frames,
        phase: "mux",
    });
    recorder.stop()?;
    JsFuture::from(stop_promise).await.ok();

    let parts = js_sys::Array::new();
    for b in chunks.borrow().iter() {
        parts.push(b);
    }
    let blob = Blob::new_with_blob_sequence(&parts)?;

    let _ = actx.close();
    let _ = audio_ctx.close();

    on_progress(ExportProgress {
        frame: total_frames,
        total_frames,
        phase: "done",
    });
    Ok(blob)
}
