//! Vikey Nôm - Lookup Table
//!
//! Bảng tra cứu cho ký tự Nôm.

use vikey_core::traits::LookupProvider;
use vikey_core::types::CharInfo;
use crate::types::{NomCharInfo, UnicodeBlock};

/// Lookup provider cho chữ Nôm
pub struct NomLookup {
    // TODO: Load from data file
}

impl NomLookup {
    /// Tạo lookup table mới
    pub fn new() -> Self {
        Self {}
    }
    
    /// Kiểm tra ký tự có phải CJK không
    fn is_cjk(c: char) -> bool {
        let cp = c as u32;
        matches!(cp,
            0x4E00..=0x9FFF |      // CJK Unified
            0x3400..=0x4DBF |      // Extension A
            0x20000..=0x2A6DF |    // Extension B
            0x2A700..=0x2B73F |    // Extension C
            0x2B740..=0x2B81F |    // Extension D
            0x2B820..=0x2CEAF |    // Extension E
            0x2CEB0..=0x2EBEF |    // Extension F
            0x30000..=0x3134F      // Extension G
        )
    }
    
    /// Lấy thông tin Nôm-specific cho ký tự
    pub fn lookup_nom(&self, c: char) -> NomCharInfo {
        let cp = c as u32;
        NomCharInfo {
            is_nom: Self::is_cjk(c),
            code_point: cp,
            unicode_block: UnicodeBlock::from_code_point(cp),
        }
    }
}

impl Default for NomLookup {
    fn default() -> Self {
        Self::new()
    }
}

impl LookupProvider for NomLookup {
    fn lookup(&self, c: char) -> CharInfo {
        // Return generic CharInfo for core compatibility
        CharInfo {
            is_separator: c.is_whitespace(),
            ..CharInfo::default()
        }
    }
    
    fn is_valid_char(&self, c: char) -> bool {
        // Accept Latin (for typing) and CJK (for output)
        c.is_ascii_alphabetic() || Self::is_cjk(c)
    }
    
    fn is_vowel(&self, c: char) -> bool {
        // Latin vowels for typing phiên âm
        matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u' | 'y')
    }
    
    fn is_consonant(&self, c: char) -> bool {
        c.is_ascii_alphabetic() && !self.is_vowel(c)
    }
    
    fn is_separator(&self, c: char) -> bool {
        c.is_whitespace() || matches!(c, ',' | '.' | ';' | ':' | '!' | '?')
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_cjk() {
        // 𡨸 (U+21A38) - Extension B
        assert!(NomLookup::is_cjk('𡨸'));
        
        // 喃 (U+5583) - CJK Unified
        assert!(NomLookup::is_cjk('喃'));
        
        // Latin
        assert!(!NomLookup::is_cjk('a'));
    }
    
    #[test]
    fn test_lookup() {
        let lookup = NomLookup::new();
        
        // Latin vowel
        assert!(lookup.is_vowel('a'));
        assert!(!lookup.is_consonant('a'));
        
        // Latin consonant
        assert!(!lookup.is_vowel('b'));
        assert!(lookup.is_consonant('b'));
    }
}
