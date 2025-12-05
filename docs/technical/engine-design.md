# Thiết Kế Kỹ Thuật Chi Tiết - Vikey Engine

> Tài liệu thiết kế chi tiết cho Vietnamese Input Engine

## Tổng Quan

Engine là thành phần cốt lõi (core component) xử lý việc chuyển đổi input thành tiếng Việt.

## Yêu Cầu

### Yêu Cầu Chức Năng (Functional Requirements)

1. **Hỗ Trợ Các Kiểu Gõ (Input Methods)**

   - [ ] Telex (độ ưu tiên: CAO)
   - [ ] VNI (độ ưu tiên: TRUNG BÌNH)
   - [ ] VIQR (độ ưu tiên: THẤP)

2. **Xử Lý Văn Bản (Text Processing)**

   - [ ] Chuyển đổi thời gian thực (Real-time conversion)
   - [ ] Xử lý phím Backspace
   - [ ] Hỗ trợ Hoàn tác/Làm lại (Undo/Redo)

3. **Xử Lý Unicode**
   - [ ] Chuẩn hóa NFC
   - [ ] Chuẩn hóa NFD
   - [ ] Xử lý ký tự kết hợp (combining characters) đúng cách

### Yêu Cầu Phi Chức Năng (Non-Functional Requirements)

1. **Hiệu Năng (Performance)**

   - Độ trễ: < 10ms mỗi lần nhấn phím
   - Bộ nhớ: < 50MB
   - CPU: Tác động tối thiểu

2. **Độ Tin Cậy (Reliability)**
   - Không mất dữ liệu
   - Xử lý lỗi nhẹ nhàng (Graceful error handling)
   - Khôi phục trạng thái

## Kiến Trúc (Architecture)

### Thiết Kế Máy Trạng Thái (State Machine Design)

```
┌─────────────┐
│   Initial   │
└──────┬──────┘
       │ nhấn phím
       ▼
┌─────────────┐
│  Buffering  │◄─────┐
└──────┬──────┘      │
       │ phát hiện   │
       │ pattern     │ thêm input
       ▼             │
┌─────────────┐      │
│ Transform   │──────┘
└──────┬──────┘
       │ output
       ▼
┌─────────────┐
│   Commit    │
└─────────────┘
```

### Các Thành Phần (Components)

#### 1. Input Buffer

```rust
struct InputBuffer {
    chars: Vec<char>,
    cursor: usize,
}
```

**Trách nhiệm:**

- Lưu trữ các ký tự input
- Theo dõi vị trí con trỏ
- Xử lý backspace

#### 2. Pattern Matcher

```rust
trait PatternMatcher {
    fn matches(&self, input: &str) -> Option<Match>;
}
```

**Trách nhiệm:**

- Phát hiện các pattern input (aa, aw, v.v.)
- Trả về các quy tắc biến đổi
- Xử lý sự mơ hồ (ambiguity)

#### 3. Transformer

```rust
struct Transformer {
    rules: HashMap<String, String>,
}
```

**Trách nhiệm:**

- Áp dụng các quy tắc biến đổi
- Xử lý dấu thanh
- Chuẩn hóa Unicode

#### 4. State Manager

```rust
struct StateManager {
    current_state: State,
    history: Vec<State>,
}
```

**Trách nhiệm:**

- Theo dõi trạng thái hiện tại
- Hỗ trợ undo/redo
- Lưu giữ trạng thái (State persistence)

## Cấu Trúc Dữ Liệu (Data Structures)

### Quy Tắc Biến Đổi (Transformation Rules)

```rust
struct TransformRule {
    pattern: String,
    replacement: char,
    priority: u8,
}
```

### Bản Đồ Ký Tự Tiếng Việt (Vietnamese Character Map)

```rust
// Ký tự gốc
const VOWELS: &[(&str, char)] = &[
    ("aa", 'â'),
    ("aw", 'ă'),
    ("ee", 'ê'),
    // ...
];

// Dấu thanh
const TONES: &[(&str, char)] = &[
    ("s", '\u{0301}'), // sắc
    ("f", '\u{0300}'), // huyền
    // ...
];
```

