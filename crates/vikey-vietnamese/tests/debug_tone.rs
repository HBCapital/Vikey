// Debug test for tone marks

use vikey_core::{Action, Engine};
use vikey_vietnamese::VietnamesePlugin;

#[test]
fn debug_tone_simple() {
    let mut engine = Engine::new();
    engine.register(Box::new(VietnamesePlugin::new())).unwrap();
    engine.set_language("vietnamese").unwrap();
    engine.set_input_method("telex_v2").unwrap();

    // Type 'h'
    let a1 = engine.process('h');
    println!("After 'h': {:?}", a1);

    // Type 'o'
    let a2 = engine.process('o');
    println!("After 'o': {:?}", a2);

    // Type 'a'
    let a3 = engine.process('a');
    println!("After 'a': {:?}", a3);

    // Type 's' (tone)
    let a4 = engine.process('s');
    println!("After 's': {:?}", a4);

    // Check final output
    let mut output = String::new();
    for action in [a1, a2, a3, a4] {
        match action {
            Action::Replace {
                backspace_count,
                ref text,
            } => {
                for _ in 0..backspace_count {
                    output.pop();
                }
                output.push_str(text);
            }
            Action::Commit(ref text) => {
                output.push_str(text);
            }
            _ => {}
        }
    }

    println!("Final output: '{}'", output);
    println!("Expected: 'hoá'");

    assert_eq!(output, "hoá");
}
