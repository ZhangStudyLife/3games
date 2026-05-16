#![windows_subsystem = "windows"]

mod core;
mod scenes;
mod utils;

use macroquad::prelude::*;
use core::scene::SceneManager;
use scenes::menu::MenuScene;

fn window_conf() -> Conf {
    Conf {
        window_title: "Mini Games Collection".to_owned(),
        window_width: 960,
        window_height: 768,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut scene_mgr = SceneManager::new();
    scene_mgr.push(Box::new(MenuScene::new()));
    scene_mgr.run().await;
}
