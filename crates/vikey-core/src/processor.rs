// processor.rs - Main Vietnamese input processor

use crate::buffer::InputBuffer;
use crate::lookup::LookupTable;
use crate::types::{
    Action, Config, InputMethod, MarkType, ToneType, Transformation, TransformEffect,
};

/// Main processor for Vietnamese input
pub struct Processor {
    buffer: InputBuffer,
    lookup: LookupTable,
    method: InputMethod,
    transformations: Vec<Transformation>,
}

impl Processor {
    /// Create a new processor
    pub fn new(method: InputMethod) -> Self {
        Self {
            buffer: InputBuffer::new(),
            lookup: LookupTable::new(method),
            method,
            transformations: Vec::new(),
        }
    }

    /// Process a single key
    pub fn process(&mut self, ch: char, config: &Config) -> Action {
        if !ch.is_ascii() {
            return Action::DoNothing;
        }

        let ascii_ch = ch as u8;
        let char_info = self.lookup.get_info(ascii_ch);

        // Handle separators
        if char_info.is_separator {
            let text = self.buffer.to_string();
            self.buffer.clear();
            self.transformations.clear();
            return if !text.is_empty() {
                Action::Commit(text)
            } else {
                Action::DoNothing
            };
        }

        // Check for double char (aa, ee, oo, dd)
        if let Some(action) = self.try_double_char(ch) {
            return action;
        }

        // Check for breve mark (w in Telex: aw→ă, ow→ơ, uw→ư)
        if config.input_method == InputMethod::Telex && char_info.is_breve {
            if let Some(action) = self.try_breve_mark(ch) {
                return action;
            }
        }

        // Check for tone mark (s, f, r, x, j in Telex)
        if char_info.tone_index > 0 {
            if let Some(action) = self.try_tone_mark(ch, char_info.tone_index, config) {
                return action;
            }
        }

        // Regular character - just append
        let is_lowercase = ch.is_lowercase();
        self.buffer.push(ch, is_lowercase);
        self.transformations
            .push(Transformation::append(ch, !is_lowercase));

        Action::DoNothing
    }

    /// Try to handle double character (aa→â, ee→ê, oo→ô, dd→đ)
    fn try_double_char(&mut self, ch: char) -> Option<Action> {
        if self.buffer.is_empty() {
            return None;
        }

        let last = *self.buffer.last()?;
        let ch_lower = ch.to_lowercase().next()?;
        let last_lower = last.to_lowercase().next()?;

        // Check if current char matches last char
        if ch_lower != last_lower {
            return None;
        }

        // Map double chars to their transformed versions
        let transformed = match ch_lower {
            'a' => 'â',
            'e' => 'ê',
            'o' => 'ô',
            'd' => 'đ',
            _ => return None,
        };

        // Apply case from the FIRST character
        let result = if last.is_uppercase() {
            transformed.to_uppercase().next()?
        } else {
            transformed
        };

        // Remove last char and add transformed char
        self.buffer.pop();
        self.buffer.push(result, result.is_lowercase());

        // Track transformation
        let pos = self.buffer.len() - 1;
        let mark = match ch_lower {
            'a' | 'e' | 'o' => MarkType::Circumflex,
            'd' => MarkType::DStroke,
            _ => MarkType::None,
        };
        self.transformations
            .push(Transformation::mark(ch, mark, pos));

        Some(Action::Replace {
            backspace_count: 1,
            text: result.to_string(),
        })
    }

    /// Try to handle breve mark (w: aw→ă, ow→ơ, uw→ư)
    fn try_breve_mark(&mut self, _ch: char) -> Option<Action> {
        if self.buffer.is_empty() {
            return None;
        }

        let last = *self.buffer.last()?;
        let last_lower = last.to_lowercase().next()?;

        // Map vowel + w to breve/horn versions
        let transformed = match last_lower {
            'a' => 'ă',
            'o' => 'ơ',
            'u' => 'ư',
            _ => return None,
        };

        // Apply case from the original vowel
        let result = if last.is_uppercase() {
            transformed.to_uppercase().next()?
        } else {
            transformed
        };

        // Remove last char and add transformed char
        self.buffer.pop();
        self.buffer.push(result, result.is_lowercase());

        // Track transformation
        let pos = self.buffer.len() - 1;
        let mark = match last_lower {
            'a' => MarkType::Breve,
            'o' | 'u' => MarkType::Horn,
            _ => MarkType::None,
        };
        self.transformations
            .push(Transformation::mark('w', mark, pos));

        Some(Action::Replace {
            backspace_count: 1,
            text: result.to_string(),
        })
    }

