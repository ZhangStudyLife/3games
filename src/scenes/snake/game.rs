use macroquad::prelude::*;
use std::collections::VecDeque;
use crate::core::input::InputState;
use crate::scenes::snake::ai::SnakeAi;

#[derive(Clone, Copy, PartialEq)]
pub enum SnakeState {
    Playing,
    Paused,
    GameOver,
}

#[derive(Clone, Copy, Debug)]
pub enum AiAlgorithm {
    Greedy,
    AStar,
}

pub struct SnakeGame {
    pub grid_w: i32,
    pub grid_h: i32,
    pub cell_size: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub snake: VecDeque<(i32, i32)>,
    pub direction: (i32, i32),
    next_direction: (i32, i32),
    pub food: (i32, i32),
    pub score: u32,
    pub speed_multiplier: f32,
    base_interval: f32,
    elapsed: f32,
    pub state: SnakeState,
    pub ai_mode: bool,
    pub ai_algorithm: AiAlgorithm,
    ai_timer: f32,
}

impl SnakeGame {
    pub fn new(grid_w: i32, grid_h: i32) -> Self {
        let sw = screen_width();
        let sh = screen_height();
        let cell_size = ((sw - 100.0) / grid_w as f32).min((sh - 150.0) / grid_h as f32).min(30.0);
        let offset_x = (sw - grid_w as f32 * cell_size) / 2.0;
        let offset_y = (sh - grid_h as f32 * cell_size) / 2.0 + 20.0;

        let start_x = grid_w / 2;
        let start_y = grid_h / 2;
        let mut snake = VecDeque::new();
        snake.push_back((start_x, start_y));
        snake.push_back((start_x - 1, start_y));
        snake.push_back((start_x - 2, start_y));

        let mut game = Self {
            grid_w,
            grid_h,
            cell_size,
            offset_x,
            offset_y,
            snake,
            direction: (1, 0),
            next_direction: (1, 0),
            food: (0, 0),
            score: 0,
            speed_multiplier: 1.0,
            base_interval: 0.15,
            elapsed: 0.0,
            state: SnakeState::Playing,
            ai_mode: false,
            ai_algorithm: AiAlgorithm::AStar,
            ai_timer: 0.0,
        };
        game.spawn_food();
        game
    }

    pub fn update(&mut self, dt: f32, input: &InputState) {
        if self.state != SnakeState::Playing {
            return;
        }

        // Manual input
        if !self.ai_mode {
            if let Some(dir) = input.any_direction() {
                if (dir.0 + self.direction.0, dir.1 + self.direction.1) != (0, 0) {
                    self.next_direction = dir;
                }
            }
        }

        // AI input
        if self.ai_mode {
            self.ai_timer += dt;
            if self.ai_timer >= 0.05 {
                self.ai_timer = 0.0;
                let obstacles = self.body_set();
                let dir = match self.ai_algorithm {
                    AiAlgorithm::Greedy => SnakeAi::greedy_step(
                        self.snake[0], self.food, &obstacles, self.grid_w, self.grid_h,
                    ),
                    AiAlgorithm::AStar => SnakeAi::astar_step(
                        self.snake[0], self.food, &obstacles, self.grid_w, self.grid_h,
                    ),
                };
                if let Some(dir) = dir {
                    if (dir.0 + self.direction.0, dir.1 + self.direction.1) != (0, 0) {
                        self.next_direction = dir;
                    }
                }
            }
        }

        self.elapsed += dt * self.speed_multiplier;
        if self.elapsed >= self.base_interval {
            self.elapsed -= self.base_interval;
            self.move_snake();
        }
    }

    fn move_snake(&mut self) {
        self.direction = self.next_direction;
        let head = self.snake[0];
        let new_head = (head.0 + self.direction.0, head.1 + self.direction.1);

        // Check wall collision
        if new_head.0 < 0 || new_head.0 >= self.grid_w || new_head.1 < 0 || new_head.1 >= self.grid_h {
            self.state = SnakeState::GameOver;
            return;
        }

        // Check self collision
        if self.snake.contains(&new_head) {
            self.state = SnakeState::GameOver;
            return;
        }

        self.snake.push_front(new_head);

        if new_head == self.food {
            self.score += 10;
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }

    fn body_set(&self) -> std::collections::HashSet<(i32, i32)> {
        self.snake.iter().skip(1).cloned().collect()
    }

    pub fn spawn_food(&mut self) {
        let body: std::collections::HashSet<(i32, i32)> = self.snake.iter().cloned().collect();
        let mut attempts = 0;
        loop {
            let x = rand::gen_range(0, self.grid_w);
            let y = rand::gen_range(0, self.grid_h);
            if !body.contains(&(x, y)) {
                self.food = (x, y);
                return;
            }
            attempts += 1;
            if attempts > 1000 {
                break;
            }
        }
    }

    pub fn set_speed(&mut self, multiplier: f32) {
        self.speed_multiplier = multiplier;
    }

    pub fn toggle_ai(&mut self) {
        self.ai_mode = !self.ai_mode;
        self.ai_timer = 0.0;
    }

    pub fn cycle_ai_algorithm(&mut self) {
        self.ai_algorithm = match self.ai_algorithm {
            AiAlgorithm::Greedy => AiAlgorithm::AStar,
            AiAlgorithm::AStar => AiAlgorithm::Greedy,
        };
    }
}
