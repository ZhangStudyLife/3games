use serde::{Deserialize, Serialize};
use crate::core::storage::Storage;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScoreEntry {
    pub name: String,
    pub score: u32,
    pub speed: f32,
    pub mode: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct ScoreBoard {
    pub entries: Vec<ScoreEntry>,
}

impl ScoreBoard {
    pub fn load() -> Self {
        Storage::load("snake_scores.json")
    }

    pub fn save(&self) {
        Storage::save("snake_scores.json", self).ok();
    }

    pub fn add_entry(&mut self, entry: ScoreEntry) {
        self.entries.push(entry);
        self.entries.sort_by(|a, b| b.score.cmp(&a.score));
        if self.entries.len() > 50 {
            self.entries.truncate(50);
        }
    }

    pub fn top_n(&self, n: usize) -> &[ScoreEntry] {
        &self.entries[..self.entries.len().min(n)]
    }
}
