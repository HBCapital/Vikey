# Phân Tích Chi Tiết UniKey

> Phân tích sâu về kiến trúc và implementation của UniKey - Bộ gõ tiếng Việt #1 trên Windows

## Tổng Quan

**UniKey** là bộ gõ tiếng Việt mã nguồn mở phổ biến nhất, được phát triển bởi **Phạm Kim Long** (1998-2002), với hơn 20 năm phát triển và hàng triệu người dùng.

| Thông tin         | Chi tiết                          |
| ----------------- | --------------------------------- |
| **Tác giả**       | Phạm Kim Long                     |
| **License**       | GNU GPL v2                        |
| **Ngôn ngữ**      | C/C++                             |
| **Platform**      | Windows (chính), Linux (x-unikey) |
| **Lines of Code** | ~30,000+                          |

---

## Kiến Trúc Tổng Thể

```
unikey/
├── unikey-win/              # Windows version
│   ├── keyhook/            # ⭐ Core Engine (13 files)
│   │   ├── vietkey.cpp     # Vietnamese processing (28KB)
│   │   ├── keyhook.cpp     # Keyboard hook (27KB)
│   │   ├── keycons.h       # Constants & bit masks
│   │   └── vietkey.h       # VietKey class definition
│   ├── newkey/             # Main application (40+ files)
│   ├── vnconv/             # Encoding converter
│   └── byteio/             # I/O utilities
├── x-unikey/               # Linux/X11 version
└── uvconv/                 # Standalone converter
```

---

## ⭐ Kỹ Thuật Nổi Bật

### 1. DT Table - Lookup Table Tối Ưu

UniKey sử dụng **DT (Data Table)** - một mảng 256 phần tử chứa thông tin bit-packed cho mỗi ký tự ASCII:

```cpp
// keycons.h - Bit layout trong DT table (32-bit per entry)
// Bits 0-4:   vowel index (0-31)
// Bits 5-8:   macro key index
// Bits 9-13:  double character index (aa, ee, oo...)
// Bits 14-17: tone mark index
// Bits 18-21: current tone of character
// Bit 22:     is breve mark (w, W)
// Bit 24:     soft separator
// Bit 25:     hard separator (clear buffer)
// Bits 26-28: VNI double mark index
// Bit 29:     word stop (for macro)

#define ATTR_VOWEL_INDEX(x)     (x & 0x1F)
#define ATTR_TONE_INDEX(x)      ((x >> 14) & 0xF)
#define ATTR_DBCHAR_INDEX(x)    ((x >> 9) & 0x1F)
#define ATTR_IS_BREVE(x)        ((x >> 22) & 0x1)
#define ATTR_IS_SEPARATOR(x)    (x & 0x2000000)
#define ATTR_CURRENT_TONE(x)    ((x >> 18) & 0xF)
#define ATTR_IS_WORD_STOP(x)    ((x >> 29) & 0x1)
```

**Ưu điểm:**

- ✅ O(1) character classification
- ✅ Single memory lookup
- ✅ Compact representation (4 bytes/char)
- ✅ Cache-friendly

**Áp dụng cho Vikey:** Sử dụng similar bit-packed lookup table trong Rust với `const` arrays.

---

### 2. VietKey Class - Buffer Management

```cpp
// vietkey.h
#define KEY_BUFSIZE 40     // Maximum buffer size
#define KEYS_MAINTAIN 20   // Characters to keep when overflow

class VietKey {
private:
    int keys;                          // Current buffer length
    unsigned char buf[KEY_BUFSIZE];    // Character buffer
    int lowerCase[KEY_BUFSIZE];        // Case tracking per char
    int lastWConverted;                // W → ư tracking
    int lastIsEscape;                  // VIQR escape mode
    int tempVietOff;                   // Temporary Vietnamese off

    DWORD *DT;                         // Lookup table pointer
    unsigned char (*BD)[6];            // Vowel + tone combinations
    unsigned char *BK;                 // Double character map
    unsigned char *BW;                 // Breve mark map
    unsigned char *BT;                 // Quick key map

public:
    int keysPushed, backs;             // Output tracking
    unsigned char ansiPush[1024];      // ANSI output buffer
    WORD uniPush[512];                 // Unicode output buffer

    void process(unsigned char ch);
    void clearBuf();
    void putToneMark(unsigned char c);
    void putBreveMark(unsigned char c);
    void doubleChar(unsigned char c);
};
```

**Key insight:** Buffer overflow handling với `throwBuf()` - giữ lại 20 ký tự cuối:

```cpp
void VietKey::throwBuf() {
    memmove(buf, &buf[keys-KEYS_MAINTAIN], KEYS_MAINTAIN);
    memmove(lowerCase, &lowerCase[keys-KEYS_MAINTAIN], KEYS_MAINTAIN);
    keys = KEYS_MAINTAIN;
}
```

---

### 3. Tone Placement Algorithm

UniKey có algorithm phức tạp để đặt dấu đúng vị trí:

