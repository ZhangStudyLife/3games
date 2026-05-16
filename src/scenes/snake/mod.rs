pub mod game;
pub mod ai;
pub mod renderer;
pub mod score;

use macroquad::prelude::*;
use crate::core::scene::{Scene, Transition};
use crate::core::input::InputState;
use crate::utils::color::*;
use crate::core::render::draw_text_centered;
use game::{SnakeGame, SnakeState};
use score::ScoreBoard;

pub struct SnakeScene {
    game: SnakeGame,
    score_board: ScoreBoard,
    show_scores: bool,
    game_over_handled: bool,
}

impl SnakeScene {
    pub fn new() -> Self {
        Self {
            game: SnakeGame::new(20, 20),
            score_board: ScoreBoard::load(),
            show_scores: false,
            game_over_handled: false,
        }
    }
}

impl Scene for SnakeScene {
    fn update(&mut self, dt: f32, input: &InputState) -> Option<Transition> {
        if input.cancel() {
            return Some(Transition::Pop);
        }

        match self.game.state {
            SnakeState::Playing => {
                // Toggle AI
                if input.is_pressed(KeyCode::E) {
                    self.game.toggle_ai();
                }
                // Switch AI algorithm
                if input.is_pressed(KeyCode::Q) {
                    self.game.cycle_ai_algorithm();
                }
                // Speed control
                if input.is_pressed(KeyCode::Key1) {
                    self.game.set_speed(1.0);
                }
                if input.is_pressed(KeyCode::Key2) {
                    self.game.set_speed(1.5);
                }
                if input.is_pressed(KeyCode::Key3) {
                    self.game.set_speed(2.0);
                }
                if input.is_pressed(KeyCode::Key4) {
                    self.game.set_speed(3.0);
                }
                // Pause
                if input.is_pressed(KeyCode::P) || input.is_pressed(KeyCode::Escape) {
                    self.game.state = SnakeState::Paused;
                    return None;
                }

                self.game.update(dt, input);
            }
            SnakeState::Paused => {
                if input.is_pressed(KeyCode::P) || input.confirm() {
                    self.game.state = SnakeState::Playing;
                }
                if input.is_pressed(KeyCode::Escape) {
                    return Some(Transition::Pop);
                }
            }
            SnakeState::GameOver => {
                if !self.game_over_handled {
                    self.score_board.add_entry(score::ScoreEntry {
                        name: if self.game.ai_mode {
                            format!("AI-{:?}", self.game.ai_algorithm)
                        } else {
                            "Player".to_string()
                        },
                        score: self.game.score,
                        speed: self.game.speed_multiplier,
                        mode: if self.game.ai_mode { "ai" } else { "manual" }.to_string(),
                    });
                    self.score_board.save();
                    self.game_over_handled = true;
                }
                if input.confirm() {
                    return Some(Transition::Pop);
                }
                if input.is_pressed(KeyCode::R) {
                    self.game = SnakeGame::new(20, 20);
                    self.game_over_handled = false;
                }
            }
        }

        None
    }

    fn draw(&self) {
        clear_background(BG_DARK);

        match self.game.state {
            SnakeState::Playing | SnakeState::Paused => {
                renderer::draw_game(&self.game);
            }
            SnakeState::GameOver => {
                renderer::draw_game(&self.game);
                renderer::draw_game_over(&self.game, &self.score_board);
            }
        }
    }
}
