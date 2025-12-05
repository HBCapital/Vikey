# Phân Tích fcitx5-unikey và ibus-bamboo

> So sánh hai bộ gõ Linux hiện đại: fcitx5-unikey (C++) và ibus-bamboo (Go)
>
> **Location**: `references/fcitx5-unikey/` và `references/ibus-bamboo/`

## Tổng Quan

| Khía cạnh      | fcitx5-unikey            | ibus-bamboo                   |
| -------------- | ------------------------ | ----------------------------- |
| **Ngôn ngữ**   | C++                      | Go                            |
| **Engine**     | UniKey (Pham Kim Long)   | Bamboo Core (Luong Thanh Lam) |
| **Framework**  | Fcitx5                   | IBus                          |
| **License**    | GPL-2.0+                 | GPL-3.0                       |
| **Trạng thái** | Đang phát triển tích cực | Đình trệ (deprecated)         |
| **Wayland**    | Hỗ trợ tốt               | Hỗ trợ hạn chế                |

## 1. fcitx5-unikey

### Cấu Trúc Dự Án

```
fcitx5-unikey/
├── unikey/              # ⭐ Core engine (24 files)
│   ├── ukengine.h/cpp   # Main engine class
│   ├── inputproc.h/cpp  # Input processor
│   ├── charset.h/cpp    # Character set handling
│   ├── mactab.h/cpp     # Macro table
│   └── vnlexi.h         # Vietnamese lexicon
├── src/                 # Fcitx5 integration
│   ├── unikey-im.cpp    # Input method implementation
│   └── unikey-config.h  # Configuration
├── keymap-editor/       # Keymap editor (Qt)
└── macro-editor/        # Macro editor (Qt)
```

### Core Engine: UkEngine

**File**: `unikey/ukengine.h`

```cpp
class UkEngine {
public:
    // Main processing function
    int process(unsigned int keyCode, int &backs,
                unsigned char *outBuf, int &outSize,
                UkOutputType &outType);

    // Backspace handling
    int processBackspace(int &backs, unsigned char *outBuf,
                         int &outSize, UkOutputType &outType);

    // Restore keystrokes (undo)
    int restoreKeyStrokes(int &backs, unsigned char *outBuf,
                          int &outSize, UkOutputType &outType);

    void reset();

protected:
    // Processing methods
    int processTone(UkKeyEvent &ev);
    int processRoof(UkKeyEvent &ev);      // â, ê, ô
    int processHook(UkKeyEvent &ev);      // ơ, ư
    int processAppend(UkKeyEvent &ev);
    int processDd(UkKeyEvent &ev);        // đ
    int processTelexW(UkKeyEvent &ev);    // w -> ư

    // Data structures
    WordInfo m_buffer[MAX_UK_ENGINE];
    KeyBufEntry m_keyStrokes[MAX_UK_ENGINE];
    int m_current;
    int m_backs;
};
```

**Đặc điểm nổi bật:**

1. **WordInfo Structure**: Lưu thông tin từng từ

   ```cpp
   struct WordInfo {
       VnWordForm form;           // c, v, cv, vc, cvc
       int c1Offset, vOffset, c2Offset;
       VowelSeq vseq;             // Vowel sequence
       int caps, tone;
       VnLexiName vnSym;          // Vietnamese symbol
   };
   ```

2. **Shared Memory**: `UkSharedMem` để chia sẻ state giữa processes

   ```cpp
   struct UkSharedMem {
       bool vietKey;
       UnikeyOptions options;
       UkInputProcessor input;
       CMacroTable macStore;
   };
   ```

3. **Output Type**: Hỗ trợ nhiều encoding
   ```cpp
   enum UkOutputType {
       UkCharOutput,      // Unicode
       UkKeyOutput,       // Raw keystrokes
       UkBlockOutput      // Block output
   };
   ```

### Ưu Điểm

- ✅ **Mature**: Dựa trên UniKey đã được kiểm chứng 20+ năm
- ✅ **Performance**: C++ native, rất nhanh
- ✅ **Fcitx5**: Framework hiện đại, hỗ trợ Wayland tốt
- ✅ **Macro System**: Hỗ trợ macro mạnh mẽ
- ✅ **Editors**: Có GUI editor cho keymap và macro

