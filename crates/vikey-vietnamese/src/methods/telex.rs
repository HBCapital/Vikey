// methods/telex.rs - Telex input method with full Vietnamese support

use lazy_static::lazy_static;
use std::collections::HashMap;
use vikey_core::traits::InputMethodTrait;
use vikey_core::traits::LookupProvider;
use vikey_core::types::Action;
use vikey_core::InputBuffer;

lazy_static! {
    /// Mark transformation map (aa→â, aw→ă, etc.)
    static ref MARK_MAP: HashMap<&'static str, char> = {
        let mut m = HashMap::new();
        // Circumflex (^)
        m.insert("aa", 'â');
        m.insert("AA", 'Â');
        m.insert("Aa", 'Â');
        m.insert("ee", 'ê');
        m.insert("EE", 'Ê');
        m.insert("Ee", 'Ê');
        m.insert("oo", 'ô');
        m.insert("OO", 'Ô');
        m.insert("Oo", 'Ô');

        // Breve (ă)
        m.insert("aw", 'ă');
        m.insert("AW", 'Ă');
        m.insert("Aw", 'Ă');

        // Horn (ơ, ư)
        m.insert("ow", 'ơ');
        m.insert("OW", 'Ơ');
        m.insert("Ow", 'Ơ');
        m.insert("uw", 'ư');
        m.insert("UW", 'Ư');
        m.insert("Uw", 'Ư');

        // D-stroke (đ)
        m.insert("dd", 'đ');
        m.insert("DD", 'Đ');
        m.insert("Dd", 'Đ');

        m
    };

    /// Tone mark characters
    static ref TONE_KEYS: HashMap<char, u8> = {
        let mut m = HashMap::new();
        m.insert('s', 1); // Sắc (acute)
        m.insert('S', 1);
        m.insert('f', 2); // Huyền (grave)
        m.insert('F', 2);
        m.insert('r', 3); // Hỏi (hook)
        m.insert('R', 3);
        m.insert('x', 4); // Ngã (tilde)
        m.insert('X', 4);
        m.insert('j', 5); // Nặng (dot below)
        m.insert('J', 5);
        m
    };

    /// Tone application map
    static ref TONE_MAP: HashMap<(char, u8), char> = {
        let mut m = HashMap::new();

        // a
        m.insert(('a', 1), 'á'); m.insert(('a', 2), 'à');
        m.insert(('a', 3), 'ả'); m.insert(('a', 4), 'ã');
        m.insert(('a', 5), 'ạ');
        m.insert(('A', 1), 'Á'); m.insert(('A', 2), 'À');
        m.insert(('A', 3), 'Ả'); m.insert(('A', 4), 'Ã');
        m.insert(('A', 5), 'Ạ');

        // ă
        m.insert(('ă', 1), 'ắ'); m.insert(('ă', 2), 'ằ');
        m.insert(('ă', 3), 'ẳ'); m.insert(('ă', 4), 'ẵ');
        m.insert(('ă', 5), 'ặ');
        m.insert(('Ă', 1), 'Ắ'); m.insert(('Ă', 2), 'Ằ');
        m.insert(('Ă', 3), 'Ẳ'); m.insert(('Ă', 4), 'Ẵ');
        m.insert(('Ă', 5), 'Ặ');

        // â
        m.insert(('â', 1), 'ấ'); m.insert(('â', 2), 'ầ');
        m.insert(('â', 3), 'ẩ'); m.insert(('â', 4), 'ẫ');
        m.insert(('â', 5), 'ậ');
        m.insert(('Â', 1), 'Ấ'); m.insert(('Â', 2), 'Ầ');
        m.insert(('Â', 3), 'Ẩ'); m.insert(('Â', 4), 'Ẫ');
        m.insert(('Â', 5), 'Ậ');

        // e
        m.insert(('e', 1), 'é'); m.insert(('e', 2), 'è');
        m.insert(('e', 3), 'ẻ'); m.insert(('e', 4), 'ẽ');
        m.insert(('e', 5), 'ẹ');
        m.insert(('E', 1), 'É'); m.insert(('E', 2), 'È');
        m.insert(('E', 3), 'Ẻ'); m.insert(('E', 4), 'Ẽ');
        m.insert(('E', 5), 'Ẹ');

        // ê
        m.insert(('ê', 1), 'ế'); m.insert(('ê', 2), 'ề');
        m.insert(('ê', 3), 'ể'); m.insert(('ê', 4), 'ễ');
        m.insert(('ê', 5), 'ệ');
        m.insert(('Ê', 1), 'Ế'); m.insert(('Ê', 2), 'Ề');
        m.insert(('Ê', 3), 'Ể'); m.insert(('Ê', 4), 'Ễ');
        m.insert(('Ê', 5), 'Ệ');

        // i
        m.insert(('i', 1), 'í'); m.insert(('i', 2), 'ì');
        m.insert(('i', 3), 'ỉ'); m.insert(('i', 4), 'ĩ');
        m.insert(('i', 5), 'ị');
        m.insert(('I', 1), 'Í'); m.insert(('I', 2), 'Ì');
        m.insert(('I', 3), 'Ỉ'); m.insert(('I', 4), 'Ĩ');
        m.insert(('I', 5), 'Ị');

        // o
        m.insert(('o', 1), 'ó'); m.insert(('o', 2), 'ò');
        m.insert(('o', 3), 'ỏ'); m.insert(('o', 4), 'õ');
        m.insert(('o', 5), 'ọ');
        m.insert(('O', 1), 'Ó'); m.insert(('O', 2), 'Ò');
        m.insert(('O', 3), 'Ỏ'); m.insert(('O', 4), 'Õ');
        m.insert(('O', 5), 'Ọ');

        // ô
        m.insert(('ô', 1), 'ố'); m.insert(('ô', 2), 'ồ');
        m.insert(('ô', 3), 'ổ'); m.insert(('ô', 4), 'ỗ');
        m.insert(('ô', 5), 'ộ');
        m.insert(('Ô', 1), 'Ố'); m.insert(('Ô', 2), 'Ồ');
        m.insert(('Ô', 3), 'Ổ'); m.insert(('Ô', 4), 'Ỗ');
        m.insert(('Ô', 5), 'Ộ');

        // ơ
        m.insert(('ơ', 1), 'ớ'); m.insert(('ơ', 2), 'ờ');
        m.insert(('ơ', 3), 'ở'); m.insert(('ơ', 4), 'ỡ');
        m.insert(('ơ', 5), 'ợ');
        m.insert(('Ơ', 1), 'Ớ'); m.insert(('Ơ', 2), 'Ờ');
        m.insert(('Ơ', 3), 'Ở'); m.insert(('Ơ', 4), 'Ỡ');
        m.insert(('Ơ', 5), 'Ợ');

        // u
        m.insert(('u', 1), 'ú'); m.insert(('u', 2), 'ù');
        m.insert(('u', 3), 'ủ'); m.insert(('u', 4), 'ũ');
        m.insert(('u', 5), 'ụ');
        m.insert(('U', 1), 'Ú'); m.insert(('U', 2), 'Ù');
        m.insert(('U', 3), 'Ủ'); m.insert(('U', 4), 'Ũ');
        m.insert(('U', 5), 'Ụ');

        // ư
        m.insert(('ư', 1), 'ứ'); m.insert(('ư', 2), 'ừ');
        m.insert(('ư', 3), 'ử'); m.insert(('ư', 4), 'ữ');
        m.insert(('ư', 5), 'ự');
        m.insert(('Ư', 1), 'Ứ'); m.insert(('Ư', 2), 'Ừ');
        m.insert(('Ư', 3), 'Ử'); m.insert(('Ư', 4), 'Ữ');
        m.insert(('Ư', 5), 'Ự');

        // y
        m.insert(('y', 1), 'ý'); m.insert(('y', 2), 'ỳ');
        m.insert(('y', 3), 'ỷ'); m.insert(('y', 4), 'ỹ');
        m.insert(('y', 5), 'ỵ');
        m.insert(('Y', 1), 'Ý'); m.insert(('Y', 2), 'Ỳ');
        m.insert(('Y', 3), 'Ỷ'); m.insert(('Y', 4), 'Ỹ');
        m.insert(('Y', 5), 'Ỵ');

        m
    };
}

