// validation.rs - Vietnamese spelling validation (ported from ibus-bamboo)

use once_cell::sync::Lazy;

// Valid initial consonant sequences
pub static INITIAL_CONSONANTS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "b", "d", "đ", "g", "gh", "m", "n", "nh", "p", "ph", "r", "s", "t", "tr", "v", "z", "c",
        "h", "k", "kh", "qu", "th", "ch", "gi", "l", "ng", "ngh", "x", "đ",
        "l", // duplicates in source? kept for compatibility
        "h", "", // Empty initial is valid
    ]
});

// Valid vowel sequences
pub static VOWELS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "ê", "i", "ua", "uê", "uy", "y", "a", "iê", "oa", "uyê", "yê", "â", "ă", "e", "o", "oo",
        "ô", "ơ", "oe", "u", "ư", "uâ", "uô", "ươ", "uo", "ie", // Intermediate vowels
        "oă", "uơ", "ai", "ao", "au", "âu", "ay", "ây", "eo", "êu", "ia", "iêu", "iu", "oai",
        "oao", "oay", "oeo", "oi", "ôi", "ơi", "ưa", "uây", "ui", "ưi", "uôi", "ươi", "ươu", "ưu",
        "uya", "uyu", "yêu", "ă", "i",
    ]
});

// Valid final consonant sequences
pub static FINAL_CONSONANTS: Lazy<Vec<&'static str>> = Lazy::new(|| {
    vec![
        "ch", "nh", "c", "ng", "m", "n", "p", "t", "k", "c", "", // Empty final is valid
    ]
});

// CV Matrix (Consonant-Vowel compatibility)
static FIRST_CONSONANT_SEQS: &[&str] = &[
    "b d đ g gh m n nh p ph r s t tr v z", // Group 0
    "c h k kh qu th",                      // Group 1
    "ch gi l ng ngh x",                    // Group 2
    "đ l",                                 // Group 3
    "h",                                   // Group 4
];

static VOWEL_SEQS: &[&str] = &[
    "ê i ua uê uy y",                                                                 // Group 0
    "a iê oa uyê yê ie",                                                              // Group 1
    "â ă e o oo ô ơ oe u ư uâ uô ươ uo",                                              // Group 2
    "oă",                                                                             // Group 3
    "uơ",                                                                             // Group 4
    "ai ao au âu ay ây eo êu ia iêu iu oai oao oay oeo oi ôi ơi ưa uây ui ưi uôi ươi ươu ưu uya uyu yêu", // Group 5
    "ă",                                                                              // Group 6
    "i",                                                                              // Group 7
];

static LAST_CONSONANT_SEQS: &[&str] = &[
    "ch nh",   // Group 0
    "c ng",    // Group 1
    "m n p t", // Group 2
    "k",       // Group 3
    "c",       // Group 4
];

// cvMatrix from spelling.go
static CV_MATRIX: &[&[usize]] = &[
    &[0, 1, 2, 5],       // Group 0 initials can go with vowels in groups 0, 1, 2, 5
    &[0, 1, 2, 3, 4, 5], // Group 1 initials ...
    &[0, 1, 2, 3, 5],    // Group 2 initials ...
    &[6],                // Group 3 initials ...
    &[7],                // Group 4 initials ...
];

// vcMatrix from spelling.go
static VC_MATRIX: &[&[usize]] = &[
    &[0, 2],    // Group 0 vowels can go with finals in groups 0, 2
    &[0, 1, 2], // Group 1 vowels ...
    &[1, 2],    // Group 2 vowels ...
    &[1, 2],    // Group 3 vowels ...
    &[],        // Group 4 vowels ...
    &[],        // Group 5 vowels ...
    &[3],       // Group 6 vowels ...
    &[4],       // Group 7 vowels ...
];

