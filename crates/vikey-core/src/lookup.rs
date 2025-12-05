// lookup.rs - DT Lookup Table implementation (based on UniKey)

use crate::types::{CharInfo, InputMethod};

// Bit masks for DT table entries (32-bit)
const VOWEL_INDEX_MASK: u32 = 0x1F;           // Bits 0-4
const MACRO_INDEX_MASK: u32 = 0xF << 5;       // Bits 5-8
const DOUBLE_CHAR_MASK: u32 = 0x1F << 9;      // Bits 9-13
const TONE_INDEX_MASK: u32 = 0xF << 14;       // Bits 14-17
const CURRENT_TONE_MASK: u32 = 0xF << 18;     // Bits 18-21
const IS_BREVE_FLAG: u32 = 1 << 22;           // Bit 22
const SOFT_SEP_FLAG: u32 = 1 << 24;           // Bit 24
const SEPARATOR_FLAG: u32 = 1 << 25;          // Bit 25
const VNI_DOUBLE_MASK: u32 = 0x7 << 26;       // Bits 26-28

/// Lookup table for fast character classification
/// Uses bit-packed 32-bit entries for O(1) lookup
pub struct LookupTable {
    /// DT table - 256 entries, one for each ASCII character
    dt: [u32; 256],
}

impl LookupTable {
    /// Create a new lookup table for the given input method
    pub fn new(method: InputMethod) -> Self {
        let mut dt = [0u32; 256];
        
        match method {
            InputMethod::Telex => Self::build_telex(&mut dt),
            InputMethod::VNI => Self::build_vni(&mut dt),
            InputMethod::VIQR => Self::build_viqr(&mut dt),
        }
        
        Self { dt }
    }

    /// Get character information from the lookup table
    #[inline]
    pub fn get_info(&self, ch: u8) -> CharInfo {
        let entry = self.dt[ch as usize];
        CharInfo {
            vowel_index: (entry & VOWEL_INDEX_MASK) as u8,
            macro_index: ((entry & MACRO_INDEX_MASK) >> 5) as u8,
            double_char_index: ((entry & DOUBLE_CHAR_MASK) >> 9) as u8,
            tone_index: ((entry & TONE_INDEX_MASK) >> 14) as u8,
            current_tone: ((entry & CURRENT_TONE_MASK) >> 18) as u8,
            is_breve: (entry & IS_BREVE_FLAG) != 0,
            is_separator: (entry & SEPARATOR_FLAG) != 0,
            is_soft_separator: (entry & SOFT_SEP_FLAG) != 0,
            vni_double_index: ((entry & VNI_DOUBLE_MASK) >> 26) as u8,
            // NEW: Phase 2 fields - initialized to defaults
            word_form: crate::types::WordForm::Empty,
            c1_offset: None,
            v_offset: None,
            c2_offset: None,
        }
    }

    /// Build Telex lookup table
    fn build_telex(dt: &mut [u32; 256]) {
        // Vowels (vowel_index 1-6)
        dt[b'a' as usize] = 1;
        dt[b'A' as usize] = 1;
        dt[b'e' as usize] = 2;
        dt[b'E' as usize] = 2;
        dt[b'i' as usize] = 3;
        dt[b'I' as usize] = 3;
        dt[b'o' as usize] = 4;
        dt[b'O' as usize] = 4;
        dt[b'u' as usize] = 5;
        dt[b'U' as usize] = 5;
        dt[b'y' as usize] = 6;
        dt[b'Y' as usize] = 6;

        // Breve marks (w, W)
        dt[b'w' as usize] = IS_BREVE_FLAG;
        dt[b'W' as usize] = IS_BREVE_FLAG;

        // Tone marks (tone_index 1-5)
        dt[b's' as usize] = 1 << 14;  // Sắc
        dt[b'S' as usize] = 1 << 14;
        dt[b'f' as usize] = 2 << 14;  // Huyền
        dt[b'F' as usize] = 2 << 14;
        dt[b'r' as usize] = 3 << 14;  // Hỏi
        dt[b'R' as usize] = 3 << 14;
        dt[b'x' as usize] = 4 << 14;  // Ngã
        dt[b'X' as usize] = 4 << 14;
        dt[b'j' as usize] = 5 << 14;  // Nặng
        dt[b'J' as usize] = 5 << 14;

        // Separators
        dt[b' ' as usize] = SEPARATOR_FLAG;
        dt[b'\n' as usize] = SEPARATOR_FLAG;
        dt[b'\t' as usize] = SEPARATOR_FLAG;
        dt[b'\r' as usize] = SEPARATOR_FLAG;

        // Soft separators (punctuation)
        dt[b',' as usize] = SOFT_SEP_FLAG;
        dt[b'.' as usize] = SOFT_SEP_FLAG;
        dt[b';' as usize] = SOFT_SEP_FLAG;
        dt[b':' as usize] = SOFT_SEP_FLAG;
        dt[b'!' as usize] = SOFT_SEP_FLAG;
        dt[b'?' as usize] = SOFT_SEP_FLAG;
        dt[b'-' as usize] = SOFT_SEP_FLAG;
        dt[b'_' as usize] = SOFT_SEP_FLAG;
        dt[b'(' as usize] = SOFT_SEP_FLAG;
        dt[b')' as usize] = SOFT_SEP_FLAG;
        dt[b'[' as usize] = SOFT_SEP_FLAG;
        dt[b']' as usize] = SOFT_SEP_FLAG;
        dt[b'{' as usize] = SOFT_SEP_FLAG;
        dt[b'}' as usize] = SOFT_SEP_FLAG;
        dt[b'"' as usize] = SOFT_SEP_FLAG;
        dt[b'\'' as usize] = SOFT_SEP_FLAG;
    }

