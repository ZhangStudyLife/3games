use macroquad::prelude::*;
use std::collections::HashSet;

pub struct InputState {
    prev_keys: HashSet<KeyCode>,
    curr_keys: HashSet<KeyCode>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            prev_keys: HashSet::new(),
            curr_keys: HashSet::new(),
        }
    }

    pub fn update(&mut self) {
        self.prev_keys = self.curr_keys.clone();
        self.curr_keys.clear();
        for key in [
            KeyCode::Up,
            KeyCode::Down,
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::W,
            KeyCode::A,
            KeyCode::S,
            KeyCode::D,
            KeyCode::Space,
            KeyCode::Enter,
            KeyCode::Escape,
            KeyCode::R,
            KeyCode::Q,
            KeyCode::E,
            KeyCode::Key1,
            KeyCode::Key2,
            KeyCode::Key3,
            KeyCode::Key4,
            KeyCode::P,
        ] {
            if is_key_down(key) {
                self.curr_keys.insert(key);
            }
        }
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.curr_keys.contains(&key) && !self.prev_keys.contains(&key)
    }

    pub fn is_down(&self, key: KeyCode) -> bool {
        self.curr_keys.contains(&key)
    }

    pub fn direction_wasd(&self) -> Option<(i32, i32)> {
        if self.is_pressed(KeyCode::W) {
            return Some((0, -1));
        }
        if self.is_pressed(KeyCode::S) {
            return Some((0, 1));
        }
        if self.is_pressed(KeyCode::A) {
            return Some((-1, 0));
        }
        if self.is_pressed(KeyCode::D) {
            return Some((1, 0));
        }
        None
    }

    pub fn direction_arrows(&self) -> Option<(i32, i32)> {
        if self.is_pressed(KeyCode::Up) {
            return Some((0, -1));
        }
        if self.is_pressed(KeyCode::Down) {
            return Some((0, 1));
        }
        if self.is_pressed(KeyCode::Left) {
            return Some((-1, 0));
        }
        if self.is_pressed(KeyCode::Right) {
            return Some((1, 0));
        }
        None
    }

    pub fn any_direction(&self) -> Option<(i32, i32)> {
        self.direction_arrows().or_else(|| self.direction_wasd())
    }

    pub fn confirm(&self) -> bool {
        self.is_pressed(KeyCode::Enter) || self.is_pressed(KeyCode::Space)
    }

    pub fn cancel(&self) -> bool {
        self.is_pressed(KeyCode::Escape)
    }
}
