use leptos::*;

use crate::exporter::{ExportOptions, ExportProgress, ProgressCb};
use crate::state::AppState;

#[component]
pub fn ExportSheet(
    open: RwSignal<bool>,
    template_id: RwSignal<String>,
    state_snapshot: RwSignal<Option<AppState>>,
    audio_bytes: RwSignal<Option<Vec<u8>>>,
    options: RwSignal<ExportOptions>,
) -> impl IntoView {
    let progress = create_rw_signal(0.0f64);
    let phase = create_rw_signal("siap");
    let result_url = create_rw_signal::<Option<String>>(None);
    let busy = create_rw_signal(false);

    let start = move |_| {
        if busy.get_untracked() {
            return;
        }
        let Some(snap) = state_snapshot.get_untracked() else {
            phase.set("tidak ada state");
            return;
        };
        let Some(bytes) = audio_bytes.get_untracked() else {
            phase.set("tidak ada audio");
            return;
        };
        busy.set(true);
        progress.set(0.0);
        phase.set("memulai");
        result_url.set(None);

        let tpl = template_id.get_untracked();
        let opts = options.get_untracked();

        spawn_local(async move {
            let cb: ProgressCb = Box::new(move |p: ExportProgress| {
                let pct = if p.total_frames > 0 {
                    p.frame as f64 / p.total_frames as f64
                } else {
                    0.0
                };
                progress.set(pct);
                phase.set(p.phase);
            });
            match crate::exporter::export(tpl, snap, bytes, opts, cb).await {
                Ok(blob) => {
                    let url = web_sys::Url::create_object_url_with_blob(&blob).ok();
                    result_url.set(url);
                    phase.set("selesai");
                }
                Err(e) => {
                    web_sys::console::error_1(&e);
                    phase.set("error");
                }
            }
            busy.set(false);
        });
    };

    view! {
        <Show when=move || open.get()>
            <div class="sheet" role="dialog" aria-modal="true">
                <div class="sheet-box">
                    <h3>"Ekspor Video"</h3>
                    <progress value=move || progress.get() max="1.0" style="width:100%"></progress>
                    <p>
                        {move || format!("{} — {:.0}%", phase.get(), progress.get() * 100.0)}
                    </p>

                    {move || {
                        result_url.get().map(|url| {
                            view! {
                                <video
                                    src=url.clone()
                                    controls
                                    playsinline
                                    style="max-width:100%;max-height:60dvh;border-radius:8px"
                                ></video>
                                <a class="btn primary" href=url download="kartu-musik.webm">
                                    "Unduh video"
                                </a>
                            }
                        })
                    }}

                    <div class="row">
                        <button
                            type="button"
                            class="btn primary"
                            on:click=start
                            disabled=move || busy.get()
                        >
                            "Mulai Ekspor"
                        </button>
                        <button
                            type="button"
                            class="btn"
                            on:click=move |_| open.set(false)
                        >
                            "Tutup"
                        </button>
                    </div>
                    <p style="font-size:12px;color:var(--muted);text-align:center">
                        "MediaRecorder merekam realtime — durasi ekspor ≈ durasi lagu. \
                        Jangan menutup atau memindah tab selama ekspor."
                    </p>
                </div>
            </div>
        </Show>
    }
}
