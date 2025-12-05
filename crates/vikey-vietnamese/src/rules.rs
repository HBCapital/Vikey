//! Vietnamese language rules

pub struct VietnameseRules;

impl VietnameseRules {
    /// Check if character is a Vietnamese vowel
    pub fn is_vowel(ch: char) -> bool {
        matches!(
            ch.to_lowercase().next().unwrap(),
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' |
            'â' | 'ă' | 'ê' | 'ô' | 'ơ' | 'ư'
        )
    }
    
    /// Check if character is a consonant
    pub fn is_consonant(ch: char) -> bool {
        ch.is_alphabetic() && !Self::is_vowel(ch)
    }
    
    /// Check if word is valid Vietnamese
    pub fn is_valid_word(_word: &str) -> bool {
        // TODO: Implement dictionary check
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_vowel() {
        assert!(VietnameseRules::is_vowel('a'));
        assert!(VietnameseRules::is_vowel('â'));
        assert!(!VietnameseRules::is_vowel('b'));
    }
    
    #[test]
    fn test_is_consonant() {
        assert!(VietnameseRules::is_consonant('b'));
        assert!(VietnameseRules::is_consonant('đ'));
        assert!(!VietnameseRules::is_consonant('a'));
    }
}
