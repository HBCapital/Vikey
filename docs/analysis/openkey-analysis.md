# Phân Tích Chi Tiết OpenKey

> Phân tích sâu về kiến trúc và implementation của OpenKey - Bộ gõ cross-platform hiện đại

## Tổng Quan

**OpenKey** là bộ gõ tiếng Việt mã nguồn mở được phát triển bởi **Tuyen Mai** (2018+), nổi bật với thiết kế modular và hỗ trợ đa nền tảng.

| Thông tin       | Chi tiết                                   |
| --------------- | ------------------------------------------ |
| **Tác giả**     | Tuyen Mai                                  |
| **License**     | GNU GPL                                    |
| **Ngôn ngữ**    | C++ (engine), Swift (macOS), C++ (Windows) |
| **Platform**    | macOS, Windows, Linux                      |
| **Engine Size** | ~90KB source code                          |

---

## Kiến Trúc Tổng Thể

```
openkey/Sources/OpenKey/
├── engine/                  # ⭐ Cross-platform core (14 files)
│   ├── Engine.cpp          # Main entry (54KB)
│   ├── Engine.h            # API definitions
│   ├── Vietnamese.cpp      # Vietnamese rules (23KB)
│   ├── Vietnamese.h        # Data structures
│   ├── DataType.h          # Type definitions
│   ├── Macro.cpp/h         # Macro system
│   ├── SmartSwitchKey.cpp  # Auto language switch
│   └── ConvertTool.cpp     # Encoding conversion
├── macOS/                  # macOS implementation (Swift)
├── win32/                  # Windows implementation
└── linux/                  # Linux implementation (WIP)
```

### Separation of Concerns

```
┌─────────────────────────────────────────────┐
│            Platform Layer                    │
│  (macOS/Swift, Windows/C++, Linux/X11)      │
└─────────────────────────────────────────────┘
                    │
                    ▼
┌─────────────────────────────────────────────┐
│           Engine API (Engine.h)              │
│  vKeyHandleEvent(), vKeyInit(), etc.        │
└─────────────────────────────────────────────┘
                    │
        ┌───────────┼───────────┐
        ▼           ▼           ▼
┌─────────────┐ ┌─────────┐ ┌─────────────┐
│ Vietnamese  │ │  Macro  │ │SmartSwitch  │
│   Rules     │ │ System  │ │   Key       │
└─────────────┘ └─────────┘ └─────────────┘
```

---

## ⭐ Kỹ Thuật Nổi Bật

### 1. Vietnamese Data Structures

OpenKey sử dụng **map-based lookup** thay vì bit-packed table như UniKey:

```cpp
// Vietnamese.cpp - Vowel patterns
map<Uint16, vector<vector<Uint16>>> _vowel = {
    { KEY_A, {
        {KEY_A, KEY_N, KEY_G}, {KEY_A, KEY_G | END_CONSONANT_MASK},
        {KEY_A, KEY_N},
        {KEY_A, KEY_M},
        {KEY_A, KEY_U},
        {KEY_A, KEY_Y},
        {KEY_A, KEY_T},
        {KEY_A, KEY_P},
        {KEY_A},
        {KEY_A, KEY_C},
    }},
    { KEY_O, { /* ... */ }},
    { KEY_E, { /* ... */ }},
    { KEY_W, { /* special handling for ư, ơ */ }},
};
```

**Ưu điểm:**

- ✅ Clear, readable structure
- ✅ Easy to add new patterns
- ✅ Self-documenting

**Nhược điểm:**

- ⚠️ Slower lookup than UniKey's DT table
- ⚠️ More memory usage

---

### 2. Comprehensive Code Table

OpenKey support 5 encodings với single data structure:

