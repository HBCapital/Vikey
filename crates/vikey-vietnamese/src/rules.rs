// rules.rs - Vietnamese language rules (tone placement, etc.)

use crate::syllable::Tone;

/// Tone placement style
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToneStyle {
    /// New Style (default): hoà, thuý
    New,
    /// Old Style: hòa, thúy
    Old,
}

impl Default for ToneStyle {
    fn default() -> Self {
        Self::New
    }
}

/// Place tone mark on vowel according to Vietnamese rules
pub fn place_tone(vowel: &str, tone: Tone, style: ToneStyle) -> String {
    let chars: Vec<char> = vowel.chars().collect();
    if chars.is_empty() {
        return vowel.to_string();
    }
    
    // If only 1 character, apply tone to it
    if chars.len() == 1 {
        return apply_tone_to_char(chars[0], tone).to_string();
    }
    
    // If 2 characters
    if chars.len() == 2 {
        let v1 = chars[0].to_lowercase().next().unwrap();
        let v2 = chars[1].to_lowercase().next().unwrap();
        
        // Special cases for "oa", "oe", "uy"
        // New Style: place on 2nd char (hoà, hoè, thuý)
        // Old Style: place on 1st char (hòa, hòe, thúy)
        if (v1 == 'o' && (v2 == 'a' || v2 == 'e' || v2 == 'ă')) || 
           (v1 == 'u' && v2 == 'y') {
            match style {
                ToneStyle::New => {
                    // Place on 2nd char
                    let mut res = String::new();
                    res.push(chars[0]);
                    res.push(apply_tone_to_char(chars[1], tone));
                    return res;
                }
                ToneStyle::Old => {
                    // Place on 1st char (except uy -> úy is standard even in old style sometimes, but strict old style is úy)
                    let mut res = String::new();
                    res.push(apply_tone_to_char(chars[0], tone));
                    res.push(chars[1]);
                    return res;
                }
            }
        }
        
        // Default rule for 2 chars: place on 1st char (e.g., "ai" -> "ái", "eu" -> "ếu")
        // UNLESS it's "uo" (uơ) or "ie" (iê) - but those are usually represented as single modified vowels in some contexts, 
        // but here we deal with raw chars.
        // Actually, "ia", "ua", "ưa" -> tone on 1st char (ía, úa, ứa)
        // "iê", "uô", "ươ" -> tone on 2nd char (iế, uố, ướ)
        
        // Let's check if the second char is a modified vowel that usually takes tone
        if v2 == 'ê' || v2 == 'ô' || v2 == 'ơ' {
             let mut res = String::new();
             res.push(chars[0]);
             res.push(apply_tone_to_char(chars[1], tone));
             return res;
        }
        
        // Default: 1st char
        let mut res = String::new();
        res.push(apply_tone_to_char(chars[0], tone));
        res.push(chars[1]);
        return res;
    }
    
    // If 3 characters (e.g., "yeu", "uoi")
    if chars.len() == 3 {
        // Usually place on 2nd char (yếu, uối)
        let mut res = String::new();
        res.push(chars[0]);
        res.push(apply_tone_to_char(chars[1], tone));
        res.push(chars[2]);
        return res;
    }
    
    // Fallback: place on 1st char
    let mut res = String::new();
    res.push(apply_tone_to_char(chars[0], tone));
    for i in 1..chars.len() {
        res.push(chars[i]);
    }
    res
}

/// Helper to apply tone to a single char (copied from syllable.rs for now, should refactor)
fn apply_tone_to_char(ch: char, tone: Tone) -> char {
    // This duplicates logic in syllable.rs. We should move it here or make it public there.
    // For now, I'll duplicate to get it working, then refactor.
    use crate::syllable::apply_tone_to_char as apply_tone_internal;
    apply_tone_internal(ch, tone)
}
