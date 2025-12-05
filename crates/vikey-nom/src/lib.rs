//! Vikey Nôm - Chữ Nôm (Han-Nom) Language Plugin
//!
//! Crate này cung cấp hỗ trợ nhập chữ Nôm (𡨸喃) - hệ thống chữ viết
//! của người Việt trước khi dùng chữ Latinh.
//!
//! # Các Input Methods
//!
//! - **Telex-Nôm**: Gõ phiên âm Quốc ngữ → Candidate list chữ Nôm
//! - **Pinyin-Nôm**: Gõ Pinyin kiểu Trung Quốc → Chữ Nôm
//!
//! # Ví dụ
//!
//! ```ignore
//! use vikey_nom::NomPlugin;
//! use vikey_core::Engine;
//!
//! let mut engine = Engine::new();
//! engine.register(Box::new(NomPlugin::new()));
//! engine.set_language("nom");
//! engine.set_input_method("telex-nom");
//!
//! // Gõ "nguoi" → Hiển thị candidate list: 𡦂, 𠊛, ...
//! ```

pub mod plugin;
pub mod lookup;
pub mod dictionary;
pub mod methods;
pub mod types;

pub use plugin::NomPlugin;
pub use types::{NomCandidate, NomCharInfo};
