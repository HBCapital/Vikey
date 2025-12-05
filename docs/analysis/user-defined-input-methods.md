# Phân Tích: User-Defined Input Methods

> Phân tích khả năng cho phép user tự định nghĩa bộ gõ trong Vikey

**Ngày phân tích**: 2025-12-05

> [!IMPORTANT] > **Quyết định cuối cùng**: Vikey sử dụng kiến trúc **Monorepo**, KHÔNG có plugin system.  
> Thêm ngôn ngữ/bộ gõ mới → Đóng góp vào repository chính qua PR.  
> Xem: [`plugin-vs-monorepo.md`](plugin-vs-monorepo.md) và [`CONTRIBUTING.md`](../../CONTRIBUTING.md)

---

## 1. Các Mức Độ Tùy Chỉnh

Có 2 mức độ user có thể tùy chỉnh bộ gõ:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Mức 1: Key Remapping (Đơn giản)                  │
│  User chỉ thay đổi mapping: 'aa' → 'â' thành 'qq' → 'â'             │
├─────────────────────────────────────────────────────────────────────┤
│                    Mức 2: Schema Definition (Trung bình)            │
│  User định nghĩa schema mới bằng file config (YAML/TOML)            │
│  Ví dụ: Tạo "My-Telex" với quy tắc riêng                            │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│         Ngôn Ngữ Mới → Đóng Góp Vào Monorepo (Nâng cao)             │
│                                                                     │
│  Developer muốn thêm ngôn ngữ mới:                                  │
│  1. Fork repository                                                 │
│  2. Tạo crate: crates/vikey-xxx/                                    │
│  3. Implement LanguagePlugin trait                                  │
│  4. Submit PR → Review → Merge                                      │
│  5. Trở thành Maintainer cho crate đó                               │
│                                                                     │
│  ❌ KHÔNG có plugin system vì lý do bảo mật                         │
│  ✅ Tất cả code trong cùng repository, có thể audit                 │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. Mức 1: Key Remapping

### 2.1 Mô Tả

User chỉ thay đổi phím → kết quả, giữ nguyên logic của bộ gõ gốc.

### 2.2 Ví Dụ Config

```toml
# ~/.config/vikey/remapping.toml

[base_on]
input_method = "telex"

[remap]
# Thay đổi phím gõ dấu
"aa" = "â"    # Mặc định
"qq" = "â"    # Thêm quy tắc mới
"ww" = "ư"    # Thay vì 'uw'

# Thay đổi phím gõ thanh
"ss" = "sắc"  # Thay vì 's'
"ff" = "huyền"

[disable]
# Tắt một số quy tắc mặc định
rules = ["w_to_uw"]
```

### 2.3 Implementation

```rust
pub struct RemappingLayer {
    base_method: Box<dyn InputMethodTrait>,
    key_map: HashMap<String, String>,
    disabled_rules: HashSet<String>,
}

impl InputMethodTrait for RemappingLayer {
    fn process(&mut self, key: char, buffer: &mut InputBuffer, lookup: &dyn LookupProvider) -> Action {
        // 1. Kiểm tra remapping
        if let Some(remapped) = self.check_remap(key, buffer) {
            return remapped;
        }

        // 2. Delegate to base method
        self.base_method.process(key, buffer, lookup)
    }
}
```

### 2.4 Ưu/Nhược Điểm

| Ưu điểm                          | Nhược điểm                         |
| -------------------------------- | ---------------------------------- |
| Đơn giản, dễ hiểu                | Giới hạn trong logic có sẵn        |
| Không cần biết lập trình         | Không tạo được bộ gõ mới hoàn toàn |
| An toàn (không có code thực thi) | Phụ thuộc vào base input method    |

---

## 3. Mức 2: Schema Definition (Như RIME)

### 3.1 Mô Tả

User định nghĩa schema mới bằng file YAML/TOML, bao gồm:

- Quy tắc mapping
- Quy tắc chính tả
- Dictionary (từ điển)
- Candidate selection rules

### 3.2 Ví Dụ Schema (Lấy cảm hứng từ RIME)