    /// Try to handle tone mark (s→sắc, f→huyền, r→hỏi, x→ngã, j→nặng)
    fn try_tone_mark(&mut self, ch: char, tone_index: u8, config: &Config) -> Option<Action> {
        if self.buffer.is_empty() {
            return None;
        }

        // Map tone index to ToneType
        let tone_type = match tone_index {
            1 => ToneType::Acute,   // s - Sắc
            2 => ToneType::Grave,   // f - Huyền
            3 => ToneType::Hook,    // r - Hỏi
            4 => ToneType::Tilde,   // x - Ngã
            5 => ToneType::Dot,     // j - Nặng
            _ => return None,
        };

        // Find position to place tone
        let tone_pos = self.find_tone_position(config.modern_orthography)?;

        // Get the character at tone position
        let target_char = *self.buffer.get(tone_pos)?;

        // Apply tone to the character
        let toned_char = self.apply_tone(target_char, tone_type)?;

        // Replace the character in buffer
        self.buffer.set(tone_pos, toned_char);

        // Track transformation
        self.transformations
            .push(Transformation::tone(ch, tone_type, tone_pos));

        // Calculate how many characters to backspace and what to output
        let backspace_count = self.buffer.len() - tone_pos;
        let new_text: String = self.buffer.chars_from(tone_pos).collect();

        Some(Action::Replace {
            backspace_count,
            text: new_text,
        })
    }

    /// Find the position where tone should be placed
    /// Returns the index in the buffer where the tone mark should go
    fn find_tone_position(&self, modern: bool) -> Option<usize> {
        let len = self.buffer.len();
        if len == 0 {
            return None;
        }

        // Collect vowels and their positions
        let mut vowels: Vec<(usize, char)> = Vec::new();
        for i in 0..len {
            let ch = *self.buffer.get(i)?;
            if self.is_vowel(ch) {
                vowels.push((i, ch));
            }
        }

        if vowels.is_empty() {
            return None;
        }

        // Single vowel: place tone there
        if vowels.len() == 1 {
            return Some(vowels[0].0);
        }

        // Multiple vowels: apply Vietnamese tone placement rules
        // Rules:
        // 1. If there are 3 vowels (triphthong): middle vowel
        // 2. If there are 2 vowels (diphthong):
        //    - oa, oe, uy: depends on modern/old orthography
        //      - modern: second vowel (oà, uý)
        //      - old: first vowel (òa, uỷ)
        //    - others: first vowel
        
        if vowels.len() >= 3 {
            // Triphthong: middle vowel
            return Some(vowels[1].0);
        }

        // Diphthong (2 vowels)
        let first = vowels[0].1.to_lowercase().next()?;
        let second = vowels[1].1.to_lowercase().next()?;

        // Check for special diphthongs
        let is_special = matches!(
            (first, second),
            ('o', 'a') | ('o', 'e') | ('u', 'y')
        );

        if is_special {
            // Modern: second vowel, Old: first vowel
            if modern {
                Some(vowels[1].0)
            } else {
                Some(vowels[0].0)
            }
        } else {
            // Default: first vowel
            Some(vowels[0].0)
        }
    }

    /// Check if a character is a vowel
    fn is_vowel(&self, ch: char) -> bool {
        let ch_lower = ch.to_lowercase().next().unwrap_or(ch);
        matches!(
            ch_lower,
            'a' | 'ă' | 'â' | 'e' | 'ê' | 'i' | 'o' | 'ô' | 'ơ' | 'u' | 'ư' | 'y'
        )
    }

