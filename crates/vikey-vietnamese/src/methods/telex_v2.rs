// methods/telex_v2.rs - Telex input method with syllable-based processing

use vikey_core::traits::InputMethodTrait;
use vikey_core::types::Action;
use vikey_core::InputBuffer;
use vikey_core::traits::LookupProvider;
use crate::syllable::{Syllable, Tone, Modification};

/// Get tone from key character
fn get_tone_from_key(ch: char) -> Option<Tone> {
    match ch {
        's' | 'S' => Some(Tone::Acute),      // sắc
        'f' | 'F' => Some(Tone::Grave),      // huyền
        'r' | 'R' => Some(Tone::HookAbove),  // hỏi
        'x' | 'X' => Some(Tone::Tilde),      // ngã
        'j' | 'J' => Some(Tone::Underdot),   // nặng
        _ => None,
    }
}

/// Telex Input Method with syllable-based processing
pub struct TelexMethodV2 {
    /// Current syllable being built
    syllable: Syllable,
    
    /// Length of last output (for backspace_count)
    last_output_len: usize,
}

impl TelexMethodV2 {
    pub fn new() -> Self {
        Self {
            syllable: Syllable::new(),
            last_output_len: 0,
        }
    }
    
    /// Check if character is a separator (space, enter, etc.)
    fn is_separator(ch: char) -> bool {
        matches!(ch, ' ' | '\n' | '\t' | '.' | ',' | '!' | '?')
    }
    
    /// Apply transformations to current syllable
    fn apply_transformations(&mut self) {
        // 1. Check for letter modifications (aa→â, aw→ă, etc.)
        self.apply_letter_modifications();
        
        // 2. Check for tone marks (s,f,r,x,j)
        self.apply_tone_marks();
    }
    
    /// Apply letter modifications (aa→â, aw→ă, ow→ơ, uw→ư, dd→đ)
    fn apply_letter_modifications(&mut self) {
        // Check vowel for double letters
        if self.syllable.vowel.len() >= 2 {
            let chars: Vec<char> = self.syllable.vowel.chars().collect();
            let last_two: String = chars[chars.len()-2..].iter().collect();
            
            match last_two.to_lowercase().as_str() {
                "aa" => {
                    // aa→â (circumflex)
                    self.syllable.vowel.truncate(self.syllable.vowel.len() - 2);
                    let is_upper = chars[chars.len()-2].is_uppercase();
                    self.syllable.vowel.push(if is_upper { 'Â' } else { 'â' });
                    self.syllable.modifications.push(Modification::Circumflex);
                }
                "aw" => {
                    // aw→ă (breve)
                    self.syllable.vowel.truncate(self.syllable.vowel.len() - 2);
                    let is_upper = chars[chars.len()-2].is_uppercase();
                    self.syllable.vowel.push(if is_upper { 'Ă' } else { 'ă' });
                    self.syllable.modifications.push(Modification::Breve);
                }
                "ee" => {
                    // ee→ê (circumflex)
                    self.syllable.vowel.truncate(self.syllable.vowel.len() - 2);
                    let is_upper = chars[chars.len()-2].is_uppercase();
                    self.syllable.vowel.push(if is_upper { 'Ê' } else { 'ê' });
                    self.syllable.modifications.push(Modification::Circumflex);
                }
                "oo" => {
                    // oo→ô (circumflex)
                    self.syllable.vowel.truncate(self.syllable.vowel.len() - 2);
                    let is_upper = chars[chars.len()-2].is_uppercase();
                    self.syllable.vowel.push(if is_upper { 'Ô' } else { 'ô' });
                    self.syllable.modifications.push(Modification::Circumflex);
                }
                "ow" => {
                    // ow→ơ (horn)
                    self.syllable.vowel.truncate(self.syllable.vowel.len() - 2);
                    let is_upper = chars[chars.len()-2].is_uppercase();
                    self.syllable.vowel.push(if is_upper { 'Ơ' } else { 'ơ' });
                    self.syllable.modifications.push(Modification::Horn);
                }
                "uw" => {
                    // uw→ư (horn)
                    self.syllable.vowel.truncate(self.syllable.vowel.len() - 2);
                    let is_upper = chars[chars.len()-2].is_uppercase();
                    self.syllable.vowel.push(if is_upper { 'Ư' } else { 'ư' });
                    self.syllable.modifications.push(Modification::Horn);
                }
                _ => {}
            }
        }
        
        // Check initial consonant for dd→đ
        if self.syllable.initial.len() >= 2 {
            let chars: Vec<char> = self.syllable.initial.chars().collect();
            let last_two: String = chars[chars.len()-2..].iter().collect();
            
            if last_two.to_lowercase() == "dd" {
                self.syllable.initial.truncate(self.syllable.initial.len() - 2);
                let is_upper = chars[chars.len()-2].is_uppercase();
                self.syllable.initial.push(if is_upper { 'Đ' } else { 'đ' });
                self.syllable.modifications.push(Modification::DStroke);
            }
        }
    }
    
