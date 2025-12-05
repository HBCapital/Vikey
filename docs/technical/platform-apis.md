# Platform APIs

> Tài liệu về các Platform APIs cần sử dụng để implement bộ gõ

## Tổng Quan

Mỗi platform có cách tiếp cận khác nhau để implement Input Method Editor (IME).

---

## Windows

### Text Services Framework (TSF)

**API chính:**

- `ITfThreadMgr` - Thread manager
- `ITfContext` - Input context
- `ITfCompositionSink` - Composition events
- `ITfTextInputProcessor` - Text input processor

**Tài liệu:**

- https://docs.microsoft.com/en-us/windows/win32/tsf/text-services-framework

### Keyboard Hook (Low-level alternative)

**API:**

- `SetWindowsHookEx` với `WH_KEYBOARD_LL`
- `CallNextHookEx`
- `UnhookWindowsHookEx`

**Ưu điểm:**

- [ ] Đơn giản hơn TSF
- [ ] Dễ implement

**Nhược điểm:**

- [ ] Không integrate với Windows IME framework
- [ ] Có thể bị antivirus block

### Rust Crates

```toml
[dependencies]
winapi = { version = "0.3", features = ["winuser", "processthreadsapi"] }
windows = "0.48"
```

### Code Example (Concept)

```rust
// Keyboard hook
unsafe extern "system" fn keyboard_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // Process keyboard input
    // Transform Vietnamese
    // Send output
}
```

---

## macOS

### Input Method Kit (IMKit)

**Classes:**

- `IMKServer` - Input method server
- `IMKInputController` - Input controller
- `NSEvent` - Keyboard events

**Tài liệu:**

- https://developer.apple.com/documentation/inputmethodkit

### Event Tap (Low-level alternative)

**API:**

- `CGEventTapCreate`
- `CGEventPost`
- `CGEventTapEnable`

**Permissions:**

- Cần Accessibility permissions

### Rust Crates

```toml
[dependencies]
cocoa = "0.25"
objc = "0.2"
core-foundation = "0.9"
core-graphics = "0.23"
```

### Code Example (Concept)

```rust
// Event tap callback
extern "C" fn event_callback(
    proxy: CGEventTapProxy,
    event_type: CGEventType,
    event: CGEventRef,
    user_info: *mut c_void,
) -> CGEventRef {
    // Process keyboard event
    // Transform Vietnamese
    // Return modified event
}
```

---

## Linux

### Multiple Approaches

Linux có nhiều input method frameworks:

#### 1. IBus (Input Bus)

**Components:**

- IBus daemon
- IBus engine
- D-Bus communication

**Tài liệu:**

- https://github.com/ibus/ibus

**Ưu điểm:**

- [ ] Phổ biến nhất
- [ ] Hỗ trợ nhiều DE

**Nhược điểm:**

- [ ] Phức tạp
- [ ] Cần D-Bus

#### 2. Fcitx

**Components:**

- Fcitx framework
- Input method modules

**Tài liệu:**

- https://fcitx-im.org/

**Ưu điểm:**

- [ ] Hiệu năng tốt
- [ ] Dễ customize

#### 3. X11 (Low-level)

**API:**

- `XGrabKeyboard`
- `XSendEvent`
- `XTestFakeKeyEvent`

**Rust Crates:**

```toml
[dependencies]
x11 = { version = "2.21", features = ["xlib", "xtest"] }
```

#### 4. Wayland

**Protocol:**

- `zwp_input_method_v2`
- `zwp_virtual_keyboard_v1`

**Tài liệu:**

- https://wayland.app/protocols/input-method-unstable-v2

---

## So Sánh Approaches

### High-level vs Low-level

| Aspect            | High-level (TSF/IMKit/IBus) | Low-level (Hooks/Events) |
| ----------------- | --------------------------- | ------------------------ |
| Complexity        | ⭐⭐⭐⭐⭐                  | ⭐⭐                     |
| Integration       | ⭐⭐⭐⭐⭐                  | ⭐⭐                     |
| Permissions       | ⭐⭐⭐                      | ⭐⭐⭐⭐                 |
| Compatibility     | ⭐⭐⭐⭐⭐                  | ⭐⭐⭐                   |
| Development Speed | ⭐⭐                        | ⭐⭐⭐⭐                 |

---

## Quyết Định Cho Vikey

### Phase 1: MVP (Low-level)

**Lý do:**

- Nhanh hơn để prototype
- Dễ debug
- Cross-platform với cùng approach

**Approach:**

- Windows: Keyboard Hook
- macOS: Event Tap
- Linux: X11 (XTest)

### Phase 2: Production (High-level)

**Lý do:**

- Better system integration
- Professional solution
- Follow platform guidelines

**Approach:**

- Windows: TSF
- macOS: IMKit
- Linux: IBus + Fcitx

---

## Security Considerations

### Permissions Required

**Windows:**

- [ ] Administrator (for low-level hook)
- [ ] No special permission (for TSF)

**macOS:**

- [ ] Accessibility permission (required)
- [ ] Input Monitoring permission

**Linux:**

- [ ] X11: No special permission
- [ ] Wayland: Compositor support

### Sandboxing

- [ ] Windows Store apps: Restricted
- [ ] macOS App Store: Restricted
- [ ] Flatpak/Snap: May need special permissions

---

## Testing Strategy

### Per-Platform Testing

- [ ] Test on multiple OS versions
- [ ] Test with different applications
- [ ] Test permission flows
- [ ] Test performance impact

### Compatibility Testing

**Windows:**

- [ ] Windows 10
- [ ] Windows 11
- [ ] Different applications (Office, browsers, etc.)

**macOS:**

- [ ] macOS 12 (Monterey)
- [ ] macOS 13 (Ventura)
- [ ] macOS 14 (Sonoma)

**Linux:**

- [ ] X11 (Ubuntu, Fedora)
- [ ] Wayland (Ubuntu, Fedora)
- [ ] Different DEs (GNOME, KDE, etc.)

---

## Resources

### Official Documentation

- Windows TSF: https://docs.microsoft.com/en-us/windows/win32/tsf/
- macOS IMKit: https://developer.apple.com/documentation/inputmethodkit
- IBus: https://github.com/ibus/ibus/wiki
- Fcitx: https://fcitx-im.org/wiki/

### Example Projects

- [Link to reference projects in references/ folder]

### Rust Resources

- winapi crate: https://docs.rs/winapi/
- cocoa crate: https://docs.rs/cocoa/
- x11 crate: https://docs.rs/x11/
