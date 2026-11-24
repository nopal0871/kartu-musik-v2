use std::rc::Rc;

use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{FileReader, HtmlAudioElement, HtmlInputElement};

use crate::catalog::TemplatePicker;
use crate::components::crop_sheet::{CropResult, CropSheet};
use crate::renderer::AudioController;
use crate::state::AppState;

#[component]
pub fn Dashboard(
    state: RwSignal<AppState>,
    template_id: RwSignal<String>,
    orientation: RwSignal<bool>,
    audio_ctrl: RwSignal<Option<Rc<AudioController>>>,
    audio_bytes: RwSignal<Option<Vec<u8>>>,
    on_go_player: Callback<()>,
    on_export: Callback<()>,
) -> impl IntoView {
    let art_file_ref = create_node_ref::<leptos::html::Input>();
    let art_crop_open = create_rw_signal(false);
    let art_crop_url = create_rw_signal::<Option<String>>(None);

    let on_art_file = move |ev: web_sys::Event| {
        let input: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
        let Some(file) = input.files().and_then(|f| f.get(0)) else { return };
        let reader = FileReader::new().unwrap();
        let r = reader.clone();
        let cb = Closure::once(move || {
            if let Ok(s) = r.result() {
                if let Some(url) = s.as_string() {
                    art_crop_url.set(Some(url));
                    art_crop_open.set(true);
                }
            }
        });
        reader.set_onload(Some(cb.as_ref().unchecked_ref()));
        reader.read_as_data_url(&file).unwrap();
        cb.forget();
    };

    let on_art_apply = Callback::from(move |res: CropResult| {
        state.update(|s| {
            s.album_art = Some(Rc::new(res.bitmap));
            s.art_version = s.art_version.wrapping_add(1);
        });
    });

    let bg_file_ref = create_node_ref::<leptos::html::Input>();
    let bg_crop_open = create_rw_signal(false);
    let bg_crop_url = create_rw_signal::<Option<String>>(None);

    let on_bg_file = move |ev: web_sys::Event| {
        let input: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
        let Some(file) = input.files().and_then(|f| f.get(0)) else { return };
        let reader = FileReader::new().unwrap();
        let r = reader.clone();
        let cb = Closure::once(move || {
            if let Ok(s) = r.result() {
                if let Some(url) = s.as_string() {
                    bg_crop_url.set(Some(url));
                    bg_crop_open.set(true);
                }
            }
        });
        reader.set_onload(Some(cb.as_ref().unchecked_ref()));
        reader.read_as_data_url(&file).unwrap();
        cb.forget();
    };

    let on_bg_apply = Callback::from(move |res: CropResult| {
        state.update(|s| {
            s.theme.background = Some(Rc::new(res.bitmap));
            s.theme.invalidate();
            s.bg_version = s.bg_version.wrapping_add(1);
        });
    });

    let audio_file_ref = create_node_ref::<leptos::html::Input>();

    let on_audio_file = move |ev: web_sys::Event| {
        let input: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
        let Some(file) = input.files().and_then(|f| f.get(0)) else { return };

        let reader = FileReader::new().unwrap();
        let r = reader.clone();
        let bytes_cb = Closure::once(move || {
            if let Ok(buf) = r.result() {
                let arr = js_sys::Uint8Array::new(&buf);
                audio_bytes.set(Some(arr.to_vec()));
            }
        });
        reader.set_onload(Some(bytes_cb.as_ref().unchecked_ref()));
        reader.read_as_array_buffer(&file).unwrap();
        bytes_cb.forget();

        let url = web_sys::Url::create_object_url_with_blob(&file).ok();
        if let Some(url) = url {
            let doc = web_sys::window().unwrap().document().unwrap();
            let audio: HtmlAudioElement = doc
                .create_element("audio")
                .unwrap()
                .dyn_into::<HtmlAudioElement>()
                .unwrap();
            audio.set_src(&url);

            let audio_clone = audio.clone();
            let state_for_dur = state;
            let cb = Closure::once(move || {
                let d = audio_clone.duration();
                if d.is_finite() {
                    state_for_dur.update(|s| s.duration = d);
                }
            });
            audio.set_onloadedmetadata(Some(cb.as_ref().unchecked_ref()));
            cb.forget();

            let ctrl = AudioController::new(audio);
            audio_ctrl.set(Some(ctrl));
        }
    };

    let on_play = move |_| {
        if let Some(c) = audio_ctrl.get_untracked() {
            if let Err(e) = c.play() {
                web_sys::console::error_1(&e);
            }
        }
    };

    let on_pause = move |_| {
        if let Some(c) = audio_ctrl.get_untracked() {
            c.pause();
        }
    };

    view! {
        <div class="panel">
            <div class="group">
                <h2>"Template"</h2>
                <div class="seg" role="group" aria-label="Orientasi">
                    <button
                        type="button"
                        aria-pressed=move || orientation.get()
                        on:click=move |_| orientation.set(true)
                    >
                        "Portrait 9:16"
                    </button>
                    <button
                        type="button"
                        aria-pressed=move || !orientation.get()
                        on:click=move |_| orientation.set(false)
                    >
                        "Landscape 16:9"
                    </button>
                </div>
                <TemplatePicker selected=template_id />
            </div>

            <div class="group">
                <h2>"Foto album"</h2>
                <button
                    type="button"
                    class="drop"
                    on:click=move |_| {
                        if let Some(i) = art_file_ref.get() {
                            i.click();
                        }
                    }
                >
                    <span class="thumb" id="artThumb"></span>
                    <span class="drop-txt">
                        <strong>"Pilih foto album"</strong>
                        <small>"PNG/JPG, akan di-crop manual."</small>
                    </span>
                </button>
                <input
                    _ref=art_file_ref
                    type="file"
                    accept="image/*"
                    hidden
                    on:change=on_art_file
                />
                <div class="rowbtn">
                    <button
                        type="button"
                        class="btn small"
                        on:click=move |_| art_crop_open.set(true)
                    >
                        "Atur crop foto album"
                    </button>
                </div>
            </div>

            <div class="group">
                <h2>"Latar belakang"</h2>
                <button
                    type="button"
                    class="drop"
                    on:click=move |_| {
                        if let Some(i) = bg_file_ref.get() {
                            i.click();
                        }
                    }
                >
                    <span class="thumb tall" id="bgThumb"></span>
                    <span class="drop-txt">
                        <strong>"Pilih gambar latar"</strong>
                        <small>"Akan di-blur sesuai slider di bawah."</small>
                    </span>
                </button>
                <input
                    _ref=bg_file_ref
                    type="file"
                    accept="image/*"
                    hidden
                    on:change=on_bg_file
                />
                <div class="rowbtn">
                    <button
                        type="button"
                        class="btn small"
                        on:click=move |_| bg_crop_open.set(true)
                    >
                        "Atur crop latar"
                    </button>
                </div>

                <div class="fields" style="margin-top:12px">
                    <div class="field">
                        <label class="lbl">"Warna atas"</label>
                        <input
                            type="color"
                            prop:value=move || state.get().theme.bg_top
                            on:input=move |ev| {
                                let v = event_target_value(&ev);
                                state.update(|s| s.theme.bg_top = v);
                            }
                        />
                    </div>
                    <div class="field">
                        <label class="lbl">"Warna bawah"</label>
                        <input
                            type="color"
                            prop:value=move || state.get().theme.bg_bottom
                            on:input=move |ev| {
                                let v = event_target_value(&ev);
                                state.update(|s| s.theme.bg_bottom = v);
                            }
                        />
                    </div>
                </div>

                <div class="sliders">
                    <div class="field">
                        <label class="lbl">
                            "Kekuatan blur latar "
                            <output>
                                {move || format!("{:.0}", state.get().theme.blur * 100.0)}
                            </output>
                        </label>
                        <input
                            type="range"
                            min="0"
                            max="100"
                            prop:value=move || (state.get().theme.blur * 100.0) as i32
                            on:input=move |ev| {
                                let v: f64 = event_target_value(&ev).parse().unwrap_or(0.0);
                                state.update(|s| s.theme.blur = v / 100.0);
                            }
                        />
                    </div>
                    <div class="field">
                        <label class="lbl">
                            "Gelapkan latar "
                            <output>
                                {move || format!("{:.0}%", state.get().theme.dim * 100.0)}
                            </output>
                        </label>
                        <input
                            type="range"
                            min="0"
                            max="70"
                            prop:value=move || (state.get().theme.dim * 100.0) as i32
                            on:input=move |ev| {
                                let v: f64 = event_target_value(&ev).parse().unwrap_or(0.0);
                                state.update(|s| s.theme.dim = v / 100.0);
                            }
                        />
                    </div>
                </div>
            </div>

            <div class="group">
                <h2>"Info lagu"</h2>
                <div class="fields">
                    <div class="field">
                        <label class="lbl">"Nama lagu"</label>
                        <input
                            type="text"
                            maxlength="80"
                            prop:value=move || state.get().title
                            on:input=move |ev| {
                                let v = event_target_value(&ev);
                                state.update(|s| s.title = v);
                            }
                        />
                    </div>
                    <div class="field">
                        <label class="lbl">"Nama artis"</label>
                        <input
                            type="text"
                            maxlength="80"
                            prop:value=move || state.get().artist
                            on:input=move |ev| {
                                let v = event_target_value(&ev);
                                state.update(|s| s.artist = v);
                            }
                        />
                    </div>
                    <div class="field full">
                        <label class="lbl">"Label atas"</label>
                        <input
                            type="text"
                            maxlength="40"
                            prop:value=move || state.get().header1
                            on:input=move |ev| {
                                let v = event_target_value(&ev);
                                state.update(|s| s.header1 = v);
                            }
                        />
                    </div>
                    <div class="field">
                        <label class="lbl">"Nama playlist"</label>
                        <input
                            type="text"
                            maxlength="40"
                            prop:value=move || state.get().header2
                            on:input=move |ev| {
                                let v = event_target_value(&ev);
                                state.update(|s| s.header2 = v);
                            }
                        />
                    </div>
                    <div class="field full">
                        <label class="lbl">"Caption catatan"</label>
                        <input
                            type="text"
                            maxlength="50"
                            prop:value=move || state.get().caption
                            on:input=move |ev| {
                                let v = event_target_value(&ev);
                                state.update(|s| s.caption = v);
                            }
                        />
                    </div>
                </div>
            </div>

            <div class="group">
                <h2>"Lagu"</h2>
                <button
                    type="button"
                    class="drop"
                    on:click=move |_| {
                        if let Some(i) = audio_file_ref.get() {
                            i.click();
                        }
                    }
                >
                    <span class="note-ico" aria-hidden="true">"♪"</span>
                    <span class="drop-txt">
                        <strong>"Tambahkan file lagu"</strong>
                        <small>"MP3, M4A, WAV, atau OGG."</small>
                    </span>
                </button>
                <input
                    _ref=audio_file_ref
                    type="file"
                    accept="audio/*"
                    hidden
                    on:change=on_audio_file
                />
                <div class="playrow">
                    <button type="button" class="btn primary" on:click=on_play>
                        "Putar"
                    </button>
                    <button type="button" class="btn" on:click=on_pause>
                        "Jeda"
                    </button>
                </div>
            </div>

            <div class="cta">
                <button
                    type="button"
                    class="btn primary wide"
                    on:click=move |_| on_go_player.call(())
                >
                    "Lanjutkan ke Tampilan Player"
                </button>
                <button
                    type="button"
                    class="btn wide"
                    style="margin-top:10px"
                    on:click=move |_| on_export.call(())
                >
                    "Ekspor Video"
                </button>
                <p>"Video direkam sepanjang lagu, dengan suara."</p>
            </div>
        </div>

        <CropSheet
            open=art_crop_open
            image_url=art_crop_url
            aspect=RwSignal::new(1.0)
            title="Crop foto album"
            on_apply=on_art_apply
        />
        <CropSheet
            open=bg_crop_open
            image_url=bg_crop_url
            aspect=RwSignal::new(9.0 / 16.0)
            title="Crop latar (9:16)"
            on_apply=on_bg_apply
        />
    }
                }
