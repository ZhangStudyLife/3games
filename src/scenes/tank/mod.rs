pub mod game;
pub mod level;
pub mod ai;
pub mod collision;
pub mod renderer;

use macroquad::prelude::*;
use crate::core::scene::{Scene, Transition};
use crate::core::input::InputState;
use crate::core::particle::ParticleSystem;
use crate::core::render::draw_text_centered;
use crate::utils::color::*;
use game::{TankGame, TankState};

pub struct TankScene {
    game: TankGame,
    ai_controller: ai::EnemyAi,
    particles: ParticleSystem,
    level_num: u8,
    ai_mode: bool,
}

impl TankScene {
    pub fn new(level: u8) -> Self {
        Self {
            game: TankGame::new(level),
            ai_controller: ai::EnemyAi::new(),
            particles: ParticleSystem::new(),
            level_num: level,
            ai_mode: false,
        }
    }
}

impl Scene for TankScene {
    fn update(&mut self, dt: f32, input: &InputState) -> Option<Transition> {
        // Toggle AI mode with E key
        if input.is_pressed(KeyCode::E) && self.game.state == TankState::Playing {
            self.ai_mode = !self.ai_mode;
        }

        match self.game.state {
            TankState::Playing => {
                if input.is_pressed(KeyCode::P) || input.is_pressed(KeyCode::Escape) {
                    self.game.state = TankState::Paused;
                    return None;
                }

                if self.ai_mode {
                    // AI controls the player tank
                    let target = if let Some(enemy) = self.game.enemies.first() {
                        enemy.pos
                    } else {
                        self.game.level.base_pos
                    };

                    self.ai_controller.update(
                        &mut self.game.player,
                        target,
                        self.game.level.base_pos,
                        &self.game.walls,
                        dt,
                    );

                    // AI shooting logic
                    self.game.player.fire_timer -= dt;
                    if self.game.player.fire_timer <= 0.0 {
                        let dir = self.game.player.direction.to_vec2();
                        let mut should_shoot = false;
                        for enemy in &self.game.enemies {
                            let to_enemy = enemy.pos - self.game.player.pos;
                            let dot = dir.x * to_enemy.x + dir.y * to_enemy.y;
                            if dot > 0.0 {
                                let dist = to_enemy.length();
                                if dist < 300.0 {
                                    should_shoot = true;
                                    break;
                                }
                            }
                        }

                        if should_shoot {
                            self.game.player.fire_timer = self.game.player.fire_cooldown;
                            let bullet_pos = self.game.player.pos + self.game.player.direction.to_vec2() * (self.game.player.size + 4.0);
                            self.game.bullets.push(game::Bullet {
                                pos: bullet_pos,
                                direction: self.game.player.direction,
                                speed: 300.0,
                                owner_is_player: true,
                                damage: 1,
                                alive: true,
                            });
                        }
                    }

                    self.game.update_ai_only(dt);
                } else {
                    self.game.update(dt, input);
                }
            }
            TankState::Paused => {
                if input.is_pressed(KeyCode::P) || input.confirm() {
                    self.game.state = TankState::Playing;
                }
                if input.is_pressed(KeyCode::Escape) {
                    return Some(Transition::Pop);
                }
            }
            TankState::LevelComplete => {
                if input.confirm() {
                    if self.level_num < 3 {
                        self.level_num += 1;
                        self.game = TankGame::new(self.level_num);
                        self.ai_controller = ai::EnemyAi::new();
                    } else {
                        self.game.state = TankState::Victory;
                    }
                }
            }
            TankState::GameOver | TankState::Victory => {
                if input.confirm() {
                    return Some(Transition::Pop);
                }
                if input.is_pressed(KeyCode::R) {
                    self.game = TankGame::new(self.level_num);
                    self.ai_controller = ai::EnemyAi::new();
                    self.ai_mode = false;
                }
            }
        }

        None
    }

    fn draw(&self) {
        clear_background(Color::new(0.08, 0.08, 0.1, 1.0));
        renderer::draw_game(&self.game);

        // Show AI mode indicator
        if self.ai_mode {
            let sw = screen_width();
            draw_text_centered("AI MODE", sw / 2.0, 60.0, 20.0, ACCENT_BLUE);
        }
    }
}
