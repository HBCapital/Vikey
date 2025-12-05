# Tài Liệu Tham Khảo Linux X11 Input

## 1. Tổng Quan

Trên Linux với X11, việc chặn input toàn cục thường được thực hiện bằng cách grab bàn phím hoặc sử dụng XInput2. Đối với các bộ gõ đơn giản hoặc công cụ macro, `XGrabKeyboard` (độc quyền) hoặc `XRecord` (không độc quyền) là phổ biến.

## 2. Ký Hiệu Phím X11 (X11 Key Symbols - Keysyms)

X11 sử dụng **Keysyms** để đại diện cho các phím.

| Phím      | Tên Keysym   | Giá Trị (Hex) |
| --------- | ------------ | ------------- |
| A         | XK_a         | 0x0061        |
| B         | XK_b         | 0x0062        |
| ...       | ...          | ...           |
| Backspace | XK_BackSpace | 0xFF08        |
| Return    | XK_Return    | 0xFF0D        |
| Escape    | XK_Escape    | 0xFF1B        |

**File Header:** `<X11/keysymdef.h>`

## 3. Các Hàm API (XLib)

### XGrabKeyboard

Chiếm quyền điều khiển toàn bộ input bàn phím.

```c
int XGrabKeyboard(
    Display *display,
    Window grab_window,
    Bool owner_events,
    int pointer_mode,
    int keyboard_mode,
    Time time
);
```

### XNextEvent

Chờ sự kiện tiếp theo.

```c
XNextEvent(
    Display *display,
    XEvent *event_return
);
```

### Cấu Trúc XEvent

```c
typedef union _XEvent {
    int type;
    XKeyEvent xkey;
    // ...
} XEvent;

typedef struct {
    int type;       // KeyPress hoặc KeyRelease
    unsigned long serial;
    Bool send_event;
    Display *display;
    Window window;
    Window root;
    Window subwindow;
    Time time;
    int x, y;
    int x_root, y_root;
    unsigned int state; // Key mask (Shift, Ctrl, v.v.)
    unsigned int keycode; // Hardware keycode
    Bool same_screen;
} XKeyEvent;
```

## 4. Gửi Input (XTest Extension)

Để giả lập nhấn phím (Backspace + Ký tự mới), sử dụng XTest extension.

### XTestFakeKeyEvent

```c
int XTestFakeKeyEvent(
    Display *display,
    unsigned int keycode,
    Bool is_press,
    unsigned long delay
);
```

**Chuyển Đổi Keycode:**
Sử dụng `XKeysymToKeycode` để chuyển đổi Keysym (như `XK_a`) sang hardware keycode cần thiết cho `XTestFakeKeyEvent`.

```c
KeyCode code = XKeysymToKeycode(display, XK_a);
XTestFakeKeyEvent(display, code, True, 0);  // Nhấn
XTestFakeKeyEvent(display, code, False, 0); // Nhả
```

## 5. Chiến Lược Implementation Cho Vikey (X11)

1.  **Kết Nối**: `XOpenDisplay(NULL)`.
2.  **Vòng Lặp**:
    - Sử dụng `XRecord` extension (tốt hơn Grab) để lắng nghe input mà không chặn các ứng dụng khác.
    - Hoặc sử dụng `XGrabKey` cho các hotkey cụ thể.
3.  **Xử Lý**:
    - Map `keycode` -> `keysym`.
    - Kiểm tra quy tắc biến đổi.
4.  **Output**:
    - Nếu cần biến đổi:
      - Gửi Backspace(s) sử dụng `XTestFakeKeyEvent`.
      - Gửi ký tự Unicode mới.
        - _Lưu ý_: Gửi Unicode trên X11 khá phức tạp. Thường yêu cầu gửi chuỗi `Ctrl+Shift+U` hoặc sử dụng Input Method Framework phù hợp (IBus/Fcitx).
        - Cho MVP: Có thể giả lập paste clipboard (`Ctrl+V`) hoặc các chuỗi phím cụ thể.

## 6. Rust `x11` Crate

Sử dụng crate `x11` cho các bindings.

```rust
use x11::xlib::{XOpenDisplay, XNextEvent, XKeysymToKeycode, XK_BackSpace};
use x11::xtest::XTestFakeKeyEvent;
```
