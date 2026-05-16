pub mod board;
pub mod renderer;
pub mod effects;

use macroquad::prelude::*;
use crate::core::scene::{Scene, Transition};
use crate::core::input::InputState;
use crate::core::particle::ParticleSystem;
use crate::utils::color::*;
use board::{Board, Direction, PuzzleState};
use effects::TileAnimation;

pub struct PuzzleScene {
    board: Board,
    animations: Vec<TileAnimation>,
    particles: ParticleSystem,
    anim_timer: f32,
    input_lock: bool,
}

impl PuzzleScene {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            animations: Vec::new(),
            particles: ParticleSystem::new(),
            anim_timer: 0.0,
            input_lock: false,
        }
    }
}

impl Scene for PuzzleScene {
    fn update(&mut self, dt: f32, input: &InputState) -> Option<Transition> {
        if input.cancel() {
            return Some(Transition::Pop);
        }

        // Update animations
        self.animations.retain_mut(|a| {
            a.update(dt);
            !a.is_done()
        });
        self.particles.update(dt);
        self.input_lock = !self.animations.is_empty();

        match self.board.state {
            PuzzleState::Playing => {
                if !self.input_lock {
                    let dir = if input.is_pressed(KeyCode::Up) || input.is_pressed(KeyCode::W) {
                        Some(Direction::Up)
                    } else if input.is_pressed(KeyCode::Down) || input.is_pressed(KeyCode::S) {
                        Some(Direction::Down)
                    } else if input.is_pressed(KeyCode::Left) || input.is_pressed(KeyCode::A) {
                        Some(Direction::Left)
                    } else if input.is_pressed(KeyCode::Right) || input.is_pressed(KeyCode::D) {
                        Some(Direction::Right)
                    } else {
                        None
                    };

                    if let Some(dir) = dir {
                        if let Some(moves) = self.board.slide(dir) {
                            // Create animations for each tile move
                            for m in &moves.tile_moves {
                                self.animations.push(TileAnimation::new(
                                    m.from, m.to, m.value, m.merged,
                                    &self.board,
                                ));
                            }
                            // Emit particles for merges
                            for m in &moves.tile_moves {
                                if m.merged {
                                    let (px, py) = renderer::tile_pixel_pos(m.to.0, m.to.1, &self.board);
                                    let colors = vec![ACCENT_YELLOW, ACCENT_ORANGE, Color::new(1.0, 0.9, 0.5, 1.0), Color::new(1.0, 1.0, 0.8, 1.0)];
                                    self.particles.emit(
                                        Vec2::new(px, py), 125, (500.0, 1600.0), (0.8, 2.0), (1.6, 5.0), &colors,
                                    );
                                }
                            }
                        }
                    }
                }
            }
            PuzzleState::Won | PuzzleState::Lost => {
                if input.confirm() || input.is_pressed(KeyCode::R) {
                    self.board.reset();
                    self.animations.clear();
                    self.particles = ParticleSystem::new();
                }
                if input.cancel() {
                    return Some(Transition::Pop);
                }
            }
        }

        None
    }

    fn draw(&self) {
        clear_background(BG_DARK);
        renderer::draw_board(&self.board, &self.animations, &self.particles);
    }
}
