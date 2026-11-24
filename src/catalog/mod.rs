use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

use crate::renderer::get_2d;
use crate::templates::TemplateRegistry;

#[component]
pub fn TemplatePicker(selected: RwSignal<String>) -> impl IntoView {
    let open = create_rw_signal(false);
    let registry = TemplateRegistry::new();
    let items: Vec<(String, String)> = registry
        .all()
        .iter()
        .map(|t| (t.id().to_string(), t.name().to_string()))
        .collect();
    let current_name = move || {
        let id = selected.get();
        items
            .iter()
            .find(|(i, _)| i == &id)
            .map(|(_, n)| n.clone())
            .unwrap_or_else(|| "Pilih Template".into())
    };

    view! {
        <button
            type="button"
            class="picker-trigger"
            aria-haspopup="dialog"
            on:click=move |_| open.set(true)
        >
            <span class="picker-label">
                <small>"Template"</small>
                <strong>{current_name}</strong>
            </span>
            <span class="picker-chev" aria-hidden="true">"›"</span>
        </button>

        <Show when=move || open.get()>
            <TemplateSheet selected=selected open=open />
        </Show>
    }
}

#[component]
fn TemplateSheet(selected: RwSignal<String>, open: RwSignal<bool>) -> impl IntoView {
    let registry = TemplateRegistry::new();
    let items: Vec<(String, String)> = registry
        .all()
        .iter()
        .map(|t| (t.id().to_string(), t.name().to_string()))
        .collect();

    view! {
        <div class="sheet" role="dialog" aria-modal="true">
            <div class="sheet-box template-sheet">
                <h3>"Pilih Template"</h3>
                <div class="template-grid">
                    {items
                        .into_iter()
                        .map(|(id, name)| {
                            let id_click = id.clone();
                            let id_sel = id.clone();
                            let is_sel = move || selected.get() == id_sel;
                            view! {
                                <button
                                    type="button"
                                    class="template-cell"
                                    class:selected=is_sel
                                    on:click=move |_| {
                                        selected.set(id_click.clone());
                                        open.set(false);
                                    }
                                >
                                    <PreviewCanvas id=id.clone() />
                                    <span class="template-cell-name">{name}</span>
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
                <button type="button" class="btn" on:click=move |_| open.set(false)>
                    "Tutup"
                </button>
            </div>
        </div>
    }
}

#[component]
fn PreviewCanvas(id: String) -> impl IntoView {
    let canvas_ref = create_node_ref::<leptos::html::Canvas>();
    let id_st = store_value(id);

    create_effect(move |_| {
        let Some(canvas_node) = canvas_ref.get() else { return };
        let el: &web_sys::HtmlElement = canvas_node.as_ref();
        let canvas: HtmlCanvasElement = el.clone().unchecked_into();

        let ctx = get_2d(&canvas);
        let registry = TemplateRegistry::new();
        id_st.with_value(|id| {
            if let Some(t) = registry.get(id) {
                let w = canvas.width() as f64;
                let h = canvas.height() as f64;
                t.render_preview(&ctx, w, h);
            }
        });
    });

    view! {
        <canvas
            _ref=canvas_ref
            width="180"
            height="320"
            class="template-preview"
        />
    }
}
