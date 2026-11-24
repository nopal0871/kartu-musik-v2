use web_sys::{CanvasRenderingContext2d, ImageBitmap};

use crate::theme::Theme;

pub fn u(cqw: f64, w: f64) -> f64 {
    cqw * w / 100.0
}

pub fn rounded_rect(ctx: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64, r: f64) {
    ctx.begin_path();
    ctx.move_to(x + r, y);
    ctx.arc_to(x + w, y, x + w, y + h, r).ok();
    ctx.arc_to(x + w, y + h, x, y + h, r).ok();
    ctx.arc_to(x, y + h, x, y, r).ok();
    ctx.arc_to(x, y, x + w, y, r).ok();
    ctx.close_path();
}

pub fn circle(ctx: &CanvasRenderingContext2d, cx: f64, cy: f64, r: f64, fill: &str) {
    ctx.set_fill_style_str(fill);
    ctx.begin_path();
    let _ = ctx.arc(cx, cy, r, 0.0, std::f64::consts::TAU);
    let _ = ctx.fill();
}

pub fn draw_theme_background(ctx: &CanvasRenderingContext2d, w: f64, h: f64, theme: &Theme) {
    let grad = ctx.create_linear_gradient(0.0, 0.0, 0.0, h);
    grad.add_color_stop(0.0, &theme.bg_top).ok();
    grad.add_color_stop(1.0, &theme.bg_bottom).ok();
    ctx.set_fill_style(&grad.into());
    let _ = ctx.fill_rect(0.0, 0.0, w, h);

    if let Some(bg) = theme.background.as_deref() {
        let _ = ctx.draw_image_with_image_bitmap_and_dw_dh(bg, 0.0, 0.0, w, h);
    }

    ctx.set_fill_style_str(&format!("rgba(0,0,0,{})", theme.dim));
    let _ = ctx.fill_rect(0.0, 0.0, w, h);
}

pub fn draw_rounded_image(
    ctx: &CanvasRenderingContext2d,
    art: Option<&ImageBitmap>,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    r: f64,
) {
    ctx.save();
    rounded_rect(ctx, x, y, w, h, r);
    ctx.clip();
    if let Some(bm) = art {
        let _ = ctx.draw_image_with_image_bitmap_and_dw_dh(bm, x, y, w, h);
    } else {
        ctx.set_fill_style_str("#444");
        let _ = ctx.fill_rect(x, y, w, h);
    }
    ctx.restore();
}

pub fn draw_glass_card(ctx: &CanvasRenderingContext2d, x: f64, y: f64, w: f64, h: f64, r: f64) {
    let grad = ctx.create_linear_gradient(0.0, y, 0.0, y + h);
    grad.add_color_stop(0.0, "rgba(255,255,255,0.34)").ok();
    grad.add_color_stop(0.55, "rgba(255,255,255,0.13)").ok();
    grad.add_color_stop(1.0, "rgba(255,255,255,0.08)").ok();
    ctx.set_fill_style(&grad.into());
    rounded_rect(ctx, x, y, w, h, r);
    let _ = ctx.fill();

    ctx.set_stroke_style_str("rgba(255,255,255,0.16)");
    ctx.set_line_width(1.2);
    rounded_rect(ctx, x, y, w, h, r);
    let _ = ctx.stroke();
}

pub fn draw_marquee(
    ctx: &CanvasRenderingContext2d,
    text: &str,
    x: f64,
    y: f64,
    max_w: f64,
    font: &str,
    color: &str,
    elapsed: f64,
    speed: f64,
) {
    ctx.set_font(font);
    ctx.set_fill_style_str(color);
    ctx.set_text_align("left");
    ctx.set_text_baseline("middle");

    let text_w = ctx.measure_text(text).ok().map(|m| m.width()).unwrap_or(0.0);

    if text_w <= max_w {
        let _ = ctx.fill_text(text, x, y);
        return;
    }

    ctx.save();
    ctx.begin_path();
    ctx.rect(x, y - 20.0, max_w, 40.0);
    ctx.clip();

    let travel = text_w + 30.0;
    let period = travel / speed;
    let phase = (elapsed % period) / period;
    let offset = -phase * travel;
    let _ = ctx.fill_text(text, x + offset, y);
    ctx.restore();
}

