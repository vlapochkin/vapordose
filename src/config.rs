use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    EN,
    RU,
}

impl Language {
    /// Pick a default language from the user's locale when nothing is saved yet.
    pub fn from_locale() -> Self {
        let locale = std::env::var("LC_ALL")
            .or_else(|_| std::env::var("LC_MESSAGES"))
            .or_else(|_| std::env::var("LANG"))
            .unwrap_or_default()
            .to_lowercase();
        if locale.starts_with("ru") { Language::RU } else { Language::EN }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub language: Option<Language>,
}

impl AppConfig {
    pub fn load() -> Self {
        fs::read_to_string(Self::path())
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        let path = Self::path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        if let Ok(content) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, content);
        }
    }

    pub fn language(&self) -> Language {
        self.language.unwrap_or_else(Language::from_locale)
    }

    fn path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home).join(".config/vapordose/config.json")
    }
}
