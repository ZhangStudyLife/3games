use macroquad::audio::Sound;
use macroquad::texture::Texture2D;
use std::collections::HashMap;

pub struct AssetManager {
    textures: HashMap<String, Texture2D>,
    sounds: HashMap<String, Sound>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            sounds: HashMap::new(),
        }
    }

    pub fn get_texture(&self, name: &str) -> Option<&Texture2D> {
        self.textures.get(name)
    }

    pub fn get_sound(&self, name: &str) -> Option<&Sound> {
        self.sounds.get(name)
    }

    pub fn add_texture(&mut self, name: &str, tex: Texture2D) {
        self.textures.insert(name.to_string(), tex);
    }

    pub fn add_sound(&mut self, name: &str, sound: Sound) {
        self.sounds.insert(name.to_string(), sound);
    }
}
