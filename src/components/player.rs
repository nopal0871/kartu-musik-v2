use std::cell::RefCell;
use std::rc::Rc;

use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use crate::renderer::{get_2d, AudioController, FrameScheduler, LayeredRenderer};
use crate::state::AppState;
use crate::templates::TemplateRegistry;

const PREVIEW_W: u32 = 720;
const PREVIEW_H: u32 = 1280;

#[component]
pub fn PlayerStage(
    state: RwSignal<AppState>,
    audio: RwSignal<Option<Rc<AudioController>>>,
) -> impl IntoView {
    let canvas_ref = create_node_ref::<leptos::html::Canvas>();
    let initialized = create_rw_signal(false);

    create_effect(move |_| {
        if initialized.get_untracked() {
            return;
        }
        let Some(canvas_node) = canvas_ref.get() else { return };
        initialized.set(true);

        let el: &web_sys::HtmlElement = canvas_node.as_ref();
        let canvas: HtmlCanvasElement = el.clone().unchecked_into();
        canvas.set_width(PREVIEW_W);
        canvas.set_height(PREVIEW_H);
        let ctx = get_2d(&canvas);

        let renderer = RefCell::new(LayeredRenderer::new(PREVIEW_W, PREVIEW_H));
        let registry = TemplateRegistry::new();
        let mut scheduler = FrameScheduler::new();

        let stopper = scheduler.start(move |_dt| {
            let mut snap = state.get_untracked();
            if let Some(a) = audio.get_untracked() {
                snap.elapsed = a.elapsed();
                snap.duration = a.duration();
                snap.playing = a.is_playing();
                snap.spectrum = a.sample_spectrum();
            }
            let Some(tpl) = registry.get(&snap.template_id) else { return };
            let rs = snap.render_state();
            let key = snap.static_key();
            if let Err(e) = renderer
                .borrow_mut()
                .render_frame(&ctx, tpl, &rs, key)
            {
                web_sys::console::error_1(&e);
            }
        });

        on_cleanup(move || {
            drop(stopper);
        });
    });

    view! {
        <canvas _ref=canvas_ref class="stage-canvas" />
    }
}
