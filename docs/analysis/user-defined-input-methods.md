# Phân Tích: User-Defined Input Methods

> Phân tích khả năng cho phép user tự định nghĩa bộ gõ trong Vikey

**Ngày phân tích**: 2025-12-05

> [!IMPORTANT] > **Quyết định cuối cùng**: Vikey sử dụng kiến trúc **Monorepo**, KHÔNG có plugin system.  
> Level 3 (Full Plugin) đã được thay thế bằng **Contribute to Monorepo**.  
> Xem: [`plugin-vs-monorepo.md`](plugin-vs-monorepo.md)

---

## 1. Các Mức Độ Tùy Chỉnh

Có 3 mức độ user có thể tùy chỉnh bộ gõ:

```
┌─────────────────────────────────────────────────────────────────────┐
│                    Mức 1: Key Remapping (Đơn giản)                  │
│  User chỉ thay đổi mapping: 'aa' → 'â' thành 'qq' → 'â'             │
├─────────────────────────────────────────────────────────────────────┤
│                    Mức 2: Schema Definition (Trung bình)            │
│  User định nghĩa schema mới bằng file config (YAML/TOML)            │
│  Ví dụ: Tạo "My-Telex" với quy tắc riêng                            │
├─────────────────────────────────────────────────────────────────────┤
│                    Mức 3: Full Plugin (Nâng cao)                    │
│  User viết code Rust/WASM để tạo một bộ gõ hoàn toàn mới            │
│  Ví dụ: Tạo bộ gõ cho ngôn ngữ chưa được hỗ trợ                     │
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

## 4. Mức 3: Full Plugin (WASM/Native)

### 4.1 Mô Tả

User viết code (Rust hoặc compile sang WASM) để implement `InputMethodTrait` hoàn toàn.

### 4.2 Ví Dụ Plugin Interface

```rust
// Crate: vikey-plugin-sdk

/// Plugin metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub id: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub license: String,

    /// Entry point (WASM hoặc shared library)
    pub entry_point: EntryPoint,

    /// Dependencies
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug)]
pub enum EntryPoint {
    /// WebAssembly module
    Wasm { path: PathBuf },

    /// Native shared library (.dll/.so/.dylib)
    Native { path: PathBuf },
}

/// Plugin phải implement trait này
pub trait VikeyPlugin: Send + Sync {
    /// Thông tin plugin
    fn manifest(&self) -> &PluginManifest;

    /// Tạo LanguagePlugin instance
    fn create_language_plugin(&self) -> Box<dyn LanguagePlugin>;
}
```

### 4.3 Ví Dụ WASM Plugin

```rust
// user-wasm-plugin/src/lib.rs

use vikey_plugin_sdk::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct MyCustomPlugin;

#[wasm_bindgen]
impl MyCustomPlugin {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self { Self }

    pub fn process(&mut self, key: char, buffer_json: &str) -> String {
        // Custom logic hoàn toàn
        // ...
        serde_json::to_string(&action).unwrap()
    }
}
```

### 4.4 Ưu/Nhược Điểm

| Ưu điểm                   | Nhược điểm                        |
| ------------------------- | --------------------------------- |
| Tự do hoàn toàn           | Cần biết lập trình Rust/WASM      |
| Có thể tạo bộ gõ mới 100% | Security risk (sandboxing cần)    |
| Full access to Vikey APIs | Compilation/distribution phức tạp |
| Native performance        | Maintenance burden on user        |

---

## 5. So Sánh Với RIME

| Aspect            | RIME                   | Vikey (Đề xuất) |
| ----------------- | ---------------------- | --------------- |
| **Config format** | YAML                   | TOML/YAML       |
| **Algebra DSL**   | Custom (xform, derive) | Regex-based     |
| **Dictionary**    | Text + compiled trie   | FST (fst crate) |
| **Plugin system** | Lua scripts            | WASM + Native   |
| **Sandboxing**    | Limited                | WASM sandbox    |
| **Performance**   | C++ native             | Rust native     |

### 5.1 RIME Algebra DSL (Tham khảo)

```yaml
# RIME syntax
speller:
  algebra:
    - derive/^([zcs])h/$1/ # zh, ch, sh → z, c, s
    - xform/^([nl])v/$1ü/ # lv, nv → lü, nü
    - abbrev/^([a-z]).+$/$1/ # Viết tắt: beijng → b
```

### 5.2 Vikey Đề Xuất (Đơn giản hóa)

```toml
# Vikey syntax (dễ đọc hơn)
[[algebra]]
pattern = "aa"
replace = "â"
type = "mark"

[[algebra]]
pattern = "([aeiouyăâêôơư])s$"
replace = "$1́"
type = "tone"
regex = true
```

---

## 6. Đề Xuất Implementation

### 6.1 Phân Pha

| Phase | Mức độ                   | Độ ưu tiên | Effort |
| ----- | ------------------------ | ---------- | ------ |
| 1     | Key Remapping            | Cao        | Thấp   |
| 2     | Schema Definition (TOML) | Cao        | Trung  |
| 3     | WASM Plugin              | Thấp       | Cao    |

### 6.2 Cấu Trúc Thư Mục User

```
~/.config/vikey/
├── config.toml              # Main config
├── remapping.toml           # Level 1: Key remapping
├── schemas/                 # Level 2: Custom schemas
│   ├── my-telex.yaml
│   └── my-vni.yaml
└── plugins/                 # Level 3: Full plugins
    ├── my-plugin.wasm
    └── my-plugin.manifest.toml
```

### 6.3 Loading Priority

```rust
pub struct InputMethodLoader {
    // Priority: User schemas > User remapping > Built-in
    builtin_methods: HashMap<String, Box<dyn InputMethodTrait>>,
    user_schemas: HashMap<String, SchemaDefinition>,
    user_remapping: Option<RemappingConfig>,
    user_plugins: Vec<Box<dyn VikeyPlugin>>,
}

impl InputMethodLoader {
    pub fn get_method(&self, id: &str) -> Option<Box<dyn InputMethodTrait>> {
        // 1. Check user plugins first
        if let Some(plugin) = self.user_plugins.find(id) {
            return Some(plugin.create_input_method());
        }

        // 2. Check user schemas
        if let Some(schema) = self.user_schemas.get(id) {
            return Some(Box::new(SchemaInputMethod::from(schema)));
        }

        // 3. Fallback to built-in with optional remapping
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

| Mức độ              | User có thể làm                 | User KHÔNG thể làm                |
| ------------------- | ------------------------------- | --------------------------------- |
| **Level 1: Remap**  | Đổi phím trigger                | Thêm quy tắc mới                  |
| **Level 2: Schema** | Định nghĩa quy tắc mới, từ điển | Thêm logic phức tạp               |
| **Level 3: Plugin** | Mọi thứ (full control)          | Chỉ giới hạn bởi security sandbox |

### Khuyến Nghị

1. **Phase 1 (MVP)**: Implement Level 1 (Remapping) - đáp ứng 80% nhu cầu
2. **Phase 2**: Implement Level 2 (Schema) - cho power users
3. **Phase 3**: Implement Level 3 (WASM) - cho developers

### "Bỏ qua" vs "Chuyển sang bộ gõ mới"

- **Level 1-2**: User _mở rộng_ bộ gõ có sẵn, không thay thế hoàn toàn
- **Level 3**: User có thể tạo bộ gõ **hoàn toàn mới**, bỏ qua built-in methods

---

**Last Updated**: 2025-12-05
