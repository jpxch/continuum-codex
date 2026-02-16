use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Zanpakuto {
    pub id: String,
    pub owner: String,
    pub sealed_name: String,
    pub shikai: Option<Shikai>,
    pub bankai: Option<Bankai>,
    pub canon_status: CanonStatus,
}

#[derive(Debug, Deserialize)]
pub struct Shikai {
    pub name: String,
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
    pub fn validate(&self) -> Result<(), String> {
        if self.bankai.is_some() && self.shikai.is_none() {
            return Err(format!(
                "Invariant violation: {} has a Bankai but no Shikai.",
                self.sealed_name
            ));
        }

        if self.id != self.id.to_lowercase() {
            return Err("invariant violation: id must be lowercase.".into());
        }

        Ok(())
    }
}
