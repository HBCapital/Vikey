//! Vikey Nôm - Dictionary
//!
//! FST-based dictionary cho tra cứu Quốc ngữ → Nôm.

use crate::types::NomCandidate;

/// Dictionary tra cứu Nôm
/// 
/// Sử dụng FST (Finite State Transducer) để tra cứu hiệu quả.
pub struct NomDictionary {
    // TODO: FST implementation
    // fst: fst::Map<Vec<u8>>,
}

impl NomDictionary {
    /// Tạo dictionary rỗng
    pub fn new() -> Self {
        Self {}
    }
    
    /// Load dictionary từ file
    pub fn load(_path: &str) -> Result<Self, DictionaryError> {
        // TODO: Load FST from file
        Ok(Self::new())
    }
    
    /// Tra cứu candidates cho một phiên âm
    /// 
    /// # Arguments
    /// * `quoc_ngu` - Phiên âm Quốc ngữ (ví dụ: "nguoi", "viet", "nam")
    /// 
    /// # Returns
    /// Danh sách các ký tự Nôm tương ứng, sắp xếp theo tần suất
    pub fn lookup(&self, quoc_ngu: &str) -> Vec<NomCandidate> {
        // TODO: Implement FST lookup
        // Placeholder: một số ví dụ cứng
        match quoc_ngu.to_lowercase().as_str() {
            "nguoi" => vec![
                NomCandidate {
                    character: '𡦂',
                    quoc_ngu: "người".to_string(),
                    pinyin: None,
                    meaning: Some("người, con người".to_string()),
                    frequency: 95,
                    category: crate::types::NomCategory::Native,
                },
            ],
            "viet" => vec![
                NomCandidate {
                    character: '越',
                    quoc_ngu: "Việt".to_string(),
                    pinyin: Some("yuè".to_string()),
                    meaning: Some("Việt Nam".to_string()),
                    frequency: 90,
                    category: crate::types::NomCategory::SinoVietnamese,
                },
            ],
            "nom" => vec![
                NomCandidate {
                    character: '喃',
                    quoc_ngu: "Nôm".to_string(),
                    pinyin: Some("nán".to_string()),
                    meaning: Some("chữ Nôm".to_string()),
                    frequency: 85,
                    category: crate::types::NomCategory::SinoVietnamese,
                },
            ],
            _ => Vec::new(),
        }
    }
    
    /// Kiểm tra phiên âm có trong dictionary không
    pub fn contains(&self, quoc_ngu: &str) -> bool {
        !self.lookup(quoc_ngu).is_empty()
    }
}

impl Default for NomDictionary {
    fn default() -> Self {
        Self::new()
    }
}

/// Lỗi khi thao tác với dictionary
#[derive(Debug)]
pub enum DictionaryError {
    /// Không tìm thấy file
    FileNotFound(String),
    
    /// Lỗi đọc file
    IoError(std::io::Error),
    
    /// Định dạng file không hợp lệ
    InvalidFormat,
}

impl std::fmt::Display for DictionaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DictionaryError::FileNotFound(path) => write!(f, "Dictionary file not found: {}", path),
            DictionaryError::IoError(e) => write!(f, "IO error: {}", e),
            DictionaryError::InvalidFormat => write!(f, "Invalid dictionary format"),
        }
    }
}

impl std::error::Error for DictionaryError {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lookup() {
        let dict = NomDictionary::new();
        
        let results = dict.lookup("nguoi");
        assert!(!results.is_empty());
        assert_eq!(results[0].character, '𡦂');
    }
    
    #[test]
    fn test_contains() {
        let dict = NomDictionary::new();
        
        assert!(dict.contains("nguoi"));
        assert!(dict.contains("viet"));
        assert!(!dict.contains("xyz123"));
    }
}
