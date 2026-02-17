use crate::types::MultilingualString;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Zanpakuto {
    pub id: String,
    pub owner: MultilingualString,
    pub sealed_name: MultilingualString,
    pub shikai: Option<Shikai>,
    pub bankai: Option<Bankai>,
    pub canon_status: CanonStatus,
}

#[derive(Debug, Deserialize)]
pub struct Shikai {
    pub name: MultilingualString,
    pub release_command: String,
}

#[derive(Debug, Deserialize)]
pub struct Bankai {
    pub name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CanonStatus {
    Manga,
    Anime,
    Novel,
    Filler,
}

impl Zanpakuto {
    fn generate_expected_id(&self) -> String {
        self.sealed_name
            .romaji
            .to_lowercase()
            .replace("ū", "u")
            .replace("ō", "o")
            .replace(" ", "_")
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.bankai.is_some() && self.shikai.is_none() {
            return Err(format!(
                "Invariant violation: {} has a Bankai but no Shikai.",
                self.sealed_name.romaji
            ));
        }

        if self.id != self.generate_expected_id() {
            return Err(format!(
                "Invariant violation: id '{}' does not match canonical romaji '{}'",
                self.id,
                self.generate_expected_id()
            ));
        }

        if let Some(shikai) = &self.shikai {
            shikai.name.validate()?;
        }

        self.sealed_name.validate()?;

        self.owner.validate()?;

        Ok(())
    }
}
