# So Sánh Kiến Trúc: UniKey vs OpenKey vs Vikey

> Phân tích so sánh ba bộ gõ tiếng Việt và hướng đi cho Vikey

## Tổng Quan

| Aspect             | UniKey     | OpenKey | Vikey (Target) |
| ------------------ | ---------- | ------- | -------------- |
| **Năm phát triển** | 1998-2002  | 2018+   | 2025+          |
| **Ngôn ngữ**       | C/C++      | C++     | Rust           |
| **License**        | GPL v2     | GPL     | Apache 2.0     |
| **Platform chính** | Windows    | macOS   | Cross-platform |
| **Kiến trúc**      | Monolithic | Modular | Modular crates |
| **Lines of Code**  | ~30,000    | ~12,000 | Target: ~5,000 |

---

## So Sánh Chi Tiết

### 1. Character Classification

#### UniKey: DT Bit-packed Table

```cpp
// O(1) lookup, 256 entries × 4 bytes = 1KB
DWORD DT[256];

#define ATTR_VOWEL_INDEX(x)  (x & 0x1F)
#define ATTR_TONE_INDEX(x)   ((x >> 14) & 0xF)
#define ATTR_IS_SEPARATOR(x) (x & 0x2000000)

// Usage
if (ATTR_VOWEL_INDEX(DT['a']) > 0) { /* is vowel */ }
```

#### OpenKey: Map-based Lookup

```cpp
// O(log n) lookup, more readable
map<Uint16, vector<vector<Uint16>>> _vowel;
map<Uint32, vector<Uint16>> _codeTable;

// Usage
if (_vowel.find(KEY_A) != _vowel.end()) { /* is vowel */ }
```

#### Vikey: Hybrid Approach (Proposed)

```rust
// O(1) for common chars, HashMap for complex patterns
const CHAR_ATTRS: [u32; 128] = [ /* compile-time computed */ ];
lazy_static! {
    static ref VOWEL_PATTERNS: HashMap<char, Vec<Pattern>> = { /* ... */ };
}
```

---

### 2. Buffer Management

| Feature       | UniKey            | OpenKey    | Vikey (Proposed) |
| ------------- | ----------------- | ---------- | ---------------- |
| Size          | 40 chars          | 32 chars   | 32 chars         |
| Type          | `unsigned char[]` | `Uint32[]` | `Vec<char>`      |
| Overflow      | Keep last 20      | Clear all  | Ring buffer      |
| Case tracking | `int lowerCase[]` | Bit flag   | Enum per char    |

---

### 3. State Machine

```
        UniKey                    OpenKey                   Vikey
┌─────────────────┐        ┌─────────────────┐       ┌─────────────────┐
│  Key Category   │        │  HookCodeState  │       │     State       │
├─────────────────┤        ├─────────────────┤       ├─────────────────┤
│ VOWEL_CHAR      │        │ vDoNothing      │       │ Initial         │
│ TONE_MARK       │        │ vWillProcess    │       │ Buffering       │
│ BREVE_MARK      │        │ vBreakWord      │       │ Processing      │
│ DOUBLE_KEY      │        │ vRestore        │       │ Committed       │
│ SEPARATOR_KEY   │        │ vReplaceMacro   │       │ Error           │
│ SHORT_KEY       │        │ vRestoreAnd...  │       │                 │
│ ESCAPE_KEY      │        │                 │       │                 │
└─────────────────┘        └─────────────────┘       └─────────────────┘
```

---

### 4. Encoding Support

| Encoding         | UniKey | OpenKey | Vikey   |
| ---------------- | ------ | ------- | ------- |
| Unicode (UTF-8)  | ✅     | ✅      | ✅      |
| Unicode (UCS-2)  | ✅     | ✅      | ✅      |
| TCVN3 (ABC)      | ✅     | ✅      | Planned |
| VNI Windows      | ✅     | ✅      | Planned |
| Unicode Compound | ✅     | ✅      | ✅      |
| CP 1258          | ✅     | ✅      | Planned |
| VISCII           | ✅     | ❌      | Planned |
| Total            | 17+    | 5       | 5+      |

---

### 5. Input Methods

| Method           | UniKey | OpenKey | Vikey       |
| ---------------- | ------ | ------- | ----------- |
| Telex            | ✅     | ✅      | ✅ Priority |
| VNI              | ✅     | ✅      | ✅          |
| VIQR             | ✅     | ❌      | Planned     |
| Quick Telex      | ❌     | ✅      | ✅          |
| Quick Consonants | ❌     | ✅      | ✅          |

