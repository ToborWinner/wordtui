use std::{error::Error, fs, io};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
    pub name: String,
    pub words: Vec<Word>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Word {
    pub word: String,
    pub url: String,
    pub audio: Option<Vec<AudioLink>>,
    pub extract: Vec<ExtractSection>,
    pub answer: Option<String>,
    #[serde(default)]
    pub streak: u32,
    #[serde(default)]
    pub correct: u32,
    #[serde(default)]
    pub wrong: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioLink {
    pub ns: i32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtractSection {
    pub name: String,
    pub content: String,
}

pub fn load_language() -> Result<Language, Box<dyn Error>> {
    let mut config = match dirs::config_dir() {
        Some(config) => config,
        None => {
            return Err(
                io::Error::new(io::ErrorKind::NotFound, "Failed to find config directory").into(),
            )
        }
    };

    config.push("wordtui");
    config.push("language.json");

    let content = fs::read_to_string(config)?;
    serde_json::from_str(&content).map_err(|e| e.into())
}

pub fn save_language(language: Language) -> Result<(), Box<dyn Error>> {
    let mut config = match dirs::config_dir() {
        Some(config) => config,
        None => {
            return Err(
                io::Error::new(io::ErrorKind::NotFound, "Failed to find config directory").into(),
            )
        }
    };

    config.push("wordtui");
    fs::create_dir_all(&config)?;

    config.push("language.json");
    let content = serde_json::to_string(&language)?;
    fs::write(config, content)?;

    Ok(())
}
