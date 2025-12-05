# Các Mẫu Rust FFI Cho Platform APIs

## 1. Tổng Quan

Vikey cần gọi các C API đặc thù của platform từ Rust. Tài liệu này bao gồm các mẫu FFI (Foreign Function Interface) và các thực hành tốt nhất (best practices).

## 2. Windows FFI (sử dụng crate `windows`)

### 2.1 Cài Đặt Cơ Bản

```toml
[dependencies]
windows = { version = "0.52", features = [
    "Win32_Foundation",
    "Win32_UI_WindowsAndMessaging",
    "Win32_UI_Input_KeyboardAndMouse",
] }
```

### 2.2 Ví Dụ Cài Đặt Hook

```rust
use windows::Win32::UI::WindowsAndMessaging::{
    SetWindowsHookExA, UnhookWindowsHookEx, CallNextHookEx,
    WH_KEYBOARD_LL, KBDLLHOOKSTRUCT, HHOOK, HOOKPROC,
};
use windows::Win32::Foundation::{WPARAM, LPARAM, LRESULT};

static mut HOOK_HANDLE: Option<HHOOK> = None;

unsafe extern "system" fn keyboard_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code >= 0 {
        let kb_struct = &*(lparam.0 as *const KBDLLHOOKSTRUCT);

        // Kiểm tra nếu sự kiện được inject (tránh vòng lặp vô hạn)
        if kb_struct.flags & 0x00000010 == 0 {
            // Xử lý phím...
            // Trả về LRESULT(1) để nuốt phím
        }
    }

    CallNextHookEx(None, code, wparam, lparam)
}

pub fn install_hook() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let hook = SetWindowsHookExA(
            WH_KEYBOARD_LL,
            Some(keyboard_proc),
            None,
            0,
        )?;
        HOOK_HANDLE = Some(hook);
    }
    Ok(())
}

pub fn uninstall_hook() {
    unsafe {
        if let Some(hook) = HOOK_HANDLE.take() {
            let _ = UnhookWindowsHookEx(hook);
        }
    }
}
```

### 2.3 Gửi Input Unicode

```rust
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_KEYBOARD, KEYBDINPUT,
    KEYEVENTF_UNICODE, KEYEVENTF_KEYUP,
};

fn send_unicode_char(ch: char) -> Result<(), Box<dyn std::error::Error>> {
    let mut inputs = vec![
        // Nhấn phím (Key down)
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: 0,
                    wScan: ch as u16,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // Nhả phím (Key up)
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: 0,
                    wScan: ch as u16,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];

    unsafe {
        SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32);
    }

    Ok(())
}
```

---

## 3. macOS FFI (sử dụng crate `core-graphics`)

### 3.1 Cài Đặt Cơ Bản

```toml
[dependencies]
core-graphics = "0.23"
core-foundation = "0.9"
```

### 3.2 Ví Dụ Event Tap

```rust
use core_graphics::event::{
    CGEvent, CGEventTap, CGEventTapLocation, CGEventTapPlacement,
    CGEventTapOptions, CGEventType, CGEventFlags, CGKeyCode,
};
use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};

unsafe extern "C" fn event_tap_callback(
    _proxy: CGEventTapProxy,
    event_type: CGEventType,
    event: CGEventRef,
    _user_info: *mut c_void,
) -> CGEventRef {
    if event_type == CGEventType::KeyDown {
        let keycode = event.get_integer_value_field(CGEventField::KeyboardEventKeycode);

        // Xử lý phím...
        // Trả về null để nuốt sự kiện
        // return std::ptr::null_mut();
    }

    event // Cho qua
}

pub fn install_event_tap() -> Result<(), Box<dyn std::error::Error>> {
    let event_mask = CGEventMaskBit(CGEventType::KeyDown) |
                     CGEventMaskBit(CGEventType::KeyUp);

    let tap = CGEventTap::new(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        event_mask,
        event_tap_callback,
        std::ptr::null_mut(),
    )?;

    let loop_source = tap.mach_port.create_runloop_source(0)?;
    let run_loop = CFRunLoop::get_current();
    run_loop.add_source(&loop_source, unsafe { kCFRunLoopCommonModes });

    tap.enable();

    Ok(())
}
```

### 3.3 Gửi Unicode

```rust
use core_graphics::event::{CGEvent, CGEventTapLocation};

fn send_unicode_string(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let event_source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)?;

    let event = CGEvent::new_keyboard_event(event_source, 0, true)?;
    event.set_string(text);
    event.post(CGEventTapLocation::HID);

    Ok(())
}
```

---

## 4. Linux X11 FFI (sử dụng crate `x11`)

### 4.1 Cài Đặt Cơ Bản

```toml
[dependencies]
x11 = { version = "2.21", features = ["xlib", "xtest"] }
```

### 4.2 Ví Dụ Vòng Lặp Sự Kiện

