# Vikey Documentation

> Tài liệu kỹ thuật và phân tích cho dự án Vikey
>
> **Lưu ý:** Dự án sử dụng **Tiếng Việt** làm ngôn ngữ chính cho tài liệu và comment. Tiếng Anh chỉ được sử dụng cho code, tên file và các thuật ngữ chuyên môn không có từ tương đương.

## 📚 Cấu Trúc Documentation

```
docs/
├── ARCHITECTURE.md              # Kiến trúc tổng quan
│
├── analysis/                    # Phân tích các dự án tham khảo
│   ├── architecture-comparison.md  # So sánh kiến trúc
│   ├── openkey-analysis.md         # Phân tích OpenKey
│   └── unikey-analysis.md          # Phân tích UniKey
│
└── technical/                   # Tài liệu kỹ thuật chi tiết
    ├── engine-design.md            # Thiết kế Engine
    ├── input-methods.md            # Phương pháp gõ (Telex/VNI/VIQR)
    └── platform-apis.md            # Platform APIs
```

## 📖 Documents

### Tổng Quan

- **[ARCHITECTURE.md](ARCHITECTURE.md)** - Kiến trúc tổng quan của Vikey
  - Cấu trúc modules
  - Data flow
  - Design patterns

### Phân Tích (analysis/)

- **[architecture-comparison.md](analysis/architecture-comparison.md)** - So sánh kiến trúc

  - UniKey vs OpenKey vs Vikey
  - Điểm mạnh/yếu của từng approach
  - Quyết định thiết kế

- **[openkey-analysis.md](analysis/openkey-analysis.md)** - Phân tích OpenKey

  - Cấu trúc code (Engine.cpp, Vietnamese.cpp)
  - Backspace technique
  - State machine design
  - Lessons learned

- **[unikey-analysis.md](analysis/unikey-analysis.md)** - Phân tích UniKey
  - Keyboard hook implementation
  - 20+ years proven techniques
  - Feature set
  - Legacy code insights

### Kỹ Thuật (technical/)

- **[engine-design.md](technical/engine-design.md)** - Thiết kế Engine chi tiết

  - State machine
  - Buffer management
  - Transformation pipeline
  - Data structures

- **[input-methods.md](technical/input-methods.md)** - Phương pháp gõ tiếng Việt

  - Telex rules và examples
  - VNI rules và examples
  - VIQR rules và examples
  - So sánh và implementation notes

- **[platform-apis.md](technical/platform-apis.md)** - Platform APIs
  - Windows: TSF vs Keyboard Hook
  - macOS: IMKit vs Event Tap
  - Linux: IBus, Fcitx, X11, Wayland
  - Security considerations

## 🎯 Reading Guide

### Cho Developers Mới

1. Bắt đầu với [ARCHITECTURE.md](ARCHITECTURE.md)
2. Đọc [input-methods.md](technical/input-methods.md) để hiểu Vietnamese input
3. Xem [engine-design.md](technical/engine-design.md) cho implementation details

### Cho Contributors

1. Đọc [architecture-comparison.md](analysis/architecture-comparison.md)
2. Nghiên cứu [openkey-analysis.md](analysis/openkey-analysis.md)
3. Tham khảo [platform-apis.md](technical/platform-apis.md)

### Cho Researchers

1. So sánh [unikey-analysis.md](analysis/unikey-analysis.md) vs [openkey-analysis.md](analysis/openkey-analysis.md)
2. Đọc [architecture-comparison.md](analysis/architecture-comparison.md)
3. Xem implementation trong `../crates/`

## 🔗 External References

Các dự án tham khảo được lưu trong [`../references/`](../references/):

- OpenKey source code
- UniKey source code

### 📚 Technical Specs (Local Copies)

Các tài liệu kỹ thuật chi tiết được tổng hợp tại [`specs/`](specs/):

**Vietnamese & Input Methods:**

- **[Vietnamese Unicode](specs/vietnamese-unicode.md)** - Normalization, Tone Marks, NFC/NFD
- **[NFC vs NFD Analysis](specs/nfc-nfd-analysis.md)** - Compatibility strategies, unification solutions
- **[Telex & VNI Rules](specs/telex-vni-rules.md)** - Transformation rules, edge cases, test cases

**Platform APIs:**

- **[Windows Hook API](specs/windows-hook-api.md)** - Low-level Keyboard Hook, SendInput
- **[macOS Event Services](specs/macos-quartz-event-services.md)** - CGEventTap, Key Codes
- **[Linux X11 Input](specs/linux-x11-keysyms.md)** - XGrabKeyboard, XTest, Keysyms

**Implementation:**

- **[Rust FFI Patterns](specs/rust-ffi-patterns.md)** - FFI best practices, cross-platform abstraction

## 📝 Contributing to Docs

Khi thêm/update documentation:

1. **Đặt đúng folder**:

   - `analysis/` - Phân tích dự án khác
   - `technical/` - Technical specs của Vikey

2. **Format**:

   - Sử dụng Markdown
   - Thêm diagrams khi cần (Mermaid)
   - Code examples với syntax highlighting

3. **Links**:

   - Sử dụng relative links
   - Link đến source code khi relevant

4. **Update README**:
   - Thêm document mới vào README này

## 🚀 Quick Links

- [Vikey Core API](../crates/vikey-core/src/lib.rs)
- [Vietnamese Transformer](../crates/vikey-vietnamese/src/telex.rs)
- [Processor Logic](../crates/vikey-core/src/processor.rs)

---

**Last Updated**: 2025-12-05  
**Maintainers**: Vikey Contributors
