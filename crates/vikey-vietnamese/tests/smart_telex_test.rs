// tests/smart_telex_test.rs

use vikey_vietnamese::rules::{place_tone, ToneStyle};
use vikey_vietnamese::Tone;

#[test]
fn test_smart_tone_placement_oa() {
    // hoa + s -> hoá (New Style)
    assert_eq!(place_tone("oa", Tone::Acute, ToneStyle::New), "oá");
    
    // hoa + s -> hóa (Old Style)
    assert_eq!(place_tone("oa", Tone::Acute, ToneStyle::Old), "hóa".chars().skip(1).collect::<String>());
}

#[test]
fn test_smart_tone_placement_oe() {
    // hoe + s -> hoé (New Style)
    assert_eq!(place_tone("oe", Tone::Acute, ToneStyle::New), "oé");
}

#[test]
fn test_smart_tone_placement_uy() {
    // thuy + s -> thuý (New Style)
    assert_eq!(place_tone("uy", Tone::Acute, ToneStyle::New), "uý");
    
    // thuy + s -> thúy (Old Style)
    assert_eq!(place_tone("uy", Tone::Acute, ToneStyle::Old), "úy");
}

#[test]
fn test_smart_tone_placement_ie() {
    // hieu + s -> hiếu (3 chars, tone on middle)
    // Input to place_tone is already modified: "iêu"
    assert_eq!(place_tone("iêu", Tone::Acute, ToneStyle::New), "iếu");
}

#[test]
fn test_smart_tone_placement_uo() {
    // cuoi + s -> cuối (3 chars, tone on middle)
    // Input to place_tone is already modified: "uôi"
    assert_eq!(place_tone("uôi", Tone::Acute, ToneStyle::New), "uối");
}

#[test]
fn test_smart_tone_placement_ai() {
    // hai + s -> hái (2 chars, tone on first)
    assert_eq!(place_tone("ai", Tone::Acute, ToneStyle::New), "ái");
}
