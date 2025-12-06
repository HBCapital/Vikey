// methods/telex_v2.rs - Telex input method with history-based processing

use crate::syllable::{Modification, Syllable, Tone};
use vikey_core::traits::InputMethodTrait;
use vikey_core::traits::LookupProvider;
use vikey_core::types::Action;
use vikey_core::InputBuffer;

/// Get tone from key character
fn get_tone_from_key(ch: char) -> Option<Tone> {
    match ch {
        's' | 'S' => Some(Tone::Acute),     // sắc
        'f' | 'F' => Some(Tone::Grave),     // huyền
        'r' | 'R' => Some(Tone::HookAbove), // hỏi
        'x' | 'X' => Some(Tone::Tilde),     // ngã
        'j' | 'J' => Some(Tone::Underdot),  // nặng
        _ => None,
    }
}

/// Telex Input Method with history-based processing
pub struct TelexMethodV2 {
    /// History of typed keys for the current word
    typed_chars: Vec<char>,

    /// Current calculated syllable state
    syllable: Syllable,

    /// Length of last output (for backspace_count)
    last_output_len: usize,
}

impl TelexMethodV2 {
    pub fn new() -> Self {
        Self {
            typed_chars: Vec::new(),
            syllable: Syllable::new(),
            last_output_len: 0,
        }
    }

    /// Check if character is a separator (space, enter, etc.)
    fn is_separator(ch: char) -> bool {
        matches!(
            ch,
            ' ' | '\n'
                | '\t'
                | '.'
                | ','
                | '!'
                | '?'
                | ';'
                | ':'
                | '('
                | ')'
                | '['
                | ']'
                | '{'
                | '}'
                | '"'
                | '\''
        )
    }

    /// Rebuild syllable from typed history
    fn parse_telex(chars: &[char]) -> Syllable {
        let mut syllable = Syllable::new();

        for &key in chars {
            // 1. Try as tone mark
            if let Some(tone) = get_tone_from_key(key) {
                // Only apply tone if we have a vowel
                if !syllable.vowel.is_empty() {
                    if key == 'z' || key == 'Z' {
                        syllable.tone = None;
                    } else {
                        syllable.tone = Some(tone);
                    }
                    continue;
                }
            }

            // 2. Try as letter modification
            let mut modified = false;
            let lower_key = key.to_lowercase().next().unwrap();

            match lower_key {
                'a' => {
                    // Check for aa -> â
                    if syllable.vowel.ends_with('a') || syllable.vowel.ends_with('A') {
                        syllable.vowel.pop();
                        let is_upper = key.is_uppercase();
                        syllable.vowel.push(if is_upper { 'Â' } else { 'â' });
                        syllable.modifications.push(Modification::Circumflex);
                        modified = true;
                    }
                }
                'e' => {
                    // Check for ee -> ê
                    if syllable.vowel.ends_with('e') || syllable.vowel.ends_with('E') {
                        syllable.vowel.pop();
                        let is_upper = key.is_uppercase();
                        syllable.vowel.push(if is_upper { 'Ê' } else { 'ê' });
                        syllable.modifications.push(Modification::Circumflex);
                        modified = true;
                    }
                }
                'o' => {
                    // Check for oo -> ô
                    if syllable.vowel.ends_with('o') || syllable.vowel.ends_with('O') {
                        syllable.vowel.pop();
                        let is_upper = key.is_uppercase();
                        syllable.vowel.push(if is_upper { 'Ô' } else { 'ô' });
                        syllable.modifications.push(Modification::Circumflex);
                        modified = true;
                    }
                }
                'd' => {
                    // Check for dd -> đ (in initial)
                    if syllable.initial.ends_with('d') || syllable.initial.ends_with('D') {
                        syllable.initial.pop();
                        let is_upper = key.is_uppercase();
                        syllable.initial.push(if is_upper { 'Đ' } else { 'đ' });
                        syllable.modifications.push(Modification::DStroke);
                        modified = true;
                    }
                }
                'w' => {
                    // Check for uo -> ươ, u -> ư, o -> ơ
                    // Priority: uo -> ươ
                    if (syllable.vowel.contains('u') || syllable.vowel.contains('U'))
                        && (syllable.vowel.contains('o') || syllable.vowel.contains('O'))
                    {
                        // Replace u with ư, o with ơ
                        let new_vowel = syllable
                            .vowel
                            .replace('u', "ư")
                            .replace('U', "Ư")
                            .replace('o', "ơ")
                            .replace('O', "Ơ");
                        syllable.vowel = new_vowel;
                        syllable.modifications.push(Modification::Horn);
                        modified = true;
                    } else if syllable.vowel.contains('u') || syllable.vowel.contains('U') {
                        let new_vowel = syllable.vowel.replace('u', "ư").replace('U', "Ư");
                        syllable.vowel = new_vowel;
                        syllable.modifications.push(Modification::Horn);
                        modified = true;
                    } else if syllable.vowel.contains('o') || syllable.vowel.contains('O') {
                        let new_vowel = syllable.vowel.replace('o', "ơ").replace('O', "Ơ");
                        syllable.vowel = new_vowel;
                        syllable.modifications.push(Modification::Horn);
                        modified = true;
                    } else if syllable.vowel.ends_with('a') || syllable.vowel.ends_with('A') {
                        // aw -> ă
                        syllable.vowel.pop();
                        let is_upper = key.is_uppercase();
                        syllable.vowel.push(if is_upper { 'Ă' } else { 'ă' });
                        syllable.modifications.push(Modification::Breve);
                        modified = true;
                    }
                }
                'z' => {
                    // z removes tone
                    syllable.tone = None;
                    modified = true;
                }
                _ => {}
            }

            if modified {
                continue;
            }

            // 3. Append to syllable
            syllable.push(key);
        }

        syllable
    }

