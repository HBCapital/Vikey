// lib.rs - Vietnamese Language Support for Vikey

pub mod types;
pub mod lookup;
pub mod plugin;
pub mod methods;
pub mod syllable;
pub mod rules;

// Re-exports
pub use plugin::VietnamesePlugin;
pub use lookup::VietnameseLookup;
pub use types::{ToneType, MarkType, WordForm, TransformEffect, Transformation};
pub use syllable::{Syllable, Tone, Modification};
