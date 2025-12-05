# Phân Tích Khả Năng Mở Rộng Kiến Trúc Vikey

> Đánh giá kiến trúc hiện tại và đề xuất cải tiến để hỗ trợ Chữ Nôm và các ngôn ngữ dân tộc thiểu số.

**Ngày phân tích**: 2025-12-05

> [!IMPORTANT]  
> **Quyết định cuối cùng**: Vikey sử dụng kiến trúc **Monorepo** (không plugin).  
> Tất cả ngôn ngữ là crates trong cùng repository, đóng góp qua PR.  
> Xem: [`plugin-vs-monorepo.md`](plugin-vs-monorepo.md) và [`CONTRIBUTING.md`](../../CONTRIBUTING.md)

---

## 1. Phân Tích Kiến Trúc Hiện Tại

### 1.1 Cấu Trúc Hiện Tại

```
crates/
├── vikey-core/          # Core engine
│   ├── types.rs         # ⚠️ VẤN ĐỀ: InputMethod enum cố định (Telex, VNI, VIQR)
│   ├── buffer.rs
│   ├── processor.rs     # ⚠️ VẤN ĐỀ: Logic xử lý gắn chặt với Vietnamese
│   └── lookup.rs        # ⚠️ VẤN ĐỀ: Lookup table chỉ cho tiếng Việt
│
└── vikey-vietnamese/    # Vietnamese transformers
    ├── telex.rs
    ├── vni.rs
    └── viqr.rs
```

### 1.2 Các Vấn Đề Khi Scale

| Vấn Đề                            | Vị Trí      | Tác Động                     |
| --------------------------------- | ----------- | ---------------------------- |
| **InputMethod là enum cố định**   | `types.rs`  | Thêm Nôm phải sửa core code  |
| **ToneType/MarkType cố định**     | `types.rs`  | Không phù hợp cho Thái, Chăm |
| **LookupTable cho Vietnamese**    | `lookup.rs` | Mỗi ngôn ngữ cần bảng riêng  |
| **Không có Language abstraction** | Toàn bộ     | Khó thêm ngôn ngữ mới        |

---

## 2. Phân Loại Khái Niệm

### 2.1 Phân Biệt: Script vs Language vs InputMethod

```
┌─────────────────────────────────────────────────────────────────┐
│                        SCRIPT (Hệ Chữ)                          │
│  [Latin] [Han/Nôm] [Tai Viet] [Pahawh Hmong] [Cham]            │
└─────────────────────────┬───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                      LANGUAGE (Ngôn Ngữ)                        │
│  [Vietnamese] [Nôm (chữ Nôm)] [Tày-Nùng] [H'Mông] [Chăm]       │
└─────────────────────────┬───────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    INPUT METHOD (Bộ Gõ)                         │
│  Vietnamese: [Telex] [VNI] [VIQR]                               │
│  Nôm:        [Telex-Nôm] [Pinyin-Nôm]                           │
│  Tày-Nùng:   [Telex-Thái]                                       │
└─────────────────────────────────────────────────────────────────┘
```

**Kết luận**:

- **Script**: Hệ thống ký tự (Latin, CJK, Tai Viet...)
- **Language**: Ngôn ngữ cụ thể sử dụng script
- **InputMethod**: Cách gõ (mapping từ keystroke → character)

**Telex, VNI, Nôm nên được coi là INPUT METHOD**, không phải Language.

---

## 3. Kiến Trúc Mới Đề Xuất

### 3.1 Layered Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                      vikey-core (Generic)                       │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │  Traits: Transformer, LookupProvider, LanguageRules       │  │
│  │  Types: Action, Buffer, KeyEvent (generic)                │  │
│  │  Engine: Processor (delegates to Language Plugin)         │  │
│  └───────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                                │
         ┌──────────────────────┼──────────────────────┐
         ▼                      ▼                      ▼
