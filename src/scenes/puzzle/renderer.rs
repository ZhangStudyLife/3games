use macroquad::prelude::*;
use crate::scenes::puzzle::board::Board;
use crate::scenes::puzzle::effects::TileAnimation;
use crate::core::particle::ParticleSystem;
use crate::core::render::{draw_rounded_rect, draw_text_centered, draw_text_shadowed};
use crate::utils::color::*;

pub fn tile_pixel_pos(col: usize, row: usize, _board: &Board) -> (f32, f32) {
    let sw = screen_width();
    let sh = screen_height();
    let tile_size = 90.0;
    let gap = 10.0;
    let grid_size = tile_size * 4.0 + gap * 5.0;
    let base_x = (sw - grid_size) / 2.0;
    let base_y = (sh - grid_size) / 2.0 + 30.0;

    let x = base_x + gap + col as f32 * (tile_size + gap) + tile_size / 2.0;
    let y = base_y + gap + row as f32 * (tile_size + gap) + tile_size / 2.0;
    (x, y)
}

pub fn draw_board(board: &Board, animations: &[TileAnimation], particles: &ParticleSystem) {
    let sw = screen_width();
    let sh = screen_height();
    let tile_size = 90.0;
    let gap = 10.0;
    let grid_size = tile_size * 4.0 + gap * 5.0;
    let base_x = (sw - grid_size) / 2.0;
    let base_y = (sh - grid_size) / 2.0 + 30.0;

    // Title
    draw_text_shadowed("2048", sw / 2.0 - 40.0, 50.0, 48.0, ACCENT_YELLOW, 2.0);

    // Score
    draw_text_shadowed(&format!("Score: {}", board.score), 30.0, 90.0, 24.0, TEXT_PRIMARY, 1.0);
    draw_text_shadowed(&format!("Best: {}", board.best_score), sw - 180.0, 90.0, 24.0, ACCENT_ORANGE, 1.0);

    // Board background
    draw_rounded_rect(
        Rect::new(base_x, base_y, grid_size, grid_size),
        12.0,
        BOARD_BG,
    );

    // Tile slots
    for r in 0..4 {
        for c in 0..4 {
            let (cx, cy) = tile_pixel_pos(c, r, board);
            draw_rounded_rect(
                Rect::new(cx - tile_size / 2.0, cy - tile_size / 2.0, tile_size, tile_size),
                8.0,
                TILE_BG,
            );
        }
    }

    // Animated tiles
    let mut animated_positions = std::collections::HashSet::new();
    for anim in animations {
        let (cx, cy) = anim.current_pos();
        let scale = if anim.is_merge && anim.progress() > 0.7 {
            let t = (anim.progress() - 0.7) / 0.3;
            1.0 + 0.15 * (t * std::f32::consts::PI).sin()
        } else {
            1.0
        };
        let size = tile_size * scale;
        let color = tile_color(anim.value);
        draw_rounded_rect(
            Rect::new(cx - size / 2.0, cy - size / 2.0, size, size),
            8.0 * scale,
            color,
        );
        let font_size = if anim.value >= 1000 { 24.0 } else if anim.value >= 100 { 28.0 } else { 32.0 };
        let text_color = if anim.value <= 4 { Color::new(0.4, 0.4, 0.4, 1.0) } else { TEXT_PRIMARY };
        draw_text_centered(&anim.value.to_string(), cx, cy, font_size, text_color);
        animated_positions.insert(anim.to);
    }

    // Static tiles (not being animated)
    for r in 0..4 {
        for c in 0..4 {
            let val = board.cells[r][c];
            if val == 0 {
                continue;
            }
            if animated_positions.contains(&(c, r)) {
                continue;
            }
            let (cx, cy) = tile_pixel_pos(c, r, board);
            let color = tile_color(val);
            draw_rounded_rect(
                Rect::new(cx - tile_size / 2.0, cy - tile_size / 2.0, tile_size, tile_size),
                8.0,
                color,
            );
            let font_size = if val >= 1000 { 24.0 } else if val >= 100 { 28.0 } else { 32.0 };
            let text_color = if val <= 4 { Color::new(0.4, 0.4, 0.4, 1.0) } else { TEXT_PRIMARY };
            draw_text_centered(&val.to_string(), cx, cy, font_size, text_color);
        }
    }

    // Particles
    particles.draw();

    // Game state overlay
    match board.state {
        crate::scenes::puzzle::board::PuzzleState::Won => {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.6));
            draw_text_centered("You Win!", sw / 2.0, sh / 2.0 - 20.0, 48.0, ACCENT_YELLOW);
            draw_text_centered("Enter/R: New Game | Esc: Menu", sw / 2.0, sh / 2.0 + 30.0, 20.0, TEXT_SECONDARY);
        }
        crate::scenes::puzzle::board::PuzzleState::Lost => {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.6));
            draw_text_centered("Game Over!", sw / 2.0, sh / 2.0 - 20.0, 48.0, ACCENT_RED);
            draw_text_centered("Enter/R: New Game | Esc: Menu", sw / 2.0, sh / 2.0 + 30.0, 20.0, TEXT_SECONDARY);
        }
        _ => {}
    }

    // Controls
    draw_text_centered(
        "Arrow Keys / WASD: Slide Tiles | Esc: Back",
        sw / 2.0,
        sh - 30.0,
        16.0,
        TEXT_DIM,
    );
}