### Nhược Điểm

- ❌ **C++ Complexity**: Code phức tạp, khó maintain
- ❌ **Fcitx5 Specific**: Chỉ chạy trên Fcitx5
- ❌ **Memory Management**: Phải quản lý memory thủ công

---

## 2. ibus-bamboo

### Cấu Trúc Dự Án

```
ibus-bamboo/
├── vendor/github.com/BambooEngine/bamboo-core/
│   ├── bamboo.go           # ⭐ Main engine
│   ├── input_method_def.go # Input method definitions
│   ├── rules_parser.go     # Rule parser
│   ├── spelling.go         # Spell checking
│   └── flattener.go        # Flatten transformations
├── engine.go               # IBus engine implementation
├── engine_backspace.go     # Backspace handling
├── engine_preedit.go       # Pre-edit mode
└── config/                 # Configuration
```

### Core Engine: BambooEngine

**File**: `vendor/github.com/BambooEngine/bamboo-core/bamboo.go`

```go
type BambooEngine struct {
    composition []*Transformation
    inputMethod InputMethod
    flags       uint
}

type Transformation struct {
    Rule        Rule
    Target      *Transformation
    IsUpperCase bool
}

// Main API
func (e *BambooEngine) ProcessKey(key rune, mode Mode)
func (e *BambooEngine) GetProcessedString(mode Mode) string
func (e *BambooEngine) RemoveLastChar(refreshLastToneTarget bool)
func (e *BambooEngine) RestoreLastWord(toVietnamese bool)
func (e *BambooEngine) Reset()
```

**Đặc điểm nổi bật:**

1. **Transformation-Based**: Mỗi phím tạo ra một transformation

   ```go
   type Rule struct {
       Key         rune
       Effect      uint8  // TONE, MARK, APPEND
       EffectType  uint8  // TONE_ACUTE, MARK_HORN, etc.
       Target      uint8  // Target position
   }
   ```

2. **Composition Chain**: Danh sách các transformations

   ```go
   // Example: "hoa" + "s" -> "hóa"
   composition = [
       {Rule: APPEND('h')},
       {Rule: APPEND('o')},
       {Rule: APPEND('a')},
       {Rule: TONE_ACUTE, Target: 'o'},  // Add tone to 'o'
   ]
   ```

3. **Modes**: Hỗ trợ nhiều mode

   ```go
   const (
       VietnameseMode Mode = 1 << iota
       EnglishMode
       ToneLess
       MarkLess
       LowerCase
       FullText
   )
   ```

4. **Spell Checking**: Built-in spell checker
   ```go
   func isValid(composition []*Transformation,
                inputIsFullComplete bool) bool
   ```

### Ưu Điểm

- ✅ **Go**: Memory safe, garbage collected
- ✅ **Clean Architecture**: Transformation-based rất rõ ràng
- ✅ **Spell Checking**: Kiểm tra chính tả tích hợp
- ✅ **Flexible**: Dễ mở rộng với rules mới
- ✅ **Emoji Support**: 2666 emojis

### Nhược Điểm

- ❌ **Deprecated**: Dự án đã ngừng phát triển
- ❌ **IBus**: Framework cũ hơn, Wayland support kém
- ❌ **Performance**: Go chậm hơn C++ một chút
- ❌ **Memory**: Transformation chain tốn memory

---

## So Sánh Kỹ Thuật

### 1. Data Structures

| Khía cạnh       | fcitx5-unikey         | ibus-bamboo               |
| --------------- | --------------------- | ------------------------- |
| **Buffer**      | Array `WordInfo[128]` | Slice `[]*Transformation` |
| **Memory**      | Fixed size, stack     | Dynamic, heap             |
| **Lookup**      | Direct array access   | Linked list traversal     |
| **Performance** | O(1)                  | O(n)                      |

### 2. Processing Approach

**fcitx5-unikey (Imperative)**:

```cpp
int UkEngine::processTone(UkKeyEvent &ev) {
    // Find vowel position
    int pos = getTonePosition(vseq, terminated);
    // Apply tone directly to buffer
    m_buffer[pos].tone = toneIndex;
    // Calculate output
    writeOutput(outBuf, outSize);
}
```

