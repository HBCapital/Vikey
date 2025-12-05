// plugin.rs - Vietnamese Language Plugin

use vikey_core::traits::{LanguagePlugin, InputMethodTrait, LookupProvider, LanguageRules};
use crate::lookup::VietnameseLookup;
use crate::methods::telex::TelexMethod;
use crate::methods::vni::VNIMethod;
use crate::methods::viqr::VIQRMethod;

/// Vietnamese Language Plugin
pub struct VietnamesePlugin {
    lookup_telex: VietnameseLookup,
    lookup_vni: VietnameseLookup,
}

impl VietnamesePlugin {
    pub fn new() -> Self {
        Self {
            lookup_telex: VietnameseLookup::new_telex(),
            lookup_vni: VietnameseLookup::new_vni(),
        }
    }
}

impl Default for VietnamesePlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguagePlugin for VietnamesePlugin {
    fn name(&self) -> &str {
        "Tiếng Việt"
    }
    
    fn id(&self) -> &str {
        "vietnamese"
    }
    
    fn input_methods(&self) -> Vec<&str> {
        vec!["telex", "vni", "viqr"]
    }
    
    fn create_input_method(&self, id: &str) -> Option<Box<dyn InputMethodTrait>> {
        match id {
            "telex" => Some(Box::new(TelexMethod::new())),
            "vni" => Some(Box::new(VNIMethod::new())),
            "viqr" => Some(Box::new(VIQRMethod::new())),
            _ => None,
        }
    }
    
    fn lookup(&self) -> &dyn LookupProvider {
        &self.lookup_telex
    }
    
    fn rules(&self) -> &dyn LanguageRules {
        // TODO: Implement VietnameseRules
        unimplemented!("VietnameseRules not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_info() {
        let plugin = VietnamesePlugin::new();
        assert_eq!(plugin.id(), "vietnamese");
        assert_eq!(plugin.name(), "Tiếng Việt");
    }
    
    #[test]
    fn test_input_methods() {
        let plugin = VietnamesePlugin::new();
        let methods = plugin.input_methods();
        assert!(methods.contains(&"telex"));
        assert!(methods.contains(&"vni"));
        assert!(methods.contains(&"viqr"));
    }
    
    #[test]
    fn test_create_telex() {
        let plugin = VietnamesePlugin::new();
        let method = plugin.create_input_method("telex");
        assert!(method.is_some());
    }
}
