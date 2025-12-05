//! Transformation trait and types

use crate::Result;

/// Result of a transformation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransformResult {
    /// Transformed output
    pub output: String,
    
    /// Number of input characters consumed
    pub consumed: usize,
}

/// Trait for transforming input text
pub trait Transformer: Send + Sync {
    /// Transform input string
    ///
    /// Returns Some(result) if transformation was applied,
    /// None if no transformation needed
    fn transform(&self, input: &str) -> Option<TransformResult>;
    
    /// Get transformer name
    fn name(&self) -> &str;
}

/// Transform engine that manages transformers
pub struct Transform {
    transformers: Vec<Box<dyn Transformer>>,
}

impl Transform {
    /// Create new transform engine
    pub fn new() -> Self {
        Self {
            transformers: Vec::new(),
        }
    }
    
    /// Add transformer
    pub fn add_transformer(&mut self, transformer: Box<dyn Transformer>) {
        self.transformers.push(transformer);
    }
    
    /// Apply transformations
    pub fn apply(&self, input: &str) -> Option<TransformResult> {
        for transformer in &self.transformers {
            if let Some(result) = transformer.transform(input) {
                return Some(result);
            }
        }
        None
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestTransformer;
    
    impl Transformer for TestTransformer {
        fn transform(&self, input: &str) -> Option<TransformResult> {
            if input == "aa" {
                Some(TransformResult {
                    output: "â".to_string(),
                    consumed: 2,
                })
            } else {
                None
            }
        }
        
        fn name(&self) -> &str {
            "test"
        }
    }

    #[test]
    fn test_transformer() {
        let transformer = TestTransformer;
        let result = transformer.transform("aa");
        assert!(result.is_some());
        assert_eq!(result.unwrap().output, "â");
    }
    
    #[test]
    fn test_transform_engine() {
        let mut transform = Transform::new();
        transform.add_transformer(Box::new(TestTransformer));
        
        let result = transform.apply("aa");
        assert!(result.is_some());
        assert_eq!(result.unwrap().output, "â");
        
        let result = transform.apply("bb");
        assert!(result.is_none());
    }
}
