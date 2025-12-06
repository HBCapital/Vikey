use std::env;
use vikey_core::engine::Engine;
use vikey_vietnamese::VietnamesePlugin;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Filter out flags like --nocapture
    let input = args.iter()
        .skip(1)
        .find(|arg| !arg.starts_with("-"))
        .cloned();

    let input = if let Some(s) = input {
        s
    } else {
        println!("Usage: cargo test --test cli_telex -- <input_string> --nocapture");
        println!("Example: cargo test --test cli_telex -- \"aa dd ee\" --nocapture");
        return;
    };

    println!("Input: {}", input);

    // Initialize Engine
    let mut engine = Engine::new();

    // Register Vietnamese Plugin
    if let Err(e) = engine.register(Box::new(VietnamesePlugin::new())) {
        eprintln!("Failed to register plugin: {:?}", e);
        return;
    }

    // Set Language and Input Method
    if let Err(e) = engine.set_language("vietnamese") {
        eprintln!("Failed to set language: {:?}", e);
        return;
    }

    if let Err(e) = engine.set_input_method("telex") {
        eprintln!("Failed to set input method: {:?}", e);
        return;
    }

    println!("Processing...");
    for c in input.chars() {
        let action = engine.process(c);
        // In a real app, we would handle Action::Commit, etc.
        // Here we just print the buffer state after each key for demonstration.
        println!("Key: '{}' -> Buffer: \"{}\" | Action: {:?}", c, engine.buffer_content(), action);
    }

    println!("Final Buffer: {}", engine.buffer_content());
}
