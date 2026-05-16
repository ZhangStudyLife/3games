use macroquad::prelude::*;
use crate::scenes::tank::game::{Tank, Direction, Wall};

pub struct EnemyAi {
    decision_timer: f32,
    current_dir: Direction,
}

impl EnemyAi {
    pub fn new() -> Self {
        Self {
            decision_timer: 0.0,
            current_dir: Direction::Down,
        }
    }

    pub fn update(&mut self, tank: &mut Tank, player_pos: Vec2, base_pos: Vec2, walls: &[Wall], dt: f32) {
        self.decision_timer -= dt;
        if self.decision_timer <= 0.0 {
            self.decision_timer = rand::gen_range(0.5, 1.5);

            let r = rand::gen_range(0.0, 1.0);
            if r < 0.6 {
                // Chase player
                self.current_dir = self.dir_toward(tank.pos, player_pos);
            } else if r < 0.85 {
                // Go toward base
                self.current_dir = self.dir_toward(tank.pos, base_pos);
            } else {
                // Random
                self.current_dir = match rand::gen_range(0, 4) {
                    0 => Direction::Up,
                    1 => Direction::Down,
                    2 => Direction::Left,
                    _ => Direction::Right,
                };
            }
        }

        tank.direction = self.current_dir;
        let new_pos = tank.pos + self.current_dir.to_vec2() * tank.speed * dt;

        // Check collision with walls
        let mut blocked = false;
        for wall in walls {
            if wall.health <= 0 {
                continue;
            }
            if new_pos.x + tank.size > wall.rect.x && new_pos.x - tank.size < wall.rect.x + wall.rect.w
                && new_pos.y + tank.size > wall.rect.y && new_pos.y - tank.size < wall.rect.y + wall.rect.h
            {
                blocked = true;
                break;
            }
        }

        // Check bounds
        if new_pos.x - tank.size < 0.0 || new_pos.x + tank.size > 960.0
            || new_pos.y - tank.size < 0.0 || new_pos.y + tank.size > 768.0
        {
            blocked = true;
        }

        if !blocked {
            tank.pos = new_pos;
        } else {
            // Change direction on blocked
            self.decision_timer = 0.0;
        }
    }

    fn dir_toward(&self, from: Vec2, to: Vec2) -> Direction {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        if dx.abs() > dy.abs() {
            if dx > 0.0 { Direction::Right } else { Direction::Left }
        } else {
            if dy > 0.0 { Direction::Down } else { Direction::Up }
        }
    }
}
