use leptos::*;
mod app;
mod state;
mod theme;
mod templates;
mod renderer;
mod exporter;
mod catalog;
mod components;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(app::App);
}
