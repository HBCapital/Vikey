//! Integration tests for Vikey Core

use vikey_core::{Action, Config, InputMethod, VikeyCore};

#[test]
fn test_telex_double_a() {
    let mut core = VikeyCore::new(Config::default());

    let action1 = core.process_key('a');
    assert_eq!(action1, Action::DoNothing);
    assert_eq!(core.buffer_content(), "a");

    let action2 = core.process_key('a');
    assert_eq!(
        action2,
        Action::Replace {
            backspace_count: 1,
            text: "â".to_string()
        }
    );
    assert_eq!(core.buffer_content(), "â");
}

#[test]
fn test_telex_word_hoa() {
    let mut core = VikeyCore::new(Config::default());

    core.process_key('h');
    core.process_key('o');
    let action = core.process_key('a');

    assert_eq!(action, Action::DoNothing);
    assert_eq!(core.buffer_content(), "hoa");
}

#[test]
fn test_telex_breve_aw() {
    let mut core = VikeyCore::new(Config::default());

    core.process_key('a');
    let action = core.process_key('w');

    assert_eq!(
        action,
        Action::Replace {
            backspace_count: 1,
            text: "ă".to_string()
        }
    );
    assert_eq!(core.buffer_content(), "ă");
}

#[test]
fn test_telex_word_tuong() {
    let mut core = VikeyCore::new(Config::default());

    core.process_key('t');
    core.process_key('u');
    core.process_key('w'); // ư
    core.process_key('o');
    core.process_key('w'); // ơ
    core.process_key('n');
    core.process_key('g');

    assert_eq!(core.buffer_content(), "tương");
}

#[test]
fn test_separator_commits() {
    let mut core = VikeyCore::new(Config::default());

    core.process_key('a');
    core.process_key('a'); // â

    let action = core.process_key(' ');
    assert_eq!(action, Action::Commit("â".to_string()));
    assert_eq!(core.buffer_content(), "");
}

#[test]
fn test_backspace() {
    let mut core = VikeyCore::new(Config::default());

    core.process_key('a');
    core.process_key('a'); // â

    let action = core.process_backspace();
    assert_eq!(
        action,
        Action::Replace {
            backspace_count: 1,
            text: String::new()
        }
    );
    assert_eq!(core.buffer_content(), "");
}

#[test]
fn test_uppercase_handling() {
    let mut core = VikeyCore::new(Config::default());

    core.process_key('A');
    let action = core.process_key('a');

    assert_eq!(
        action,
        Action::Replace {
            backspace_count: 1,
            text: "Â".to_string()
        }
    );
    assert_eq!(core.buffer_content(), "Â");
}

#[test]
fn test_dd_to_d_stroke() {
    let mut core = VikeyCore::new(Config::default());

    core.process_key('d');
    let action = core.process_key('d');

    assert_eq!(
        action,
        Action::Replace {
            backspace_count: 1,
            text: "đ".to_string()
        }
    );
    assert_eq!(core.buffer_content(), "đ");
}

// Tone mark tests
#[test]
fn test_tone_simple_word() {
    let mut core = VikeyCore::new(Config::default());

    // "hoa" + "s" -> "hoá"
    core.process_key('h');
    core.process_key('o');
    core.process_key('a');
    let action = core.process_key('s');

    assert_eq!(
        action,
        Action::Replace {
            backspace_count: 2,
            text: "oá".to_string()
        }
    );
    assert_eq!(core.buffer_content(), "hoá");
}

#[test]
fn test_tone_complex_word() {
    let mut core = VikeyCore::new(Config::default());

    // "chuyen" -> "chuyển"
    core.process_key('c');
    core.process_key('h');
    core.process_key('u');
    core.process_key('y');
    core.process_key('e');
    core.process_key('e'); // ê
    core.process_key('n');
    core.process_key('r'); // tone hỏi

    assert_eq!(core.buffer_content(), "chuyển");
}

#[test]
fn test_tone_with_marks() {
    let mut core = VikeyCore::new(Config::default());

    // "tuong" -> "tương"
    core.process_key('t');
    core.process_key('u');
    core.process_key('w'); // ư
    core.process_key('o');
    core.process_key('w'); // ơ
    core.process_key('n');
    core.process_key('g');
    core.process_key('s'); // tone sắc

    assert_eq!(core.buffer_content(), "tướng");
}

#[test]
fn test_all_tones() {
    let mut core = VikeyCore::new(Config::default());

    // Sắc
    core.process_key('a');
    core.process_key('s');
    assert_eq!(core.buffer_content(), "á");
    core.reset();

    // Huyền
    core.process_key('a');
    core.process_key('f');
    assert_eq!(core.buffer_content(), "à");
    core.reset();

    // Hỏi
    core.process_key('a');
    core.process_key('r');
    assert_eq!(core.buffer_content(), "ả");
    core.reset();

    // Ngã
    core.process_key('a');
    core.process_key('x');
    assert_eq!(core.buffer_content(), "ã");
    core.reset();

    // Nặng
    core.process_key('a');
    core.process_key('j');
    assert_eq!(core.buffer_content(), "ạ");
}
