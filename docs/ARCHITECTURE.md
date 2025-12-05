# Kiến Trúc Vikey

## Tổng Quan

Vikey được thiết kế theo kiến trúc **Core/Shell Separation** (Tách biệt Lõi/Vỏ) để đảm bảo tính linh hoạt và khả năng tái sử dụng tối đa.

### Triết Lý Thiết Kế

1.  **Core (Lõi)**: Là một thư viện Rust thuần túy (Library), không phụ thuộc vào giao diện người dùng hay hệ điều hành cụ thể. Có thể nhúng vào bất kỳ ứng dụng nào (Game, WebAssembly, Embedded).
2.  **Shell (Vỏ)**: Là ứng dụng thực thi (Application), chịu trách nhiệm tương tác với Hệ điều hành (Windows/macOS/Linux), quản lý cấu hình và giao diện người dùng.

## Cấu Trúc Tổng Thể

```
┌─────────────────────────────────────┐
│          Application Shell          │
│       (vikey-cli / vikey-gui)       │
│  [OS Integration, UI, Config Mgmt]  │
└──────────────────┬──────────────────┘
                   │
                   ▼
┌─────────────────────────────────────┐
│             Core Library            │
│            (vikey-core)             │
│   [Logic, State, Transformation]    │
└──────────────────┬──────────────────┘
                   │
          ┌────────┴────────┐
          ▼                 ▼
  ┌───────────────┐  ┌─────────────────┐
  │vikey-vietnamese│  │ vikey-platform  │
  │ (Input Rules) │  │ (OS Adapters)   │
  └───────────────┘  └─────────────────┘
```

## Cấu Trúc Workspace

```
vikey/
├── Cargo.toml              # Cấu hình Workspace
├── crates/
│   ├── vikey-core/         # Core processing engine (Xử lý chính)
│   ├── vikey-vietnamese/   # Vietnamese input methods (Bộ gõ tiếng Việt)
│   ├── vikey-platform/     # Platform integrations (Tích hợp hệ điều hành)
│   └── vikey-config/       # Configuration management (Quản lý cấu hình)
├── docs/                   # Tài liệu
└── references/             # Mã nguồn tham khảo
```

## Các Crate Chính

### 1. vikey-core (`crates/vikey-core`)

Core engine xử lý input, độc lập với platform (platform-agnostic):

- **Processor**: Xử lý sự kiện phím (key events) và điều phối
- **Buffer**: Quản lý bộ đệm input
- **State Machine**: Quản lý trạng thái (Initial → Buffering → Processing → Committed)
- **Transform**: Pipeline chuyển đổi ký tự
- **Types**: Các kiểu dữ liệu chung (KeyEvent, Action, KeyModifiers)

**Dependencies**: `thiserror`, `anyhow`

### 2. vikey-vietnamese (`crates/vikey-vietnamese`)

Logic đặc thù cho tiếng Việt:

