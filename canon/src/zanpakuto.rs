use crate::types::MultilingualString;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Zanpakuto {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<MultilingualString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abilities: Option<Vec<MultilingualString>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_appearance: Option<Appearance>,
    pub owner: MultilingualString,
    pub sealed_name: MultilingualString,
    pub shikai: Option<Shikai>,
    pub bankai: Option<Bankai>,
    pub canon_status: CanonStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Shikai {
    pub name: MultilingualString,
    pub release_command: MultilingualString,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bankai {
    pub name: MultilingualString,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Appearance {
    pub source: String,
    pub arc: Option<String>,
    pub chapter: Option<u32>,
    pub episode: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
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
            shikai.release_command.validate()?;
        }

        if let Some(bankai) = &self.bankai {
            bankai.name.validate()?;
        }

        self.sealed_name.validate()?;

        self.owner.validate()?;

        Ok(())
    }
}
