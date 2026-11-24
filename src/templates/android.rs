//use linkme::distributed_slice;
use web_sys::CanvasRenderingContext2d;

use super::{FieldId, MusicTemplate, Orientation, RenderState, REGISTERED};
use crate::renderer::drawing::*;

pub struct AndroidTemplate;

impl MusicTemplate for AndroidTemplate {
    fn id(&self) -> &'static str { "android" }
    fn name(&self) -> &'static str { "Pemutar Android" }
    fn description(&self) -> &'static str { "Cover besar, tombol Putar lebar." }
    fn orientation(&self) -> Orientation { Orientation::Portrait }
    fn visible_fields(&self) -> &'static [FieldId] {
        &[FieldId::Title, FieldId::Artist, FieldId::Device, FieldId::Header2]
    }

    fn render_static(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64, s: &RenderState) {
        draw_theme_background(ctx, w, h, s.theme);
        let art = w * 0.72;
        let ax = (w - art) / 2.0;
        let ay = u(14.0, w);
        draw_rounded_image(ctx, s.album_art, ax, ay, art, art, u(4.0, w));
    }

    fn render_dynamic(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64, s: &RenderState) {
        ctx.set_font(&format!("600 {}px Inter, sans-serif", u(3.6, w)));
        ctx.set_fill_style_str("rgba(255,255,255,0.82)");
        ctx.set_text_align("center");
        let _ = ctx.fill_text(s.header1, w / 2.0, u(8.0, w));

        let art = w * 0.72;
        let ay = u(14.0, w);

        ctx.set_font(&format!("600 {}px Inter, sans-serif", u(5.4, w)));
        ctx.set_fill_style_str("#fff");
        let _ = ctx.fill_text(s.title, w / 2.0, ay + art + u(9.0, w));

        ctx.set_font(&format!("400 {}px Inter, sans-serif", u(3.8, w)));
        ctx.set_fill_style_str("rgba(255,255,255,0.82)");
        let _ = ctx.fill_text(s.artist, w / 2.0, ay + art + u(15.5, w));

        let bar_y = h - u(30.0, w);
        let bar_x = u(9.0, w);
        let bar_w = w - 2.0 * bar_x;
        let bar_h = u(1.4, w);
        let pct = if s.duration > 0.0 { (s.elapsed / s.duration).clamp(0.0, 1.0) } else { 0.0 };

        ctx.set_fill_style_str("rgba(255,255,255,0.45)");
        rounded_rect(ctx, bar_x, bar_y, bar_w, bar_h, bar_h / 2.0);
        let _ = ctx.fill();

        ctx.set_fill_style_str("#fff");
        rounded_rect(ctx, bar_x, bar_y, bar_w * pct, bar_h, bar_h / 2.0);
        let _ = ctx.fill();

        ctx.set_font(&format!("500 {}px Inter, sans-serif", u(3.0, w)));
        ctx.set_fill_style_str("rgba(255,255,255,0.9)");
        ctx.set_text_align("left");
        let _ = ctx.fill_text(&fmt_time(s.elapsed), bar_x, bar_y + u(6.0, w));
        ctx.set_text_align("right");
        let _ = ctx.fill_text(&fmt_time(s.duration), bar_x + bar_w, bar_y + u(6.0, w));

        let cy = h - u(16.0, w);
        draw_glyph_button(ctx, w / 2.0, cy, u(11.0, w),
            if s.playing { Glyph::Pause } else { Glyph::Play }, true);
        draw_glyph_button(ctx, w / 2.0 - u(20.0, w), cy, u(8.0, w), Glyph::Prev, false);
        draw_glyph_button(ctx, w / 2.0 + u(20.0, w), cy, u(8.0, w), Glyph::Next, false);

        if !s.device.is_empty() {
            draw_pill(ctx, w / 2.0, h - u(5.0, w), s.device, w);
        }
    }
}

fn fmt_time(t: f64) -> String {
    let t = t.max(0.0) as u32;
    format!("{}:{:02}", t / 60, t % 60)
}

#[distributed_slice(REGISTERED)]
static _REG: fn() -> Box<dyn MusicTemplate> = || Box::new(AndroidTemplate);
