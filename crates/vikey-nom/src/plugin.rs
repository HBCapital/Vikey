//! Vikey Nôm - Plugin Implementation
//!
//! Implementation của LanguagePlugin trait cho chữ Nôm.

use crate::lookup::NomLookup;
use crate::methods::telex_nom::TelexNomMethod;
use vikey_core::traits::{InputMethodTrait, LanguagePlugin, LanguageRules, LookupProvider};

/// Chữ Nôm Language Plugin
///
/// Plugin này cung cấp khả năng nhập chữ Nôm (𡨸喃) bằng
/// nhiều phương pháp khác nhau.
pub struct NomPlugin {
    lookup: NomLookup,
    rules: NomRules,
}

impl NomPlugin {
    /// Tạo NomPlugin mới
    pub fn new() -> Self {
        Self {
            lookup: NomLookup::new(),
            rules: NomRules::new(),
        }
    }
}

impl Default for NomPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguagePlugin for NomPlugin {
    fn name(&self) -> &str {
        "Chữ Nôm (𡨸喃)"
    }

    fn id(&self) -> &str {
        "nom"
    }

    fn input_methods(&self) -> Vec<&str> {
        vec!["telex-nom", "pinyin-nom"]
    }

    fn create_input_method(&self, id: &str) -> Option<Box<dyn InputMethodTrait>> {
        match id {
            "telex-nom" => Some(Box::new(TelexNomMethod::new())),
            // "pinyin-nom" => Some(Box::new(PinyinNomMethod::new())),
            _ => None,
        }
    }

    fn lookup(&self) -> &dyn LookupProvider {
        &self.lookup
    }

    fn rules(&self) -> &dyn LanguageRules {
        &self.rules
    }
}

/// Quy tắc cho chữ Nôm
pub struct NomRules;

impl NomRules {
    pub fn new() -> Self {
        Self
    }
}

impl Default for NomRules {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageRules for NomRules {
    fn is_valid_word(&self, _word: &str) -> bool {
        // Chữ Nôm mỗi ký tự là một từ/âm tiết
        true
    }

    fn is_valid_syllable(&self, _syllable: &str) -> bool {
        true
    }

    fn suggest(&self, _word: &str) -> Vec<String> {
        // TODO: Implement dictionary lookup
        Vec::new()
    }

    fn find_tone_position(&self, _syllable: &str) -> Option<usize> {
        // Chữ Nôm không có dấu thanh riêng
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_info() {
        let plugin = NomPlugin::new();
        assert_eq!(plugin.id(), "nom");
        assert!(plugin.name().contains("Nôm"));
    }

    #[test]
    fn test_input_methods() {
        let plugin = NomPlugin::new();
        let methods = plugin.input_methods();
        assert!(methods.contains(&"telex-nom"));
    }
}
