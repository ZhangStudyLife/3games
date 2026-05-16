use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::PathBuf;

pub struct Storage;

impl Storage {
    pub fn data_dir() -> PathBuf {
        let mut dir = dirs_next::data_dir().unwrap_or_else(|| PathBuf::from("."));
        dir.push("mini_games");
        fs::create_dir_all(&dir).ok();
        dir
    }

    pub fn save<T: Serialize>(filename: &str, data: &T) -> Result<(), String> {
        let path = Self::data_dir().join(filename);
        let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
        fs::write(path, json).map_err(|e| e.to_string())
    }

    pub fn load<T: DeserializeOwned + Default>(filename: &str) -> T {
        let path = Self::data_dir().join(filename);
        if let Ok(json) = fs::read_to_string(path) {
            serde_json::from_str(&json).unwrap_or_default()
        } else {
            T::default()
        }
    }
}
