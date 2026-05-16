use macroquad::prelude::*;
use crate::core::scene::{Scene, Transition};
use crate::core::input::InputState;
use crate::utils::color::*;
use crate::core::render::{draw_rounded_rect_with_border, draw_text_centered};

pub struct MenuScene {
    selected: usize,
    time: f32,
}

impl MenuScene {
    pub fn new() -> Self {
        Self { selected: 0, time: 0.0 }
    }
}

impl Scene for MenuScene {
    fn update(&mut self, _dt: f32, input: &InputState) -> Option<Transition> {
        self.time += _dt;

        if input.is_pressed(KeyCode::Up) || input.is_pressed(KeyCode::W) {
            self.selected = self.selected.wrapping_sub(1) % 3;
        }
        if input.is_pressed(KeyCode::Down) || input.is_pressed(KeyCode::S) {
            self.selected = (self.selected + 1) % 3;
        }

        if input.confirm() {
            return match self.selected {
                0 => Some(Transition::Push(Box::new(crate::scenes::snake::SnakeScene::new()))),
                1 => Some(Transition::Push(Box::new(crate::scenes::puzzle::PuzzleScene::new()))),
                2 => Some(Transition::Push(Box::new(crate::scenes::tank::TankScene::new(1)))),
                _ => None,
            };
        }

        None
    }

    fn draw(&self) {
        clear_background(BG_DARK);

        let sw = screen_width();
        let sh = screen_height();

        // Title
        draw_text_centered("Mini Games Collection", sw / 2.0, 120.0, 48.0, ACCENT_YELLOW);
        draw_text_centered("Select a game to play", sw / 2.0, 165.0, 20.0, TEXT_SECONDARY);

        // Game buttons
        let games = [
            ("1. Snake", "Use arrow keys or WASD to play", ACCENT_GREEN),
            ("2. 2048", "Slide and merge tiles", ACCENT_ORANGE),
            ("3. Tank Battle", "3 levels of tank combat (E: AI mode)", ACCENT_RED),
        ];

        let button_w = 300.0;
        let button_h = 80.0;
        let gap = 20.0;
        let start_y = 220.0;

        for (i, (name, desc, color)) in games.iter().enumerate() {
            let y = start_y + i as f32 * (button_h + gap);
            let x = (sw - button_w) / 2.0;
            let selected = i == self.selected;

            let bg = if selected {
                Color::new(color.r * 0.3, color.g * 0.3, color.b * 0.3, 1.0)
            } else {
                BG_MEDIUM
            };
            let border_color = if selected { *color } else { BG_LIGHT };

            draw_rounded_rect_with_border(Rect::new(x, y, button_w, button_h), 12.0, bg, border_color, 3.0);

            draw_text_centered(name, sw / 2.0, y + 30.0, 28.0, if selected { *color } else { TEXT_PRIMARY });
            draw_text_centered(desc, sw / 2.0, y + 58.0, 16.0, TEXT_DIM);
        }

        // Controls hint
        draw_text_centered("Arrow Keys / WASD: Navigate  |  Enter / Space: Select", sw / 2.0, sh - 40.0, 16.0, TEXT_DIM);
    }
}
