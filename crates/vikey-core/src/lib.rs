//! Vikey Core - Vietnamese Input Method Engine
//!
//! This is a pure Rust library for processing Vietnamese input.
//! It can be embedded in any application (desktop, web, mobile, games).
//!
//! # Example
//!
//! ```
//! use vikey_core::{VikeyCore, Config, InputMethod, Action};
//!
//! let mut core = VikeyCore::new(Config::default());
//!
//! // Process 'a' then 'a' -> should get 'â'
//! let action1 = core.process_key('a');
//! let action2 = core.process_key('a');
//!
//! // action2 should be Replace { backspace_count: 1, text: "â" }
//! ```

pub mod types;
mod buffer;
mod lookup;
mod spelling;
mod processor;

// NEW: Plugin system modules
pub mod traits;
pub mod registry;
pub mod engine;

pub use types::{
    Action, CharInfo, Config, InputMethod,
    // NEW: Enhanced types from Phase 2
    WordForm, Transformation, TransformEffect, ToneType, MarkType,
};
pub use buffer::InputBuffer;
pub use lookup::LookupTable;
pub use spelling::SpellChecker;
pub use processor::Processor;

// NEW: Plugin system exports
pub use traits::{LanguagePlugin, InputMethodTrait, LookupProvider, LanguageRules};
pub use registry::{PluginRegistry, RegistryError};
pub use engine::Engine;

/// Main Vikey Core processor
pub struct VikeyCore {
    config: Config,
    processor: Processor,
}

impl VikeyCore {
    /// Create a new Vikey Core instance with the given configuration
    pub fn new(config: Config) -> Self {
        let processor = Processor::new(config.input_method);
        
        Self {
            config,
            processor,
        }
    }

    /// Process a single key press
    ///
    /// Returns an Action indicating what should be done (replace text, commit, or do nothing)
    pub fn process_key(&mut self, key: char) -> Action {
        self.processor.process(key, &self.config)
    }

    /// Process backspace key
    pub fn process_backspace(&mut self) -> Action {
        self.processor.process_backspace()
    }

    /// Reset the buffer (e.g., when switching applications)
    pub fn reset(&mut self) {
        self.processor.reset();
    }

    /// Get current buffer content (for debugging)
    pub fn buffer_content(&self) -> String {
        self.processor.buffer_content()
    }
    
    /// Get transformation history (for debugging/undo)
    pub fn transformations(&self) -> &[Transformation] {
        self.processor.transformations()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_core() {
        let core = VikeyCore::new(Config::default());
        assert_eq!(core.buffer_content(), "");
    }

    #[test]
    fn test_reset() {
        let mut core = VikeyCore::new(Config::default());
        core.reset();
        assert_eq!(core.buffer_content(), "");
    }
}