- **Telex**: Kiểu gõ Telex (aa → â, aw → ă, dd → đ)
- **VNI**: Kiểu gõ VNI (a6 → ă, a7 → â)
- **VIQR**: Kiểu gõ VIQR (a( → ă, a^ → â)
- **Rules**: Quy tắc tiếng Việt và kiểm tra hợp lệ (validation)
- **Unicode**: Chuẩn hóa Unicode (NFC)

**Dependencies**: `vikey-core`, `unicode-normalization`, `lazy_static`

### 3. vikey-platform (`crates/vikey-platform`)

Các implementation đặc thù cho từng platform:

#### Windows

- Windows API hooks
- Chặn sự kiện bàn phím
- Tích hợp IME

#### macOS

- Cocoa framework
- Event tap
- Tích hợp Input method

#### Linux

- Xử lý sự kiện X11
- Tích hợp IBus/Fcitx
- Hỗ trợ Wayland (kế hoạch)

### 4. vikey-config (`crates/vikey-config`)

Quản lý cấu hình:

- Lưu trữ cài đặt người dùng
- Tùy chọn kiểu gõ
- Cấu hình giao diện (theme)

## Luồng Xử Lý

```
User Input → Platform Layer → Processor → Transform → Output
```

1. **User Input**: Người dùng gõ phím
2. **Platform Layer**: Bắt KeyEvent từ Hệ điều hành
3. **Processor**: Xử lý event, quản lý buffer và state
4. **Transform**: Áp dụng quy tắc tiếng Việt (Telex/VNI/VIQR)
5. **Output**: Trả về Action (Replace/Commit/DoNothing)

## Mẫu Thiết Kế (Design Patterns)

### 1. Strategy Pattern

Trait `Transformer` cho các kiểu gõ (input methods):

```rust
pub trait Transformer {
    fn transform(&self, input: &str) -> Option<TransformResult>;
    fn name(&self) -> &str;
}
```

### 2. State Machine Pattern

Quản lý trạng thái input:

```
Initial → Buffering → Processing → Committed
                ↑          │
                └──────────┘
```

### 3. Platform Abstraction

`vikey-platform` trừu tượng hóa mã nguồn đặc thù của platform với giao diện chung (common interface).

## Biểu Đồ Phụ Thuộc (Dependency Graph)

```
vikey-cli (planned)
├── vikey-config
├── vikey-core
│   └── (thiserror, anyhow)
├── vikey-vietnamese
│   ├── vikey-core
│   └── (unicode-normalization, lazy_static)
└── vikey-platform
    └── (windows/cocoa/x11 - conditional)
```

## Chiến Lược Kiểm Thử (Testing Strategy)

- **Unit Tests**: Mỗi crate có tests trong `src/*.rs`
- **Integration Tests**: Test tương tác giữa các crates
- **Platform Tests**: Test trên từng platform

```bash
cargo test              # Chạy tất cả tests
cargo test -p vikey-core       # Test crate cụ thể
```

## Lộ Trình Phát Triển (Roadmap)

### Giai Đoạn 1: MVP (Hiện Tại)

1. **Core Engine**: Xử lý Telex/VNI cơ bản
2. **Platform Support**: Windows, macOS, Linux (X11)
3. **Unicode**: Hỗ trợ NFC/NFD
4. **Basic UI**: Cấu hình đơn giản

### Giai Đoạn 2: Mở Rộng Tính Năng

#### 2.1 Hỗ Trợ Ngôn Ngữ Dân Tộc Thiểu Số

**Tiếng Việt Cổ & Chữ Nôm:**

- Chữ Nôm (𡨸喃): Hỗ trợ Unicode Extension B, C, D
- Tiếng Việt cổ: Các dấu thanh và ký tự không còn sử dụng
- Input method: Telex mở rộng cho Nôm
- Font rendering: Tích hợp font Nôm (HAN NOM A, B)

**Ngôn Ngữ Dân Tộc:**

- **Chữ Thái** (Tày, Nùng, Thái): Unicode U+1A20–U+1AAF
- **Chữ Mường**: Dựa trên chữ Latinh mở rộng
- **Chữ H'Mông**: Pahawh Hmong (U+16B00–U+16B8F), Pollard (U+A4D0–U+A4FF)
- **Chữ Chăm**: Cham script (U+AA00–U+AA5F)

**Tính năng:**

- Bộ gõ riêng cho từng ngôn ngữ
- Từ điển và gợi ý thông minh
- Chuyển đổi giữa các hệ chữ viết
- Hỗ trợ rendering đúng cho các ký tự phức tạp

#### 2.2 Voice-to-Text (Giọng Nói Sang Văn Bản)

**Kiến trúc:**

```
Microphone → Audio Processing → Speech Recognition → Text Output
                                        ↓
                                Vietnamese Language Model
                                        ↓
                                Vikey Engine (Post-processing)
```

**Tính năng:**

- **Offline Mode**: Sử dụng model nhẹ (Whisper tiny/base)
- **Online Mode**: Tích hợp Google Speech API / Azure Speech
- **Hybrid Mode**: Offline + cloud fallback
- **Accent Support**: Miền Bắc, Trung, Nam
- **Punctuation**: Tự động thêm dấu câu
- **Commands**: Voice commands để điều khiển (bật/tắt, chuyển kiểu gõ)

**Technical Stack:**

- **Speech Recognition**: Whisper (OpenAI), Vosk, hoặc cloud APIs
- **Audio Processing**: `cpal` crate (Rust audio)
- **VAD**: Voice Activity Detection để tiết kiệm tài nguyên
- **Post-processing**: Vikey engine để chuẩn hóa Unicode, sửa lỗi

**Challenges:**

- Độ chính xác với giọng địa phương
- Latency (mục tiêu < 500ms)
- Privacy (ưu tiên offline mode)
- Resource usage (CPU/Memory cho model)

### Giai Đoạn 3: Nâng Cao

1. **Machine Learning**: Gợi ý thông minh dựa trên ngữ cảnh
2. **Cloud Sync**: Đồng bộ cấu hình và từ điển cá nhân
3. **Plugin System**: Mở rộng tính năng qua plugins
4. **Multi-language**: Hỗ trợ các ngôn ngữ khác (Lào, Khmer, Thái Lan)