    /// Apply tone marks (s,f,r,x,j)
    fn apply_tone_marks(&mut self) {
        // Check if last character in final_consonant is a tone key
        if let Some(last_char) = self.syllable.final_consonant.chars().last() {
            if let Some(tone) = get_tone_from_key(last_char) {
                // Remove tone key from final consonant
                self.syllable.final_consonant.pop();
                
                // Apply tone to syllable
                self.syllable.tone = Some(tone);
            }
        }
    }
    
    /// Commit current syllable and reset
    fn commit(&mut self) -> Action {
        let output = self.syllable.to_string();
        let backspace = self.last_output_len;
        
        self.syllable.clear();
        self.last_output_len = 0;
        
        Action::Replace {
            backspace_count: backspace,
            text: output,
        }
    }
}

impl Default for TelexMethodV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl InputMethodTrait for TelexMethodV2 {
    fn name(&self) -> &str {
        "Telex V2"
    }
    
    fn id(&self) -> &str {
        "telex_v2"
    }
    
    fn process(
        &mut self,
        key: char,
        buffer: &mut InputBuffer,
        _lookup: &dyn LookupProvider
    ) -> Action {
        // Check for separator - commit current syllable
        if Self::is_separator(key) {
            let action = self.commit();
            // Also add the separator
            buffer.clear();
            return action;
        }
        
        // Add key to syllable
        self.syllable.push(key);
        
        // Apply transformations
        self.apply_transformations();
        
        // Get output
        let output = self.syllable.to_string();
        let backspace = self.last_output_len;
        self.last_output_len = output.chars().count();
        
        // Update buffer to match syllable
        buffer.clear();
        for ch in output.chars() {
            buffer.push(ch, ch.is_lowercase());
        }
        
        Action::Replace {
            backspace_count: backspace,
            text: output,
        }
    }
    
    fn process_backspace(&mut self, buffer: &mut InputBuffer) -> Action {
        if self.syllable.is_empty() {
            return Action::DoNothing;
        }
        
        // Remove last character from syllable
        // This is simplified - should properly handle syllable structure
        let raw = self.syllable.raw_text();
        if !raw.is_empty() {
            let mut chars: Vec<char> = raw.chars().collect();
            chars.pop();
            
            // Rebuild syllable
            self.syllable.clear();
            for ch in chars {
                self.syllable.push(ch);
            }
            
            // Reapply transformations
            self.apply_transformations();
        }
        
        let output = self.syllable.to_string();
        let backspace = self.last_output_len;
        self.last_output_len = output.chars().count();
        
        // Update buffer
        buffer.clear();
        for ch in output.chars() {
            buffer.push(ch, ch.is_lowercase());
        }
        
        Action::Replace {
            backspace_count: backspace,
            text: output,
        }
    }
    
    fn reset(&mut self) {
        self.syllable.clear();
        self.last_output_len = 0;
    }
    
    fn can_undo(&self, _buffer: &InputBuffer) -> bool {
        !self.syllable.is_empty()
    }
    
    fn undo(&mut self, buffer: &mut InputBuffer) -> Action {
        self.process_backspace(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::VietnameseLookup;
    
    #[test]
    fn test_telex_v2_basic() {
        let mut method = TelexMethodV2::new();
        let mut buffer = InputBuffer::new();
        let lookup = VietnameseLookup::new_telex();
        
        // Type 'a'
        let action = method.process('a', &mut buffer, &lookup);
        assert!(matches!(action, Action::Replace { ref text, .. } if text == "a"));
    }
    
    #[test]
    fn test_telex_v2_circumflex() {
        let mut method = TelexMethodV2::new();
        let mut buffer = InputBuffer::new();
        let lookup = VietnameseLookup::new_telex();
        
        method.process('a', &mut buffer, &lookup);
        let action = method.process('a', &mut buffer, &lookup);
        
        assert!(matches!(action, Action::Replace { ref text, .. } if text == "â"));
    }
}
