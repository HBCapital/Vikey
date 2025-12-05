//! Vietnamese language processing for Vikey
//!
//! This crate provides Vietnamese-specific logic including:
//! - Telex input method
//! - VNI input method
//! - VIQR input method
//! - Vietnamese rules and validation
//! - Dictionary support

pub mod telex;
pub mod vni;
pub mod viqr;
pub mod rules;
pub mod unicode;

// Re-exports
pub use telex::TelexTransformer;
pub use vni::VNITransformer;
pub use viqr::VIQRTransformer;
pub use rules::VietnameseRules;

/// Input method type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethod {
    Telex,
    VNI,
    VIQR,
}
