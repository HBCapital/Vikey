// lib.rs - Vietnamese Language Support for Vikey

pub mod types;
pub mod lookup;
pub mod plugin;
pub mod methods;

// Re-exports
pub use plugin::VietnamesePlugin;
pub use lookup::VietnameseLookup;
pub use types::{ToneType, MarkType, WordForm, TransformEffect, Transformation};