**ibus-bamboo (Functional)**:

```go
func (e *BambooEngine) ProcessKey(key rune, mode Mode) {
    // Generate new transformations
    trans := e.generateTransformations(composition, key, isUpper)
    // Append to composition
    e.composition = append(e.composition, trans...)
    // Refresh tone target if needed
    e.composition = append(e.composition,
        e.refreshLastToneTarget(e.composition)...)
}
```

### 3. Backspace Handling

**fcitx5-unikey**:

```cpp
int UkEngine::processBackspace(int &backs, ...) {
    // Remove last character from buffer
    m_current--;
    // Recalculate word form
    // Return number of backspaces needed
    backs = calculateBackspaces();
}
```

**ibus-bamboo**:

```go
func (e *BambooEngine) RemoveLastChar(refresh bool) {
    // Find last APPENDING transformation
    lastAppending := findLastAppendingTrans(e.composition)
    // Remove it and all transformations targeting it
    newComp := filterOut(e.composition, lastAppending)
    // Refresh tone if needed
    if refresh {
        newComp = append(newComp,
            e.refreshLastToneTarget(newComp)...)
    }
    e.composition = newComp
}
```

---

## Bài Học Cho Vikey

### 1. Từ fcitx5-unikey

✅ **Nên học:**

- **WordInfo structure**: Lưu thông tin từng vị trí trong từ
- **Shared memory**: Cho multi-process support
- **Macro system**: Tính năng hữu ích
- **Output types**: Hỗ trợ nhiều encoding

❌ **Không nên:**

- **C++ complexity**: Quá phức tạp
- **Fixed buffer size**: Không linh hoạt
- **Platform coupling**: Gắn chặt với Fcitx5

### 2. Từ ibus-bamboo

✅ **Nên học:**

- **Transformation-based**: Kiến trúc rõ ràng, dễ debug
- **Spell checking**: Tính năng quan trọng
- **Mode system**: Linh hoạt (ToneLess, MarkLess, etc.)
- **Clean API**: Interface đơn giản

❌ **Không nên:**

- **Linked list**: Chậm hơn array
- **Too many allocations**: Tốn memory
- **IBus dependency**: Framework cũ

### 3. Đề Xuất Cho Vikey Core

**Kết hợp điểm mạnh:**

```rust
pub struct VikeyCore {
    // Fixed-size buffer (như fcitx5-unikey)
    buffer: [CharInfo; 128],
    current: usize,

    // Transformation history (như ibus-bamboo, nhưng dùng Vec)
    transformations: Vec<Transformation>,

    // Spell checker (như ibus-bamboo)
    spell_checker: SpellChecker,

    // Config
    config: Config,
}

pub enum Transformation {
    Append(char),
    Tone { target_pos: usize, tone: ToneType },
    Mark { target_pos: usize, mark: MarkType },
}

impl VikeyCore {
    // Clean API (như ibus-bamboo)
    pub fn process_key(&mut self, key: char) -> Action;
    pub fn get_output(&self, mode: Mode) -> String;
    pub fn remove_last_char(&mut self) -> Action;

    // Performance (như fcitx5-unikey)
    // - Use array instead of linked list
    // - Direct buffer manipulation
    // - Minimal allocations
}
```

**Ưu điểm của approach này:**

- ✅ Performance cao (array-based như fcitx5-unikey)
- ✅ Kiến trúc rõ ràng (transformation-based như ibus-bamboo)
- ✅ Memory safe (Rust)
- ✅ Spell checking built-in
- ✅ Pure library (không phụ thuộc framework)

---

## Kết Luận

**Khuyến nghị:**

1. **Dùng fcitx5-unikey** nếu cần bộ gõ production-ready trên Linux ngay bây giờ
2. **Học từ cả hai** khi implement Vikey Core:
   - Performance approach từ fcitx5-unikey
   - Architecture từ ibus-bamboo
   - Rust safety và zero-cost abstractions

**Next Steps:**

- Implement Transformation-based architecture trong Vikey Phase 2
- Add spell checking trong Phase 3
- Benchmark performance so với fcitx5-unikey
