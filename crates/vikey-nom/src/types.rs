//! Vikey Nôm - Types
//!
//! Các kiểu dữ liệu đặc thù cho chữ Nôm.

/// Thông tin về một ký tự Nôm candidate
#[derive(Debug, Clone)]
pub struct NomCandidate {
    /// Ký tự Nôm (Unicode)
    pub character: char,

    /// Phiên âm Quốc ngữ
    pub quoc_ngu: String,

    /// Phiên âm Pinyin (nếu có)
    pub pinyin: Option<String>,

    /// Nghĩa tiếng Việt hiện đại
    pub meaning: Option<String>,

    /// Tần suất sử dụng (0-100)
    pub frequency: u8,

    /// Phân loại (tự tạo, mượn Hán, ...)
    pub category: NomCategory,
}

/// Phân loại ký tự Nôm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NomCategory {
    /// Chữ Nôm tự tạo (phần lớn)
    Native,

    /// Mượn từ chữ Hán
    SinoVietnamese,

    /// Chữ Hán dùng nguyên
    PureHan,
}

/// Thông tin tra cứu cho ký tự Nôm
#[derive(Debug, Clone, Copy, Default)]
pub struct NomCharInfo {
    /// Ký tự có phải Nôm valid không
    pub is_nom: bool,

    /// Unicode code point
    pub code_point: u32,

    /// Block trong Unicode (Extension B, C, D, ...)
    pub unicode_block: UnicodeBlock,
}

/// Unicode block cho CJK characters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UnicodeBlock {
    /// CJK Unified Ideographs (U+4E00–U+9FFF)
    #[default]
    CjkUnified,

    /// Extension A (U+3400–U+4DBF)
    ExtensionA,

    /// Extension B (U+20000–U+2A6DF) - Chứa nhiều Nôm
    ExtensionB,

    /// Extension C (U+2A700–U+2B73F)
    ExtensionC,

    /// Extension D (U+2B740–U+2B81F)
    ExtensionD,

    /// Other
    Other,
}

impl UnicodeBlock {
    /// Xác định block từ code point
    pub fn from_code_point(cp: u32) -> Self {
        match cp {
            0x4E00..=0x9FFF => Self::CjkUnified,
            0x3400..=0x4DBF => Self::ExtensionA,
            0x20000..=0x2A6DF => Self::ExtensionB,
            0x2A700..=0x2B73F => Self::ExtensionC,
            0x2B740..=0x2B81F => Self::ExtensionD,
            _ => Self::Other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_block() {
        // 𡨸 = U+21A38 (Extension B)
        assert_eq!(
            UnicodeBlock::from_code_point(0x21A38),
            UnicodeBlock::ExtensionB
        );

        // 喃 = U+5583 (CJK Unified)
        assert_eq!(
            UnicodeBlock::from_code_point(0x5583),
            UnicodeBlock::CjkUnified
        );
    }
}
