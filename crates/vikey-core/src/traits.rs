//! Vikey Core - Traits Module
//!
//! Defines the core traits for the plugin system, enabling
//! multi-language support (Vietnamese, Nôm, Tai, Cham, etc.)

use crate::buffer::InputBuffer;
use crate::types::{Action, CharInfo};

/// Trait cho một Language Plugin
///
/// Mỗi ngôn ngữ (Vietnamese, Nôm, Tày-Nùng...) implement trait này.
pub trait LanguagePlugin: Send + Sync {
    /// Tên ngôn ngữ (hiển thị cho user)
    fn name(&self) -> &str;

    /// ID duy nhất (internal, ví dụ: "vietnamese", "nom", "tai")
    fn id(&self) -> &str;

    /// Danh sách input methods mà ngôn ngữ này hỗ trợ
    fn input_methods(&self) -> Vec<&str>;

    /// Tạo một instance của input method theo ID
    fn create_input_method(&self, id: &str) -> Option<Box<dyn InputMethodTrait>>;

    /// Lookup provider cho ngôn ngữ này
    fn lookup(&self) -> &dyn LookupProvider;

    /// Quy tắc chính tả cho ngôn ngữ này
    fn rules(&self) -> &dyn LanguageRules;
}

/// Trait cho một Input Method (Telex, VNI, NomPinyin...)
///
/// Input method là cách gõ, không phải ngôn ngữ.
/// Ví dụ: Telex và VNI đều dùng để gõ tiếng Việt.
pub trait InputMethodTrait: Send + Sync {
    /// Tên bộ gõ (hiển thị)
    fn name(&self) -> &str;

    /// ID duy nhất
    fn id(&self) -> &str;

    /// Xử lý một keystroke
    ///
    /// # Arguments
    /// * `key` - Ký tự vừa gõ
    /// * `buffer` - Buffer hiện tại
    /// * `lookup` - Bảng tra cứu ký tự
    ///
    /// # Returns
    /// Action cần thực hiện (Replace, Commit, DoNothing)
    fn process(
        &mut self,
        key: char,
        buffer: &mut InputBuffer,
        lookup: &dyn LookupProvider,
    ) -> Action;

    /// Xử lý phím Backspace
    fn process_backspace(&mut self, buffer: &mut InputBuffer) -> Action;

    /// Reset trạng thái internal của input method
    fn reset(&mut self);

    /// Có thể undo transformation gần nhất không?
    fn can_undo(&self, buffer: &InputBuffer) -> bool;

    /// Thực hiện undo transformation gần nhất
    fn undo(&mut self, buffer: &mut InputBuffer) -> Action;
}

/// Trait cho Lookup Provider (bảng tra cứu ký tự)
///
/// Mỗi ngôn ngữ có bảng tra riêng với thông tin khác nhau.
pub trait LookupProvider: Send + Sync {
    /// Tra cứu thông tin ký tự
    fn lookup(&self, c: char) -> CharInfo;

    /// Kiểm tra ký tự có thuộc ngôn ngữ này không
    fn is_valid_char(&self, c: char) -> bool;

    /// Kiểm tra ký tự có phải nguyên âm không
    fn is_vowel(&self, c: char) -> bool;

    /// Kiểm tra ký tự có phải phụ âm không
    fn is_consonant(&self, c: char) -> bool;

    /// Kiểm tra ký tự có phải separator không (space, enter...)
    fn is_separator(&self, c: char) -> bool;
}

/// Trait cho Language Rules (quy tắc chính tả)
pub trait LanguageRules: Send + Sync {
    /// Kiểm tra từ có hợp lệ không
    fn is_valid_word(&self, word: &str) -> bool;

    /// Kiểm tra âm tiết có hợp lệ không
    fn is_valid_syllable(&self, syllable: &str) -> bool;

    /// Gợi ý sửa chính tả
    fn suggest(&self, word: &str) -> Vec<String>;

    /// Tìm vị trí đặt dấu thanh (cho các ngôn ngữ có thanh điệu)
    fn find_tone_position(&self, syllable: &str) -> Option<usize>;
}

/// Default implementation cho LanguageRules
///
/// Các ngôn ngữ đơn giản có thể dùng implementation mặc định này.
pub struct DefaultLanguageRules;

impl LanguageRules for DefaultLanguageRules {
    fn is_valid_word(&self, _word: &str) -> bool {
        true // Accept all by default
    }

    fn is_valid_syllable(&self, _syllable: &str) -> bool {
        true
    }

    fn suggest(&self, _word: &str) -> Vec<String> {
        Vec::new() // No suggestions by default
    }

    fn find_tone_position(&self, _syllable: &str) -> Option<usize> {
        None // No tone by default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_rules() {
        let rules = DefaultLanguageRules;
        assert!(rules.is_valid_word("anything"));
        assert!(rules.suggest("test").is_empty());
    }
}