```cpp
void VietKey::putToneMark(unsigned char c) {
    // 1. Tìm nguyên âm đầu tiên từ phải sang
    i = keys - 1;
    while (i >= leftMost) {
        if (ATTR_VOWEL_INDEX(DT[buf[i]]) > 0) break;
        i--;
    }

    // 2. Tìm chuỗi nguyên âm liên tiếp
    cuoi = i;
    while (i >= leftMost && ATTR_VOWEL_INDEX(DT[buf[i]])) i--;

    // 3. Xác định vị trí đặt dấu theo quy tắc tiếng Việt
    l = cuoi - i;  // độ dài chuỗi nguyên âm
    switch (l) {
        case 2:  // oa, oe, uy → dấu ở nguyên âm sau nếu modernStyle
                 // gi, qu → dấu ở nguyên âm sau
                 // có phụ âm theo sau → dấu ở nguyên âm sau
            break;
        case 3:  // oai, uai → dấu ở nguyên âm giữa
            i = cuoi - 1;
            break;
        default: // single vowel → dấu ở nguyên âm đó
            i = cuoi;
    }
}
```

**Modern orthography support:**

```cpp
if (modernStyle &&
    ((buf[cuoi-1] == 'o' && buf[cuoi] == 'a') ||  // oà vs òa
     (buf[cuoi-1] == 'o' && buf[cuoi] == 'e') ||  // oè vs òe
     (buf[cuoi-1] == 'u' && buf[cuoi] == 'y')))   // uỳ vs ùy
    i = cuoi;  // dấu ở nguyên âm cuối
```

---

### 4. Backspace Technique

UniKey tính số backspace cần gửi dựa trên encoding:

```cpp
void VietKey::processBackspace() {
    backs = 1;  // minimum 1 backspace

    if (UNICODE_CHARSET) {
        // UTF-8: 1-3 bytes per char
        mapping = ToUniL[buf[keys-1]];
        backs += uniCharLen(mapping, encoding) - 1;
    }
    else if (VIQR_CHARSET) {
        // VIQR: có thể cần 2-3 backspace cho composed chars
        if (HIBYTE(mapping)) backs++;
    }

    keys--;
}
```

---

### 5. Macro System với Binary Search

```cpp
int VietKey::checkMacro(unsigned char lastChar) {
    // 1. Build macro key từ buffer
    for (j = i+1; j < keys; j++) {
        key[j-i] = lowerCase[j] ? buf[j] : toupper(buf[j]);
    }

    // 2. Binary search trong sorted macro table
    pMacro = (HookMacroDef *)bsearch(
        key,
        pShMem->macroTable,
        pShMem->macroCount,
        sizeof(HookMacroDef),
        macroKeyCompare
    );

    // 3. Replace with macro text
    backs = keyLen;
    // copy macro text to output...
}
```

---

### 6. Unicode Encoding Support

```cpp
// UTF-8 encoding
unsigned char *putUnicodeCharUtf8(unsigned char *buf, WORD ch, int &len) {
    if (ch < 0x0080) {
        len = 1;
        *buf++ = (unsigned char)ch;
    } else if (ch < 0x0800) {
        len = 2;
        *buf++ = (0xC0 | (BYTE)(ch >> 6));
        *buf++ = (0x80 | (BYTE)(ch & 0x3F));
    } else {
        len = 3;
        *buf++ = (0xE0 | (BYTE)(ch >> 12));
        *buf++ = (0x80 | (BYTE)((ch >> 6) & 0x3F));
        *buf++ = (0x80 | (BYTE)(ch & 0x3F));
    }
    return buf;
}
```

---

## Ưu Điểm

| Aspect                | Details                                               |
| --------------------- | ----------------------------------------------------- |
| **🚀 Performance**    | DT lookup O(1), bit manipulation, minimal allocations |
| **💾 Memory**         | Fixed buffers, no heap allocations during typing      |
| **🔧 Stability**      | 20+ years battle-tested, handles edge cases           |
| **📊 Comprehensive**  | Support 17+ encodings, 4 input methods                |
| **🔄 State Recovery** | `tempVietOff` flag for invalid word handling          |

---

## Nhược Điểm

| Aspect               | Details                                       |
| -------------------- | --------------------------------------------- |
| **📝 Code Quality**  | Minimal comments, magic numbers, global state |
| **🔒 Thread Safety** | Global `pShMem`, not thread-safe              |
| **🖥️ Windows-only**  | Deep Windows API dependencies                 |
| **📦 Monolithic**    | Hard to extract engine as library             |
| **⚖️ GPL License**   | Not compatible with Apache 2.0                |

---

## Lessons cho Vikey

### Nên Áp Dụng

1. **✅ DT Lookup Table** - Implement as const array trong Rust
2. **✅ Bit-packed Attributes** - Sử dụng bitflags crate
3. **✅ Fixed Buffer** - Pre-allocated Vec với capacity
4. **✅ Tone Placement Rules** - Port algorithm sang Rust
5. **✅ Macro Binary Search** - Sử dụng Rust's `binary_search`

### Nên Cải Tiến

1. **🔧 Encapsulation** - Struct thay vì global state
2. **🔧 Error Handling** - Result<T, E> thay vì magic return values
3. **🔧 Documentation** - Inline docs với `///`
4. **🔧 Testing** - Property-based tests cho tone placement
5. **🔧 Thread Safety** - Arc<Mutex<T>> hoặc lock-free structures

---

## Code Reference

**Key files to study:**

- [vietkey.cpp](../references/unikey/unikey-win/keyhook/vietkey.cpp) - Core processing logic
- [keycons.h](../references/unikey/unikey-win/keyhook/keycons.h) - Constants and bit masks
- [keyhook.cpp](../references/unikey/unikey-win/keyhook/keyhook.cpp) - Windows hook implementation

---

**Last Updated**: 2025-12-05
