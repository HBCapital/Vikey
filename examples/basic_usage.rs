// examples/basic_usage.rs - Example of using Vikey with Vietnamese

use vikey_core::{Engine, Action};
use vikey_vietnamese::VietnamesePlugin;

fn main() {
    println!("=== Vikey Basic Usage Example ===\n");
    
    // Create engine
    let mut engine = Engine::new();
    
    // Register Vietnamese plugin
    let plugin = VietnamesePlugin::new();
    engine.register(Box::new(plugin)).expect("Failed to register Vietnamese plugin");
    
    // Set language and input method
    engine.set_language("vietnamese").expect("Failed to set language");
    engine.set_input_method("telex").expect("Failed to set input method");
    
    println!("✅ Engine initialized with Vietnamese Telex");
    println!("Current buffer: '{}'", engine.buffer_content());
    println!();
    
    // Test 1: Simple character
    println!("Test 1: Type 'a'");
    let action = engine.process('a');
    println!("  Action: {:?}", action);
    println!("  Buffer: '{}'", engine.buffer_content());
    println!();
    
    // Test 2: Backspace
    println!("Test 2: Backspace");
    let action = engine.process_backspace();
    println!("  Action: {:?}", action);
    println!("  Buffer: '{}'", engine.buffer_content());
    println!();
    
    // Test 3: Reset
    println!("Test 3: Reset");
    engine.reset();
    println!("  Buffer: '{}'", engine.buffer_content());
    println!();
    
    println!("=== Example Complete ===");
}