┌─────────────────┐   ┌─────────────────┐   ┌─────────────────┐
│ vikey-vietnamese│   │ vikey-nom       │   │ vikey-tai       │
│                 │   │                 │   │                 │
│ - TelexMethod   │   │ - NomTelex      │   │ - TaiVietTelex  │
│ - VNIMethod     │   │ - NomPinyin     │   │                 │
│ - VietRules     │   │ - NomDict       │   │ - TaiRules      │
│ - VietLookup    │   │ - NomLookup     │   │ - TaiLookup     │
└─────────────────┘   └─────────────────┘   └─────────────────┘
```

### 3.2 Core Traits (Plugin System)

```rust
// vikey-core/src/traits.rs

/// Trait cho một Language Plugin
pub trait LanguagePlugin: Send + Sync {
    /// Tên ngôn ngữ (hiển thị)
    fn name(&self) -> &str;

    /// ID duy nhất (internal)
    fn id(&self) -> &str;

    /// Danh sách input methods hỗ trợ
    fn input_methods(&self) -> Vec<Box<dyn InputMethod>>;

    /// Lookup table cho ngôn ngữ này
    fn lookup(&self) -> &dyn LookupProvider;

    /// Quy tắc ngôn ngữ (spelling, validation)
    fn rules(&self) -> &dyn LanguageRules;
}

/// Trait cho một Input Method (Telex, VNI, NomPinyin...)
pub trait InputMethod: Send + Sync {
    /// Tên bộ gõ
    fn name(&self) -> &str;

    /// ID duy nhất
    fn id(&self) -> &str;

    /// Xử lý một keystroke
    fn process(&mut self, key: char, buffer: &mut Buffer, lookup: &dyn LookupProvider) -> Action;

    /// Có thể undo không?
    fn can_undo(&self, buffer: &Buffer) -> bool;

    /// Thực hiện undo
    fn undo(&mut self, buffer: &mut Buffer) -> Action;
}

/// Trait cho Lookup Provider (bảng tra cứu ký tự)
pub trait LookupProvider: Send + Sync {
    /// Tra cứu thông tin ký tự
    fn lookup(&self, c: char) -> CharInfo;

    /// Kiểm tra ký tự có thuộc ngôn ngữ này không
    fn is_valid_char(&self, c: char) -> bool;
}

/// Trait cho Language Rules (quy tắc chính tả)
pub trait LanguageRules: Send + Sync {
    /// Kiểm tra từ có hợp lệ không
    fn is_valid_word(&self, word: &str) -> bool;

    /// Gợi ý sửa chính tả
    fn suggest(&self, word: &str) -> Vec<String>;
}
```

### 3.3 Cấu Trúc Crates Mới

```
vikey/
├── Cargo.toml
├── crates/
│   │
│   ├── vikey-core/              # 🔵 Generic Core (Platform Agnostic)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── traits.rs        # NEW: LanguagePlugin, InputMethod, LookupProvider
│   │   │   ├── buffer.rs        # Generic buffer (no language-specific logic)
│   │   │   ├── engine.rs        # NEW: Orchestrator using plugins
│   │   │   ├── types.rs         # Generic types (Action, KeyEvent)
│   │   │   └── registry.rs      # NEW: Plugin registry
│   │   └── Cargo.toml
│   │
│   ├── vikey-vietnamese/        # 🟢 Vietnamese Plugin
│   │   ├── src/
│   │   │   ├── lib.rs           # Implements LanguagePlugin
│   │   │   ├── plugin.rs        # VietnamesePlugin struct
│   │   │   ├── lookup.rs        # Vietnamese lookup table
│   │   │   ├── rules.rs         # Vietnamese spelling rules
│   │   │   ├── methods/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── telex.rs     # TelexMethod: impl InputMethod
│   │   │   │   ├── vni.rs       # VNIMethod: impl InputMethod
│   │   │   │   └── viqr.rs      # VIQRMethod: impl InputMethod
│   │   │   └── types.rs         # Vietnamese-specific types (ToneType, MarkType)
│   │   └── Cargo.toml
│   │
│   ├── vikey-nom/               # 🟡 Chữ Nôm Plugin
│   │   ├── src/
│   │   │   ├── lib.rs           # Implements LanguagePlugin
│   │   │   ├── plugin.rs        # NomPlugin struct
│   │   │   ├── lookup.rs        # Nôm lookup (CJK Extension B,C,D)
│   │   │   ├── dictionary.rs    # FST dictionary for Nôm
│   │   │   ├── methods/
│   │   │   │   ├── mod.rs
│   │   │   │   ├── telex_nom.rs # TelexNom: quốc ngữ → chữ Nôm
│   │   │   │   └── pinyin_nom.rs# PinyinNom: pinyin → chữ Nôm
│   │   │   └── types.rs         # Nôm-specific types
│   │   └── Cargo.toml
│   │
│   ├── vikey-tai/               # 🟠 Chữ Thái (Tày-Nùng) Plugin
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── plugin.rs
│   │   │   ├── lookup.rs        # Tai Viet lookup (U+AA80–U+AADF)
│   │   │   └── methods/
│   │   │       ├── mod.rs
│   │   │       └── telex_tai.rs
│   │   └── Cargo.toml
│   │
│   ├── vikey-cham/              # 🔴 Chữ Chăm Plugin
│   │   └── ...
│   │
│   ├── vikey-hmong/             # 🟣 Chữ H'Mông Plugin
│   │   └── ...
│   │
│   ├── vikey-broker/            # 🟤 Engine Service
│   │   └── ...
│   │
│   └── platform/                # 🟠 Platform Bridges
│       ├── vikey-windows-tsf/
│       ├── vikey-macos-imk/
│       └── vikey-wayland/
│
└── data/                        # 📁 Shared Data
    ├── vietnamese/
    │   └── lookup.dat
    ├── nom/
    │   └── dictionary.fst
    └── tai/
        └── lookup.dat