```yaml
# ~/.config/vikey/schemas/my-telex.yaml

schema:
  name: "My Telex"
  id: "my-telex"
  version: "1.0"
  author: "User"

engine:
  # Dựa trên Vietnamese rules
  base_language: "vietnamese"

  # Processors xử lý theo thứ tự
  processors:
    - ascii_composer # Xử lý ASCII
    - key_binder # Bind phím
    - speller # Xử lý chính tả
    - punctuator # Dấu câu
    - selector # Chọn candidate

# Định nghĩa spelling algebra (quy tắc chuyển đổi)
speller:
  alphabet: "abcdefghijklmnopqrstuvwxyz"
  delimiter: " "

  # Quy tắc chuyển đổi
  algebra:
    - "xform/aa/â/" # aa → â
    - "xform/aw/ă/" # aw → ă
    - "xform/ee/ê/" # ee → ê
    - "xform/oo/ô/" # oo → ô
    - "xform/ow/ơ/" # ow → ơ
    - "xform/uw/ư/" # uw → ư
    - "xform/dd/đ/" # dd → đ

  # Quy tắc thanh điệu (áp dụng sau mark)
  tones:
    - "xform/([aeiouy])s/$1́/" # sắc: as → á
    - "xform/([aeiouy])f/$1̀/" # huyền: af → à
    - "xform/([aeiouy])r/$1̉/" # hỏi: ar → ả
    - "xform/([aeiouy])x/$1̃/" # ngã: ax → ã
    - "xform/([aeiouy])j/$1̣/" # nặng: aj → ạ

# Từ điển (tuỳ chọn)
dictionary:
  name: "vietnamese-modern"
  enable_completion: true
  enable_sentence: false

# Key bindings
key_binder:
  bindings:
    - { when: "composing", accept: "Shift+Return", send: "Escape" }
    - { when: "has_menu", accept: "Tab", send: "Page_Down" }
```

### 3.3 Implementation

```rust
/// Schema được load từ YAML file
pub struct SchemaDefinition {
    pub name: String,
    pub id: String,
    pub base_language: Option<String>,
    pub algebra: Vec<AlgebraRule>,
    pub tones: Vec<ToneRule>,
    pub dictionary: Option<DictionaryConfig>,
}

/// Schema-based Input Method
pub struct SchemaInputMethod {
    schema: SchemaDefinition,
    compiled_rules: CompiledRules,  // Pre-compiled regex/patterns
    dictionary: Option<Box<dyn Dictionary>>,
}

impl InputMethodTrait for SchemaInputMethod {
    fn process(&mut self, key: char, buffer: &mut InputBuffer, lookup: &dyn LookupProvider) -> Action {
        // 1. Apply algebra rules
        let transformed = self.apply_algebra(buffer);

        // 2. Apply tone rules
        let with_tones = self.apply_tones(transformed);

        // 3. Validate with lookup
        if lookup.is_valid_char(with_tones.last_char()) {
            Action::Replace { ... }
        } else {
            Action::DoNothing
        }
    }
}
```

### 3.4 Ưu/Nhược Điểm

| Ưu điểm                             | Nhược điểm                      |
| ----------------------------------- | ------------------------------- |
| Mạnh mẽ, linh hoạt                  | Cần học cú pháp schema          |
| Không cần biết Rust                 | Compile-time cho regex/patterns |
| Dễ chia sẻ (chỉ là YAML file)       | Giới hạn trong DSL              |
| Đã proven bởi RIME (millions users) | Không thể thêm logic tùy ý      |

---

## 4. Thêm Ngôn Ngữ Mới → Đóng Góp Vào Monorepo

> **Lưu ý**: Đây KHÔNG phải plugin system. Tất cả code nằm trong cùng repository.

### 4.1 Tại Sao Không Dùng Plugin?

Xem phân tích chi tiết: [`plugin-vs-monorepo.md`](plugin-vs-monorepo.md)

**Tóm tắt**:

- ❌ Plugin = potential keylogger (có quyền truy cập mọi keystroke)
- ❌ Không kiểm soát được supply chain
- ❌ Khó đạt chứng nhận bảo mật cấp quốc gia
- ✅ Monorepo = auditability, security, certification

### 4.2 Quy Trình Đóng Góp

```
Developer muốn thêm ngôn ngữ mới (ví dụ: Tiếng Mường):

1. Mở Issue (RFC)
   └─> Đề xuất ngôn ngữ, Unicode block, input method

2. Community Review (30 ngày)
   └─> Core team + Language experts review

3. Fork Repository
   └─> git clone https://github.com/YOUR_USERNAME/vikey.git

4. Tạo Crate
   └─> mkdir -p crates/vikey-muong/src

5. Implement LanguagePlugin
   └─> impl LanguagePlugin for MuongPlugin { ... }

6. Viết Tests (coverage >= 80%)
   └─> cargo test

7. Submit PR
   └─> Review bởi Core Team + Language Experts

8. Merge vào Main
   └─> Bạn trở thành Maintainer cho crate đó
```

### 4.3 Ví Dụ Implementation

