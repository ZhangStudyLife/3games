use macroquad::prelude::*;
use crate::scenes::puzzle::renderer::tile_pixel_pos;
use crate::scenes::puzzle::board::Board;

pub struct TileAnimation {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub value: u32,
    pub is_merge: bool,
    from_pixel: (f32, f32),
    to_pixel: (f32, f32),
    progress: f32,
    duration: f32,
    done: bool,
}

impl TileAnimation {
    pub fn new(from: (usize, usize), to: (usize, usize), value: u32, is_merge: bool, board: &Board) -> Self {
        let from_pixel = tile_pixel_pos(from.0, from.1, board);
        let to_pixel = tile_pixel_pos(to.0, to.1, board);
        Self {
            from,
            to,
            value,
            is_merge,
            from_pixel,
            to_pixel,
            progress: 0.0,
            duration: if is_merge { 0.2 } else { 0.12 },
            done: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.progress += dt / self.duration;
        if self.progress >= 1.0 {
            self.progress = 1.0;
            self.done = true;
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }

    pub fn current_pos(&self) -> (f32, f32) {
        let t = ease_out_cubic(self.progress.clamp(0.0, 1.0));
        (
            self.from_pixel.0 + (self.to_pixel.0 - self.from_pixel.0) * t,
            self.from_pixel.1 + (self.to_pixel.1 - self.from_pixel.1) * t,
        )
    }
}

fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}
