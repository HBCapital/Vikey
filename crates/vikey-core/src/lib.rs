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
//! // Process 'a' then 'a' -> should suggest 'â'
//! let action1 = core.process_key('a');
//! let action2 = core.process_key('a');
//!
//! // action2 should be Replace { backspace_count: 1, text: "â" }
//! ```

mod types;
mod buffer;
mod lookup;
mod spelling;

pub use types::{
    Action, CharInfo, Config, InputMethod,
    // NEW: Enhanced types from Phase 2
    WordForm, Transformation, TransformEffect, ToneType, MarkType,
};
pub use buffer::InputBuffer;
pub use lookup::LookupTable;
pub use spelling::SpellChecker;

/// Main Vikey Core processor
pub struct VikeyCore {
    config: Config,
    lookup: LookupTable,
    buffer: InputBuffer,
}

impl VikeyCore {
    /// Create a new Vikey Core instance with the given configuration
    pub fn new(config: Config) -> Self {
        let lookup = LookupTable::new(config.input_method);
        let buffer = InputBuffer::new();
        
        Self {
            config,
            lookup,
            buffer,
        }
    }

    /// Process a single key press
    ///
    /// Returns an Action indicating what should be done (replace text, commit, or do nothing)
    pub fn process_key(&mut self, key: char) -> Action {
        // For Phase 1, just return DoNothing
        // Full implementation will come in Phase 2
        Action::DoNothing
    }

    /// Process backspace key
    pub fn process_backspace(&mut self) -> Action {
        if let Some((ch, _)) = self.buffer.pop() {
            Action::Replace {
                backspace_count: 1,
                text: String::new(),
            }
        } else {
            Action::DoNothing
        }
    }

    /// Reset the buffer (e.g., when switching applications)
    pub fn reset(&mut self) {
        self.buffer.clear();
    }

    /// Get current buffer content (for debugging)
    pub fn buffer_content(&self) -> String {
        self.buffer.to_string()
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