```

---

## 4. Migration Path (Lộ Trình Di Chuyển)

### Phase 1: Refactor Core

1. Tạo `traits.rs` với `LanguagePlugin`, `InputMethod`, `LookupProvider`
2. Di chuyển Vietnamese-specific code sang `vikey-vietnamese`
3. `vikey-core` chỉ giữ generic logic

### Phase 2: Plugin System

1. Implement `VietnamesePlugin` trong `vikey-vietnamese`
2. Tạo `Registry` để quản lý plugins
3. Cập nhật `Engine` để sử dụng plugins

### Phase 3: Add New Languages

1. Implement `NomPlugin` trong `vikey-nom`
2. Implement `TaiPlugin` trong `vikey-tai`
3. Mỗi plugin tự chứa lookup table và input methods riêng

---

## 5. Ví Dụ Sử Dụng

### 5.1 Khởi Tạo Engine

```rust
use vikey_core::{Engine, Config};
use vikey_vietnamese::VietnamesePlugin;
use vikey_nom::NomPlugin;

fn main() {
    let mut engine = Engine::new();

    // Đăng ký plugins
    engine.register(Box::new(VietnamesePlugin::new()));
    engine.register(Box::new(NomPlugin::new()));

    // Chọn language và input method
    engine.set_language("vietnamese");
    engine.set_input_method("telex");

    // Xử lý input
    let action = engine.process('a');
}
```

### 5.2 Thêm Ngôn Ngữ Mới

```rust
// vikey-muong/src/lib.rs
use vikey_core::{LanguagePlugin, InputMethod, LookupProvider};

pub struct MuongPlugin;

impl LanguagePlugin for MuongPlugin {
    fn name(&self) -> &str { "Tiếng Mường" }
    fn id(&self) -> &str { "muong" }

    fn input_methods(&self) -> Vec<Box<dyn InputMethod>> {
        vec![Box::new(MuongTelexMethod::new())]
    }

    fn lookup(&self) -> &dyn LookupProvider {
        &MUONG_LOOKUP
    }

    fn rules(&self) -> &dyn LanguageRules {
        &MUONG_RULES
    }
}
```

---

## 6. Kết Luận

### Kiến Trúc Hiện Tại: ❌ KHÔNG Scale Được

- `InputMethod` enum trong core → Thêm ngôn ngữ phải sửa core
- Lookup table hardcoded cho Vietnamese
- Không có abstraction cho Language

### Kiến Trúc Mới: ✅ Scale Được

- Plugin system với traits
- Mỗi ngôn ngữ là một crate độc lập
- Core chỉ là orchestrator
- Thêm ngôn ngữ = thêm crate mới, không sửa core

---

**Last Updated**: 2025-12-05