```cpp
// Vietnamese.cpp - Code tables
map<Uint32, vector<Uint16>> _codeTable[] = {
    { // 0: Unicode
        { KEY_A, {0x00C2, 0x00E2, 0x0102, 0x0103, 0x00C1, 0x00E1, ...}},
        //       { Â,     â,      Ă,      ă,      Á,      á,     ...}
        { KEY_O, {0x00D4, 0x00F4, 0x01A0, 0x01A1, 0x00D3, 0x00F3, ...}},
        { KEY_E, {0x00CA, 0x00EA, 0x0000, 0x0000, 0x00C9, 0x00E9, ...}},
        { KEY_D, {0x0110, 0x0111}},
        // With tone marks
        { KEY_A|TONE_MASK,  {0x1EA4, 0x1EA5, 0x1EA6, 0x1EA7, ...}}, // Ấ, ấ, Ầ, ầ...
        { KEY_A|TONEW_MASK, {0x1EAE, 0x1EAF, 0x1EB0, 0x1EB1, ...}}, // Ắ, ắ, Ằ, ằ...
    },
    { // 1: TCVN3 (ABC)
        // Same structure, different code points
    },
    { // 2: VNI Windows
        // 2-byte encoding for special chars
    },
    { // 3: Unicode Compound (NFD)
        // Base char + combining marks
    },
    { // 4: Vietnamese Locale CP 1258
        // Windows codepage
    }
};
```

---

### 3. Bit Masking System

```cpp
// DataType.h - Bit masks
#define CAPS_MASK           0x10000    // Uppercase flag
#define TONE_MASK           0x20000    // Circumflex (^): â, ê, ô
#define TONEW_MASK          0x40000    // Breve/horn: ă, ơ, ư
#define MARK1_MASK          0x80000    // Sắc (́)
#define MARK2_MASK          0x100000   // Huyền (̀)
#define MARK3_MASK          0x200000   // Hỏi (̉)
#define MARK4_MASK          0x400000   // Ngã (̃)
#define MARK5_MASK          0x800000   // Nặng (̣)
#define STANDALONE_MASK     0x1000000
#define CHAR_CODE_MASK      0x2000000
#define END_CONSONANT_MASK  0x4000000  // Phụ âm cuối (ng, nh, ch)
#define CONSONANT_ALLOW_MASK 0x8000000 // Z, F, W, J allowed
```

---

### 4. Hook State Machine

```cpp
// DataType.h
enum HookCodeState {
    vDoNothing,              // Pass through
    vWillProcess,            // Buffer and process
    vBreakWord,              // Word boundary
    vRestore,                // Restore original on invalid
    vReplaceMacro,           // Macro expansion
    vRestoreAndStartNewSession
};

struct vKeyHookState {
    Byte code;               // Action code
    Byte backspaceCount;     // Backspaces to send
    Byte newCharCount;       // New chars to send
    Byte extCode;            // Extended info
    Uint32 charData[MAX_BUFF];  // Character buffer (32 chars)
    vector<Uint32> macroKey;
    vector<Uint32> macroData;
};
```

---

### 5. Quick Telex Shortcuts

```cpp
// Vietnamese.cpp
map<Uint32, vector<Uint16>> _quickTelex = {
    {KEY_C, {KEY_C, KEY_H}},  // cc → ch
    {KEY_G, {KEY_G, KEY_I}},  // gg → gi
    {KEY_K, {KEY_K, KEY_H}},  // kk → kh
    {KEY_N, {KEY_N, KEY_G}},  // nn → ng
    {KEY_Q, {KEY_Q, KEY_U}},  // qq → qu
    {KEY_P, {KEY_P, KEY_H}},  // pp → ph
    {KEY_T, {KEY_T, KEY_H}},  // tt → th
    {KEY_U, {KEY_U, KEY_U}},  // uu → ươ (special case)
};

// Quick start consonants
map<Uint16, vector<Uint16>> _quickStartConsonant = {
    {KEY_F, {KEY_P, KEY_H}},  // f → ph
    {KEY_J, {KEY_G, KEY_I}},  // j → gi
    {KEY_W, {KEY_Q, KEY_U}},  // w → qu
};

// Quick end consonants
map<Uint16, vector<Uint16>> _quickEndConsonant = {
    {KEY_G, {KEY_N, KEY_G}},  // g → ng (word end)
    {KEY_H, {KEY_N, KEY_H}},  // h → nh (word end)
    {KEY_K, {KEY_C, KEY_H}},  // k → ch (word end)
};
```

---

### 6. Flexible Configuration