/// Telex Input Method
pub struct TelexMethod {}

impl TelexMethod {
    pub fn new() -> Self {
        Self {}
    }

    /// Check if character is a vowel
    fn is_vowel(c: char) -> bool {
        matches!(
            c.to_lowercase().next().unwrap(),
            'a' | 'ă' | 'â' | 'e' | 'ê' | 'i' | 'o' | 'ô' | 'ơ' | 'u' | 'ư' | 'y'
        )
    }

    /// Apply tone to a character
    fn apply_tone(base: char, tone: u8) -> Option<char> {
        TONE_MAP.get(&(base, tone)).copied()
    }

    /// Find vowel position for tone placement
    /// Vietnamese rules: oa→oá, oe→oé, uy→uý
    fn find_tone_position(buffer: &InputBuffer) -> Option<usize> {
        let content = buffer.to_string();
        let chars: Vec<char> = content.chars().collect();

        // Find vowels
        let mut vowel_positions = Vec::new();
        for (i, &ch) in chars.iter().enumerate() {
            if Self::is_vowel(ch) {
                vowel_positions.push(i);
            }
        }

        if vowel_positions.is_empty() {
            return None;
        }

        // Single vowel: place tone on it
        if vowel_positions.len() == 1 {
            return Some(vowel_positions[0]);
        }

        // Two vowels: follow Vietnamese rules
        if vowel_positions.len() == 2 {
            let v1 = chars[vowel_positions[0]].to_lowercase().next().unwrap();
            let v2 = chars[vowel_positions[1]].to_lowercase().next().unwrap();

            // Special cases: oa, oe, uy → tone on second vowel
            if (v1 == 'o' && v2 == 'a') || (v1 == 'o' && v2 == 'e') || (v1 == 'u' && v2 == 'y') {
                return Some(vowel_positions[1]);
            }

            // Default: tone on first vowel
            return Some(vowel_positions[0]);
        }

        // Three vowels: tone on middle vowel
        if vowel_positions.len() >= 3 {
            return Some(vowel_positions[1]);
        }

        Some(vowel_positions[0])
    }
}

