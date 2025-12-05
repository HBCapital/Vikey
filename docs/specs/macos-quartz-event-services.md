# Tài Liệu Tham Khảo macOS Quartz Event Services

## 1. Tổng Quan

Trên macOS, tương đương cấp thấp (low-level) của Windows Hooks là **Quartz Event Services** (cụ thể là `CGEventTap`). Nó cho phép chặn các sự kiện input toàn hệ thống.

**Lưu ý:** Điều này yêu cầu **Quyền Trợ Năng (Accessibility Permissions)** (System Settings -> Privacy & Security -> Accessibility).

## 2. Các Hàm API

### CGEventTapCreate

```c
CFMachPortRef CGEventTapCreate(
    CGEventTapLocation tap,           // kCGSessionEventTap
    CGEventTapPlacement place,        // kCGHeadInsertEventTap
    CGEventTapOptions options,        // kCGEventTapOptionDefault
    CGEventMask eventsOfInterest,     // Mask các sự kiện cần theo dõi
    CGEventTapCallBack callback,      // Hàm callback
    void *userInfo                    // Dữ liệu người dùng
);
```

### CGEventMask

Bitmask của các sự kiện cần chặn:

```c
CGEventMask mask = CGEventMaskBit(kCGEventKeyDown) |
                   CGEventMaskBit(kCGEventKeyUp) |
                   CGEventMaskBit(kCGEventFlagsChanged);
```

### CGEventTapCallBack

```c
CGEventRef callback(
    CGEventTapProxy proxy,
    CGEventType type,
    CGEventRef event,
    void *userInfo
) {
    // Xử lý sự kiện
    // Trả về event để cho qua
    // Trả về NULL để nuốt sự kiện
}
```

## 3. Mã Phím (Key Codes)

macOS sử dụng mã phím ảo độc lập phần cứng (khác với mã VK của Windows).

| Phím      | Code (Thập phân) | Code (Hex) |
| --------- | ---------------- | ---------- |
| A         | 0                | 0x00       |
| S         | 1                | 0x01       |
| D         | 2                | 0x02       |
| F         | 3                | 0x03       |
| H         | 4                | 0x04       |
| G         | 5                | 0x05       |
| Z         | 6                | 0x06       |
| X         | 7                | 0x07       |
| C         | 8                | 0x08       |
| V         | 9                | 0x09       |
| B         | 11               | 0x0B       |
| Q         | 12               | 0x0C       |
| W         | 13               | 0x0D       |
| E         | 14               | 0x0E       |
| R         | 15               | 0x0F       |
| Y         | 16               | 0x10       |
| T         | 17               | 0x11       |
| 1         | 18               | 0x12       |
| 2         | 19               | 0x13       |
| 3         | 20               | 0x14       |
| 4         | 21               | 0x15       |
| 6         | 22               | 0x16       |
| 5         | 23               | 0x17       |
| =         | 24               | 0x18       |
| 9         | 25               | 0x19       |
| 7         | 26               | 0x1A       |
| -         | 27               | 0x1B       |
| 8         | 28               | 0x1C       |
| 0         | 29               | 0x1D       |
| ]         | 30               | 0x1E       |
| O         | 31               | 0x1F       |
| U         | 32               | 0x20       |
| [         | 33               | 0x21       |
| I         | 34               | 0x22       |
| P         | 35               | 0x23       |
| L         | 37               | 0x25       |
| J         | 38               | 0x26       |
| '         | 39               | 0x27       |
| K         | 40               | 0x28       |
| ;         | 41               | 0x29       |
| \         | 42               | 0x2A       |
| ,         | 43               | 0x2B       |
| /         | 44               | 0x2C       |
| N         | 45               | 0x2D       |
| M         | 46               | 0x2E       |
| .         | 47               | 0x2F       |
| Space     | 49               | 0x31       |
| Backspace | 51               | 0x33       |

## 4. Gửi Input (Sending Input)

Để output các ký tự đã sửa đổi, tạo và post các sự kiện mới.

### CGEventCreateKeyboardEvent

```c
CGEventRef CGEventCreateKeyboardEvent(
    CGEventSourceRef source,
    CGKeyCode virtualKey,
    bool keyDown
);
```

### CGEventKeyboardSetUnicodeString

Để gửi ký tự Unicode (thay vì key codes):

```c
void CGEventKeyboardSetUnicodeString(
    CGEventRef event,
    UniCharCount stringLength,
    const UniChar *unicodeString
);
```

### CGEventPost

```c
void CGEventPost(
    CGEventTapLocation tap,
    CGEventRef event
);
```

## 5. Chiến Lược Implementation Cho Vikey

1.  **Tạo RunLoop Source**: `CGEventTapCreate` trả về một Mach port. Bọc nó trong `CFRunLoopSource` và thêm vào main run loop.
2.  **Logic Callback**:
    - Kiểm tra loại sự kiện (`kCGEventKeyDown`).
    - Lấy key code: `CGEventGetIntegerValueField(event, kCGKeyboardEventKeycode)`.
    - Nếu cần biến đổi:
      - Trả về `NULL` (nuốt sự kiện).
      - Tạo các sự kiện mới (Backspace + Unicode chars).
      - Post sự kiện với `CGEventPost`.
    - Nếu không cần biến đổi:
      - Trả về `event` (cho qua).

## 6. Rust `core-graphics` Crate

Sử dụng crate `core-graphics` cho các bindings.

```rust
use core_graphics::event::{CGEvent, CGEventTap, CGEventTapLocation, CGEventTapPlacement, CGEventTapOptions, CGEventMask, CGKeyCode};
```
