// lookup.rs - Vietnamese character lookup implementation

use std::collections::HashMap;
use vikey_core::traits::LookupProvider;

/// Vietnamese character information
#[derive(Debug, Clone, Copy)]
pub struct VietCharInfo {
    pub is_vowel: bool,
    pub is_consonant: bool,
    pub is_separator: bool,
    pub is_soft_separator: bool,
    pub vowel_index: u8,      // 0=none, 1=a, 2=e, 3=i, 4=o, 5=u, 6=y
    pub tone_index: u8,       // 0=none, 1-5=tones
    pub is_breve: bool,       // w in Telex
    pub vni_double_index: u8, // VNI: 6,7,8,9
}

impl Default for VietCharInfo {
    fn default() -> Self {
        Self {
            is_vowel: false,
            is_consonant: false,
            is_separator: false,
            is_soft_separator: false,
            vowel_index: 0,
            tone_index: 0,
            is_breve: false,
            vni_double_index: 0,
        }
    }
}

/// Vietnamese lookup provider for Telex input method
pub struct VietnameseLookup {
    /// Fast lookup table for ASCII characters
    ascii_table: [VietCharInfo; 128],
    /// Extended lookup for Unicode Vietnamese characters
    unicode_map: HashMap<char, VietCharInfo>,
}

impl VietnameseLookup {
    pub fn new_telex() -> Self {
        let mut ascii_table = [VietCharInfo::default(); 128];

        // Vowels
        for &(ch, idx) in &[('a', 1), ('e', 2), ('i', 3), ('o', 4), ('u', 5), ('y', 6)] {
            ascii_table[ch as usize].is_vowel = true;
            ascii_table[ch as usize].vowel_index = idx;
            ascii_table[ch.to_ascii_uppercase() as usize].is_vowel = true;
            ascii_table[ch.to_ascii_uppercase() as usize].vowel_index = idx;
        }

        // Breve mark (w, W)
        ascii_table[b'w' as usize].is_breve = true;
        ascii_table[b'W' as usize].is_breve = true;

        // Tone marks
        for &(ch, idx) in &[('s', 1), ('f', 2), ('r', 3), ('x', 4), ('j', 5)] {
            ascii_table[ch as usize].tone_index = idx;
            ascii_table[ch.to_ascii_uppercase() as usize].tone_index = idx;
        }

        // Separators
        for &ch in &[' ', '\n', '\t', '\r'] {
            ascii_table[ch as usize].is_separator = true;
        }

        // Soft separators
        for &ch in &[
            ',', '.', ';', ':', '!', '?', '-', '_', '(', ')', '[', ']', '{', '}', '"', '\'',
        ] {
            ascii_table[ch as usize].is_soft_separator = true;
        }

        // Consonants (all other letters)
        for ch in b'a'..=b'z' {
            if !ascii_table[ch as usize].is_vowel {
                ascii_table[ch as usize].is_consonant = true;
                ascii_table[ch.to_ascii_uppercase() as usize].is_consonant = true;
            }
        }

        Self {
            ascii_table,
            unicode_map: HashMap::new(),
        }
    }

    pub fn new_vni() -> Self {
        let mut lookup = Self::new_telex();

        // VNI uses numbers for tones
        lookup.ascii_table[b'1' as usize].tone_index = 1; // Sắc
        lookup.ascii_table[b'2' as usize].tone_index = 2; // Huyền
        lookup.ascii_table[b'3' as usize].tone_index = 3; // Hỏi
        lookup.ascii_table[b'4' as usize].tone_index = 4; // Ngã
        lookup.ascii_table[b'5' as usize].tone_index = 5; // Nặng

        // VNI double marks
        lookup.ascii_table[b'6' as usize].vni_double_index = 1; // ă
        lookup.ascii_table[b'7' as usize].vni_double_index = 2; // â
        lookup.ascii_table[b'8' as usize].vni_double_index = 3; // ơ
        lookup.ascii_table[b'9' as usize].vni_double_index = 4; // đ

        // Clear Telex tone marks
        for &ch in &['s', 'f', 'r', 'x', 'j', 'S', 'F', 'R', 'X', 'J'] {
            lookup.ascii_table[ch as usize].tone_index = 0;
        }

        lookup
    }

    pub fn get_char_info(&self, c: char) -> VietCharInfo {
        if c.is_ascii() {
            self.ascii_table[c as usize]
        } else {
            self.unicode_map.get(&c).copied().unwrap_or_default()
        }
    }
}

impl LookupProvider for VietnameseLookup {
    fn lookup(&self, c: char) -> vikey_core::types::CharInfo {
        let viet_info = self.get_char_info(c);

        // Convert VietCharInfo to vikey_core::types::CharInfo
        vikey_core::types::CharInfo {
            vowel_index: viet_info.vowel_index,
            macro_index: 0,
            double_char_index: 0,
            tone_index: viet_info.tone_index,
            current_tone: 0,
            is_breve: viet_info.is_breve,
            is_separator: viet_info.is_separator,
            is_soft_separator: viet_info.is_soft_separator,
            vni_double_index: viet_info.vni_double_index,
            word_form: vikey_core::types::WordForm::Empty,
            c1_offset: None,
            v_offset: None,
            c2_offset: None,
        }
    }

    fn is_valid_char(&self, c: char) -> bool {
        let info = self.get_char_info(c);
        info.is_vowel || info.is_consonant || c.is_ascii_alphabetic()
    }

    fn is_vowel(&self, c: char) -> bool {
        self.get_char_info(c).is_vowel
    }

    fn is_consonant(&self, c: char) -> bool {
        self.get_char_info(c).is_consonant
    }

    fn is_separator(&self, c: char) -> bool {
        let info = self.get_char_info(c);
        info.is_separator || info.is_soft_separator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telex_vowels() {
        let lookup = VietnameseLookup::new_telex();
        assert!(lookup.is_vowel('a'));
        assert!(lookup.is_vowel('e'));
        assert!(!lookup.is_vowel('b'));
    }

    #[test]
    fn test_telex_tones() {
        let lookup = VietnameseLookup::new_telex();
        let info_s = lookup.get_char_info('s');
        assert_eq!(info_s.tone_index, 1);
    }

    #[test]
    fn test_vni_numbers() {
        let lookup = VietnameseLookup::new_vni();
        let info_1 = lookup.get_char_info('1');
        assert_eq!(info_1.tone_index, 1);

        let info_6 = lookup.get_char_info('6');
        assert_eq!(info_6.vni_double_index, 1);
    }
}
