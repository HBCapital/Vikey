//! Vikey Core - Plugin Registry
//!
//! Quản lý việc đăng ký và tra cứu các Language Plugins.

use std::collections::HashMap;
use crate::traits::LanguagePlugin;

/// Registry để quản lý các Language Plugins
pub struct PluginRegistry {
    /// Map từ language ID → Plugin instance
    plugins: HashMap<String, Box<dyn LanguagePlugin>>,
    
    /// Language hiện tại đang được sử dụng
    current_language: Option<String>,
    
    /// Input method hiện tại
    current_input_method: Option<String>,
}

impl PluginRegistry {
    /// Tạo registry mới (rỗng)
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            current_language: None,
            current_input_method: None,
        }
    }
    
    /// Đăng ký một Language Plugin
    /// 
    /// # Arguments
    /// * `plugin` - Plugin cần đăng ký
    /// 
    /// # Returns
    /// Ok nếu đăng ký thành công, Err nếu ID đã tồn tại
    pub fn register(&mut self, plugin: Box<dyn LanguagePlugin>) -> Result<(), RegistryError> {
        let id = plugin.id().to_string();
        
        if self.plugins.contains_key(&id) {
            return Err(RegistryError::DuplicateId(id));
        }
        
        self.plugins.insert(id, plugin);
        Ok(())
    }
    
    /// Hủy đăng ký một plugin
    pub fn unregister(&mut self, id: &str) -> Option<Box<dyn LanguagePlugin>> {
        self.plugins.remove(id)
    }
    
    /// Lấy plugin theo ID
    pub fn get(&self, id: &str) -> Option<&dyn LanguagePlugin> {
        self.plugins.get(id).map(|p| p.as_ref())
    }
    
    /// Lấy danh sách tất cả language IDs đã đăng ký
    pub fn languages(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }
    
    /// Đặt language hiện tại
    pub fn set_language(&mut self, id: &str) -> Result<(), RegistryError> {
        if !self.plugins.contains_key(id) {
            return Err(RegistryError::LanguageNotFound(id.to_string()));
        }
        
        self.current_language = Some(id.to_string());
        
        // Auto-select first input method of this language
        if let Some(plugin) = self.plugins.get(id) {
            let methods = plugin.input_methods();
            if let Some(first_method) = methods.first() {
                self.current_input_method = Some(first_method.to_string());
            }
        }
        
        Ok(())
    }
    
    /// Đặt input method hiện tại
    pub fn set_input_method(&mut self, id: &str) -> Result<(), RegistryError> {
        // Kiểm tra input method có tồn tại trong current language không
        if let Some(lang_id) = &self.current_language {
            if let Some(plugin) = self.plugins.get(lang_id) {
                if plugin.input_methods().contains(&id) {
                    self.current_input_method = Some(id.to_string());
                    return Ok(());
                }
            }
        }
        
        Err(RegistryError::InputMethodNotFound(id.to_string()))
    }
    
    /// Lấy plugin hiện tại
    pub fn current_plugin(&self) -> Option<&dyn LanguagePlugin> {
        self.current_language
            .as_ref()
            .and_then(|id| self.plugins.get(id))
            .map(|p| p.as_ref())
    }
    
    /// Lấy ID của language hiện tại
    pub fn current_language_id(&self) -> Option<&str> {
        self.current_language.as_deref()
    }
    
    /// Lấy ID của input method hiện tại
    pub fn current_input_method_id(&self) -> Option<&str> {
        self.current_input_method.as_deref()
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Lỗi có thể xảy ra khi thao tác với Registry
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistryError {
    /// Plugin với ID này đã tồn tại
    DuplicateId(String),
    
    /// Không tìm thấy language với ID này
    LanguageNotFound(String),
    
    /// Không tìm thấy input method với ID này
    InputMethodNotFound(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::DuplicateId(id) => write!(f, "Plugin with ID '{}' already exists", id),
            RegistryError::LanguageNotFound(id) => write!(f, "Language '{}' not found", id),
            RegistryError::InputMethodNotFound(id) => write!(f, "Input method '{}' not found", id),
        }
    }
}

impl std::error::Error for RegistryError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_registry_new() {
        let registry = PluginRegistry::new();
        assert!(registry.languages().is_empty());
        assert!(registry.current_plugin().is_none());
    }
}
