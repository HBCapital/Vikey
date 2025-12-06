// lib.rs - Vietnamese Language Support for Vikey

pub mod lookup;
pub mod methods;
pub mod plugin;
pub mod rules;
pub mod syllable;
pub mod types;
pub mod validation;

// Re-exports
pub use lookup::VietnameseLookup;
pub use plugin::VietnamesePlugin;
pub use syllable::{Modification, Syllable, Tone};
pub use types::{MarkType, ToneType, TransformEffect, Transformation, WordForm};
