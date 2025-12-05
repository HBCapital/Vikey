//! Telex input method implementation

use lazy_static::lazy_static;
use std::collections::HashMap;
use vikey_core::{TransformResult, Transformer};

lazy_static! {
    /// Vowel transformations for Telex
    static ref VOWEL_MAP: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        m.insert("aa", 'â');
        m.insert("aw", 'ă');
        m.insert("ee", 'ê');
        m.insert("oo", 'ô');
        m.insert("ow", 'ơ');
        m.insert("w", 'ư');
        m.insert("dd", 'đ');
        m
    };
    
    /// Tone mark transformations
    static ref TONE_MAP: HashMap<char, char> = {
        let mut m = HashMap::new();
        m.insert('s', '\u{0301}'); // sắc
        m.insert('f', '\u{0300}'); // huyền
        m.insert('r', '\u{0309}'); // hỏi
        m.insert('x', '\u{0303}'); // ngã
        m.insert('j', '\u{0323}'); // nặng
        m
    };
}

/// Telex transformer
pub struct TelexTransformer;

impl TelexTransformer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TelexTransformer {
    fn default() -> Self {
        Self::new()
    }
}

impl Transformer for TelexTransformer {
    fn transform(&self, input: &str) -> Option<TransformResult> {
        // Try vowel transformations first
        for (pattern, replacement) in VOWEL_MAP.iter() {
            if input.ends_with(pattern) {
                let mut output = input[..input.len() - pattern.len()].to_string();
                output.push(*replacement);
                
                return Some(TransformResult {
                    output,
                    consumed: pattern.len(),
                });
            }
        }
        
        // Try tone marks
        if input.len() >= 2 {
            let last_char = input.chars().last()?;
            if let Some(tone) = TONE_MAP.get(&last_char) {
                // Find vowel to apply tone
                let chars: Vec<char> = input.chars().collect();
                for i in (0..chars.len() - 1).rev() {
                    if is_vowel(chars[i]) {
                        let mut output = String::new();
                        for (j, &ch) in chars.iter().enumerate() {
                            if j == i {
                                output.push(ch);
                                output.push(*tone);
                            } else if j != chars.len() - 1 {
                                output.push(ch);
                            }
                        }
                        
                        return Some(TransformResult {
                            output: normalize_unicode(&output),
                            consumed: input.len(),
                        });
                    }
                }
            }
        }
        
        None
    }
    
    fn name(&self) -> &str {
        "telex"
    }
}

fn is_vowel(ch: char) -> bool {
    matches!(
        ch.to_lowercase().next().unwrap(),
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'â' | 'ă' | 'ê' | 'ô' | 'ơ' | 'ư'
    )
}

fn normalize_unicode(s: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    s.nfc().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telex_vowels() {
        let transformer = TelexTransformer::new();
        
        let result = transformer.transform("aa");
        assert!(result.is_some());
        assert_eq!(result.unwrap().output, "â");
        
        let result = transformer.transform("aw");
        assert!(result.is_some());
        assert_eq!(result.unwrap().output, "ă");
    }
    
    #[test]
    fn test_telex_dd() {
        let transformer = TelexTransformer::new();
        
        let result = transformer.transform("dd");
        assert!(result.is_some());
        assert_eq!(result.unwrap().output, "đ");
    }
}
