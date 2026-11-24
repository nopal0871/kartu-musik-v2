use web_sys::CanvasRenderingContext2d;

use super::{FieldId, MusicTemplate, Orientation, RenderState};
use crate::renderer::drawing::*;

pub struct IgTemplate;

impl MusicTemplate for IgTemplate {
    fn id(&self) -> &'static str { "ig" }
    fn name(&self) -> &'static str { "IG" }
    fn description(&self) -> &'static str { "Catatan musik ala Instagram." }
    fn orientation(&self) -> Orientation { Orientation::Portrait }
    fn visible_fields(&self) -> &'static [FieldId] {
        &[FieldId::Title, FieldId::Artist, FieldId::Header1, FieldId::Caption]
    }

    fn render_static(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64, s: &RenderState) {
        draw_theme_background(ctx, w, h, s.theme);

        let cw = u(92.0, w);
        let cx = (w - cw) / 2.0;
        let ch = u(50.0, w);
        let cy = h - ch - u(12.0, w);

        ctx.set_fill_style_str("#1b1e23");
        rounded_rect(ctx, cx, cy, cw, ch, u(5.0, w));
        let _ = ctx.fill();

        let av = u(9.0, w);
        let avx = cx + u(5.0, w);
        let avy = cy + u(5.0, w);
        draw_rounded_image(ctx, s.album_art, avx, avy, av, av, av / 2.0);
        if s.album_art.is_none() {
            circle(ctx, avx + av / 2.0, avy + av / 2.0, av / 2.0, "rgba(255,255,255,0.22)");
        }

        ctx.set_fill_style_str("rgba(255,255,255,0.11)");
        let _ = ctx.fill_rect(cx + u(5.0, w), cy + u(17.0, w), cw - u(10.0, w), 1.0);

        let btn_y = cy + ch + u(5.0, w);
        let btn_w = u(46.0, w);
        let btn_h = u(9.0, w);
        let bx = cx + (cw - btn_w) / 2.0;
        ctx.set_fill_style_str("#4f5bf3");
        rounded_rect(ctx, bx, btn_y, btn_w, btn_h, btn_h / 2.0);
        let _ = ctx.fill();
    }

    fn render_dynamic(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64, s: &RenderState) {
        let cw = u(92.0, w);
        let cx = (w - cw) / 2.0;
        let ch = u(50.0, w);
        let cy = h - ch - u(12.0, w);

        let av = u(9.0, w);
        let avx = cx + u(5.0, w);
        let avy = cy + u(5.0, w);

        ctx.set_text_baseline("middle");
        ctx.set_text_align("left");
        ctx.set_font(&format!("600 {}px Inter, sans-serif", u(3.4, w)));
        ctx.set_fill_style_str("rgba(255,255,255,0.78)");
        let _ = ctx.fill_text(s.header1, avx + av + u(3.0, w), avy + av * 0.35);

        let cap = if s.caption.is_empty() { "Mendengarkan sekarang" } else { s.caption };
        ctx.set_font(&format!("600 {}px Inter, sans-serif", u(3.6, w)));
        ctx.set_fill_style_str("#fff");
        let _ = ctx.fill_text(cap, avx + av + u(3.0, w), avy + av * 0.75);

        ctx.set_font(&format!("600 {}px Inter, sans-serif", u(4.0, w)));
        ctx.set_fill_style_str("#fff");
        let _ = ctx.fill_text(s.title, cx + u(5.0, w), cy + u(24.0, w));

        ctx.set_font(&format!("400 {}px Inter, sans-serif", u(3.2, w)));
        ctx.set_fill_style_str("rgba(255,255,255,0.66)");
        let _ = ctx.fill_text(s.artist, cx + u(5.0, w), cy + u(29.5, w));

        let bar_y = cy + ch - u(11.0, w);
        let bar_x = cx + u(5.0, w);
        let bar_w = cw - u(10.0, w);
        let bar_h = u(0.7, w);
        let pct = if s.duration > 0.0 { (s.elapsed / s.duration).clamp(0.0, 1.0) } else { 0.0 };

        ctx.set_fill_style_str("rgba(255,255,255,0.22)");
        rounded_rect(ctx, bar_x, bar_y, bar_w, bar_h, bar_h / 2.0);
        let _ = ctx.fill();

        ctx.set_fill_style_str("rgba(255,255,255,0.9)");
        rounded_rect(ctx, bar_x, bar_y, bar_w * pct, bar_h, bar_h / 2.0);
        let _ = ctx.fill();

        circle(ctx, bar_x + bar_w * pct, bar_y + bar_h / 2.0, u(1.6, w), "#fff");

        let btn_y = cy + ch + u(5.0, w);
        let btn_w = u(46.0, w);
        let btn_h = u(9.0, w);
        let bx = cx + (cw - btn_w) / 2.0;
        ctx.set_fill_style_str("#fff");
        ctx.set_font(&format!("600 {}px Inter, sans-serif", u(3.4, w)));
        ctx.set_text_align("center");
        let _ = ctx.fill_text("Putar di Instagram", bx + btn_w / 2.0, btn_y + btn_h / 2.0);
    }
}
