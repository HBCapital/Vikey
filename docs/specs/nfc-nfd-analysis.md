# Vấn Đề Bảng Mã NFC vs NFD Trong Tiếng Việt

## 1. Tổng Quan Vấn Đề

Tiếng Việt được biểu diễn trong Unicode với hai dạng chuẩn hóa (Normalization Forms):

| Dạng    | Tên đầy đủ                    | Đặc điểm                      | Ví dụ                                                          |
| ------- | ----------------------------- | ----------------------------- | -------------------------------------------------------------- |
| **NFC** | Normalization Form Composed   | Pre-composed (ký tự ghép sẵn) | `ắ` = U+1EAF (1 code point)                                    |
| **NFD** | Normalization Form Decomposed | Decomposed (ký tự tách rời)   | `ắ` = `a` (U+0061) + `̆` (U+0306) + `́` (U+0301) (3 code points) |

### 1.1 Tại Sao Có Hai Dạng?

- **NFC**: Được thiết kế cho **compatibility** với các encoding cũ (TCVN3, VNI). Mỗi ký tự là 1 code point.
- **NFD**: Được thiết kế cho **flexibility**. Base character + combining marks cho phép xử lý ngôn ngữ linh hoạt hơn.

### 1.2 Hệ Sinh Thái Hiện Tại

| Platform/System | Mặc định | Ghi chú                                 |
| --------------- | -------- | --------------------------------------- |
| **Windows**     | NFC      | Hầu hết apps sử dụng NFC                |
| **macOS**       | NFD      | HFS+ file system lưu NFD, APFS flexible |
| **Linux**       | NFC      | Hầu hết distros dùng NFC                |
| **Web/HTML**    | NFC      | W3C khuyến nghị NFC                     |
| **iOS**         | NFD      | NSString sử dụng NFD internally         |
| **Android**     | NFC      | Java String sử dụng NFC                 |

---

## 2. Chi Tiết Kỹ Thuật

### 2.1 So Sánh Byte Representation

**Ví dụ: Chữ "ắ"**

```
NFC (1 code point):
┌─────────────────────────────┐
│ U+1EAF = ắ                  │
│ UTF-8: E1 BA AF (3 bytes)   │
└─────────────────────────────┘

NFD (3 code points):
┌─────────────────────────────────────────────────────────┐
│ U+0061 (a) + U+0306 (breve) + U+0301 (acute)            │
│ UTF-8: 61 + CC 86 + CC 81 (5 bytes)                     │
└─────────────────────────────────────────────────────────┘
```

**Ví dụ: Chữ "ệ"**

```
NFC:
│ U+1EC7 = ệ
│ UTF-8: E1 BB 87 (3 bytes)

NFD:
│ U+0065 (e) + U+0302 (circumflex) + U+0323 (dot below)
│ UTF-8: 65 + CC 82 + CC A3 (5 bytes)
```

### 2.2 Vấn Đề Thực Tế

#### Problem 1: String Comparison Fails

```python
# Python example
nfc = "việt"  # NFC form
nfd = "việt"  # NFD form (looks same but different bytes)

print(nfc == nfd)  # FALSE! (không bằng nhau)
print(len(nfc))    # 4 (NFC)
print(len(nfd))    # 6 hoặc 8 (NFD, tùy số combining marks)
```

#### Problem 2: Search Không Tìm Thấy

```javascript
// User search "việt" (NFC)
// Database contains "việt" (NFD)
// Result: NOT FOUND
```

#### Problem 3: Backspace Behavior

```
NFC: 1 backspace xóa 1 ký tự "ắ"
NFD: 1 backspace xóa 1 combining mark, cần 3 lần để xóa "ắ"
```

#### Problem 4: Filename Issues

```bash
# macOS (NFD) vs Windows (NFC)
# File: "Việt Nam.txt"

# On macOS: Vie\u0302\u0323t Nam.txt (NFD)
# On Windows: Vi\u1EC7t Nam.txt (NFC)

# Copy từ macOS sang Windows -> có thể broken
```

---

## 3. Phương Án Xử Lý

### 3.1 Phương Án 1: Output NFC Only (Khuyến nghị cho Vikey)

**Mô tả**: Vikey luôn output NFC, bất kể platform.

**Ưu điểm**:

- ✅ Tương thích với đa số systems (Windows, Web, Android)
- ✅ Behavior nhất quán
- ✅ Backspace xóa đúng 1 ký tự
- ✅ Search/compare hoạt động đúng trong hầu hết apps

**Nhược điểm**:

- ⚠️ Có thể không match với existing NFD content trên macOS
- ⚠️ Cần normalize khi compare

**Implementation**:

```rust
use unicode_normalization::UnicodeNormalization;

fn output_vietnamese(text: &str) -> String {
    text.nfc().collect()
}
```

---

### 3.2 Phương Án 2: Platform-Aware Output

**Mô tả**: Detect platform và output form phù hợp.

| Platform | Output Form |
| -------- | ----------- |
| Windows  | NFC         |
| macOS    | NFD         |
| Linux    | NFC         |