## Thuật Toán (Algorithms)

### 1. Phát Hiện Pattern (Pattern Detection)

```
Input: "tieng"
Buffer: [t, i, e, n, g]

Bước 1: Kiểm tra "ie" → không khớp
Bước 2: Tiếp tục buffer
Bước 3: Chờ dấu thanh
```

### 2. Đặt Dấu Thanh (Tone Mark Placement)

**Quy tắc:**

1. Nguyên âm đơn: đặt trên nguyên âm đó
2. Nguyên âm đôi: tuân theo quy tắc tiếng Việt
   - oa, oe, uy: nguyên âm đầu
   - Khác: nguyên âm thứ hai

```rust
fn find_tone_position(word: &str) -> Option<usize> {
    // Implementation
}
```

### 3. Xử Lý Backspace

```
Input: "tiees" → "tiếs"
Backspace: "tiếs" → "tiê" → "tie" → "ti"

Cần theo dõi:
- Input gốc
- Output đã biến đổi
- Ánh xạ giữa chúng
```

## Các Trường Hợp Đặc Biệt (Edge Cases)

### 1. Từ Tiếng Anh

```
Input: "book"
KHÔNG NÊN biến đổi thành: "boôk"
Giải pháp: Kiểm tra từ điển hoặc xác nhận của người dùng
```

### 2. Input Mơ Hồ

```
Input: "hoa"
Có thể là: "hoa" (bông hoa) hoặc "hoà" (hòa bình)
Giải pháp: Hiển thị các ứng viên (candidates)
```

### 3. Trộn Lẫn Ngôn Ngữ

```
Input: "Hello xin chao"
Nên giữ nguyên: "Hello xin chào"
```

## Tối Ưu Hóa Hiệu Năng

### 1. Bảng Tra Cứu (Lookup Tables)

- Tính toán trước tất cả các biến đổi
- Sử dụng HashMap để tra cứu O(1)

### 2. Đánh Giá Lười (Lazy Evaluation)

- Chỉ biến đổi khi cần thiết
- Cache kết quả

### 3. Memory Pool

- Tái sử dụng buffer
- Tránh cấp phát bộ nhớ (allocations)

## Chiến Lược Kiểm Thử (Testing Strategy)

### Unit Tests

```rust
#[test]
fn test_telex_aa() {
    let mut engine = Engine::new(InputMethod::Telex);
    assert_eq!(engine.process("aa"), "â");
}
```

### Integration Tests

```rust
#[test]
fn test_full_word() {
    let mut engine = Engine::new(InputMethod::Telex);
    assert_eq!(engine.process("tieng"), "tiếng");
}
```

### Property-Based Tests

- Tính lũy đẳng (Idempotency)
- Tính khả nghịch (Reversibility - với undo)
- Tính hợp lệ Unicode (Unicode validity)

## Các Giai Đoạn Thực Hiện (Implementation Phases)

### Giai Đoạn 1: Telex Cơ Bản

- [ ] Biến đổi nguyên âm đơn giản
- [ ] Dấu thanh cơ bản
- [ ] Chưa xử lý backspace

### Giai Đoạn 2: Telex Hoàn Chỉnh

- [ ] Tất cả các biến đổi
- [ ] Hỗ trợ Backspace
- [ ] Undo/redo

### Giai Đoạn 3: Hỗ Trợ VNI

- [ ] Quy tắc VNI
- [ ] Chia sẻ máy trạng thái (Shared state machine)

### Giai Đoạn 4: Tối Ưu Hóa

- [ ] Tinh chỉnh hiệu năng
- [ ] Tối ưu hóa bộ nhớ
- [ ] Xử lý edge case

## Các Câu Hỏi Mở (Open Questions)

- [ ] Làm thế nào để tích hợp từ điển?
- [ ] Chúng ta có nên hỗ trợ quy tắc tùy chỉnh (custom rules)?
- [ ] Làm thế nào để xử lý tự động sửa lỗi (auto-correction)?
- [ ] Machine learning cho dự đoán?

---

**Trạng Thái**: Bản nháp - Cần review và hoàn thiện