    /// Apply tone to a character
    fn apply_tone(&self, ch: char, tone: ToneType) -> Option<char> {
        let is_upper = ch.is_uppercase();
        let ch_lower = ch.to_lowercase().next()?;

        // Remove existing tone first
        let base = self.remove_tone(ch_lower);

        // Apply new tone
        let result = match (base, tone) {
            // Base 'a'
            ('a', ToneType::Acute) => 'á',
            ('a', ToneType::Grave) => 'à',
            ('a', ToneType::Hook) => 'ả',
            ('a', ToneType::Tilde) => 'ã',
            ('a', ToneType::Dot) => 'ạ',
            ('a', ToneType::None) => 'a',
            
            // Base 'ă'
            ('ă', ToneType::Acute) => 'ắ',
            ('ă', ToneType::Grave) => 'ằ',
            ('ă', ToneType::Hook) => 'ẳ',
            ('ă', ToneType::Tilde) => 'ẵ',
            ('ă', ToneType::Dot) => 'ặ',
            ('ă', ToneType::None) => 'ă',
            
            // Base 'â'
            ('â', ToneType::Acute) => 'ấ',
            ('â', ToneType::Grave) => 'ầ',
            ('â', ToneType::Hook) => 'ẩ',
            ('â', ToneType::Tilde) => 'ẫ',
            ('â', ToneType::Dot) => 'ậ',
            ('â', ToneType::None) => 'â',
            
            // Base 'e'
            ('e', ToneType::Acute) => 'é',
            ('e', ToneType::Grave) => 'è',
            ('e', ToneType::Hook) => 'ẻ',
            ('e', ToneType::Tilde) => 'ẽ',
            ('e', ToneType::Dot) => 'ẹ',
            ('e', ToneType::None) => 'e',
            
            // Base 'ê'
            ('ê', ToneType::Acute) => 'ế',
            ('ê', ToneType::Grave) => 'ề',
            ('ê', ToneType::Hook) => 'ể',
            ('ê', ToneType::Tilde) => 'ễ',
            ('ê', ToneType::Dot) => 'ệ',
            ('ê', ToneType::None) => 'ê',
            
            // Base 'i'
            ('i', ToneType::Acute) => 'í',
            ('i', ToneType::Grave) => 'ì',
            ('i', ToneType::Hook) => 'ỉ',
            ('i', ToneType::Tilde) => 'ĩ',
            ('i', ToneType::Dot) => 'ị',
            ('i', ToneType::None) => 'i',
            
            // Base 'o'
            ('o', ToneType::Acute) => 'ó',
            ('o', ToneType::Grave) => 'ò',
            ('o', ToneType::Hook) => 'ỏ',
            ('o', ToneType::Tilde) => 'õ',
            ('o', ToneType::Dot) => 'ọ',
            ('o', ToneType::None) => 'o',
            
            // Base 'ô'
            ('ô', ToneType::Acute) => 'ố',
            ('ô', ToneType::Grave) => 'ồ',
            ('ô', ToneType::Hook) => 'ổ',
            ('ô', ToneType::Tilde) => 'ỗ',
            ('ô', ToneType::Dot) => 'ộ',
            ('ô', ToneType::None) => 'ô',
            
            // Base 'ơ'
            ('ơ', ToneType::Acute) => 'ớ',
            ('ơ', ToneType::Grave) => 'ờ',
            ('ơ', ToneType::Hook) => 'ở',
            ('ơ', ToneType::Tilde) => 'ỡ',
            ('ơ', ToneType::Dot) => 'ợ',
            ('ơ', ToneType::None) => 'ơ',
            
            // Base 'u'
            ('u', ToneType::Acute) => 'ú',
            ('u', ToneType::Grave) => 'ù',
            ('u', ToneType::Hook) => 'ủ',
            ('u', ToneType::Tilde) => 'ũ',
            ('u', ToneType::Dot) => 'ụ',
            ('u', ToneType::None) => 'u',
            
            // Base 'ư'
            ('ư', ToneType::Acute) => 'ứ',
            ('ư', ToneType::Grave) => 'ừ',
            ('ư', ToneType::Hook) => 'ử',
            ('ư', ToneType::Tilde) => 'ữ',
            ('ư', ToneType::Dot) => 'ự',
            ('ư', ToneType::None) => 'ư',
            
            // Base 'y'
            ('y', ToneType::Acute) => 'ý',
            ('y', ToneType::Grave) => 'ỳ',
            ('y', ToneType::Hook) => 'ỷ',
            ('y', ToneType::Tilde) => 'ỹ',
            ('y', ToneType::Dot) => 'ỵ',
            ('y', ToneType::None) => 'y',
            
            _ => return None,
        };

        // Apply original case
        if is_upper {
            result.to_uppercase().next()
        } else {
            Some(result)
        }
    }

