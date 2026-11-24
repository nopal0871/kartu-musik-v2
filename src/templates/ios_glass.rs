use web_sys::CanvasRenderingContext2d;

use super::{FieldId, MusicTemplate, Orientation, RenderState};
use crate::renderer::drawing::*;

pub struct IosGlassTemplate;

impl MusicTemplate for IosGlassTemplate {
    fn id(&self) -> &'static str { "ios" }
    fn name(&self) -> &'static str { "iOS Kaca" }
    fn description(&self) -> &'static str { "Kartu kaca di atas latar blur." }
    fn orientation(&self) -> Orientation { Orientation::Portrait }
    fn visible_fields(&self) -> &'static [FieldId] {
        &[FieldId::Title, FieldId::Artist, FieldId::Header1, FieldId::Header2]
    }

    fn render_static(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64, s: &RenderState) {
        draw_theme_background(ctx, w, h, s.theme);

        let cw = u(88.0, w);
        let cx = (w - cw) / 2.0;
        let ch = u(52.0, w);
        let cy = h - ch - u(9.0, w);

        draw_glass_card(ctx, cx, cy, cw, ch, u(4.5, w));

        let pad = u(4.0, w);
        let art = u(20.0, w);
        let ax = cx + pad;
        let ay = cy + pad;
        draw_rounded_image(ctx, s.album_art, ax, ay, art, art, u(2.5, w));
    }

    fn render_dynamic(&self, ctx: &CanvasRenderingContext2d, w: f64, h: f64, s: &RenderState) {
        let cw = u(88.0, w);
        let cx = (w - cw) / 2.0;
        let ch = u(52.0, w);
        let cy = h - ch - u(9.0, w);
        let pad = u(4.0, w);
        let art = u(20.0, w);
        let ax = cx + pad;
        let ay = cy + pad;
        let tx = ax + art + u(4.0, w);
        let tmax = cx + cw - pad - tx;

        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");

        ctx.set_font(&format!("500 {}px Inter, sans-serif", u(3.0, w)));
        ctx.set_fill_style_str("rgba(255,255,255,0.56)");
        let _ = ctx.fill_text(s.header2, tx, ay + u(3.0, w));

        let title_y = ay + u(8.5, w);
        draw_marquee(
            ctx,
            s.title,
            tx,
            title_y,
            tmax,
            &format!("600 {}px Inter, sans-serif", u(4.4, w)),
            "#fff",
            s.elapsed,
            u(30.0, w),
        );

        ctx.set_font(&format!("400 {}px Inter, sans-serif", u(3.4, w)));
        ctx.set_fill_style_str("rgba(255,255,255,0.62)");
        let _ = ctx.fill_text(s.artist, tx, title_y + u(5.4, w));

        draw_spectrum(
            ctx,
            cx + cw - pad - u(12.0, w),
            ay,
            u(12.0, w),
            u(10.0, w),
            s.spectrum,
            s.playing,
        );

        let bar_y = cy + ch - pad - u(6.0, w);
        let bar_x = cx + pad;
        let bar_w = cw - 2.0 * pad;
        let bar_h = u(1.1, w);
        let pct = if s.duration > 0.0 { (s.elapsed / s.duration).clamp(0.0, 1.0) } else { 0.0 };

        ctx.set_fill_style_str("rgba(255,255,255,0.28)");
        rounded_rect(ctx, bar_x, bar_y, bar_w, bar_h, bar_h / 2.0);
        let _ = ctx.fill();

        ctx.set_fill_style_str("rgba(255,255,255,0.9)");
        rounded_rect(ctx, bar_x, bar_y, bar_w * pct, bar_h, bar_h / 2.0);
        let _ = ctx.fill();

        let knob = u(2.2, w);
        circle(ctx, bar_x + bar_w * pct, bar_y + bar_h / 2.0, knob, "#fff");

        ctx.set_font(&format!("500 {}px Inter, sans-serif", u(2.6, w)));
        ctx.set_fill_style_str("rgba(255,255,255,0.6)");
        ctx.set_text_align("left");
        let _ = ctx.fill_text(&fmt_time(s.elapsed), bar_x, bar_y - u(3.0, w));
        ctx.set_text_align("right");
        let _ = ctx.fill_text(&fmt_time(s.duration), bar_x + bar_w, bar_y - u(3.0, w));

        let row_y = cy + ch - u(3.0, w);
        let icons: [Glyph; 5] = [
            Glyph::Shuffle,
            Glyph::Prev,
            if s.playing { Glyph::Pause } else { Glyph::Play },
            Glyph::Next,
            Glyph::Repeat,
        ];
        let spacing = u(9.0, w);
        let total = spacing * (icons.len() as f64 - 1.0);
        let start = cx + cw / 2.0 - total / 2.0;
        for (i, g) in icons.iter().enumerate() {
            let big = matches!(g, Glyph::Play | Glyph::Pause);
            draw_glyph_button(ctx, start + i as f64 * spacing, row_y, u(2.4, w), *g, big);
        }
    }
}

fn fmt_time(t: f64) -> String {
    let t = t.max(0.0) as u32;
    format!("{}:{:02}", t / 60, t % 60)
}