```rust
use x11::xlib::*;
use x11::xtest::*;
use std::ptr;

pub struct X11Handler {
    display: *mut Display,
}

impl X11Handler {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        unsafe {
            let display = XOpenDisplay(ptr::null());
            if display.is_null() {
                return Err("Không thể mở display".into());
            }
            Ok(Self { display })
        }
    }

    pub fn run_event_loop(&self) {
        unsafe {
            let root = XDefaultRootWindow(self.display);
            XSelectInput(self.display, root, KeyPressMask | KeyReleaseMask);

            let mut event: XEvent = std::mem::zeroed();
            loop {
                XNextEvent(self.display, &mut event);

                if event.get_type() == KeyPress {
                    let key_event = event.key;
                    let keycode = key_event.keycode;

                    // Xử lý phím...
                }
            }
        }
    }

    pub fn send_backspace(&self) {
        unsafe {
            let backspace_code = XKeysymToKeycode(self.display, XK_BackSpace as u64);
            XTestFakeKeyEvent(self.display, backspace_code as u32, 1, 0); // Nhấn
            XTestFakeKeyEvent(self.display, backspace_code as u32, 0, 0); // Nhả
            XFlush(self.display);
        }
    }
}

impl Drop for X11Handler {
    fn drop(&mut self) {
        unsafe {
            if !self.display.is_null() {
                XCloseDisplay(self.display);
            }
        }
    }
}
```

---

## 5. Hướng Dẫn An Toàn (Safety Guidelines)

### 5.1 Khối Unsafe

- Giữ các khối `unsafe` **nhỏ nhất có thể**
- Ghi chú tại sao mỗi khối unsafe là an toàn
- Validate tất cả các con trỏ trước khi dereference

### 5.2 Xử Lý Lỗi

```rust
// Tệ: Unwrap trong FFI
unsafe {
    let result = some_ffi_call().unwrap(); // ❌ Có thể panic
}

// Tốt: Xử lý lỗi đúng cách
unsafe {
    let result = some_ffi_call()
        .map_err(|e| format!("FFI call failed: {}", e))?; // ✅
}
```

### 5.3 Quản Lý Bộ Nhớ

```rust
// Tệ: Rò rỉ bộ nhớ
let ptr = Box::into_raw(Box::new(data)); // ❌ Không bao giờ được giải phóng

// Tốt: Cleanup đúng cách
let ptr = Box::into_raw(Box::new(data));
// ... sử dụng ptr ...
unsafe { Box::from_raw(ptr); } // ✅ Được giải phóng đúng cách
```

---

## 6. Trừu Tượng Hóa Đa Nền Tảng (Cross-Platform Abstraction)

### 6.1 Thiết Kế Dựa Trên Trait

```rust
pub trait PlatformHook {
    fn install(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn uninstall(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn send_backspace(&self, count: usize) -> Result<(), Box<dyn std::error::Error>>;
    fn send_unicode(&self, text: &str) -> Result<(), Box<dyn std::error::Error>>;
}

#[cfg(target_os = "windows")]
pub struct WindowsHook { /* ... */ }

#[cfg(target_os = "macos")]
pub struct MacOSHook { /* ... */ }

#[cfg(target_os = "linux")]
pub struct LinuxHook { /* ... */ }

// Implement PlatformHook cho từng struct
```

### 6.2 Biên Dịch Có Điều Kiện (Conditional Compilation)

```rust
#[cfg(target_os = "windows")]
use crate::platform::windows::WindowsHook as PlatformImpl;

#[cfg(target_os = "macos")]
use crate::platform::macos::MacOSHook as PlatformImpl;

#[cfg(target_os = "linux")]
use crate::platform::linux::LinuxHook as PlatformImpl;

pub fn create_hook() -> Box<dyn PlatformHook> {
    Box::new(PlatformImpl::new())
}
```

---

## 7. Testing FFI Code

### 7.1 Mock Platform Layer

```rust
#[cfg(test)]
pub struct MockHook {
    sent_keys: Vec<String>,
}

#[cfg(test)]
impl PlatformHook for MockHook {
    fn send_unicode(&self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.sent_keys.push(text.to_string());
        Ok(())
    }
    // ... các phương thức khác
}
```

### 7.2 Integration Tests

```rust
#[test]
#[cfg(not(target_os = "windows"))] // Bỏ qua trên CI nếu không phải Windows
fn test_real_hook() {
    let mut hook = create_hook();
    hook.install().unwrap();
    // ... test hành vi thực tế
    hook.uninstall().unwrap();
}
```

---

## 8. Các Lỗi Thường Gặp

| Vấn Đề                                    | Giải Pháp                                |
| ----------------------------------------- | ---------------------------------------- |
| Vòng lặp vô hạn từ sự kiện của chính mình | Kiểm tra cờ `LLKHF_INJECTED`             |
| Rò rỉ bộ nhớ                              | Sử dụng RAII pattern, implement Drop     |
| Thread safety                             | Sử dụng `Arc<Mutex<T>>` cho shared state |
| Null pointers                             | Luôn validate trước khi dereference      |
| Khác biệt platform                        | Sử dụng trait abstraction                |

---

## 9. Tài Liệu Tham Khảo

- **Rust FFI Guide**: https://doc.rust-lang.org/nomicon/ffi.html
- **windows crate**: https://docs.rs/windows/
- **core-graphics crate**: https://docs.rs/core-graphics/
- **x11 crate**: https://docs.rs/x11/
