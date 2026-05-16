use macroquad::prelude::*;

pub fn draw_rounded_rect(rect: Rect, radius: f32, color: Color) {
    let r = radius.min(rect.w / 2.0).min(rect.h / 2.0);
    draw_rectangle(rect.x + r, rect.y, rect.w - 2.0 * r, rect.h, color);
    draw_rectangle(rect.x, rect.y + r, rect.w, rect.h - 2.0 * r, color);
    draw_circle(rect.x + r, rect.y + r, r, color);
    draw_circle(rect.x + rect.w - r, rect.y + r, r, color);
    draw_circle(rect.x + r, rect.y + rect.h - r, r, color);
    draw_circle(rect.x + rect.w - r, rect.y + rect.h - r, r, color);
}

pub fn draw_rounded_rect_with_border(rect: Rect, radius: f32, fill_color: Color, border_color: Color, border_width: f32) {
    // Draw border (larger rect)
    draw_rounded_rect(rect, radius, border_color);
    // Draw fill (smaller rect)
    let inner = Rect::new(
        rect.x + border_width,
        rect.y + border_width,
        rect.w - border_width * 2.0,
        rect.h - border_width * 2.0,
    );
    draw_rounded_rect(inner, radius - border_width, fill_color);
}

pub fn draw_text_centered(text: &str, center_x: f32, center_y: f32, font_size: f32, color: Color) {
    let dims = measure_text(text, None, font_size as u16, 1.0);
    let x = center_x - dims.width / 2.0;
    let y = center_y - dims.height / 2.0 + dims.offset_y;
    draw_text(text, x, y, font_size, color);
}

pub fn draw_text_shadowed(text: &str, x: f32, y: f32, font_size: f32, color: Color, shadow_offset: f32) {
    draw_text(text, x + shadow_offset, y + shadow_offset, font_size, Color::new(0.0, 0.0, 0.0, 0.5));
    draw_text(text, x, y, font_size, color);
}

pub fn draw_progress_bar(x: f32, y: f32, w: f32, h: f32, progress: f32, bg: Color, fill: Color) {
    draw_rectangle(x, y, w, h, bg);
    draw_rectangle(x, y, w * progress.clamp(0.0, 1.0), h, fill);
}