pub fn draw_spectrum(
    ctx: &CanvasRenderingContext2d,
    x: f64,
    y: f64,
    w: f64,
    h: f64,
    spectrum: &[f32],
    playing: bool,
) {
    let bars = spectrum.len().max(1);
    let gap = w / bars as f64 * 0.4;
    let bw = (w / bars as f64) - gap;
    let base = y + h;

    for (i, &v) in spectrum.iter().enumerate() {
        let val = if playing { v.clamp(0.0, 1.0) as f64 } else { 0.15 };
        let bh = h * val;
        ctx.set_fill_style_str("rgba(255,255,255,0.64)");
        let bx = x + i as f64 * (bw + gap);
        rounded_rect(ctx, bx, base - bh, bw, bh, bw / 2.0);
        let _ = ctx.fill();
    }
}

pub fn draw_pill(ctx: &CanvasRenderingContext2d, cx: f64, cy: f64, text: &str, w: f64) {
    let pad = u(3.6, w);
    let h = u(8.2, w);
    let font = format!("600 {}px Inter, sans-serif", u(3.6, w));
    ctx.set_font(&font);
    let tw = ctx.measure_text(text).ok().map(|m| m.width()).unwrap_or(0.0);
    let pill_w = tw + pad * 2.0;

    ctx.set_fill_style_str("rgba(255,255,255,0.14)");
    rounded_rect(ctx, cx - pill_w / 2.0, cy - h / 2.0, pill_w, h, h / 2.0);
    let _ = ctx.fill();

    ctx.set_fill_style_str("rgba(255,255,255,0.95)");
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");
    let _ = ctx.fill_text(text, cx, cy);
}

#[derive(Clone, Copy)]
pub enum Glyph {
    Play,
    Pause,
    Prev,
    Next,
    Shuffle,
    Repeat,
}

pub fn draw_glyph_button(
    ctx: &CanvasRenderingContext2d,
    cx: f64,
    cy: f64,
    size: f64,
    glyph: Glyph,
    big: bool,
) {
    if big {
        circle(ctx, cx, cy, size, "rgba(255,255,255,0.95)");
    } else {
        circle(ctx, cx, cy, size, "rgba(255,255,255,0.2)");
    }

    ctx.save();
    let stroke = if big { "#14171c" } else { "#fff" };
    ctx.set_stroke_style_str(stroke);
    ctx.set_fill_style_str(stroke);
    ctx.set_line_width(size * 0.22);
    ctx.set_line_cap("round");
    ctx.set_line_join("round");

    let s = size * 0.42;
    match glyph {
        Glyph::Play => {
            ctx.begin_path();
            ctx.move_to(cx - s * 0.6, cy - s);
            ctx.line_to(cx + s * 0.9, cy);
            ctx.line_to(cx - s * 0.6, cy + s);
            ctx.close_path();
            let _ = ctx.fill();
        }
        Glyph::Pause => {
            ctx.begin_path();
            ctx.rect(cx - s * 0.7, cy - s, s * 0.55, s * 2.0);
            ctx.rect(cx + s * 0.15, cy - s, s * 0.55, s * 2.0);
            let _ = ctx.fill();
        }
        Glyph::Prev | Glyph::Next => {
            let dir = if matches!(glyph, Glyph::Prev) { -1.0 } else { 1.0 };
            ctx.begin_path();
            ctx.move_to(cx - dir * s * 0.6, cy - s);
            ctx.line_to(cx + dir * s * 0.4, cy);
            ctx.line_to(cx - dir * s * 0.6, cy + s);
            ctx.close_path();
            let _ = ctx.fill();
            ctx.begin_path();
            ctx.move_to(cx + dir * s * 0.9, cy - s);
            ctx.line_to(cx + dir * s * 0.9, cy + s);
            let _ = ctx.stroke();
        }
        Glyph::Shuffle | Glyph::Repeat => {
            ctx.begin_path();
            ctx.move_to(cx - s, cy - s * 0.4);
            ctx.line_to(cx + s, cy - s * 0.4);
            ctx.move_to(cx - s, cy + s * 0.4);
            ctx.line_to(cx + s, cy + s * 0.4);
            let _ = ctx.stroke();
        }
    }
    ctx.restore();
}
