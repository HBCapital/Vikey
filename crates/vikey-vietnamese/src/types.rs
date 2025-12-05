// types.rs - Vietnamese-specific types

/// Vietnamese tone types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToneType {
    None = 0,
    Acute = 1,   // Sắc (á)
    Grave = 2,   // Huyền (à)
    Hook = 3,    // Hỏi (ả)
    Tilde = 4,   // Ngã (ã)
    Dot = 5,     // Nặng (ạ)
}

/// Vietnamese mark types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkType {
    None = 0,
    Horn = 1,      // Ơ, ư
    Breve = 2,     // Ă
    Circumflex = 3, // Â, ê, ô
    DStroke = 4,   // Đ
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

/// Transformation effect type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransformEffect {
    /// Append a character
    Append,
    /// Add tone mark
    Tone,
    /// Add mark (â, ê, ô, ơ, ư, ă, đ)
    Mark,
}

/// Transformation record
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
}

impl Transformation {
    pub fn new_append(key: char) -> Self {
        Self {
            key,
            effect: TransformEffect::Append,
            tone: ToneType::None,
            mark: MarkType::None,
            target_pos: None,
        }
    }
    
    pub fn new_tone(key: char, tone: ToneType, target_pos: usize) -> Self {
        Self {
            key,
            effect: TransformEffect::Tone,
            tone,
            mark: MarkType::None,
            target_pos: Some(target_pos),
        }
    }
    
    pub fn new_mark(key: char, mark: MarkType, target_pos: usize) -> Self {
        Self {
            key,
            effect: TransformEffect::Mark,
            tone: ToneType::None,
            mark,
            target_pos: Some(target_pos),
        }
    }
}