---

### 6. Advanced Features

| Feature            | UniKey | OpenKey | Vikey   |
| ------------------ | ------ | ------- | ------- |
| Macro expansion    | ✅     | ✅      | Planned |
| Spell checking     | Basic  | ✅      | Planned |
| Auto-capitalize    | ✅     | ✅      | ✅      |
| Modern orthography | ✅     | ✅      | ✅      |
| Per-app settings   | ❌     | ✅      | Planned |
| Smart switch       | ❌     | ✅      | Planned |
| Quick shortcuts    | ❌     | ✅      | ✅      |

---

## Điểm Mạnh Cần Học Hỏi

### Từ UniKey

1. **🚀 DT Lookup Table** - O(1) character classification
2. **💾 Memory Efficiency** - Fixed buffers, no allocations
3. **🔧 Bit Manipulation** - Compact state representation
4. **📊 Tone Placement Algorithm** - 20+ years proven rules
5. **🔄 Buffer Overflow Handling** - Keep recent chars

### Từ OpenKey

1. **🏗️ Modular Architecture** - Engine/Platform separation
2. **📖 Readable Data Structures** - Map-based tables
3. **⚙️ Configuration System** - 15+ options
4. **🔧 Quick Shortcuts** - cc→ch, gg→gi productivity
5. **🌍 Cross-platform Design** - Platform abstraction

---

## Điểm Yếu Cần Tránh

### Từ Cả Hai

| Issue                   | Solution in Vikey      |
| ----------------------- | ---------------------- |
| Global state (`extern`) | Encapsulated structs   |
| Minimal documentation   | Inline docs + examples |
| No unit tests           | Property-based testing |
| GPL license             | Apache 2.0             |
| Manual memory           | Rust ownership         |

---

## Vikey Architecture Decision

### Layer 1: Core (vikey-core)

```rust
// Platform-agnostic processing
pub struct Processor {
    buffer: Buffer,
    state: StateMachine,
    config: ProcessorConfig,
}

impl Processor {
    pub fn process_key(&mut self, event: KeyEvent) -> Result<Action>;
    pub fn reset(&mut self);
}
```

### Layer 2: Vietnamese (vikey-vietnamese)

```rust
// Vietnamese-specific rules
pub trait Transformer {
    fn transform(&self, input: &str) -> Option<TransformResult>;
    fn name(&self) -> &str;
}

pub struct TelexTransformer { /* ... */ }
pub struct VNITransformer { /* ... */ }
```

### Layer 3: Platform (vikey-platform)

```rust
// Platform abstraction
pub trait PlatformHook {
    fn install(&mut self) -> Result<()>;
    fn uninstall(&mut self) -> Result<()>;
    fn send_backspace(&self, count: usize) -> Result<()>;
    fn send_chars(&self, chars: &[char]) -> Result<()>;
}
```

---

## Performance Target

| Metric            | UniKey | OpenKey | Vikey Target |
| ----------------- | ------ | ------- | ------------ |
| Latency/keystroke | <5ms   | <10ms   | <5ms         |
| Memory usage      | <10MB  | <20MB   | <15MB        |
| Binary size       | ~500KB | ~2MB    | <1MB         |
| Startup time      | <100ms | <200ms  | <100ms       |

---

## Recommended Implementation Order

### Phase 1: Core Engine

1. ✅ State machine (`vikey-core`)
2. ✅ Buffer management
3. ✅ Transformer trait
4. ✅ Telex implementation

### Phase 2: Vietnamese Rules

5. [ ] Complete Telex rules
6. [ ] VNI implementation
7. [ ] Tone placement algorithm
8. [ ] Modern orthography

### Phase 3: Platform Integration

9. [ ] Windows hook
10. [ ] macOS event tap
11. [ ] Linux X11/Wayland

### Phase 4: Advanced Features

12. [ ] Macro system
13. [ ] Spell checking
14. [ ] Configuration UI

---

## Kết Luận

| Approach         | Recommendation                               |
| ---------------- | -------------------------------------------- |
| **Lookup**       | Hybrid: const array + HashMap (best of both) |
| **Architecture** | OpenKey-style modular + Rust crates          |
| **Performance**  | UniKey-style bit manipulation for hot path   |
| **Features**     | OpenKey-style comprehensive options          |
| **Code Quality** | Modern Rust idioms + extensive testing       |

---

**Last Updated**: 2025-12-05
