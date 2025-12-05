# Tài Liệu Tham Khảo Windows Low-Level Keyboard Hook

## 1. Tổng Quan

Để chặn và sửa đổi input bàn phím toàn cục trên Windows mà không cần viết driver đầy đủ, phương pháp tiêu chuẩn (được sử dụng bởi UniKey, OpenKey) là `SetWindowsHookEx` với `WH_KEYBOARD_LL`.

## 2. Các Hàm API

### SetWindowsHookExA

```c
HHOOK SetWindowsHookExA(
  int       idHook,     // WH_KEYBOARD_LL (13)
  HOOKPROC  lpfn,       // Con trỏ tới hàm hook procedure
  HINSTANCE hmod,       // Handle tới DLL (NULL cho low-level hook)
  DWORD     dwThreadId  // 0 cho tất cả các threads
);
```

### LowLevelKeyboardProc

Chữ ký hàm callback:

```c
LRESULT CALLBACK LowLevelKeyboardProc(
  int    nCode,  // HC_ACTION (0)
  WPARAM wParam, // WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN, WM_SYSKEYUP
  LPARAM lParam  // Con trỏ tới KBDLLHOOKSTRUCT
);
```

**Giá Trị Trả Về:**

- `CallNextHookEx(NULL, nCode, wParam, lParam)` để chuyển tiếp phím.
- `1` để nuốt phím (ngăn các ứng dụng khác nhìn thấy nó).

### KBDLLHOOKSTRUCT

```c
typedef struct tagKBDLLHOOKSTRUCT {
  DWORD     vkCode;      // Virtual Key Code (1-254)
  DWORD     scanCode;    // Hardware Scan Code
  DWORD     flags;       // LLKHF_EXTENDED, LLKHF_INJECTED, v.v.
  DWORD     time;        // Timestamp
  ULONG_PTR dwExtraInfo; // Thông tin bổ sung
} KBDLLHOOKSTRUCT, *PKBDLLHOOKSTRUCT;
```

**Flags Quan Trọng:**

- `LLKHF_INJECTED (0x00000010)`: Sự kiện được inject (bởi `SendInput`). Rất quan trọng để phát hiện output của chính mình nhằm tránh vòng lặp vô hạn.

## 3. Gửi Input (Sending Input)

Để output các ký tự đã sửa đổi (ví dụ: Backspace + Ký tự mới), sử dụng `SendInput`.

### SendInput

```c
UINT SendInput(
  UINT    cInputs,  // Số lượng inputs
  LPINPUT pInputs,  // Mảng các cấu trúc INPUT
  int     cbSize    // sizeof(INPUT)
);
```

### Cấu Trúc INPUT

```c
typedef struct tagINPUT {
  DWORD type; // INPUT_KEYBOARD (1)
  union {
    MOUSEINPUT    mi;
    KEYBDINPUT    ki;
    HARDWAREINPUT hi;
  } DUMMYUNIONNAME;
} INPUT, *PINPUT, *LPINPUT;
```

### Cấu Trúc KEYBDINPUT

```c
typedef struct tagKEYBDINPUT {
  WORD      wVk;         // Virtual Key Code
  WORD      wScan;       // Scan Code
  DWORD     dwFlags;     // KEYEVENTF_KEYUP, KEYEVENTF_UNICODE
  DWORD     time;        // 0
  ULONG_PTR dwExtraInfo; // Thông tin bổ sung
} KEYBDINPUT, *PKEYBDINPUT;
```

**Gửi Unicode:**

- Đặt `wVk = 0`.
- Đặt `wScan = unicode_character`.
- Đặt `dwFlags = KEYEVENTF_UNICODE`.

## 4. Chiến Lược Implementation Cho Vikey

1.  **Cài Đặt Hook**: Khi khởi động, gọi `SetWindowsHookEx(WH_KEYBOARD_LL, ...)`.
2.  **Chặn (Intercept)**: Trong `LowLevelKeyboardProc`:
    - Kiểm tra `nCode == HC_ACTION`.
    - Kiểm tra `!(p->flags & LLKHF_INJECTED)` để bỏ qua output của chính mình.
    - Nếu phím liên quan (A-Z, 0-9), xử lý nó.
3.  **Xử Lý (Process)**:
    - Cập nhật trạng thái/buffer nội bộ.
    - Nếu có biến đổi (transformation):
      - Trả về `1` để nuốt phím gốc.
      - Gọi `SendInput` để gửi Backspace(s) + Ký tự mới.
    - Nếu không có biến đổi:
      - Trả về `CallNextHookEx(...)`.
4.  **Gỡ Bỏ (Uninstall)**: Gọi `UnhookWindowsHookEx` khi thoát.

## 5. Rust `windows` Crate

Sử dụng crate `windows` để truy cập API an toàn kiểu (type-safe).

```rust
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowsHookExA, UnhookWindowsHookEx, CallNextHookEx,
    WH_KEYBOARD_LL, KBDLLHOOKSTRUCT, HOOKPROC,
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_UNICODE, KEYEVENTF_KEYUP
};
```
