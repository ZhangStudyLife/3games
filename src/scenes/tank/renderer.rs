use macroquad::prelude::*;
use crate::scenes::tank::game::{TankGame, TankState, TankType, Direction};
use crate::utils::color::*;
use crate::core::render::{draw_text_centered, draw_text_shadowed};

pub fn draw_game(game: &TankGame) {
    // Draw walls
    for wall in &game.walls {
        if wall.health <= 0 {
            continue;
        }
        let color = match wall.wall_type {
            crate::scenes::tank::game::WallType::Brick => WALL_BRICK,
            crate::scenes::tank::game::WallType::Steel => WALL_STEEL,
        };
        draw_rectangle(wall.rect.x, wall.rect.y, wall.rect.w, wall.rect.h, color);
        // Brick pattern
        if wall.wall_type == crate::scenes::tank::game::WallType::Brick {
            draw_rectangle_lines(wall.rect.x, wall.rect.y, wall.rect.w, wall.rect.h, 1.0, Color::new(0.6, 0.35, 0.2, 1.0));
        }
    }

    // Draw base
    if game.base_alive {
        draw_rectangle(
            game.level.base_pos.x - 16.0,
            game.level.base_pos.y - 16.0,
            32.0,
            32.0,
            BASE_COLOR,
        );
        draw_text_centered("B", game.level.base_pos.x, game.level.base_pos.y + 6.0, 20.0, BG_DARK);
    } else {
        draw_rectangle(
            game.level.base_pos.x - 16.0,
            game.level.base_pos.y - 16.0,
            32.0,
            32.0,
            Color::new(0.3, 0.3, 0.3, 1.0),
        );
    }

    // Draw player tank
    draw_tank(&game.player);

    // Draw enemy tanks
    for enemy in &game.enemies {
        draw_tank(enemy);
    }

    // Draw bullets
    for bullet in &game.bullets {
        let color = if bullet.owner_is_player { ACCENT_GREEN } else { ACCENT_RED };
        draw_circle(bullet.pos.x, bullet.pos.y, 3.0, color);
    }

    // Draw explosions
    for (pos, life) in &game.explosions {
        let alpha = (life * 2.0).min(1.0);
        let radius = 20.0 * (1.0 - life * 2.0).max(0.0);
        draw_circle(pos.x, pos.y, radius, Color::new(1.0, 0.6, 0.2, alpha));
        draw_circle(pos.x, pos.y, radius * 0.6, Color::new(1.0, 0.9, 0.3, alpha));
    }

    // HUD
    let hud_y = 30.0;
    draw_text_shadowed(&format!("Level: {}", game.current_level), 20.0, hud_y, 24.0, ACCENT_YELLOW, 1.0);
    draw_text_shadowed(&format!("Health: {}/{}", game.player.health, game.player.max_health), 200.0, hud_y, 24.0, ACCENT_GREEN, 1.0);
    draw_text_shadowed(&format!("Enemies: {}", game.enemies.len() + game.enemies_remaining as usize), 420.0, hud_y, 24.0, ACCENT_RED, 1.0);

    // Controls
    draw_text_centered(
        "WASD/Arrows: Move | Space: Shoot | P: Pause | Esc: Menu",
        screen_width() / 2.0,
        screen_height() - 20.0,
        16.0,
        TEXT_DIM,
    );

    // State overlays
    match game.state {
        TankState::Paused => {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.5));
            draw_text_centered("PAUSED", screen_width() / 2.0, screen_height() / 2.0, 48.0, TEXT_PRIMARY);
            draw_text_centered("Press P or Enter to resume", screen_width() / 2.0, screen_height() / 2.0 + 40.0, 20.0, TEXT_SECONDARY);
        }
        TankState::LevelComplete => {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.6));
            draw_text_centered(&format!("Level {} Complete!", game.current_level), screen_width() / 2.0, screen_height() / 2.0 - 20.0, 48.0, ACCENT_GREEN);
            draw_text_centered("Press Enter for next level", screen_width() / 2.0, screen_height() / 2.0 + 30.0, 20.0, TEXT_SECONDARY);
        }
        TankState::GameOver => {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.6));
            draw_text_centered("Game Over!", screen_width() / 2.0, screen_height() / 2.0 - 20.0, 48.0, ACCENT_RED);
            draw_text_centered("Enter: Menu | R: Restart", screen_width() / 2.0, screen_height() / 2.0 + 30.0, 20.0, TEXT_SECONDARY);
        }
        TankState::Victory => {
            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.6));
            draw_text_centered("Victory!", screen_width() / 2.0, screen_height() / 2.0 - 20.0, 48.0, ACCENT_YELLOW);
            draw_text_centered("You completed all levels!", screen_width() / 2.0, screen_height() / 2.0 + 20.0, 24.0, ACCENT_GREEN);
            draw_text_centered("Enter: Menu | R: Play Again", screen_width() / 2.0, screen_height() / 2.0 + 50.0, 20.0, TEXT_SECONDARY);
        }
        _ => {}
    }
}

fn draw_tank(tank: &crate::scenes::tank::game::Tank) {
    let color = if tank.is_player {
        TANK_PLAYER
    } else {
        match tank.tank_type {
            TankType::Heavy => TANK_HEAVY,
            _ => TANK_ENEMY,
        }
    };

    let s = tank.size;
    let x = tank.pos.x;
    let y = tank.pos.y;

    // Body (square)
    draw_rectangle(x - s, y - s, s * 2.0, s * 2.0, color);

    // Barrel
    let barrel_len = s * 1.2;
    let barrel_w = 6.0;
    match tank.direction {
        Direction::Up => draw_rectangle(x - barrel_w / 2.0, y - s - barrel_len, barrel_w, barrel_len, color),
        Direction::Down => draw_rectangle(x - barrel_w / 2.0, y + s, barrel_w, barrel_len, color),
        Direction::Left => draw_rectangle(x - s - barrel_len, y - barrel_w / 2.0, barrel_len, barrel_w, color),
        Direction::Right => draw_rectangle(x + s, y - barrel_w / 2.0, barrel_len, barrel_w, color),
    }

    // Health indicator for heavy tanks
    if tank.max_health > 1 {
        let bar_w = s * 2.0;
        let bar_h = 3.0;
        let bar_y = y - s - 8.0;
        draw_rectangle(x - bar_w / 2.0, bar_y, bar_w, bar_h, BG_LIGHT);
        let fill = bar_w * (tank.health as f32 / tank.max_health as f32);
        draw_rectangle(x - bar_w / 2.0, bar_y, fill, bar_h, ACCENT_GREEN);
    }
}
