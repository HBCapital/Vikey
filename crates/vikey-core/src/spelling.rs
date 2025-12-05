// spelling.rs - Vietnamese spell checking (inspired by ibus-bamboo)

use std::collections::HashSet;

/// Spell checker for Vietnamese words
pub struct SpellChecker {
    /// Valid vowel sequences (a, e, i, o, u, y, ia, ua, ưa, etc.)
    valid_vowel_seqs: HashSet<&'static str>,
    
    /// Valid consonant starts (b, c, ch, d, đ, g, gh, gi, h, k, kh, l, m, n, ng, nh, p, ph, qu, r, s, t, th, tr, v, x)
    valid_consonant_starts: HashSet<&'static str>,
    
    /// Valid consonant ends (c, ch, m, n, ng, nh, p, t)
    valid_consonant_ends: HashSet<&'static str>,
}

impl SpellChecker {
    /// Create a new spell checker with Vietnamese rules
    pub fn new() -> Self {
        let mut valid_vowel_seqs = HashSet::new();
        
        // Single vowels
        valid_vowel_seqs.insert("a");
        valid_vowel_seqs.insert("ă");
        valid_vowel_seqs.insert("â");
        valid_vowel_seqs.insert("e");
        valid_vowel_seqs.insert("ê");
        valid_vowel_seqs.insert("i");
        valid_vowel_seqs.insert("o");
        valid_vowel_seqs.insert("ô");
        valid_vowel_seqs.insert("ơ");
        valid_vowel_seqs.insert("u");
        valid_vowel_seqs.insert("ư");
        valid_vowel_seqs.insert("y");
        
        // Diphthongs
        valid_vowel_seqs.insert("ai");
        valid_vowel_seqs.insert("ao");
        valid_vowel_seqs.insert("au");
        valid_vowel_seqs.insert("ay");
        valid_vowel_seqs.insert("ây");
        valid_vowel_seqs.insert("eo");
        valid_vowel_seqs.insert("êu");
        valid_vowel_seqs.insert("ia");
        valid_vowel_seqs.insert("iê");
        valid_vowel_seqs.insert("iu");
        valid_vowel_seqs.insert("oa");
        valid_vowel_seqs.insert("oă");
        valid_vowel_seqs.insert("oe");
        valid_vowel_seqs.insert("oi");
        valid_vowel_seqs.insert("ôi");
        valid_vowel_seqs.insert("ơi");
        valid_vowel_seqs.insert("ua");
        valid_vowel_seqs.insert("uă");
        valid_vowel_seqs.insert("uâ");
        valid_vowel_seqs.insert("uê");
        valid_vowel_seqs.insert("ui");
        valid_vowel_seqs.insert("uô");
        valid_vowel_seqs.insert("ươ");
        valid_vowel_seqs.insert("uy");
        valid_vowel_seqs.insert("ưa");
        valid_vowel_seqs.insert("ưi");
        valid_vowel_seqs.insert("ưu");
        valid_vowel_seqs.insert("yê");
        
        // Triphthongs
        valid_vowel_seqs.insert("iêu");
        valid_vowel_seqs.insert("oai");
        valid_vowel_seqs.insert("oao");
        valid_vowel_seqs.insert("oay");
        valid_vowel_seqs.insert("oeo");
        valid_vowel_seqs.insert("uao");
        valid_vowel_seqs.insert("uây");
        valid_vowel_seqs.insert("uôi");
        valid_vowel_seqs.insert("ươi");
        valid_vowel_seqs.insert("ươu");
        valid_vowel_seqs.insert("uyê");
        
        let mut valid_consonant_starts = HashSet::new();
        valid_consonant_starts.insert("b");
        valid_consonant_starts.insert("c");
        valid_consonant_starts.insert("ch");
        valid_consonant_starts.insert("d");
        valid_consonant_starts.insert("đ");
        valid_consonant_starts.insert("g");
        valid_consonant_starts.insert("gh");
        valid_consonant_starts.insert("gi");
        valid_consonant_starts.insert("h");
        valid_consonant_starts.insert("k");
        valid_consonant_starts.insert("kh");
        valid_consonant_starts.insert("l");
        valid_consonant_starts.insert("m");
        valid_consonant_starts.insert("n");
        valid_consonant_starts.insert("ng");
        valid_consonant_starts.insert("ngh");
        valid_consonant_starts.insert("nh");
        valid_consonant_starts.insert("p");
        valid_consonant_starts.insert("ph");
        valid_consonant_starts.insert("qu");
        valid_consonant_starts.insert("r");
        valid_consonant_starts.insert("s");
        valid_consonant_starts.insert("t");
        valid_consonant_starts.insert("th");
        valid_consonant_starts.insert("tr");
        valid_consonant_starts.insert("v");
        valid_consonant_starts.insert("x");
        
        let mut valid_consonant_ends = HashSet::new();
        valid_consonant_ends.insert("c");
        valid_consonant_ends.insert("ch");
        valid_consonant_ends.insert("m");
        valid_consonant_ends.insert("n");
        valid_consonant_ends.insert("ng");
        valid_consonant_ends.insert("nh");
        valid_consonant_ends.insert("p");
        valid_consonant_ends.insert("t");
        
        Self {
            valid_vowel_seqs,
            valid_consonant_starts,
            valid_consonant_ends,
        }
    }
    
