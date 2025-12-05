// syllable.rs - Vietnamese syllable structure (inspired by vi-rs)

use std::fmt;

/// Vietnamese tone marks
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tone {
    /// Dấu sắc (acute) - rising tone
    Acute,
    /// Dấu huyền (grave) - falling tone  
    Grave,
    /// Dấu hỏi (hook above) - dipping tone
    HookAbove,
    /// Dấu ngã (tilde) - creaky rising tone
    Tilde,
    /// Dấu nặng (dot below) - creaky falling tone
    Underdot,
}

/// Letter modifications for Vietnamese
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modification {
    /// Circumflex (^): a→â, e→ê, o→ô
    Circumflex,
    /// Breve (˘): a→ă
    Breve,
    /// Horn: o→ơ, u→ư
    Horn,
    /// D-stroke: d→đ
    DStroke,
}

/// Vietnamese syllable structure
/// 
/// A Vietnamese syllable consists of:
/// - initial consonant (optional): ch, tr, ng, etc.
/// - vowel (required): a, uo, ie, etc.
/// - final consonant (optional): ng, nh, t, etc.
/// - tone mark (optional): sắc, huyền, etc.
/// - letter modifications (optional): circumflex, breve, horn
#[derive(Debug, Clone, Default)]
pub struct Syllable {
    /// Initial consonant (e.g., "ch", "tr", "ng")
    pub initial: String,
    
    /// Vowel cluster (e.g., "a", "uo", "ie")
    pub vowel: String,
    
    /// Final consonant (e.g., "ng", "nh", "t")
    pub final_consonant: String,
    
    /// Tone mark
    pub tone: Option<Tone>,
    
    /// Letter modifications applied
    pub modifications: Vec<Modification>,
}

impl Syllable {
    /// Create a new empty syllable
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Check if syllable is empty
    pub fn is_empty(&self) -> bool {
        self.initial.is_empty() && self.vowel.is_empty() && self.final_consonant.is_empty()
    }
    
    /// Get length in characters
    pub fn len(&self) -> usize {
        self.initial.chars().count() 
            + self.vowel.chars().count() 
            + self.final_consonant.chars().count()
    }
    
    /// Push a character to the syllable
    /// This is a simplified version - just appends to the appropriate part
    pub fn push(&mut self, ch: char) {
        // Simple heuristic: 
        // - If vowel is empty, check if ch is vowel
        // - If vowel exists and ch is consonant, add to final
        // - Otherwise add to initial or vowel
        
        if self.vowel.is_empty() {
            if is_vowel(ch) {
                self.vowel.push(ch);
            } else {
                self.initial.push(ch);
            }
        } else {
            // After vowel
            if is_vowel(ch) {
                self.vowel.push(ch);
            } else {
                self.final_consonant.push(ch);
            }
        }
    }
    
    /// Clear the syllable
    pub fn clear(&mut self) {
        self.initial.clear();
        self.vowel.clear();
        self.final_consonant.clear();
        self.tone = None;
        self.modifications.clear();
    }
    
    /// Get the raw text without transformations
    pub fn raw_text(&self) -> String {
        format!("{}{}{}", self.initial, self.vowel, self.final_consonant)
    }
}

impl fmt::Display for Syllable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Apply transformations and render
        let mut result = String::new();
        
        // Initial consonant
        result.push_str(&self.initial);
        
        // Vowel with modifications and tone
        let vowel = apply_modifications(&self.vowel, &self.modifications);
        let vowel_with_tone = apply_tone(&vowel, self.tone);
        result.push_str(&vowel_with_tone);
        
        // Final consonant
        result.push_str(&self.final_consonant);
        
        write!(f, "{}", result)
    }
}

/// Check if character is a vowel
fn is_vowel(ch: char) -> bool {
    matches!(ch.to_lowercase().next().unwrap(), 
        'a' | 'ă' | 'â' | 'e' | 'ê' | 'i' | 'o' | 'ô' | 'ơ' | 'u' | 'ư' | 'y')
}

