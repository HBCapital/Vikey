# Đóng Góp Cho Vikey

Cảm ơn bạn đã quan tâm đến việc đóng góp cho Vikey! 🎉

---

## 1. Kiến Trúc: Monorepo (Không Plugin)

> **Quan trọng**: Vikey sử dụng kiến trúc **Monorepo**, KHÔNG có plugin system.

Lý do:

- ✅ **Auditability**: Toàn bộ code có thể audit
- ✅ **Security**: Không có code bên ngoài load lúc runtime
- ✅ **Certification**: Có thể đạt chứng nhận bảo mật cấp quốc gia

Xem chi tiết: [`docs/analysis/plugin-vs-monorepo.md`](docs/analysis/plugin-vs-monorepo.md)

---

## 2. Đóng Góp Ngôn Ngữ Mới

### Bước 1: Mở Issue (RFC)

Trước khi viết code, hãy mở một Issue với template RFC:

```markdown
## RFC: Thêm hỗ trợ [Tên Ngôn Ngữ]

### Thông tin ngôn ngữ

- Tên: [Tên ngôn ngữ]
- Unicode Block: [U+XXXX–U+XXXX]
- Số người sử dụng: [Ước tính]
- Tài liệu tham khảo: [Links]

### Input Method đề xuất

- [Mô tả cách gõ]

### Người đề xuất

- [Tên/Liên hệ]
- [Kinh nghiệm với ngôn ngữ này]
```

### Bước 2: Fork và Tạo Crate

```bash
# Fork repository
git clone https://github.com/YOUR_USERNAME/vikey.git
cd vikey

# Tạo branch mới
git checkout -b feature/add-language-xxx

# Tạo crate mới
mkdir -p crates/vikey-xxx/src
```

### Bước 3: Implement LanguagePlugin

```rust
// crates/vikey-xxx/src/lib.rs

use vikey_core::traits::{LanguagePlugin, InputMethodTrait, LookupProvider, LanguageRules};

pub struct XxxPlugin {
    lookup: XxxLookup,
    rules: XxxRules,
}

impl LanguagePlugin for XxxPlugin {
    fn name(&self) -> &str { "Tiếng XXX" }
    fn id(&self) -> &str { "xxx" }

    fn input_methods(&self) -> Vec<&str> {
        vec!["telex-xxx"]
    }

    fn create_input_method(&self, id: &str) -> Option<Box<dyn InputMethodTrait>> {
        match id {
            "telex-xxx" => Some(Box::new(TelexXxxMethod::new())),
            _ => None,
        }
    }

    fn lookup(&self) -> &dyn LookupProvider { &self.lookup }
    fn rules(&self) -> &dyn LanguageRules { &self.rules }
}
```

### Bước 4: Viết Tests (Coverage >= 80%)

```rust
#[test]
fn test_basic_input() {
    let plugin = XxxPlugin::new();
    let method = plugin.create_input_method("telex-xxx").unwrap();
    // Test cases
}
```

### Bước 5: Submit PR

Tạo Pull Request với checklist:

- [ ] Implement LanguagePlugin trait
- [ ] Implement InputMethodTrait
- [ ] Tests coverage >= 80%
- [ ] README.md cho crate
- [ ] API documentation

### Bước 6: Trở Thành Maintainer

Sau khi PR được merge, bạn sẽ:

- Được thêm vào **CODEOWNERS** cho crate của mình
- Có **write access** cho crate đó
- Chịu trách nhiệm review PRs liên quan

---

## 3. Quy Trình Đóng Góp Chung

### 3.1 Fork Repository

```bash
git clone https://github.com/your-username/vikey.git
cd vikey
```

### 3.2 Tạo Branch Mới

```bash
git checkout -b feature/amazing-feature
```

Quy tắc đặt tên branch:

- `feature/` - Tính năng mới
- `fix/` - Sửa lỗi
- `docs/` - Cập nhật tài liệu
- `refactor/` - Refactor code

### 3.3 Development Environment

```bash
# Build project
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt
```

### 3.4 Commit Message Format

Sử dụng [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(xxx): add support for XXX language
fix(input): resolve keyboard hook memory leak
docs(readme): update installation instructions
```

---

## 4. Code Review Process

| Giai đoạn          | Thời gian    |
| ------------------ | ------------ |
| Initial review     | 3-5 ngày     |
| Revision (nếu cần) | 1-2 tuần     |
| Final approval     | 2-3 ngày     |
| **Tổng cộng**      | **2-4 tuần** |

### Reviewers

| Reviewer             | Trách nhiệm                          |
| -------------------- | ------------------------------------ |
| **Core Team**        | Code quality, security, architecture |
| **Language Experts** | Linguistic accuracy                  |
| **CI/CD**            | Tests, lint, build                   |

---

## 5. Coding Standards

### Rust Style

```rust
// ✅ Good
pub fn process_key(&mut self, key: char) -> Action {
    match key {
        'a'..='z' => self.handle_letter(key),
        _ => Action::DoNothing,
    }
}

// ❌ Bad - Don't panic in production
fn lookup(&self, key: &str) -> char {
    self.dict.get(key).unwrap()  // BAD!
}
```

### Documentation

```rust
/// Process a single keystroke
///
/// # Arguments
/// * `key` - The character that was typed
///
/// # Returns
/// An `Action` indicating what should happen
pub fn process(&mut self, key: char) -> Action { ... }
```

---

## 6. Báo Cáo Lỗi

Sử dụng [GitHub Issues](https://github.com/HBCapital/vikey/issues):

```markdown
**Mô tả lỗi**
Mô tả ngắn gọn về lỗi.

**Cách tái hiện**

1. Bước 1
2. Bước 2

**Môi trường**

- OS: [e.g., Windows 11]
- Vikey version: [e.g., 0.1.0]
```

---

## 7. License

Bằng việc đóng góp, bạn đồng ý rằng contributions của bạn sẽ được licensed dưới **Apache License 2.0**.

---

**Cảm ơn bạn đã đóng góp cho Vikey!** 🇻🇳
