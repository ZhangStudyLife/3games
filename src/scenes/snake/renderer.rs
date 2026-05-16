use macroquad::prelude::*;
use crate::scenes::snake::game::SnakeGame;
use crate::scenes::snake::score::ScoreBoard;
use crate::utils::color::*;
use crate::core::render::{draw_rounded_rect, draw_text_centered, draw_text_shadowed};

pub fn draw_game(game: &SnakeGame) {
    let time = macroquad::time::get_time() as f32;

    // Draw grid background
    for y in 0..game.grid_h {
        for x in 0..game.grid_w {
            let px = game.offset_x + x as f32 * game.cell_size;
            let py = game.offset_y + y as f32 * game.cell_size;
            let bg = if (x + y) % 2 == 0 {
                Color::new(0.18, 0.18, 0.22, 1.0)
            } else {
                Color::new(0.16, 0.16, 0.20, 1.0)
            };
            draw_rectangle(px, py, game.cell_size - 1.0, game.cell_size - 1.0, bg);
        }
    }

    // Draw snake
    let len = game.snake.len() as f32;
    for (i, &(x, y)) in game.snake.iter().enumerate() {
        let px = game.offset_x + x as f32 * game.cell_size;
        let py = game.offset_y + y as f32 * game.cell_size;
        let t = i as f32 / len.max(1.0);
        let color = lerp_color(SNAKE_HEAD, SNAKE_TAIL, t);
        let margin = 1.0;
        draw_rounded_rect(
            Rect::new(px + margin, py + margin, game.cell_size - margin * 2.0, game.cell_size - margin * 2.0),
            4.0,
            color,
        );
    }

    // Draw food with pulse
    let pulse = (time * 4.0).sin() * 0.15 + 0.85;
    let fx = game.offset_x + game.food.0 as f32 * game.cell_size + game.cell_size / 2.0;
    let fy = game.offset_y + game.food.1 as f32 * game.cell_size + game.cell_size / 2.0;
    let food_r = game.cell_size * 0.4 * pulse;
    draw_circle(fx, fy, food_r, FOOD_COLOR);
    draw_circle(fx, fy, food_r * 0.6, Color::new(1.0, 0.5, 0.4, 1.0));

    // Draw HUD
    let hud_y = game.offset_y - 10.0;
    draw_text_shadowed(&format!("Score: {}", game.score), 20.0, hud_y, 24.0, TEXT_PRIMARY, 2.0);
    draw_text_shadowed(&format!("Speed: {:.1}x", game.speed_multiplier), 200.0, hud_y, 20.0, ACCENT_YELLOW, 1.0);

    if game.ai_mode {
        draw_text_shadowed(&format!("AI: {:?}", game.ai_algorithm), 380.0, hud_y, 20.0, ACCENT_BLUE, 1.0);
    }

    // Controls
    let controls_y = game.offset_y + game.grid_h as f32 * game.cell_size + 20.0;
    draw_text_centered(
        "WASD/Arrows: Move | 1-4: Speed | E: Toggle AI | Q: Switch AI | P: Pause",
        screen_width() / 2.0,
        controls_y,
        16.0,
        TEXT_DIM,
    );
}

pub fn draw_game_over(game: &SnakeGame, scores: &ScoreBoard) {
    let sw = screen_width();
    let sh = screen_height();

    // Overlay
    draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.7));

    // Panel
    let panel_w = 400.0;
    let panel_h = 350.0;
    let panel_x = (sw - panel_w) / 2.0;
    let panel_y = (sh - panel_h) / 2.0;
    draw_rounded_rect(Rect::new(panel_x, panel_y, panel_w, panel_h), 16.0, BG_MEDIUM);

    draw_text_centered("Game Over!", sw / 2.0, panel_y + 40.0, 36.0, ACCENT_RED);
    draw_text_centered(&format!("Score: {}", game.score), sw / 2.0, panel_y + 80.0, 28.0, ACCENT_YELLOW);

    // Top scores
    draw_text_centered("Top Scores:", sw / 2.0, panel_y + 120.0, 20.0, TEXT_SECONDARY);
    for (i, entry) in scores.top_n(5).iter().enumerate() {
        let y = panel_y + 150.0 + i as f32 * 25.0;
        draw_text_centered(
            &format!("{}. {} - {} pts", i + 1, entry.name, entry.score),
            sw / 2.0,
            y,
            18.0,
            TEXT_PRIMARY,
        );
    }

    draw_text_centered("Enter: Back to Menu | R: Restart", sw / 2.0, panel_y + panel_h - 20.0, 16.0, TEXT_DIM);
}
