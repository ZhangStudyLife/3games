use macroquad::prelude::*;
use crate::scenes::tank::game::Wall;

pub struct CollisionSystem;

impl CollisionSystem {
    pub fn tank_wall_collision(pos: Vec2, size: f32, walls: &[Wall]) -> bool {
        for wall in walls {
            if wall.health <= 0 {
                continue;
            }
            if pos.x + size > wall.rect.x && pos.x - size < wall.rect.x + wall.rect.w
                && pos.y + size > wall.rect.y && pos.y - size < wall.rect.y + wall.rect.h
            {
                return true;
            }
        }
        false
    }

    pub fn tank_base_collision(pos: Vec2, size: f32, base_pos: Vec2, base_size: f32) -> bool {
        pos.x + size > base_pos.x - base_size / 2.0 && pos.x - size < base_pos.x + base_size / 2.0
            && pos.y + size > base_pos.y - base_size / 2.0 && pos.y - size < base_pos.y + base_size / 2.0
    }

    pub fn point_in_rect(point: Vec2, center: Vec2, size: f32) -> bool {
        point.x >= center.x - size && point.x <= center.x + size
            && point.y >= center.y - size && point.y <= center.y + size
    }
}
