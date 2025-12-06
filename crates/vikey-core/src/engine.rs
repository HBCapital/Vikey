//! Vikey Core - Engine Module
//!
//! Main orchestrator that uses plugins to process input.

use crate::buffer::InputBuffer;
use crate::registry::{PluginRegistry, RegistryError};
use crate::traits::{InputMethodTrait, LanguagePlugin};
use crate::types::Action;

/// Main Vikey Engine
///
/// Sử dụng plugin system để xử lý input cho nhiều ngôn ngữ.
pub struct Engine {
    /// Plugin registry
    registry: PluginRegistry,

    /// Input buffer
    buffer: InputBuffer,

    /// Current input method instance
    current_method: Option<Box<dyn InputMethodTrait>>,
}

impl Engine {
    /// Tạo engine mới với registry rỗng
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
            buffer: InputBuffer::new(),
            current_method: None,
        }
    }

    /// Tạo engine với một registry có sẵn
    pub fn with_registry(registry: PluginRegistry) -> Self {
        Self {
            registry,
            buffer: InputBuffer::new(),
            current_method: None,
        }
    }

    /// Đăng ký một Language Plugin
    pub fn register(&mut self, plugin: Box<dyn LanguagePlugin>) -> Result<(), RegistryError> {
        self.registry.register(plugin)
    }

    /// Đặt ngôn ngữ hiện tại
    pub fn set_language(&mut self, id: &str) -> Result<(), RegistryError> {
        self.registry.set_language(id)?;
        self.update_input_method();
        Ok(())
    }

    /// Đặt input method hiện tại
    pub fn set_input_method(&mut self, id: &str) -> Result<(), RegistryError> {
        self.registry.set_input_method(id)?;
        self.update_input_method();
        Ok(())
    }

    /// Cập nhật input method instance dựa trên selection hiện tại
    fn update_input_method(&mut self) {
        self.current_method = None;

        if let Some(plugin) = self.registry.current_plugin() {
            if let Some(method_id) = self.registry.current_input_method_id() {
                self.current_method = plugin.create_input_method(method_id);
            }
        }
    }

    /// Xử lý một keystroke
    pub fn process(&mut self, key: char) -> Action {
        if let (Some(method), Some(plugin)) =
            (&mut self.current_method, self.registry.current_plugin())
        {
            method.process(key, &mut self.buffer, plugin.lookup())
        } else {
            // Không có plugin/method nào active, passthrough
            Action::DoNothing
        }
    }

    /// Xử lý phím Backspace
    pub fn process_backspace(&mut self) -> Action {
        if let Some(method) = &mut self.current_method {
            method.process_backspace(&mut self.buffer)
        } else {
            Action::DoNothing
        }
    }

    /// Reset buffer và input method state
    pub fn reset(&mut self) {
        self.buffer.clear();
        if let Some(method) = &mut self.current_method {
            method.reset();
        }
    }

    /// Lấy nội dung buffer hiện tại
    pub fn buffer_content(&self) -> String {
        self.buffer.to_string()
    }

    /// Lấy danh sách ngôn ngữ đã đăng ký
    pub fn languages(&self) -> Vec<&str> {
        self.registry.languages()
    }

    /// Lấy danh sách input methods của ngôn ngữ hiện tại
    pub fn input_methods(&self) -> Vec<&str> {
        if let Some(plugin) = self.registry.current_plugin() {
            plugin.input_methods()
        } else {
            Vec::new()
        }
    }

    /// Lấy ID ngôn ngữ hiện tại
    pub fn current_language(&self) -> Option<&str> {
        self.registry.current_language_id()
    }

    /// Lấy ID input method hiện tại
    pub fn current_input_method(&self) -> Option<&str> {
        self.registry.current_input_method_id()
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_new() {
        let engine = Engine::new();
        assert!(engine.languages().is_empty());
        assert!(engine.current_language().is_none());
    }

    #[test]
    fn test_engine_process_without_plugin() {
        let mut engine = Engine::new();
        let action = engine.process('a');
        assert_eq!(action, Action::DoNothing);
    }
}
