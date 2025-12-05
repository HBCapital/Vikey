//! Vikey Core - Generic Input Method Engine
//!
//! This is a pure Rust library providing a plugin-based architecture
//! for multi-language input method support.

pub mod types;
mod buffer;
// TODO Phase 2: Remove Vietnamese-specific modules (will be in vikey-vietnamese)
// mod lookup;
// mod spelling;
// mod processor;

// Plugin system modules
pub mod traits;
pub mod registry;
pub mod engine;

// Re-exports
pub use buffer::InputBuffer;
pub use types::{Action, CharInfo, Config, WordForm};

// Plugin system exports
pub use traits::{LanguagePlugin, InputMethodTrait, LookupProvider, LanguageRules};
pub use registry::{PluginRegistry, RegistryError};
pub use engine::Engine;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer() {
        let mut buffer = InputBuffer::new();
        buffer.push('a', true);
        assert_eq!(buffer.len(), 1);
    }
    
    #[test]
    fn test_engine() {
        let engine = Engine::new();
        assert!(engine.buffer_content().is_empty());
    }
}
