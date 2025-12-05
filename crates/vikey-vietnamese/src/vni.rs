//! VNI input method (placeholder)

use vikey_core::{TransformResult, Transformer};

pub struct VNITransformer;

impl VNITransformer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for VNITransformer {
    fn default() -> Self {
        Self::new()
    }
}

impl Transformer for VNITransformer {
    fn transform(&self, _input: &str) -> Option<TransformResult> {
        // TODO: Implement VNI transformation
        None
    }
    
    fn name(&self) -> &str {
        "vni"
    }
}
