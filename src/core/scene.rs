use crate::core::transition::TransitionAnimator;
use crate::core::input::InputState;
use macroquad::prelude::*;

pub enum Transition {
    Push(Box<dyn Scene>),
    Pop,
    Replace(Box<dyn Scene>),
    Quit,
}

pub trait Scene {
    fn on_enter(&mut self) {}
    fn update(&mut self, dt: f32, input: &InputState) -> Option<Transition>;
    fn draw(&self);
    fn on_exit(&mut self) {}
}

pub struct SceneManager {
    stack: Vec<Box<dyn Scene>>,
    transition: TransitionAnimator,
    pending: Option<Transition>,
    input: InputState,
}

impl SceneManager {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            transition: TransitionAnimator::new(),
            pending: None,
            input: InputState::new(),
        }
    }

    pub fn push(&mut self, mut scene: Box<dyn Scene>) {
        scene.on_enter();
        self.stack.push(scene);
    }

    pub async fn run(&mut self) {
        loop {
            let dt = get_frame_time();
            self.input.update();

            // Handle transition animation
            if self.transition.is_active() {
                self.transition.update(dt);

                // Draw current scene underneath
                if let Some(scene) = self.stack.last() {
                    scene.draw();
                }
                self.transition.draw();

                // When fade_out completes, apply pending transition
                if self.transition.is_fade_out_done() {
                    if let Some(pending) = self.pending.take() {
                        self.apply_transition(pending);
                    }
                }

                // When fade_in completes, reset and resume normal flow
                if self.transition.is_fade_in_done() {
                    self.transition.reset();
                }

                next_frame().await;
                continue;
            }

            // Normal scene update
            if let Some(scene) = self.stack.last_mut() {
                if let Some(trans) = scene.update(dt, &self.input) {
                    match &trans {
                        Transition::Quit => return,
                        _ => {
                            self.pending = Some(trans);
                            self.transition.start_fade_out();
                        }
                    }
                } else {
                    scene.draw();
                }
            } else {
                return;
            }

            next_frame().await;
        }
    }

    fn apply_transition(&mut self, trans: Transition) {
        match trans {
            Transition::Push(mut scene) => {
                scene.on_enter();
                self.stack.push(scene);
                self.transition.start_fade_in();
            }
            Transition::Pop => {
                if let Some(mut scene) = self.stack.pop() {
                    scene.on_exit();
                }
                self.transition.start_fade_in();
            }
            Transition::Replace(mut scene) => {
                if let Some(mut old) = self.stack.pop() {
                    old.on_exit();
                }
                scene.on_enter();
                self.stack.push(scene);
                self.transition.start_fade_in();
            }
            Transition::Quit => {}
        }
    }
}
