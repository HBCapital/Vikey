//! VIQR input method (placeholder)

use vikey_core::{TransformResult, Transformer};

pub struct VIQRTransformer;

impl VIQRTransformer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for VIQRTransformer {
    fn default() -> Self {
        Self::new()
    }
}

impl Transformer for VIQRTransformer {
    fn transform(&self, _input: &str) -> Option<TransformResult> {
        // TODO: Implement VIQR transformation
        None
    }
    
    fn name(&self) -> &str {
        "viqr"
    }
}
