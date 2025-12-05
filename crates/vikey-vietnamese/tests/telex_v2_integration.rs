// Integration tests for TelexMethodV2

use vikey_core::{Engine, Action};
use vikey_vietnamese::VietnamesePlugin;

fn create_engine_v2() -> Engine {
    let mut engine = Engine::new();
    engine.register(Box::new(VietnamesePlugin::new())).unwrap();
    engine.set_language("vietnamese").unwrap();
    engine.set_input_method("telex_v2").unwrap();
    engine
}

fn process_string(engine: &mut Engine, input: &str) -> String {
    let mut output = String::new();
    
    for c in input.chars() {
        let action = engine.process(c);
        match action {
            Action::Commit(text) => {
                output.push_str(&text);
            }
            Action::Replace { backspace_count, text } => {
                // Remove last N characters
                for _ in 0..backspace_count {
                    output.pop();
                }
                output.push_str(&text);
            }
            Action::DoNothing => {}
        }
    }
    
    output
}

#[test]
fn test_v2_simple_word() {
    let mut engine = create_engine_v2();
    let result = process_string(&mut engine, "hoa");
    println!("Input: 'hoa' -> Output: '{}'", result);
    assert_eq!(result, "hoa");
}

#[test]
fn test_v2_tone_mark() {
    let mut engine = create_engine_v2();
    let result = process_string(&mut engine, "hoas");
    println!("Input: 'hoas' -> Output: '{}'", result);
    assert_eq!(result, "hoá");
}

#[test]
fn test_v2_circumflex() {
    let mut engine = create_engine_v2();
    let result = process_string(&mut engine, "aa");
    println!("Input: 'aa' -> Output: '{}'", result);
    assert_eq!(result, "â");
}

#[test]
fn test_v2_breve() {
    let mut engine = create_engine_v2();
    let result = process_string(&mut engine, "aw");
    println!("Input: 'aw' -> Output: '{}'", result);
    assert_eq!(result, "ă");
}

#[test]
fn test_v2_horn_o() {
    let mut engine = create_engine_v2();
    let result = process_string(&mut engine, "ow");
    println!("Input: 'ow' -> Output: '{}'", result);
    assert_eq!(result, "ơ");
}

#[test]
fn test_v2_horn_u() {
    let mut engine = create_engine_v2();
    let result = process_string(&mut engine, "uw");
    println!("Input: 'uw' -> Output: '{}'", result);
    assert_eq!(result, "ư");
}

#[test]
fn test_v2_d_stroke() {
    let mut engine = create_engine_v2();
    let result = process_string(&mut engine, "dd");
    println!("Input: 'dd' -> Output: '{}'", result);
    assert_eq!(result, "đ");
}

#[test]
fn test_v2_complex_word() {
    let mut engine = create_engine_v2();
    let result = process_string(&mut engine, "tuowng");
    println!("Input: 'tuowng' -> Output: '{}'", result);
    assert_eq!(result, "tương");
}

#[test]
fn test_v2_complex_with_tone() {
    let mut engine = create_engine_v2();
    let result = process_string(&mut engine, "tuowngs");
    println!("Input: 'tuowngs' -> Output: '{}'", result);
    assert_eq!(result, "tướng");
}

#[test]
fn test_v2_all_tones() {
    let mut engine = create_engine_v2();
    
    let tests = vec![
        ("as", "á"),  // sắc
        ("af", "à"),  // huyền
        ("ar", "ả"),  // hỏi
        ("ax", "ã"),  // ngã
        ("aj", "ạ"),  // nặng
    ];
    
    for (input, expected) in tests {
        engine.reset();
        let result = process_string(&mut engine, input);
        println!("Input: '{}' -> Output: '{}' (expected: '{}')", input, result, expected);
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}

#[test]
fn test_v2_word_building() {
    let mut engine = create_engine_v2();
    
    // Build word character by character
    assert_eq!(process_string(&mut engine, "t"), "t");
    engine.reset();
    
    assert_eq!(process_string(&mut engine, "tu"), "tu");
    engine.reset();
    
    assert_eq!(process_string(&mut engine, "tuo"), "tuo");
    engine.reset();
    
    assert_eq!(process_string(&mut engine, "tuow"), "tuơ");  // ow→ơ
    engine.reset();
    
    assert_eq!(process_string(&mut engine, "tuown"), "tuơn");
    engine.reset();
    
    assert_eq!(process_string(&mut engine, "tuowng"), "tương");
    engine.reset();
    
    assert_eq!(process_string(&mut engine, "tuowngs"), "tướng"); // tone
}