    /// Remove tone from a character, returning the base character
    fn remove_tone(&self, ch: char) -> char {
        match ch {
            // a family
            'á' | 'à' | 'ả' | 'ã' | 'ạ' => 'a',
            'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' => 'ă',
            'ấ' | 'ầ' | 'ẩ' | 'ẫ' | 'ậ' => 'â',
            
            // e family
            'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' => 'e',
            'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' => 'ê',
            
            // i family
            'í' | 'ì' | 'ỉ' | 'ĩ' | 'ị' => 'i',
            
            // o family
            'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' => 'o',
            'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' => 'ô',
            'ớ' | 'ờ' | 'ở' | 'ỡ' | 'ợ' => 'ơ',
            
            // u family
            'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' => 'u',
            'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' => 'ư',
            
            // y family
            'ý' | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' => 'y',
            
            // Already base
            _ => ch,
        }
    }

    /// Process backspace
    pub fn process_backspace(&mut self) -> Action {
        if let Some((ch, _)) = self.buffer.pop() {
            // Also remove last transformation
            self.transformations.pop();

            Action::Replace {
                backspace_count: 1,
                text: String::new(),
            }
        } else {
            Action::DoNothing
        }
    }

    /// Reset the processor
    pub fn reset(&mut self) {
        self.buffer.clear();
        self.transformations.clear();
    }

    /// Get current buffer content
    pub fn buffer_content(&self) -> String {
        self.buffer.to_string()
    }