/// Apply letter modifications to vowel
fn apply_modifications(vowel: &str, mods: &[Modification]) -> String {
    let mut result = vowel.to_string();
    
    for &modification in mods {
        result = match modification {
            Modification::Circumflex => {
                result.replace('a', "â").replace('A', "Â")
                    .replace('e', "ê").replace('E', "Ê")
                    .replace('o', "ô").replace('O', "Ô")
            }
            Modification::Breve => {
                result.replace('a', "ă").replace('A', "Ă")
            }
            Modification::Horn => {
                result.replace('o', "ơ").replace('O', "Ơ")
                    .replace('u', "ư").replace('U', "Ư")
            }
            Modification::DStroke => {
                result.replace('d', "đ").replace('D', "Đ")
            }
        };
    }
    
    result
}

/// Apply tone mark to vowel
fn apply_tone(vowel: &str, tone: Option<Tone>) -> String {
    let Some(tone) = tone else {
        return vowel.to_string();
    };
    
    // Use smart tone placement rules
    use crate::rules::{place_tone, ToneStyle};
    place_tone(vowel, tone, ToneStyle::New)
}

/// Apply tone to a single character
/// Apply tone to a single character
pub fn apply_tone_to_char(ch: char, tone: Tone) -> char {
    match (ch.to_lowercase().next().unwrap(), tone) {
        ('a', Tone::Acute) => if ch.is_uppercase() { 'Á' } else { 'á' },
        ('a', Tone::Grave) => if ch.is_uppercase() { 'À' } else { 'à' },
        ('a', Tone::HookAbove) => if ch.is_uppercase() { 'Ả' } else { 'ả' },
        ('a', Tone::Tilde) => if ch.is_uppercase() { 'Ã' } else { 'ã' },
        ('a', Tone::Underdot) => if ch.is_uppercase() { 'Ạ' } else { 'ạ' },
        
        ('ă', Tone::Acute) => if ch.is_uppercase() { 'Ắ' } else { 'ắ' },
        ('ă', Tone::Grave) => if ch.is_uppercase() { 'Ằ' } else { 'ằ' },
        ('ă', Tone::HookAbove) => if ch.is_uppercase() { 'Ẳ' } else { 'ẳ' },
        ('ă', Tone::Tilde) => if ch.is_uppercase() { 'Ẵ' } else { 'ẵ' },
        ('ă', Tone::Underdot) => if ch.is_uppercase() { 'Ặ' } else { 'ặ' },
        
        ('â', Tone::Acute) => if ch.is_uppercase() { 'Ấ' } else { 'ấ' },
        ('â', Tone::Grave) => if ch.is_uppercase() { 'Ầ' } else { 'ầ' },
        ('â', Tone::HookAbove) => if ch.is_uppercase() { 'Ẩ' } else { 'ẩ' },
        ('â', Tone::Tilde) => if ch.is_uppercase() { 'Ẫ' } else { 'ẫ' },
        ('â', Tone::Underdot) => if ch.is_uppercase() { 'Ậ' } else { 'ậ' },
        
        ('e', Tone::Acute) => if ch.is_uppercase() { 'É' } else { 'é' },
        ('e', Tone::Grave) => if ch.is_uppercase() { 'È' } else { 'è' },
        ('e', Tone::HookAbove) => if ch.is_uppercase() { 'Ẻ' } else { 'ẻ' },
        ('e', Tone::Tilde) => if ch.is_uppercase() { 'Ẽ' } else { 'ẽ' },
        ('e', Tone::Underdot) => if ch.is_uppercase() { 'Ẹ' } else { 'ẹ' },
        
        ('ê', Tone::Acute) => if ch.is_uppercase() { 'Ế' } else { 'ế' },
        ('ê', Tone::Grave) => if ch.is_uppercase() { 'Ề' } else { 'ề' },
        ('ê', Tone::HookAbove) => if ch.is_uppercase() { 'Ể' } else { 'ể' },
        ('ê', Tone::Tilde) => if ch.is_uppercase() { 'Ễ' } else { 'ễ' },
        ('ê', Tone::Underdot) => if ch.is_uppercase() { 'Ệ' } else { 'ệ' },
        
        ('i', Tone::Acute) => if ch.is_uppercase() { 'Í' } else { 'í' },
        ('i', Tone::Grave) => if ch.is_uppercase() { 'Ì' } else { 'ì' },
        ('i', Tone::HookAbove) => if ch.is_uppercase() { 'Ỉ' } else { 'ỉ' },
        ('i', Tone::Tilde) => if ch.is_uppercase() { 'Ĩ' } else { 'ĩ' },
        ('i', Tone::Underdot) => if ch.is_uppercase() { 'Ị' } else { 'ị' },
        
        ('o', Tone::Acute) => if ch.is_uppercase() { 'Ó' } else { 'ó' },
        ('o', Tone::Grave) => if ch.is_uppercase() { 'Ò' } else { 'ò' },
        ('o', Tone::HookAbove) => if ch.is_uppercase() { 'Ỏ' } else { 'ỏ' },
        ('o', Tone::Tilde) => if ch.is_uppercase() { 'Õ' } else { 'õ' },
        ('o', Tone::Underdot) => if ch.is_uppercase() { 'Ọ' } else { 'ọ' },
        
        ('ô', Tone::Acute) => if ch.is_uppercase() { 'Ố' } else { 'ố' },
        ('ô', Tone::Grave) => if ch.is_uppercase() { 'Ồ' } else { 'ồ' },
        ('ô', Tone::HookAbove) => if ch.is_uppercase() { 'Ổ' } else { 'ổ' },
        ('ô', Tone::Tilde) => if ch.is_uppercase() { 'Ỗ' } else { 'ỗ' },
        ('ô', Tone::Underdot) => if ch.is_uppercase() { 'Ộ' } else { 'ộ' },
        
        ('ơ', Tone::Acute) => if ch.is_uppercase() { 'Ớ' } else { 'ớ' },
        ('ơ', Tone::Grave) => if ch.is_uppercase() { 'Ờ' } else { 'ờ' },
        ('ơ', Tone::HookAbove) => if ch.is_uppercase() { 'Ở' } else { 'ở' },
        ('ơ', Tone::Tilde) => if ch.is_uppercase() { 'Ỡ' } else { 'ỡ' },
        ('ơ', Tone::Underdot) => if ch.is_uppercase() { 'Ợ' } else { 'ợ' },
        
        ('u', Tone::Acute) => if ch.is_uppercase() { 'Ú' } else { 'ú' },
        ('u', Tone::Grave) => if ch.is_uppercase() { 'Ù' } else { 'ù' },
        ('u', Tone::HookAbove) => if ch.is_uppercase() { 'Ủ' } else { 'ủ' },
        ('u', Tone::Tilde) => if ch.is_uppercase() { 'Ũ' } else { 'ũ' },
        ('u', Tone::Underdot) => if ch.is_uppercase() { 'Ụ' } else { 'ụ' },
        
        ('ư', Tone::Acute) => if ch.is_uppercase() { 'Ứ' } else { 'ứ' },
        ('ư', Tone::Grave) => if ch.is_uppercase() { 'Ừ' } else { 'ừ' },
        ('ư', Tone::HookAbove) => if ch.is_uppercase() { 'Ử' } else { 'ử' },
        ('ư', Tone::Tilde) => if ch.is_uppercase() { 'Ữ' } else { 'ữ' },
        ('ư', Tone::Underdot) => if ch.is_uppercase() { 'Ự' } else { 'ự' },
        
        ('y', Tone::Acute) => if ch.is_uppercase() { 'Ý' } else { 'ý' },
        ('y', Tone::Grave) => if ch.is_uppercase() { 'Ỳ' } else { 'ỳ' },
        ('y', Tone::HookAbove) => if ch.is_uppercase() { 'Ỷ' } else { 'ỷ' },
        ('y', Tone::Tilde) => if ch.is_uppercase() { 'Ỹ' } else { 'ỹ' },
        ('y', Tone::Underdot) => if ch.is_uppercase() { 'Ỵ' } else { 'ỵ' },
        
        _ => ch, // No tone for this character
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syllable_creation() {
        let syllable = Syllable::new();
        assert!(syllable.is_empty());
    }
    
    #[test]
    fn test_syllable_push() {
        let mut syllable = Syllable::new();
        syllable.push('h');
        syllable.push('o');
        syllable.push('a');
        
        assert_eq!(syllable.initial, "h");
        assert_eq!(syllable.vowel, "oa");
    }
    
    #[test]
    fn test_tone_application() {
        let mut syllable = Syllable::new();
        syllable.vowel = "a".to_string();
        syllable.tone = Some(Tone::Acute);
        
        assert_eq!(syllable.to_string(), "á");
    }
}