```rust
// crates/vikey-muong/src/lib.rs

use vikey_core::traits::{LanguagePlugin, InputMethodTrait, LookupProvider, LanguageRules};

pub struct MuongPlugin {
    lookup: MuongLookup,
    rules: MuongRules,
}

impl LanguagePlugin for MuongPlugin {
    fn name(&self) -> &str { "Tiếng Mường" }
    fn id(&self) -> &str { "muong" }

    fn input_methods(&self) -> Vec<&str> {
        vec!["telex-muong"]
    }

    fn create_input_method(&self, id: &str) -> Option<Box<dyn InputMethodTrait>> {
        match id {
            "telex-muong" => Some(Box::new(MuongTelexMethod::new())),
            _ => None,
        }
    }

    fn lookup(&self) -> &dyn LookupProvider { &self.lookup }
    fn rules(&self) -> &dyn LanguageRules { &self.rules }
}
```

### 4.4 Lợi Ích Của Monorepo

| Lợi Ích                 | Mô Tả                                             |
| ----------------------- | ------------------------------------------------- |
| **Auditability**        | Toàn bộ code có thể audit                         |
| **Security**            | Không có code bên ngoài load lúc runtime          |
| **Unified Build**       | Một lệnh build tất cả, tối ưu hóa toàn cục        |
| **Version Control**     | Tất cả components có cùng version                 |
| **Certification**       | Có thể đạt chứng nhận Common Criteria, FIPS       |
| **Trusted Maintainers** | Mỗi ngôn ngữ có maintainer riêng, được core trust |

---

## 5. So Sánh Với RIME

| Aspect            | RIME                   | Vikey                          |
| ----------------- | ---------------------- | ------------------------------ |
| **Config format** | YAML                   | TOML/YAML                      |
| **Algebra DSL**   | Custom (xform, derive) | Regex-based                    |
| **Dictionary**    | Text + compiled trie   | FST (fst crate)                |
| **Extensibility** | Lua scripts            | ❌ Không plugin, ✅ Contribute |
| **Security**      | Limited                | High (monorepo)                |
| **Performance**   | C++ native             | Rust native                    |

---

## 6. Đề Xuất Implementation

### 6.1 Phân Pha

| Phase | Mức độ                   | Độ ưu tiên | Effort |
| ----- | ------------------------ | ---------- | ------ |
| 1     | Key Remapping            | Cao        | Thấp   |
| 2     | Schema Definition (TOML) | Cao        | Trung  |

### 6.2 Cấu Trúc Thư Mục User

```
~/.config/vikey/
├── config.toml              # Main config
├── remapping.toml           # Level 1: Key remapping
└── schemas/                 # Level 2: Custom schemas
    ├── my-telex.yaml
    └── my-vni.yaml
```

### 6.3 Loading Priority

```rust
pub struct InputMethodLoader {
    // Priority: User schemas > User remapping > Built-in
    builtin_methods: HashMap<String, Box<dyn InputMethodTrait>>,
    user_schemas: HashMap<String, SchemaDefinition>,
    user_remapping: Option<RemappingConfig>,
}

impl InputMethodLoader {
    pub fn get_method(&self, id: &str) -> Option<Box<dyn InputMethodTrait>> {
        // 1. Check user schemas
        if let Some(schema) = self.user_schemas.get(id) {
            return Some(Box::new(SchemaInputMethod::from(schema)));
        }

        // 2. Fallback to built-in with optional remapping
        if let Some(method) = self.builtin_methods.get(id) {
            return self.wrap_with_remapping(method);
        }

        None
    }
}
```

---

## 7. Kết Luận

### User Định Nghĩa Được Tới Đâu?

| Mức độ              | User có thể làm                 | User KHÔNG thể làm  |
| ------------------- | ------------------------------- | ------------------- |
| **Level 1: Remap**  | Đổi phím trigger                | Thêm quy tắc mới    |
| **Level 2: Schema** | Định nghĩa quy tắc mới, từ điển | Thêm logic phức tạp |

### Developer Muốn Thêm Ngôn Ngữ Mới?

→ **Đóng góp vào Monorepo** qua PR, KHÔNG phải viết plugin.

Xem hướng dẫn: [`CONTRIBUTING.md`](../../CONTRIBUTING.md)

### Khuyến Nghị

1. **Phase 1 (MVP)**: Implement Level 1 (Remapping) - đáp ứng 80% nhu cầu
2. **Phase 2**: Implement Level 2 (Schema) - cho power users
3. **Ngôn ngữ mới**: Đóng góp vào monorepo, không plugin system

---

**Last Updated**: 2025-12-05
