/// Input method types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMethod {
    /// Telex input method (aa -> â, aw -> ă, s -> sắc)
    Telex,
    /// VNI input method (a6 -> ă, a1 -> á)
    VNI,
    /// VIQR input method (a( -> ă, a' -> á)
    VIQR,
}

/// Word form classification (from fcitx5-unikey)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordForm {
    /// Not Vietnamese
    NonVn,
    /// Empty
    Empty,
    /// Consonant only (b, c, d)
    C,
    /// Vowel only (a, e, i)
    V,
    /// Consonant + Vowel (ba, ca)
    CV,
    /// Vowel + Consonant (an, am)
    VC,
    /// Consonant + Vowel + Consonant (ban, cam)
    CVC,
}

/// Transformation effect type (from ibus-bamboo)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformEffect {
    /// Append a character
    Append,
    /// Add tone mark
    Tone,
    /// Add mark (â, ê, ô, ơ, ư, ă, đ)
    Mark,
}

/// Tone type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToneType {
    None = 0,
    Acute = 1,   // Sắc (á)
    Grave = 2,   // Huyền (à)
    Hook = 3,    // Hỏi (ả)
    Tilde = 4,   // Ngã (ã)
    Dot = 5,     // Nặng (ạ)
}

/// Mark type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkType {
    None = 0,
    Horn = 1,      // Ơ, ư
    Breve = 2,     // Ă
    Circumflex = 3, // Â, ê, ô
    DStroke = 4,   // Đ
}

/// Transformation record (inspired by ibus-bamboo)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transformation {
    /// The key that triggered this transformation
    pub key: char,
    
    /// Effect type
    pub effect: TransformEffect,
    
    /// Tone type (if effect is Tone)
    pub tone: ToneType,
    
    /// Mark type (if effect is Mark)
    pub mark: MarkType,
    
    /// Target position in buffer (for Tone/Mark effects)
    pub target_pos: Option<usize>,
    
    /// Was the key uppercase?
    pub is_uppercase: bool,
}

impl Transformation {
    /// Create an APPEND transformation
    pub fn append(key: char, is_uppercase: bool) -> Self {
        Self {
            key,
            effect: TransformEffect::Append,
            tone: ToneType::None,
            mark: MarkType::None,
            target_pos: None,
            is_uppercase,
        }
    }
    
    /// Create a TONE transformation
    pub fn tone(key: char, tone: ToneType, target_pos: usize) -> Self {
        Self {
            key,
            effect: TransformEffect::Tone,
            tone,
            mark: MarkType::None,
            target_pos: Some(target_pos),
            is_uppercase: false,
        }
    }
    
    /// Create a MARK transformation
    pub fn mark(key: char, mark: MarkType, target_pos: usize) -> Self {
        Self {
            key,
            effect: TransformEffect::Mark,
            tone: ToneType::None,
            mark,
            target_pos: Some(target_pos),
            is_uppercase: false,
        }
    }
}

/// Configuration for Vikey Core
#[derive(Debug, Clone)]
pub struct Config {
    /// Input method to use
    pub input_method: InputMethod,
    
    /// Enable Quick Telex shortcuts (cc->ch, gg->gi, kk->kh, etc.)
    pub quick_telex: bool,
    
    /// Use modern orthography (oà vs òa, úy vs uý)
    /// true = modern (oà, úy), false = old (òa, uý)
    pub modern_orthography: bool,
    
    /// Allow consonants Z, F, W, J in words
    pub allow_consonant_zfwj: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            input_method: InputMethod::Telex,
            quick_telex: true,
            modern_orthography: true,
            allow_consonant_zfwj: false,
        }
    }
}

/// Action to be taken after processing a key
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Do nothing, key was not processed
    DoNothing,
    
    /// Replace N characters with new text
    /// The application should delete `backspace_count` characters
    /// and insert `text` in their place
    Replace {
        backspace_count: usize,
        text: String,
    },
    
    /// Commit the text (when separator is encountered)
    Commit(String),
}

/// Information extracted from DT lookup table for a character
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CharInfo {
    /// Vowel index (0-31)
    pub vowel_index: u8,
    
    /// Macro key index (0-15)
    pub macro_index: u8,
    
    /// Double character index (0-31) for aa, ee, oo, dd
    pub double_char_index: u8,
    
    /// Tone mark index (0-15) for s, f, r, x, j
    pub tone_index: u8,
    
    /// Current tone of this character (0-15)
    pub current_tone: u8,
    
    /// Is this a breve character (w, W in Telex)
    pub is_breve: bool,
    
    /// Is this a separator (space, enter, tab)
    pub is_separator: bool,
    
    /// Is this a soft separator (comma, period, etc.)
    pub is_soft_separator: bool,
    
    /// VNI double mark index (0-7) for 6, 7, 8, 9
    pub vni_double_index: u8,
    
    // NEW: Word structure tracking (from fcitx5-unikey)
    /// Word form at this position
    pub word_form: WordForm,
    
    /// Position of first consonant (if any)
    pub c1_offset: Option<usize>,
    
    /// Position of vowel (if any)
    pub v_offset: Option<usize>,
    
    /// Position of second consonant (if any)
    pub c2_offset: Option<usize>,
}

impl Default for CharInfo {
    fn default() -> Self {
        Self {
            vowel_index: 0,
            macro_index: 0,
            double_char_index: 0,
            tone_index: 0,
            current_tone: 0,
            is_breve: false,
            is_separator: false,
            is_soft_separator: false,
            vni_double_index: 0,
            word_form: WordForm::Empty,
            c1_offset: None,
            v_offset: None,
            c2_offset: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.input_method, InputMethod::Telex);
        assert!(config.quick_telex);
        assert!(config.modern_orthography);
        assert!(!config.allow_consonant_zfwj);
    }

    #[test]
    fn test_action_equality() {
        let action1 = Action::Replace {
            backspace_count: 1,
            text: "â".to_string(),
        };
        let action2 = Action::Replace {
            backspace_count: 1,
            text: "â".to_string(),
        };
        assert_eq!(action1, action2);
    }

    #[test]
    fn test_char_info_default() {
        let info = CharInfo::default();
        assert_eq!(info.vowel_index, 0);
        assert!(!info.is_breve);
        assert!(!info.is_separator);
    }
}
