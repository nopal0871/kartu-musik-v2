use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlImageElement, ImageBitmap};

#[derive(Clone)]
pub struct CropResult {
    pub bitmap: ImageBitmap,
    pub data_url: String,
}

#[component]
pub fn CropSheet(
    open: RwSignal<bool>,
    image_url: RwSignal<Option<String>>,
    aspect: RwSignal<f64>,
    title: &'static str,
    on_apply: Callback<CropResult>,
) -> impl IntoView {
    let zoom = create_rw_signal(1.0f64);
    let offset_x = create_rw_signal(0.0f64);
    let offset_y = create_rw_signal(0.0f64);
    let dragging = create_rw_signal(false);
    let drag_start = create_rw_signal((0.0f64, 0.0f64, 0.0f64, 0.0f64));
    let view_ref = create_node_ref::<leptos::html::Div>();
    let img_ref = create_node_ref::<leptos::html::Img>();

    create_effect(move |_| {
        if open.get() {
            zoom.set(1.0);
            offset_x.set(0.0);
            offset_y.set(0.0);
        }
    });

    let on_wheel = move |ev: web_sys::WheelEvent| {
        ev.prevent_default();
        let delta = -ev.delta_y() * 0.001;
        zoom.update(|z| *z = (*z + delta).clamp(1.0, 4.0));
    };

    let on_pointer_down = move |ev: web_sys::PointerEvent| {
        let Some(v) = view_ref.get() else { return };
        let el: &HtmlElement = v.as_ref();
        el.set_pointer_capture(ev.pointer_id()).ok();
        dragging.set(true);
        drag_start.set((
            ev.client_x() as f64,
            ev.client_y() as f64,
            offset_x.get_untracked(),
            offset_y.get_untracked(),
        ));
    };

    let on_pointer_move = move |ev: web_sys::PointerEvent| {
        if !dragging.get_untracked() {
            return;
        }
        let (sx, sy, ox, oy) = drag_start.get_untracked();
        offset_x.set(ox + (ev.client_x() as f64 - sx));
        offset_y.set(oy + (ev.client_y() as f64 - sy));
    };

    let on_pointer_up = move |_ev: web_sys::PointerEvent| {
        dragging.set(false);
    };

    let apply = move |_| {
        let Some(v) = view_ref.get() else { return };
        let Some(img) = img_ref.get() else { return };

        let v_node: &HtmlElement = v.as_ref();
        let view_rect = v_node.get_bounding_client_rect();

        let img_node: &HtmlElement = img.as_ref();
        let img_el: HtmlImageElement = img_node.clone().unchecked_into();
        let nat_w = img_el.natural_width() as f64;
        let nat_h = img_el.natural_height() as f64;
        let vw = view_rect.width();
        let vh = view_rect.height();
        let z = zoom.get_untracked();
        let ox = offset_x.get_untracked();
        let oy = offset_y.get_untracked();

        let cover = (vw / nat_w).max(vh / nat_h) * z;
        let drawn_w = nat_w * cover;
        let drawn_h = nat_h * cover;
        let draw_x = (vw - drawn_w) / 2.0 + ox;
        let draw_y = (vh - drawn_h) / 2.0 + oy;

        let sx = (-draw_x / cover).max(0.0);
        let sy = (-draw_y / cover).max(0.0);
        let sw = (vw / cover).min(nat_w - sx);
        let sh = (vh / cover).min(nat_h - sy);

        let out_w = 512u32;
        let aspect_val = aspect.get_untracked();
        let out_h = (out_w as f64 / aspect_val).max(1.0) as u32;

        let doc = web_sys::window().unwrap().document().unwrap();
        let canvas = doc
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(out_w);
        canvas.set_height(out_h);
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();
        ctx.draw_image_with_html_image_element_and_sw_sh_and_dw_dh(
            &img_el,
            sx,
            sy,
            sw,
            sh,
            0.0,
            0.0,
            out_w as f64,
            out_h as f64,
        )
        .ok();

        let data_url = canvas.to_data_url_with_type("image/png").unwrap_or_default();

        let cb = on_apply;
        let open_cb = open;
        let canvas_owned = canvas.clone();
        spawn_local(async move {
            let promise = canvas_owned.convert_to_blob().ok();
            let Some(promise) = promise else { return };
            let blob_js = wasm_bindgen_futures::JsFuture::from(promise).await.ok();
            let Some(blob) = blob_js.and_then(|b| b.dyn_into::<web_sys::Blob>().ok()) else {
                return;
            };
            let bitmap_promise = web_sys::window()
                .unwrap()
                .create_image_bitmap_with_blob(&blob)
                .ok();
            let Some(bp) = bitmap_promise else { return };
            let bm = wasm_bindgen_futures::JsFuture::from(bp).await.ok();
            if let Some(bm) = bm.and_then(|b| b.dyn_into::<ImageBitmap>().ok()) {
                cb.call(CropResult {
                    bitmap: bm,
                    data_url,
                });
                open_cb.set(false);
            }
        });
    };

    view! {
        <Show when=move || open.get()>
            <div class="sheet" role="dialog" aria-modal="true">
                <div class="sheet-box">
                    <h3>{title}</h3>

                    <div
                        _ref=view_ref
                        class="crop-view"
                        class:drag=move || dragging.get()
                        style=move || {
                            format!(
                                "width:min(90vw,340px);aspect-ratio:{};",
                                aspect.get(),
                            )
                        }
                        on:wheel=on_wheel
                        on:pointerdown=on_pointer_down
                        on:pointermove=on_pointer_move
                        on:pointerup=on_pointer_up
                        on:pointercancel=on_pointer_up
                    >
                        <img
                            _ref=img_ref
                            src=move || image_url.get().unwrap_or_default()
                            alt=""
                            draggable="false"
                            style=move || {
                                format!(
                                    "transform: translate({}px, {}px) scale({}); \
                                    transform-origin: 50% 50%; \
                                    position: absolute; top: 50%; left: 50%; \
                                    translate: -50% -50%; \
                                    max-width: none; max-height: none; \
                                    width: 100%; height: 100%; object-fit: cover;",
                                    offset_x.get(),
                                    offset_y.get(),
                                    zoom.get(),
                                )
                            }
                        />
                        <div class="crop-grid"></div>
                    </div>

                    <div class="zoomrow">
                        <label>"Zoom"</label>
                        <input
                            type="range"
                            min="1"
                            max="4"
                            step="0.01"
                            prop:value=move || zoom.get()
                            on:input=move |ev| {
                                let v: f64 =
                                    event_target_value(&ev).parse().unwrap_or(1.0);
                                zoom.set(v);
                            }
                        />
                    </div>
                    <p>"Seret foto untuk menggeser. Scroll atau slider untuk memperbesar."</p>

                    <div class="row">
                        <button
                            type="button"
                            class="btn"
                            on:click=move |_| open.set(false)
                        >
                            "Batal"
                        </button>
                        <button type="button" class="btn primary" on:click=apply>
                            "Terapkan"
                        </button>
                    </div>
                </div>
            </div>
        </Show>
    }
}
