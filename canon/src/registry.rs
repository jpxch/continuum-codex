use crate::zanpakuto::Zanpakuto;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct ZanpakutoRegistry {
    pub items: Vec<Zanpakuto>,
}

impl ZanpakutoRegistry {
    pub fn load_from_dir(dir: &Path) -> Result<Self, String> {
        if !dir.exists() {
            return Err(format!("Lore directory does not exist: {}", dir.display()));
        }

        let mut items = Vec::new();

        for entry in WalkDir::new(dir)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if !is_yaml(path) {
                continue;
            }

            let raw = std::fs::read_to_string(path)
                .map_err(|e| format!("Failed to read {}: {e}", path.display()))?;

            let zan: Zanpakuto = serde_yaml::from_str(&raw)
                .map_err(|e| format!("YAML parse error in {}: {e}", path.display()))?;

            zan.validate()
                .map_err(|e| format!("Validation error in {}: {e}", path.display()))?;

            enforce_filename_id(path, &zan.id)?;

            items.push(zan);
        }

        let reg = Self { items };
        reg.validate_cross_record()?;
        Ok(reg)
    }

    fn validate_cross_record(&self) -> Result<(), String> {
        let mut ids = HashSet::<String>::new();
        for z in &self.items {
            if !ids.insert(z.id.clone()) {
                return Err(format!("Duplicate id detected: {}", z.id));
            }
        }

        let mut bankai_by_jp = HashMap::<String, String>::new();
        for z in &self.items {
            let Some(bankai) = &z.bankai else { continue };
            let name = bankai.name.jp.trim().to_string();
            if name.is_empty() {
                return Err(format!("Bankai name empty for id {}", z.id));
            }

            if let Some(prev) = bankai_by_jp.insert(name.clone(), z.id.clone()) {
                return Err(format!(
                    "Duplicate Bankai JP name '{}' detected across ids '{}' and '{}'",
                    name, prev, z.id
                ));
            }
        }

        Ok(())
    }
}

fn is_yaml(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|s| s.to_str()),
        Some("yaml") | Some("yml")
    )
}

fn enforce_filename_id(path: &Path, id: &str) -> Result<(), String> {
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format!("Invalid filename (no stem): {}", path.display()))?;

    if file_stem != id {
        return Err(format!(
            "Filename/id mismatch: file '{}' vs is '{}'",
            file_stem, id
        ));
    }
    Ok(())
}
