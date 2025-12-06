// methods/vni.rs - VNI input method

use vikey_core::traits::InputMethodTrait;
use vikey_core::traits::LookupProvider;
use vikey_core::types::Action;
use vikey_core::InputBuffer;

/// VNI Input Method
pub struct VNIMethod {}

impl VNIMethod {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for VNIMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl InputMethodTrait for VNIMethod {
    fn name(&self) -> &str {
        "VNI"
    }

    fn id(&self) -> &str {
        "vni"
    }

    fn process(
        &mut self,
        key: char,
        buffer: &mut InputBuffer,
        _lookup: &dyn LookupProvider,
    ) -> Action {
        // TODO: Implement VNI logic
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

    fn reset(&mut self) {}
    fn can_undo(&self, _buffer: &InputBuffer) -> bool {
        false
    }
    fn undo(&mut self, _buffer: &mut InputBuffer) -> Action {
        Action::DoNothing
    }
}
