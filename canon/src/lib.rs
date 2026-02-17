pub mod zanpakuto;
pub use zanpakuto::Zanpakuto;
pub mod types;

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
}