/// Check if a syllable is valid according to Vietnamese spelling rules
pub fn is_valid_syllable(initial: &str, vowel: &str, final_cons: &str) -> bool {
    let initial = initial.to_lowercase();
    let vowel = vowel.to_lowercase();
    let final_cons = final_cons.to_lowercase();

    // 1. Find all initial group indices
    let initial_groups = find_group_indices(FIRST_CONSONANT_SEQS, &initial);

    // 2. Find all vowel group indices
    let vowel_groups = find_group_indices(VOWEL_SEQS, &vowel);

    // 3. Find all final group indices
    let final_groups = if final_cons.is_empty() {
        Vec::new()
    } else {
        find_group_indices(LAST_CONSONANT_SEQS, &final_cons)
    };

    // Validation rules:
    // - Initial must exist (or be empty)
    // - Vowel must exist
    // - Final must exist (or be empty)

    let initial_valid = initial.is_empty() || !initial_groups.is_empty();
    let vowel_valid = !vowel.is_empty() && !vowel_groups.is_empty();
    let final_valid = final_cons.is_empty() || !final_groups.is_empty();

    if !initial_valid || !vowel_valid || !final_valid {
        return false;
    }

    // Check CV compatibility
    let cv_valid = if initial.is_empty() {
        true
    } else {
        // Check if ANY pair of (initial_group, vowel_group) is valid
        let mut found = false;
        for &i_idx in &initial_groups {
            for &v_idx in &vowel_groups {
                if CV_MATRIX[i_idx].contains(&v_idx) {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        found
    };

    if !cv_valid {
        return false;
    }

    // Check VC compatibility
    let vc_valid = if final_cons.is_empty() {
        true
    } else {
        // Check if ANY pair of (vowel_group, final_group) is valid
        let mut found = false;
        for &v_idx in &vowel_groups {
            for &f_idx in &final_groups {
                if VC_MATRIX[v_idx].contains(&f_idx) {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        found
    };

    vc_valid
}

fn find_group_indices(groups: &[&str], target: &str) -> Vec<usize> {
    let mut indices = Vec::new();
    for (i, group) in groups.iter().enumerate() {
        for item in group.split_whitespace() {
            if item == target {
                indices.push(i);
                break; // Found in this group, move to next group
            }
        }
    }
    indices
}

use unicode_normalization::UnicodeNormalization;

/// Check if a string is a valid prefix of any string in the groups
pub fn is_valid_prefix(groups: &[&str], target: &str) -> bool {
    if target.is_empty() {
        return true;
    }

    let target_nfc: String = target.nfc().collect();

    for group in groups.iter() {
        for item in group.split_whitespace() {
            // Assume items in groups are already NFC
            if item.starts_with(&target_nfc) {
                return true;
            }
        }
    }
    false
}

/// Check if syllable structure is permissible (components are valid prefixes)
pub fn is_permissible_syllable(initial: &str, vowel: &str, final_cons: &str) -> bool {
    let initial = initial.to_lowercase();
    let vowel = vowel.to_lowercase();
    let final_cons = final_cons.to_lowercase();

    // Check if components are valid prefixes
    if !is_valid_prefix(&INITIAL_CONSONANTS, &initial) {
        eprintln!("Invalid initial: {} {:?}", initial, initial.as_bytes());
        return false;
    }
    if !is_valid_prefix(&VOWELS, &vowel) {
        eprintln!("Invalid vowel: {} {:?}", vowel, vowel.as_bytes());
        return false;
    }
    if !is_valid_prefix(&FINAL_CONSONANTS, &final_cons) {
        eprintln!("Invalid final: {} {:?}", final_cons, final_cons.as_bytes());
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_syllables() {
        assert!(is_valid_syllable("t", "oa", "n")); // toán
        assert!(is_valid_syllable("ng", "uyê", "n")); // nguyên
        assert!(is_valid_syllable("", "a", "")); // a
        assert!(is_valid_syllable("b", "a", "")); // ba
    }

    #[test]
    fn test_invalid_syllables() {
        // k + a is actually allowed by the matrix (Group 1 initial + Group 1 vowel)
        // So we test something else that is invalid

        // Let's try a fake vowel "z".
        assert!(!is_valid_syllable("b", "z", ""));

        // Let's try a fake initial "w" (if not in list).
        // w is not in INITIAL_CONSONANTS.
        assert!(!is_valid_syllable("w", "a", ""));
    }

    #[test]
    fn test_permissible_syllables() {
        // q is not a valid initial, but is a prefix of qu
        assert!(is_permissible_syllable("q", "", ""));

        // ng is valid initial
        assert!(is_permissible_syllable("ng", "", ""));

        // ngh is valid initial
        assert!(is_permissible_syllable("ngh", "", ""));

        // ngk is NOT valid initial or prefix
        assert!(!is_permissible_syllable("ngk", "", ""));

        // ang is valid vowel+final
        assert!(is_permissible_syllable("", "a", "ng"));

        // angk is NOT valid final
        assert!(!is_permissible_syllable("", "a", "ngk"));

        // dương components
        assert!(is_permissible_syllable("d", "ươ", "ng"));
    }
}