    /// Check if a vowel sequence is valid
    pub fn is_valid_vowel_seq(&self, seq: &str) -> bool {
        // Remove tone marks for checking
        let normalized = self.remove_tones(seq);
        self.valid_vowel_seqs.contains(normalized.as_str())
    }
    
    /// Check if a consonant start is valid
    pub fn is_valid_consonant_start(&self, consonant: &str) -> bool {
        self.valid_consonant_starts.contains(consonant)
    }
    
    /// Check if a consonant end is valid
    pub fn is_valid_consonant_end(&self, consonant: &str) -> bool {
        self.valid_consonant_ends.contains(consonant)
    }
    
    /// Remove tone marks from a string (simplified version)
    fn remove_tones(&self, s: &str) -> String {
        s.chars()
            .map(|c| match c {
                'á' | 'à' | 'ả' | 'ã' | 'ạ' => 'a',
                'ắ' | 'ằ' | 'ẳ' | 'ẵ' | 'ặ' => 'ă',
                'ấ' | 'ầ' | 'ẩ' | 'ẫ' | 'ậ' => 'â',
                'é' | 'è' | 'ẻ' | 'ẽ' | 'ẹ' => 'e',
                'ế' | 'ề' | 'ể' | 'ễ' | 'ệ' => 'ê',
                'í' | 'ì' | 'ỉ' | 'ĩ' | 'ị' => 'i',
                'ó' | 'ò' | 'ỏ' | 'õ' | 'ọ' => 'o',
                'ố' | 'ồ' | 'ổ' | 'ỗ' | 'ộ' => 'ô',
                'ớ' | 'ờ' | 'ở' | 'ỡ' | 'ợ' => 'ơ',
                'ú' | 'ù' | 'ủ' | 'ũ' | 'ụ' => 'u',
                'ứ' | 'ừ' | 'ử' | 'ữ' | 'ự' => 'ư',
                'ý' | 'ỳ' | 'ỷ' | 'ỹ' | 'ỵ' => 'y',
                _ => c,
            })
            .collect()
    }
}

impl Default for SpellChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_vowel_sequences() {
        let checker = SpellChecker::new();
        
        assert!(checker.is_valid_vowel_seq("a"));
        assert!(checker.is_valid_vowel_seq("ă"));
        assert!(checker.is_valid_vowel_seq("â"));
        assert!(checker.is_valid_vowel_seq("oa"));
        assert!(checker.is_valid_vowel_seq("ươ"));
        assert!(checker.is_valid_vowel_seq("iêu"));
    }

    #[test]
    fn test_invalid_vowel_sequences() {
        let checker = SpellChecker::new();
        
        // These should not be valid
        assert!(!checker.is_valid_vowel_seq("ơơ"));
        assert!(!checker.is_valid_vowel_seq("ưư"));
        assert!(!checker.is_valid_vowel_seq("aaa"));
    }

    #[test]
    fn test_valid_consonant_starts() {
        let checker = SpellChecker::new();
        
        assert!(checker.is_valid_consonant_start("b"));
        assert!(checker.is_valid_consonant_start("ch"));
        assert!(checker.is_valid_consonant_start("tr"));
        assert!(checker.is_valid_consonant_start("qu"));
        assert!(checker.is_valid_consonant_start("ngh"));
    }

    #[test]
    fn test_invalid_consonant_starts() {
        let checker = SpellChecker::new();
        
        assert!(!checker.is_valid_consonant_start("zz"));
        assert!(!checker.is_valid_consonant_start("qq"));
    }

    #[test]
    fn test_valid_consonant_ends() {
        let checker = SpellChecker::new();
        
        assert!(checker.is_valid_consonant_end("n"));
        assert!(checker.is_valid_consonant_end("ng"));
        assert!(checker.is_valid_consonant_end("ch"));
        assert!(checker.is_valid_consonant_end("nh"));
    }

    #[test]
    fn test_remove_tones() {
        let checker = SpellChecker::new();
        
        assert_eq!(checker.remove_tones("hóa"), "hoa");
        assert_eq!(checker.remove_tones("hoá"), "hoa");
        assert_eq!(checker.remove_tones("thủy"), "thuy");
    }
}