    /// Get transformation history (for debugging/undo)
    pub fn transformations(&self) -> &[Transformation] {
        &self.transformations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_telex_processor() -> Processor {
        Processor::new(InputMethod::Telex)
    }

    fn create_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_double_char_a() {
        let mut proc = create_telex_processor();
        let config = create_config();

        let action1 = proc.process('a', &config);
        assert_eq!(action1, Action::DoNothing);

        let action2 = proc.process('a', &config);
        assert_eq!(
            action2,
            Action::Replace {
                backspace_count: 1,
                text: "â".to_string()
            }
        );
        assert_eq!(proc.buffer_content(), "â");
    }

    #[test]
    fn test_double_char_uppercase() {
        let mut proc = create_telex_processor();
        let config = create_config();

        proc.process('A', &config);
        let action = proc.process('a', &config);

        assert_eq!(
            action,
            Action::Replace {
                backspace_count: 1,
                text: "Â".to_string()
            }
        );
        assert_eq!(proc.buffer_content(), "Â");
    }

    #[test]
    fn test_double_char_d() {
        let mut proc = create_telex_processor();
        let config = create_config();

        proc.process('d', &config);
        let action = proc.process('d', &config);

        assert_eq!(
            action,
            Action::Replace {
                backspace_count: 1,
                text: "đ".to_string()
            }
        );
        assert_eq!(proc.buffer_content(), "đ");
    }

    #[test]
    fn test_breve_a() {
        let mut proc = create_telex_processor();
        let config = create_config();

        proc.process('a', &config);
        let action = proc.process('w', &config);

        assert_eq!(
            action,
            Action::Replace {
                backspace_count: 1,
                text: "ă".to_string()
            }
        );
        assert_eq!(proc.buffer_content(), "ă");
    }

    #[test]
    fn test_breve_o() {
        let mut proc = create_telex_processor();
        let config = create_config();

        proc.process('o', &config);
        let action = proc.process('w', &config);

        assert_eq!(
            action,
            Action::Replace {
                backspace_count: 1,
                text: "ơ".to_string()
            }
        );
        assert_eq!(proc.buffer_content(), "ơ");
    }

    #[test]
    fn test_breve_u() {
        let mut proc = create_telex_processor();
        let config = create_config();

        proc.process('u', &config);
        let action = proc.process('w', &config);

        assert_eq!(
            action,
            Action::Replace {
                backspace_count: 1,
                text: "ư".to_string()
            }
        );
        assert_eq!(proc.buffer_content(), "ư");
    }

    #[test]
    fn test_backspace() {
        let mut proc = create_telex_processor();
        let config = create_config();

        proc.process('a', &config);
        proc.process('a', &config); // â

        let action = proc.process_backspace();
        assert_eq!(
            action,
            Action::Replace {
                backspace_count: 1,
                text: String::new()
            }
        );
        assert_eq!(proc.buffer_content(), "");
    }

    #[test]
    fn test_separator_commits() {
        let mut proc = create_telex_processor();
        let config = create_config();

        proc.process('a', &config);
        proc.process('a', &config); // â

        let action = proc.process(' ', &config);
        assert_eq!(action, Action::Commit("â".to_string()));
        assert_eq!(proc.buffer_content(), "");
    }

    #[test]
    fn test_transformation_tracking() {
        let mut proc = create_telex_processor();
        let config = create_config();

        proc.process('a', &config);
        assert_eq!(proc.transformations().len(), 1);

        proc.process('a', &config); // â
        assert_eq!(proc.transformations().len(), 2);
        assert_eq!(proc.transformations()[1].effect, TransformEffect::Mark);
        assert_eq!(proc.transformations()[1].mark, MarkType::Circumflex);
    }

    // Tone mark tests
    #[test]
    fn test_tone_single_vowel() {
        let mut proc = create_telex_processor();
        let config = create_config();

        proc.process('a', &config);
        let action = proc.process('s', &config); // á

        assert_eq!(
            action,
            Action::Replace {
                backspace_count: 1,
                text: "á".to_string()
            }
        );
        assert_eq!(proc.buffer_content(), "á");
    }

    #[test]
    fn test_tone_diphthong_oa_modern() {
        let mut proc = create_telex_processor();
        let config = create_config(); // modern by default

        proc.process('h', &config);
        proc.process('o', &config);
        proc.process('a', &config);
        let action = proc.process('s', &config); // hoá (modern)

        assert_eq!(
            action,
            Action::Replace {
                backspace_count: 2,
                text: "oá".to_string()
            }
        );
        assert_eq!(proc.buffer_content(), "hoá");
    }

    #[test]
    fn test_tone_all_types() {
        let mut proc = create_telex_processor();
        let config = create_config();

        // Sắc
        proc.process('a', &config);
        proc.process('s', &config);
        assert_eq!(proc.buffer_content(), "á");
        proc.reset();

        // Huyền
        proc.process('a', &config);
        proc.process('f', &config);
        assert_eq!(proc.buffer_content(), "à");
        proc.reset();

        // Hỏi
        proc.process('a', &config);
        proc.process('r', &config);
        assert_eq!(proc.buffer_content(), "ả");
        proc.reset();

        // Ngã
        proc.process('a', &config);
        proc.process('x', &config);
        assert_eq!(proc.buffer_content(), "ã");
        proc.reset();

        // Nặng
        proc.process('a', &config);
        proc.process('j', &config);
        assert_eq!(proc.buffer_content(), "ạ");
    }

    #[test]
    fn test_tone_with_circumflex() {
        let mut proc = create_telex_processor();
        let config = create_config();

        proc.process('a', &config);
        proc.process('a', &config); // â
        let action = proc.process('s', &config); // ấ

        assert_eq!(
            action,
            Action::Replace {
                backspace_count: 1,
                text: "ấ".to_string()
            }
        );
        assert_eq!(proc.buffer_content(), "ấ");
    }

    #[test]
    fn test_tone_replacement() {
        let mut proc = create_telex_processor();
        let config = create_config();

        proc.process('a', &config);
        proc.process('s', &config); // á
        let action = proc.process('f', &config); // à (replace tone)

        assert_eq!(
            action,
            Action::Replace {
                backspace_count: 1,
                text: "à".to_string()
            }
        );
        assert_eq!(proc.buffer_content(), "à");
    }
}
