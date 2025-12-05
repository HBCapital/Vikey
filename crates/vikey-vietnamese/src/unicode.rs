//! Unicode handling for Vietnamese

use unicode_normalization::UnicodeNormalization;

/// Normalize Vietnamese text to NFC form
pub fn normalize_nfc(text: &str) -> String {
    text.nfc().collect()
}

/// Normalize Vietnamese text to NFD form
pub fn normalize_nfd(text: &str) -> String {
    text.nfd().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize() {
        let text = "tiếng việt";
        let nfc = normalize_nfc(text);
        let nfd = normalize_nfd(text);
        
        // Both should represent the same text
        assert_eq!(normalize_nfc(&nfd), nfc);
    }
}