    /// Commit current syllable and reset
    fn commit(&mut self) -> Action {
        let output = self.syllable.to_string();
        let backspace = self.last_output_len;

        self.typed_chars.clear();
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
        "Telex V2 (Smart)"
    }

    fn id(&self) -> &str {
        "telex_v2"
    }

    fn process(
        &mut self,
        key: char,
        buffer: &mut InputBuffer,
        _lookup: &dyn LookupProvider,
    ) -> Action {
        // Check for separator - commit current syllable
        if Self::is_separator(key) {
            let action = self.commit();
            // Also add the separator
            buffer.clear();
            return action;
        }

        // Add key to history
        self.typed_chars.push(key);

        // Rebuild syllable
        let new_syllable = Self::parse_telex(&self.typed_chars);

        // Validate
        if !new_syllable.is_permissible() {
            eprintln!("Invalid syllable rejected: {}", new_syllable.to_string());
            // Revert history
            self.typed_chars.pop();

            // Let's try word breaking if the syllable was valid BEFORE this key.
            if !self.syllable.is_empty() {
                // Commit previous
                let _output = self.syllable.to_string();
                let _backspace = self.last_output_len;

                // Reset and start new with current key
                self.typed_chars.clear();
                self.typed_chars.push(key);
                self.syllable = Self::parse_telex(&self.typed_chars);
                self.last_output_len = self.syllable.to_string().chars().count();

                // Update buffer
                buffer.clear();
                let new_output = self.syllable.to_string();
                for ch in new_output.chars() {
                    buffer.push(ch, ch.is_lowercase());
                }

                return Action::DoNothing;
            } else {
                return Action::DoNothing;
            }
        }

        self.syllable = new_syllable;

        // Get output
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

    fn process_backspace(&mut self, buffer: &mut InputBuffer) -> Action {
        if self.typed_chars.is_empty() {
            return Action::DoNothing;
        }

        // Pop last key from history
        self.typed_chars.pop();

        // Rebuild
        self.syllable = Self::parse_telex(&self.typed_chars);

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
        self.typed_chars.clear();
        self.syllable.clear();
        self.last_output_len = 0;
    }

    fn can_undo(&self, _buffer: &InputBuffer) -> bool {
        !self.typed_chars.is_empty()
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
    fn test_telex_v2_history_basic() {
        let mut method = TelexMethodV2::new();
        let mut buffer = InputBuffer::new();
        let lookup = VietnameseLookup::new_telex();

        // Type 'a'
        let action = method.process('a', &mut buffer, &lookup);
        assert!(matches!(action, Action::Replace { ref text, .. } if text == "a"));

        // Type 'a' again -> 'â'
        let action = method.process('a', &mut buffer, &lookup);
        assert!(matches!(action, Action::Replace { ref text, .. } if text == "â"));

        // Backspace -> 'a' (Intelligent Backspace!)
        let action = method.process_backspace(&mut buffer);
        assert!(matches!(action, Action::Replace { ref text, .. } if text == "a"));
    }

    #[test]
    fn test_telex_v2_uwo_shortcut() {
        let mut method = TelexMethodV2::new();
        let mut buffer = InputBuffer::new();
        let lookup = VietnameseLookup::new_telex();

        // d u o n g w -> dương
        let keys = "duongw";
        for ch in keys.chars() {
            method.process(ch, &mut buffer, &lookup);
        }

        assert_eq!(method.syllable.to_string(), "dương");

        // Reset
        method.reset();
        buffer.clear();

        // dd u o n g w -> đương
        let keys = "dduongw";
        for ch in keys.chars() {
            method.process(ch, &mut buffer, &lookup);
        }
        assert_eq!(method.syllable.to_string(), "đương");

        // Add tone: f -> đường
        method.process('f', &mut buffer, &lookup);
        assert_eq!(method.syllable.to_string(), "đường");
    }

    #[test]
    fn test_telex_v2_complex() {
        let mut method = TelexMethodV2::new();
        let mut buffer = InputBuffer::new();
        let lookup = VietnameseLookup::new_telex();

        // toanf -> toàn
        let keys = "toanf";
        for ch in keys.chars() {
            method.process(ch, &mut buffer, &lookup);
        }
        assert_eq!(method.syllable.to_string(), "toàn");

        // z -> toan (remove tone)
        method.process('z', &mut buffer, &lookup);
        assert_eq!(method.syllable.to_string(), "toan");

        // s -> toán (add acute)
        method.process('s', &mut buffer, &lookup);
        assert_eq!(method.syllable.to_string(), "toán");

        // reset
        method.reset();
        buffer.clear();

        // duong -> duong (no w)
        let keys = "duong";
        for ch in keys.chars() {
            method.process(ch, &mut buffer, &lookup);
        }
        assert_eq!(method.syllable.to_string(), "duong"); // literal because uo is valid now

        // w -> dương
        method.process('w', &mut buffer, &lookup);
        assert_eq!(method.syllable.to_string(), "dương");
    }
}
