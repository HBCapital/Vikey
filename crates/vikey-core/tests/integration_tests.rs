// integration_tests.rs - Integration tests for vikey-core

use vikey_core::{Action, Engine, InputBuffer};

#[test]
fn test_engine_creation() {
    let engine = Engine::new();
    assert!(engine.buffer_content().is_empty());
}

#[test]
fn test_buffer_operations() {
    let mut buffer = InputBuffer::new();

    buffer.push('a', true);
    assert_eq!(buffer.len(), 1);
    assert_eq!(buffer.to_string(), "a");

    buffer.push('b', true);
    assert_eq!(buffer.len(), 2);
    assert_eq!(buffer.to_string(), "ab");

    buffer.pop();
    assert_eq!(buffer.len(), 1);
    assert_eq!(buffer.to_string(), "a");

    buffer.clear();
    assert_eq!(buffer.len(), 0);
    assert!(buffer.is_empty());
}

#[test]
fn test_action_types() {
    let action1 = Action::DoNothing;
    assert!(matches!(action1, Action::DoNothing));

    let action2 = Action::Commit("test".to_string());
    assert!(matches!(action2, Action::Commit(_)));

    let action3 = Action::Replace {
        backspace_count: 1,
        text: "new".to_string(),
    };
    assert!(matches!(action3, Action::Replace { .. }));
}
