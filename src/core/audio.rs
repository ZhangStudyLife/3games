use macroquad::audio::*;

pub struct AudioManager {
    bgm_volume: f32,
    sfx_volume: f32,
    current_bgm: Option<Sound>,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            bgm_volume: 0.5,
            sfx_volume: 0.7,
            current_bgm: None,
        }
    }

    pub async fn load_sound(&self, path: &str) -> Sound {
        load_sound(path).await.unwrap_or_else(|_| {
            // Return a dummy sound if loading fails
            // macroquad doesn't easily create empty Sound, so we handle this gracefully
            panic!("Failed to load sound: {}", path)
        })
    }

    pub fn play_bgm(&mut self, sound: &Sound) {
        self.stop_bgm();
        play_sound(
            sound,
            PlaySoundParams {
                looped: true,
                volume: self.bgm_volume,
            },
        );
        self.current_bgm = Some(sound.clone());
    }

    pub fn stop_bgm(&mut self) {
        if let Some(bgm) = &self.current_bgm {
            stop_sound(bgm);
        }
        self.current_bgm = None;
    }

    pub fn play_sfx(&self, sound: &Sound) {
        play_sound(
            sound,
            PlaySoundParams {
                looped: false,
                volume: self.sfx_volume,
            },
        );
    }

    pub fn set_bgm_volume(&mut self, vol: f32) {
        self.bgm_volume = vol.clamp(0.0, 1.0);
    }

    pub fn set_sfx_volume(&mut self, vol: f32) {
        self.sfx_volume = vol.clamp(0.0, 1.0);
    }
}
