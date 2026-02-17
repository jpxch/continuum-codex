use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MultilingualString {
    pub jp: String,
    pub romaji: String,
    pub en: Option<String>,
}

impl MultilingualString {
    pub fn validate(&self) -> Result<(), String> {
        if self.jp.trim().is_empty() {
            return Err("Japansese canonical string cannot be empty.".into());
        }

        if self.romaji.trim().is_empty() {
            return Err("Romaji representation cannot be empty.".into());
        }

        Ok(())
    }
}
