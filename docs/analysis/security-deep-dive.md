# Phân Tích Chuyên Sâu: Security & Kiến Trúc IME Hiện Đại

> Góc nhìn Senior Developer & System Architect về cách tích hợp Input Method Editor vào các OS hiện đại theo đúng quy chuẩn.

**Ngày phân tích**: 2025-12-05

---

## 1. Bối Cảnh: Mô Hình Bảo Mật OS Hiện Đại

### 1.1 Tại Sao OS Hạn Chế Key Capture

Các OS hiện đại áp dụng các biện pháp bảo mật nghiêm ngặt để bảo vệ người dùng:

| Mối Đe Dọa            | Mô Tả                         | Hậu Quả                          |
| --------------------- | ----------------------------- | -------------------------------- |
| **Keylogger**         | Malware ghi lại mọi keystroke | Mất password, thông tin nhạy cảm |
| **Credential Theft**  | Inject vào password field     | Đánh cắp thông tin đăng nhập     |
| **Data Exfiltration** | Key capture + network access  | Gửi dữ liệu ra ngoài             |

### 1.2 Triết Lý Bảo Mật Của Từng Platform

| Platform    | Triết Lý                                                                 |
| ----------- | ------------------------------------------------------------------------ |
| **Windows** | "Trust but Verify" - Cho phép nhưng yêu cầu ký số + manifest đặc biệt    |
| **macOS**   | "User Consent" - Yêu cầu người dùng cấp quyền rõ ràng                    |
| **Wayland** | "Zero Trust" - Client không biết gì về client khác, Compositor kiểm soát |
| **Android** | "Sandbox Everything" - Mỗi IME chạy trong sandbox riêng biệt             |

---

## 2. Phương Pháp Tích Hợp Hợp Lệ Theo Platform

### 2.1 Windows: Text Services Framework (TSF)

#### Kiến Trúc Chính Thức

```
┌─────────────────────────────────────────────────────────────┐
│                    Application (TSF-Enabled)                │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              ITfContext (Text Store)                 │   │
│  └────────────────────────┬─────────────────────────────┘   │
└───────────────────────────│─────────────────────────────────┘
                            │ COM Interface
┌───────────────────────────▼─────────────────────────────────┐
│                    TSF Manager (msctf.dll)                  │
│              [System Component - Immutable]                 │
└───────────────────────────┬─────────────────────────────────┘
                            │
┌───────────────────────────▼─────────────────────────────────┐
│              Text Service (Vikey TSF DLL)                   │
│  [Nhận keystroke từ TSF → Xử lý → Gửi text về TSF Manager]  │
└─────────────────────────────────────────────────────────────┘
```

**Đặc điểm**: TSF là **Inversion of Control** - App gọi TSF, TSF gọi IME. IME không chủ động bắt phím mà được hệ thống phân phối sự kiện.

#### Yêu Cầu Cho Elevated Apps (uiAccess)

Để IME hoạt động với các ứng dụng chạy quyền Admin:

1. ✅ **Code Signing**: Binary PHẢI được ký bằng certificate có trust chain
2. ✅ **Install Location**: PHẢI nằm trong `Program Files` hoặc `Windows\System32`
3. ✅ **Manifest**: PHẢI có `uiAccess="true"` trong embedded manifest
4. ✅ **Installer**: PHẢI cài qua installer yêu cầu Admin

```xml
<!-- Vikey.exe.manifest -->
<requestedExecutionLevel level="asInvoker" uiAccess="true" />
```

### 2.2 macOS: InputMethodKit

#### Kiến Trúc Chính Thức

```
┌─────────────────────────────────────────────────────────────┐
│              Input Method Bundle (.app)                     │
│  ┌─────────────────────────────────────────────────────┐    │
│  │ IMKInputController                                   │    │
│  │   - handle(_:client:) → Nhận NSEvent từ system      │    │
│  │   - commitComposition() → Gửi text ra app           │    │
│  │   - candidates() → Trả về danh sách gợi ý           │    │
│  └─────────────────────────────────────────────────────┘    │
├─────────────────────────────────────────────────────────────┤
│  Requirements:                                              │
│  ✅ Code Signed (Developer ID or Mac App Store)            │
│  ✅ Notarized (required macOS 10.15+)                      │
│  ✅ Input Sources entitlement                              │
└─────────────────────────────────────────────────────────────┘
```