```cpp
// Engine.h - Global configuration
extern int vLanguage;           // 0: English, 1: Vietnamese
extern int vInputType;          // 0: Telex, 1: VNI
extern int vFreeMark;           // Free tone placement
extern int vCodeTable;          // Encoding selection
extern int vCheckSpelling;      // Spell check on/off
extern int vUseModernOrthography; // oà vs òa
extern int vQuickTelex;         // Quick shortcuts
extern int vUseMacro;           // Macro enabled
extern int vAutoCapsMacro;      // Auto-caps for macros
extern int vUseSmartSwitchKey;  // Per-app language
extern int vTempOffSpelling;    // Ctrl to disable spelling
extern int vAllowConsonantZFWJ; // Allow Z, F, W, J
extern int vQuickStartConsonant; // f→ph, j→gi, w→qu
extern int vQuickEndConsonant;   // g→ng, h→nh, k→ch
extern int vRememberCode;       // Per-app encoding
```

---

## Ưu Điểm

| Aspect                 | Details                              |
| ---------------------- | ------------------------------------ |
| **🏗️ Architecture**    | Clean separation: engine/platform/ui |
| **🌍 Cross-platform**  | Single engine, multiple platforms    |
| **📖 Readability**     | Clear data structures, better naming |
| **⚙️ Configurability** | 15+ options, per-app settings        |
| **🔧 Modern Features** | Smart switch, quick consonants       |

---

## Nhược Điểm

| Aspect               | Details                            |
| -------------------- | ---------------------------------- |
| **⚡ Performance**   | Map lookup slower than UniKey's DT |
| **💾 Memory**        | Vector copies, more allocations    |
| **🔒 Global State**  | Still uses extern globals          |
| **📄 Documentation** | Minimal inline docs                |
| **⚖️ GPL License**   | Same restriction as UniKey         |

---

## So Sánh với UniKey

| Feature            | UniKey             | OpenKey         |
| ------------------ | ------------------ | --------------- |
| **Lookup**         | DT bit-packed O(1) | Map O(log n)    |
| **Memory**         | Fixed buffers      | Dynamic vectors |
| **Encoding**       | 17+ encodings      | 5 encodings     |
| **Architecture**   | Monolithic         | Modular         |
| **Cross-platform** | Windows-focused    | macOS-first     |
| **Code Quality**   | Legacy C++         | Modern C++      |

---

## Lessons cho Vikey

### Nên Áp Dụng

1. **✅ Modular Architecture** - Engine tách biệt platform
2. **✅ Clear Data Structures** - Map-based vowel/consonant tables
3. **✅ Comprehensive Config** - Nhiều options linh hoạt
4. **✅ Quick Shortcuts** - cc→ch, gg→gi rất tiện
5. **✅ Code Table Design** - Single structure cho nhiều encodings

### Nên Cải Tiến

1. **🔧 Performance** - Hybrid: DT table + Map fallback
2. **🔧 Encapsulation** - Eliminate extern globals
3. **🔧 Rust Idioms** - Use enums, traits, Result<T,E>
4. **🔧 Testing** - Unit tests cho Vietnamese rules

---

## Rust Implementation Suggestions

```rust
// Hybrid approach: const array + HashMap
pub struct VietnameseRules {
    // Fast lookup for common chars (like UniKey's DT)
    char_attrs: [CharAttributes; 128],

    // Flexible vowel patterns (like OpenKey's map)
    vowel_patterns: HashMap<char, Vec<VowelPattern>>,

    // Code tables for encodings
    code_tables: [CodeTable; 5],
}

#[derive(Clone, Copy)]
pub struct CharAttributes {
    vowel_idx: u8,
    tone_idx: u8,
    flags: u16,  // is_separator, is_breve, etc.
}

impl CharAttributes {
    pub const fn new(vowel: u8, tone: u8, flags: u16) -> Self {
        Self { vowel_idx: vowel, tone_idx: tone, flags }
    }

    pub fn is_vowel(&self) -> bool { self.vowel_idx > 0 }
    pub fn is_separator(&self) -> bool { self.flags & 0x01 != 0 }
}
```

---

## Code Reference

**Key files to study:**

- [Engine.cpp](../references/openkey/Sources/OpenKey/engine/Engine.cpp) - Main processing
- [Vietnamese.cpp](../references/openkey/Sources/OpenKey/engine/Vietnamese.cpp) - Rules & tables
- [DataType.h](../references/openkey/Sources/OpenKey/engine/DataType.h) - Type definitions

---

**Last Updated**: 2025-12-05
