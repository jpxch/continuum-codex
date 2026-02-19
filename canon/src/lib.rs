pub mod zanpakuto;
pub use zanpakuto::Zanpakuto;
pub mod registry;
pub mod types;
pub use registry::ZanpakutoRegistry;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_yaml;

    #[test]
    fn test_valid_zanpakuto() {
        let data = std::fs::read_to_string("../lore/bleach/zanpakuto/ryujin_jakka.yaml").unwrap();

        let zan: Zanpakuto = serde_yaml::from_str(&data).unwrap();
        assert!(zan.validate().is_ok());
    }

    #[test]
    fn test_registry_load_and_cross_record_rules() {
        let dir = std::path::Path::new("../lore/bleach/zanpakuto");
        let reg = ZanpakutoRegistry::load_from_dir(dir).unwrap();
        assert!(!reg.items.is_empty());
    }
}
