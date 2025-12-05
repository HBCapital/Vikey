# Phân Tích Wayland & Chiến Lược Kiến Trúc Vikey

## 1. Thách Thức: Wayland vs. Legacy (X11/Windows)

| Tính Năng        | Legacy (X11/Windows)              | Wayland                                 | Ảnh Hưởng Tới Vikey                                |
| ---------------- | --------------------------------- | --------------------------------------- | -------------------------------------------------- |
| **Key Sniffing** | Global hooks / Grab keyboard      | ❌ **Bị cấm** bởi Security Model        | Không thể dùng global hook để bắt phím.            |
| **Input Flow**   | Application -> IME -> Application | Compositor -> IME -> Application        | Cần giao tiếp với Compositor (KWin, Mutter, Sway). |
| **Protocol**     | XIM, Windows IME API              | `text-input` & `input-method` protocols | Phải implement protocol riêng cho Wayland.         |
| **Coordinates**  | Global screen coordinates         | ❌ Không có tọa độ global cho client    | Khó vẽ popup/candidate window đúng chỗ.            |

## 2. Các Giao Thức Input Trên Wayland

### 2.1 `input-method-v2` (Unstable/Experimental)

- **Vai trò**: Giao thức dành riêng cho IME.
- **Chức năng**: Cho phép Vikey nhận raw keyboard events từ Compositor và gửi lại committed text.
- **Trạng thái**: Hỗ trợ tốt trên Sway, Hyprland, KWin (gần đây). GNOME hỗ trợ hạn chế.

### 2.2 `text-input-v3`

- **Vai trò**: Giao thức cho Client App (Text Editor, Browser, Terminal).
- **Chức năng**: App thông báo cho Compositor về state (vị trí con trỏ, văn bản xung quanh). Compositor sẽ chuyển tiếp thông tin này cho IME.

## 3. Chiến Lược Tham Khảo: Fcitx5

Fcitx5 sử dụng chiến lược **Hybrid** rất thông minh để đảm bảo tương thích:

1.  **Native Wayland**: Sử dụng `text-input-v3` khi Compositor hỗ trợ.
2.  **Legacy Modules**: Vẫn duy trì `GTK_IM_MODULE` và `QT_IM_MODULE` để inject vào các ứng dụng chưa hỗ trợ tốt Wayland native input.
3.  **Extension**: Sử dụng extension riêng để hỗ trợ GNOME (kimpanel) trong việc vẽ popup.

## 4. Kiến Trúc Đề Xuất Cho Vikey (Linux/Wayland)

Vikey nên được tách thành kiến trúc dạng module:

```rust
// vikey-core: Logic xử lý tiếng Việt (Platform Agnostic)
pub struct Engine { ... }

// vikey-wayland: Backend dành riêng cho Wayland
pub struct WaylandBackend {
    // Sử dụng smithay-client-toolkit hoặc wayland-client
    input_method_manager: InputMethodManager,
}
```

### 4.1 Recommended Rust Stack

1.  **`wayland-client`**: Core Rust implementation của giao thức Wayland.
2.  **`smithay-client-toolkit` (SCTK)**: High-level wrapper, có sẵn module xử lý input method cơ bản.
3.  **`zwp-input-method-service`**: Crate chuyên biệt để implement giao thức `input-method-v2`.

### 4.2 Chiến Lược Vẽ Popup Window

Vẽ Candidate Window (cửa sổ gợi ý) là thách thức lớn nhất trên Wayland do thiếu tọa độ toàn cục.

- **Option 1 (Layer Shell)**: Sử dụng `zwlr_layer_shell_v1` để vẽ overlay layer.
- **Option 2 (Client-side)**: Render cửa sổ ngay bên trong ứng dụng (khó implement và phụ thuộc toolkit).
- **Option 3 (Hybrid)**: Sử dụng chiến lược như Fcitx5 (nhờ Compositor vẽ giúp nếu được hỗ trợ qua protocol).

## 5. Các Bước Tiếp Theo Cho Vikey

1.  **Research Prototype**: Tạo một Rust prototype nhỏ sử dụng `zwp-input-method-service` để thử nghiệm việc "chặn" ký tự 'a' và output ra 'â'.
2.  **Architectural Split**: Tách biệt rõ ràng `vikey-core` khỏi các logic đặc thù của từng nền tảng (platform-specific).
3.  **Roadmap Update**: Thêm Wayland support vào Phase 3 hoặc 4 (sau khi hoàn thành framework cho chữ Nôm).
