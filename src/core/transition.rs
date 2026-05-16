use macroquad::prelude::*;

pub struct TransitionAnimator {
    alpha: f32,
    fading_out: bool,
    active: bool,
    speed: f32,
    fade_out_done: bool,
}

impl TransitionAnimator {
    pub fn new() -> Self {
        Self {
            alpha: 0.0,
            fading_out: false,
            active: false,
            speed: 3.0,
            fade_out_done: false,
        }
    }

    pub fn start_fade_out(&mut self) {
        self.active = true;
        self.fading_out = true;
        self.fade_out_done = false;
        self.alpha = 0.0;
    }

    pub fn start_fade_in(&mut self) {
        self.active = true;
        self.fading_out = false;
        self.fade_out_done = false;
        self.alpha = 1.0;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn is_fade_out_done(&self) -> bool {
        self.fade_out_done
    }

    pub fn is_fade_in_done(&self) -> bool {
        self.active && !self.fading_out && self.alpha <= 0.0
    }

    pub fn reset(&mut self) {
        self.active = false;
        self.alpha = 0.0;
        self.fade_out_done = false;
    }

    pub fn update(&mut self, dt: f32) {
        if !self.active {
            return;
        }
        if self.fading_out {
            self.alpha += self.speed * dt;
            if self.alpha >= 1.0 {
                self.alpha = 1.0;
                self.fade_out_done = true;
            }
        } else {
            self.alpha -= self.speed * dt;
            if self.alpha <= 0.0 {
                self.alpha = 0.0;
            }
        }
    }

    pub fn draw(&self) {
        if self.alpha > 0.01 {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, self.alpha),
            );
        }
    }
}
