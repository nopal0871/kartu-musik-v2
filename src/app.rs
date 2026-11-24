use std::rc::Rc;

use leptos::*;

use crate::components::dashboard::Dashboard;
use crate::components::export_sheet::ExportSheet;
use crate::components::player::PlayerStage;
use crate::exporter::ExportOptions;
use crate::renderer::AudioController;
use crate::state::AppState;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum View {
    Dashboard,
    Player,
}

#[component]
pub fn App() -> impl IntoView {
    let state = create_rw_signal(AppState::default());
    let template_id = create_rw_signal("android".to_string());
    let orientation = create_rw_signal(true);
    let view = create_rw_signal(View::Dashboard);

    let audio_ctrl: RwSignal<Option<Rc<AudioController>>> = create_rw_signal(None);
    let audio_bytes: RwSignal<Option<Vec<u8>>> = create_rw_signal(None);
    let state_snapshot: RwSignal<Option<AppState>> = create_rw_signal(None);
    let export_open = create_rw_signal(false);
    let export_opts = create_rw_signal(ExportOptions::from_resolution(720));

    let on_go_player = Callback::from(move |_| view.set(View::Player));
    let on_back = Callback::from(move |_| view.set(View::Dashboard));
    let on_export = Callback::from(move |_| {
        state_snapshot.set(Some(state.get_untracked()));
        export_open.set(true);
    });

    view! {
        <div class="wrap">
            <header class="top">
                <div class="brand">
                    <h1>"Kartu Musik iOS"</h1>
                    <p>"Atur kartu pemutar, lalu simpan sebagai video."</p>
                </div>
                <ol class="steps">
                    <li class="step"
                        aria-current=move || if view.get() == View::Dashboard { Some("step") } else { None }>
                        "1 Dashboard"
                    </li>
                    <li class="step"
                        aria-current=move || if view.get() == View::Player { Some("step") } else { None }>
                        "2 Player"
                    </li>
                </ol>
            </header>

            <main>
                <Show when=move || view.get() == View::Dashboard>
                    <div class="dash">
                        <Dashboard
                            state=state
                            template_id=template_id
                            orientation=orientation
                            audio_ctrl=audio_ctrl
                            audio_bytes=audio_bytes
                            on_go_player=on_go_player
                            on_export=on_export
                        />
                        <aside class="preview">
                            <div class="stage-wrap">
                                <PlayerStage state=state audio=audio_ctrl />
                            </div>
                            <p class="cap">"Pratinjau 9:16"</p>
                        </aside>
                    </div>
                </Show>

                <Show when=move || view.get() == View::Player>
                    <div class="player">
                        <div class="stage-wrap">
                            <PlayerStage state=state audio=audio_ctrl />
                        </div>
                        <div class="actions">
                            <button type="button" class="btn" on:click=move |_| on_back.call(())>
                                "Kembali"
                            </button>
                            <button type="button" class="btn primary" on:click=move |_| on_export.call(())>
                                "Simpan video"
                            </button>
                        </div>
                    </div>
                </Show>
            </main>
        </div>

        <ExportSheet
            open=export_open
            template_id=template_id
            state_snapshot=state_snapshot
            audio_bytes=audio_bytes
            options=export_opts
        />
    }
}