#### Secure Input Mode

Khi macOS bật Secure Input Mode (password fields):

- IME **KHÔNG nhận được** keystroke
- Đây là thiết kế **By Design** của Apple
- **Chiến lược**: Detect và hiển thị thông báo cho user

```swift
func isSecureInputActive() -> Bool {
    return IsSecureEventInputEnabled()
}
```

### 2.3 Wayland: Protocol-Based

#### Kiến Trúc Chính Thức

```
┌─────────────────────────────────────────────────────────────┐
│                    Wayland Compositor                       │
│  (Sway / KWin / Mutter / Hyprland)                         │
└──────────────┬──────────────────────────────────────────────┘
               │ zwp_input_method_v2
               ▼
┌─────────────────────────────────────────────────────────────┐
│                    vikey-wayland                            │
│  - Nhận keyboard events từ Compositor                       │
│  - Gửi commit_string / set_preedit_string                   │
│  - Quản lý zwp_input_popup_surface_v2                       │
└─────────────────────────────────────────────────────────────┘
```

**Đặc điểm**: IME chỉ nhận events qua protocol chính thức. Không có cách nào để "grab" toàn bộ keyboard như X11.

---

## 3. Nguyên Tắc Thiết Kế

### 3.1 Tuân Thủ Security Model

1. **Separation of Concerns**: Core logic PHẢI platform-agnostic
2. **Minimal Permissions**: Không yêu cầu quyền cao hơn cần thiết
3. **Graceful Degradation**: Detect limitations và thông báo user
4. **Security by Design**: Không cố làm những gì OS không cho phép

### 3.2 Kiến Trúc Đề Xuất

```
┌─────────────────────────────────────────────────────────────────────┐
│                          Platform Layer                             │
├──────────────────────┬──────────────────────┬───────────────────────┤
│   vikey-windows-tsf  │   vikey-macos-imk    │    vikey-wayland      │
│   (TSF Text Service) │   (IMK Bundle)       │    (Wayland Client)   │
│                      │                      │                       │
│   Requirements:      │   Requirements:      │   Requirements:       │
│   - Code signed      │   - Code signed      │   - None              │
│   - uiAccess=true    │   - Notarized        │                       │
│   - Program Files    │                      │   Protocols:          │
│                      │                      │   - input-method-v2   │
└──────────┬───────────┴──────────┬───────────┴───────────┬───────────┘
           │                      │                       │
           ▼                      ▼                       ▼
┌─────────────────────────────────────────────────────────────────────┐
│                           Core Layer                                │
├─────────────────────────────────────────────────────────────────────┤
│  vikey-core         │  vikey-vietnamese  │  vikey-nom              │
│  - Engine           │  - Telex           │  - Nôm transformer      │
│  - Buffer           │  - VNI             │  - Dictionary (FST)     │
│  - State Machine    │  - Tone placement  │                         │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 4. Bảng Tổng Hợp API Chính Thức

| Platform    | API Chính Thức                | Popup Window               |
| ----------- | ----------------------------- | -------------------------- |
| **Windows** | TSF (Text Services Framework) | Win32 API                  |
| **macOS**   | InputMethodKit                | AppKit                     |
| **Wayland** | input-method-v2 protocol      | zwp_input_popup_surface_v2 |
| **X11**     | XIM protocol                  | X11 API                    |
| **Android** | InputMethodService            | Android View system        |

---

## 5. Kết Luận

Vikey sẽ tích hợp vào từng OS bằng cách sử dụng **các API chính thức** được thiết kế dành riêng cho Input Method:

- **Windows**: TSF với code signing và uiAccess
- **macOS**: InputMethodKit với notarization
- **Wayland**: input-method-v2 protocol

Tất cả đều là các phương pháp được Microsoft, Apple, và Wayland project thiết kế và khuyến khích sử dụng cho IME.