**Ưu điểm**:

- ✅ "Native" feeling trên mỗi platform
- ✅ Consistent với existing content

**Nhược điểm**:

- ⚠️ Phức tạp hơn
- ⚠️ Cross-platform documents vẫn có issues
- ⚠️ macOS apps hiện đại đã support NFC

**Implementation**:

```rust
fn output_vietnamese(text: &str) -> String {
    #[cfg(target_os = "macos")]
    { text.nfd().collect() }

    #[cfg(not(target_os = "macos"))]
    { text.nfc().collect() }
}
```

---

### 3.3 Phương Án 3: User-Configurable

**Mô tả**: Cho phép user chọn NFC hoặc NFD trong settings.

**Use cases**:

- Developer làm việc với legacy systems
- User cần NFD cho specific applications
- Cross-platform users muốn consistency

**Implementation**:

```rust
enum NormalizationForm {
    NFC,
    NFD,
    Auto, // Platform-aware
}

fn output_vietnamese(text: &str, form: NormalizationForm) -> String {
    match form {
        NormalizationForm::NFC => text.nfc().collect(),
        NormalizationForm::NFD => text.nfd().collect(),
        NormalizationForm::Auto => {
            #[cfg(target_os = "macos")]
            { text.nfd().collect() }
            #[cfg(not(target_os = "macos"))]
            { text.nfc().collect() }
        }
    }
}
```

---

## 4. Hợp Nhất và Chuẩn Hóa

### 4.1 Normalize on Input

Khi nhận text từ bất kỳ nguồn nào, normalize về form chuẩn:

```rust
fn normalize_input(text: &str) -> String {
    text.nfc().collect() // Chuẩn hóa về NFC
}
```

### 4.2 Compare với Normalization

```rust
fn compare_vietnamese(a: &str, b: &str) -> bool {
    let a_normalized: String = a.nfc().collect();
    let b_normalized: String = b.nfc().collect();
    a_normalized == b_normalized
}
```

### 4.3 Search với Normalization

```rust
fn search_vietnamese(haystack: &str, needle: &str) -> bool {
    let h: String = haystack.nfc().collect();
    let n: String = needle.nfc().collect();
    h.contains(&n)
}
```

### 4.4 Canonical Equivalence Check

```rust
use unicode_normalization::is_nfc;

fn check_form(text: &str) -> &'static str {
    if is_nfc(text) { "NFC" }
    else { "Not NFC (possibly NFD or mixed)" }
}
```

---

## 5. Khuyến Nghị Cho Vikey

### 5.1 Default Behavior

```
┌────────────────────────────────────────┐
│ Vikey Output: Always NFC (Default)     │
│                                        │
│ Reason: Maximum compatibility          │
└────────────────────────────────────────┘
```

### 5.2 Configuration Option

```toml
# vikey.toml
[output]
normalization_form = "nfc"  # "nfc", "nfd", "auto"
```

### 5.3 Implementation Priority

1. **Phase 1 (MVP)**: Always output NFC
2. **Phase 2**: Add configuration option
3. **Phase 3**: Smart detection (analyze existing content)

### 5.4 Rust Crate

```toml
[dependencies]
unicode-normalization = "0.1"
```

---

## 6. Test Cases

### 6.1 NFC/NFD Equivalence

```rust
#[test]
fn test_nfc_nfd_equivalence() {
    let nfc = "việt";
    let nfd = "việt"; // NFD version

    assert!(compare_vietnamese(nfc, nfd)); // Should pass
}
```

### 6.2 Output Form

```rust
#[test]
fn test_output_is_nfc() {
    let output = output_vietnamese("viet");
    assert!(is_nfc(&output));
}
```

### 6.3 Backspace Behavior

```rust
#[test]
fn test_backspace_deletes_full_char() {
    let text = "việt";
    let after_backspace = &text[..text.len() - "t".len()];
    assert_eq!(after_backspace.chars().count(), 3); // "việ"
}
```

---

## 7. Tài Liệu Tham Khảo

- **UAX #15**: Unicode Normalization Forms - https://unicode.org/reports/tr15/
- **RFC 3629**: UTF-8 Encoding - https://tools.ietf.org/html/rfc3629
- **W3C**: Character Model for the World Wide Web - https://www.w3.org/TR/charmod-norm/
- **Apple**: String Programming Guide - https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Strings/

---

## 8. Kết Luận

| Vấn đề         | Giải pháp Vikey                     |
| -------------- | ----------------------------------- |
| Output form    | NFC (default), configurable         |
| Input handling | Normalize to NFC                    |
| Search/Compare | Always normalize before compare     |
| Backspace      | Delete full NFC code point          |
| Cross-platform | Consistent NFC across all platforms |

**Recommended**: Vikey nên sử dụng **NFC** làm default vì:

1. Tương thích với đa số applications
2. Backspace behavior tự nhiên (1 key = 1 char)
3. Search/compare hoạt động đúng
4. Là chuẩn được W3C và hầu hết frameworks khuyến nghị

---

**Last Updated**: 2025-12-05
