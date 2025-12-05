// methods/telex.rs - Telex input method

use vikey_core::traits::InputMethodTrait;
use vikey_core::types::Action;
use vikey_core::InputBuffer;
use vikey_core::traits::LookupProvider;

/// Telex Input Method
pub struct TelexMethod {
    // Internal state if needed
}

impl TelexMethod {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for TelexMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl InputMethodTrait for TelexMethod {
    fn name(&self) -> &str {
        "Telex"
    }
    
    fn id(&self) -> &str {
        "telex"
    }
    
    fn process(
        &mut self,
        key: char,
        buffer: &mut InputBuffer,
        _lookup: &dyn LookupProvider
    ) -> Action {
        // TODO: Implement full Telex logic
        // For now, just append the character
        buffer.push(key, key.is_lowercase());
        Action::Commit(key.to_string())
    }
    
    fn process_backspace(&mut self, buffer: &mut InputBuffer) -> Action {
        if buffer.pop().is_some() {
            Action::Replace {
                backspace_count: 1,
                text: String::new(),
            }
        } else {
            Action::DoNothing
        }
    }
    
    fn reset(&mut self) {
        // Reset internal state if any
    }
    
    fn can_undo(&self, _buffer: &InputBuffer) -> bool {
        false
    }
    
    fn undo(&mut self, _buffer: &mut InputBuffer) -> Action {
        Action::DoNothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_telex_info() {
        let method = TelexMethod::new();
        assert_eq!(method.id(), "telex");
        assert_eq!(method.name(), "Telex");
    }
}