    /// Build VNI lookup table
    fn build_vni(dt: &mut [u32; 256]) {
        // Vowels (same as Telex)
        dt[b'a' as usize] = 1;
        dt[b'A' as usize] = 1;
        dt[b'e' as usize] = 2;
        dt[b'E' as usize] = 2;
        dt[b'i' as usize] = 3;
        dt[b'I' as usize] = 3;
        dt[b'o' as usize] = 4;
        dt[b'O' as usize] = 4;
        dt[b'u' as usize] = 5;
        dt[b'U' as usize] = 5;
        dt[b'y' as usize] = 6;
        dt[b'Y' as usize] = 6;

        // VNI tone marks (1-5)
        dt[b'1' as usize] = 1 << 14;  // Sắc
        dt[b'2' as usize] = 2 << 14;  // Huyền
        dt[b'3' as usize] = 3 << 14;  // Hỏi
        dt[b'4' as usize] = 4 << 14;  // Ngã
        dt[b'5' as usize] = 5 << 14;  // Nặng

        // VNI double marks (6, 7, 8, 9)
        dt[b'6' as usize] = 1 << 26;  // ă
        dt[b'7' as usize] = 2 << 26;  // â
        dt[b'8' as usize] = 3 << 26;  // ơ
        dt[b'9' as usize] = 4 << 26;  // đ

        // Separators (same as Telex)
        dt[b' ' as usize] = SEPARATOR_FLAG;
        dt[b'\n' as usize] = SEPARATOR_FLAG;
        dt[b'\t' as usize] = SEPARATOR_FLAG;
        dt[b'\r' as usize] = SEPARATOR_FLAG;

        // Soft separators (same as Telex)
        dt[b',' as usize] = SOFT_SEP_FLAG;
        dt[b'.' as usize] = SOFT_SEP_FLAG;
        dt[b';' as usize] = SOFT_SEP_FLAG;
        dt[b':' as usize] = SOFT_SEP_FLAG;
        dt[b'!' as usize] = SOFT_SEP_FLAG;
        dt[b'?' as usize] = SOFT_SEP_FLAG;
    }

    /// Build VIQR lookup table (placeholder)
    fn build_viqr(_dt: &mut [u32; 256]) {
        // TODO: Implement VIQR table in Phase 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telex_vowels() {
        let table = LookupTable::new(InputMethod::Telex);
        
        let info_a = table.get_info(b'a');
        assert_eq!(info_a.vowel_index, 1);
        assert!(!info_a.is_breve);
        assert!(!info_a.is_separator);
        
        let info_e = table.get_info(b'e');
        assert_eq!(info_e.vowel_index, 2);
    }

    #[test]
    fn test_telex_breve() {
        let table = LookupTable::new(InputMethod::Telex);
        
        let info_w = table.get_info(b'w');
        assert!(info_w.is_breve);
        assert_eq!(info_w.vowel_index, 0);
    }

    #[test]
    fn test_telex_tone_marks() {
        let table = LookupTable::new(InputMethod::Telex);
        
        let info_s = table.get_info(b's');
        assert_eq!(info_s.tone_index, 1);  // Sắc
        
        let info_f = table.get_info(b'f');
        assert_eq!(info_f.tone_index, 2);  // Huyền
        
        let info_j = table.get_info(b'j');
        assert_eq!(info_j.tone_index, 5);  // Nặng
    }

    #[test]
    fn test_separators() {
        let table = LookupTable::new(InputMethod::Telex);
        
        let info_space = table.get_info(b' ');
        assert!(info_space.is_separator);
        assert!(!info_space.is_soft_separator);
        
        let info_comma = table.get_info(b',');
        assert!(!info_comma.is_separator);
        assert!(info_comma.is_soft_separator);
    }

    #[test]
    fn test_vni_numbers() {
        let table = LookupTable::new(InputMethod::VNI);
        
        let info_1 = table.get_info(b'1');
        assert_eq!(info_1.tone_index, 1);  // Sắc
        
        let info_6 = table.get_info(b'6');
        assert_eq!(info_6.vni_double_index, 1);  // ă
        
        let info_9 = table.get_info(b'9');
        assert_eq!(info_9.vni_double_index, 4);  // đ
    }

    #[test]
    fn test_non_special_chars() {
        let table = LookupTable::new(InputMethod::Telex);
        
        let info_b = table.get_info(b'b');
        assert_eq!(info_b.vowel_index, 0);
        assert_eq!(info_b.tone_index, 0);
        assert!(!info_b.is_breve);
        assert!(!info_b.is_separator);
    }
}
