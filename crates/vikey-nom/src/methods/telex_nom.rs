//! Vikey Nôm - Telex-Nôm Input Method
//!
//! Gõ phiên âm Quốc ngữ bằng Telex → Hiển thị candidate list chữ Nôm.

use crate::dictionary::NomDictionary;
use crate::types::NomCandidate;
use vikey_core::traits::{InputMethodTrait, LookupProvider};
use vikey_core::types::Action;
use vikey_core::InputBuffer;

/// Telex-Nôm Input Method
///
/// Người dùng gõ phiên âm Quốc ngữ (ví dụ: "nguoi", "viet")
/// và chọn chữ Nôm tương ứng từ candidate list.
pub struct TelexNomMethod {
    /// Dictionary tra cứu
    dictionary: NomDictionary,

    /// Buffer nội bộ cho phiên âm đang gõ
    syllable_buffer: String,

    /// Candidates hiện tại
    candidates: Vec<NomCandidate>,

    /// Index của candidate đang chọn
    selected_index: usize,
}

impl TelexNomMethod {
    /// Tạo TelexNomMethod mới
    pub fn new() -> Self {
        Self {
            dictionary: NomDictionary::new(),
            syllable_buffer: String::new(),
            candidates: Vec::new(),
            selected_index: 0,
        }
    }

    /// Lấy candidates hiện tại
    pub fn candidates(&self) -> &[NomCandidate] {
        &self.candidates
    }

    /// Chọn candidate theo index
    pub fn select(&mut self, index: usize) -> Option<char> {
        self.candidates.get(index).map(|c| c.character)
    }

    /// Cập nhật candidates dựa trên syllable buffer
    fn update_candidates(&mut self) {
        self.candidates = self.dictionary.lookup(&self.syllable_buffer);
        self.selected_index = 0;
    }
}

impl Default for TelexNomMethod {
    fn default() -> Self {
        Self::new()
    }
}

impl InputMethodTrait for TelexNomMethod {
    fn name(&self) -> &str {
        "Telex-Nôm"
    }

    fn id(&self) -> &str {
        "telex-nom"
    }

    fn process(
        &mut self,
        key: char,
        buffer: &mut InputBuffer,
        _lookup: &dyn LookupProvider,
    ) -> Action {
        // Xử lý phím đặc biệt
        match key {
            // Space hoặc Enter: commit candidate đã chọn
            ' ' | '\n' => {
                if let Some(candidate) = self.candidates.get(self.selected_index) {
                    let nom_char = candidate.character;
                    let backspace_count = self.syllable_buffer.len();

                    self.syllable_buffer.clear();
                    self.candidates.clear();
                    buffer.clear();

                    return Action::Replace {
                        backspace_count,
                        text: nom_char.to_string(),
                    };
                }

                // Không có candidate, commit buffer thường
                if !self.syllable_buffer.is_empty() {
                    let text = self.syllable_buffer.clone();
                    self.syllable_buffer.clear();
                    self.candidates.clear();
                    buffer.clear();

                    return Action::Commit(text);
                }

                Action::DoNothing
            }

            // Số 1-9: chọn candidate
            '1'..='9' => {
                let index = (key as usize) - ('1' as usize);
                if let Some(candidate) = self.candidates.get(index) {
                    let nom_char = candidate.character;
                    let backspace_count = self.syllable_buffer.len();

                    self.syllable_buffer.clear();
                    self.candidates.clear();
                    buffer.clear();

                    return Action::Replace {
                        backspace_count,
                        text: nom_char.to_string(),
                    };
                }
                Action::DoNothing
            }

            // Ký tự Latin: thêm vào syllable buffer
            c if c.is_ascii_alphabetic() => {
                self.syllable_buffer.push(c.to_ascii_lowercase());
                buffer.push(c, c.is_lowercase());
                self.update_candidates();

                Action::Replace {
                    backspace_count: 0,
                    text: c.to_string(),
                }
            }

            // Các ký tự khác: passthrough
            _ => Action::DoNothing,
        }
    }

    fn process_backspace(&mut self, buffer: &mut InputBuffer) -> Action {
        if self.syllable_buffer.pop().is_some() {
            buffer.pop();
            self.update_candidates();
            Action::Replace {
                backspace_count: 1,
                text: String::new(),
            }
        } else {
            Action::DoNothing
        }
    }

    fn reset(&mut self) {
        self.syllable_buffer.clear();
        self.candidates.clear();
        self.selected_index = 0;
    }

    fn can_undo(&self, _buffer: &InputBuffer) -> bool {
        false // Nôm input không hỗ trợ undo
    }

    fn undo(&mut self, _buffer: &mut InputBuffer) -> Action {
        Action::DoNothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telex_nom_info() {
        let method = TelexNomMethod::new();
        assert_eq!(method.id(), "telex-nom");
        assert!(method.name().contains("Nôm"));
    }

    #[test]
    fn test_candidates_lookup() {
        let mut method = TelexNomMethod::new();

        // Simulate typing "nguoi"
        method.syllable_buffer = "nguoi".to_string();
        method.update_candidates();

        assert!(!method.candidates().is_empty());
    }
}