impl Default for TelexMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl InputMethodTrait for TelexMethod {
    fn name(&self) -> &str {
        "Telex"
    }

    fn id(&self) -> &str {
        "telex"
    }

    fn process(
        &mut self,
        key: char,
        buffer: &mut InputBuffer,
        _lookup: &dyn LookupProvider,
    ) -> Action {
        // Check for tone mark
        if let Some(&tone) = TONE_KEYS.get(&key) {
            if let Some(pos) = Self::find_tone_position(buffer) {
                let content = buffer.to_string();
                let mut chars: Vec<char> = content.chars().collect();

                if let Some(toned) = Self::apply_tone(chars[pos], tone) {
                    chars[pos] = toned;
                    let new_text: String = chars.iter().collect();

                    return Action::Replace {
                        backspace_count: content.len(),
                        text: new_text,
                    };
                }
            }
        }

        // Check for mark transformation (aa→â, aw→ă, etc.)
        buffer.push(key, key.is_lowercase());
        let content = buffer.to_string();

        // Try 2-character patterns
        if content.len() >= 2 {
            let last_two: String = content
                .chars()
                .rev()
                .take(2)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect();

            if let Some(&replacement) = MARK_MAP.get(last_two.as_str()) {
                // Remove last 2 chars and replace with marked char
                buffer.pop();
                buffer.pop();
                buffer.push(replacement, replacement.is_lowercase());

                return Action::Replace {
                    backspace_count: 2,
                    text: replacement.to_string(),
                };
            }
        }

        // No transformation, just commit the character
        Action::Commit(key.to_string())
    }

    fn process_backspace(&mut self, buffer: &mut InputBuffer) -> Action {
        if buffer.pop().is_some() {
            Action::Replace {
                backspace_count: 1,
                text: String::new(),
            }
        } else {
            Action::DoNothing
        }
    }

    fn reset(&mut self) {
        // No state to reset
    }

    fn can_undo(&self, _buffer: &InputBuffer) -> bool {
        false
    }

    fn undo(&mut self, _buffer: &mut InputBuffer) -> Action {
        Action::DoNothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lookup::VietnameseLookup;

    #[test]
    fn test_telex_info() {
        let method = TelexMethod::new();
        assert_eq!(method.id(), "telex");
        assert_eq!(method.name(), "Telex");
    }

    #[test]
    fn test_mark_aa_to_a_circumflex() {
        let mut method = TelexMethod::new();
        let mut buffer = InputBuffer::new();
        let lookup = VietnameseLookup::new_telex();

        method.process('a', &mut buffer, &lookup);
        let action = method.process('a', &mut buffer, &lookup);

        assert!(matches!(action, Action::Replace { backspace_count: 2, ref text } if text == "â"));
    }

    #[test]
    fn test_mark_aw_to_a_breve() {
        let mut method = TelexMethod::new();
        let mut buffer = InputBuffer::new();
        let lookup = VietnameseLookup::new_telex();

        method.process('a', &mut buffer, &lookup);
        let action = method.process('w', &mut buffer, &lookup);

        assert!(matches!(action, Action::Replace { backspace_count: 2, ref text } if text == "ă"));
    }

    #[test]
    fn test_tone_simple() {
        let mut method = TelexMethod::new();
        let mut buffer = InputBuffer::new();
        let lookup = VietnameseLookup::new_telex();

        method.process('a', &mut buffer, &lookup);
        let action = method.process('s', &mut buffer, &lookup); // sắc

        assert!(matches!(action, Action::Replace { ref text, .. } if text == "á"));
    }
}
