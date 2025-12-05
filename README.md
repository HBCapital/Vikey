# Vikey - Bộ Gõ Tiếng Việt Đa Nền Tảng

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

Vikey là bộ gõ tiếng Việt đa nền tảng được phát triển bằng ngôn ngữ Rust, mã nguồn mở với giấy phép Apache 2.0.

## ✨ Tính Năng

- 🚀 **Hiệu năng cao**: Được viết bằng Rust, tối ưu hóa tốc độ và hiệu suất
- 🌍 **Đa nền tảng**: Hỗ trợ Windows, macOS và Linux
- ⌨️ **Nhiều kiểu gõ**: Telex, VNI, VIQR
- 🎯 **Chính xác**: Engine xử lý tiếng Việt thông minh
- 🔧 **Dễ tùy chỉnh**: Cấu hình linh hoạt theo nhu cầu

## 📋 Yêu Cầu Hệ Thống

- Rust 1.70 trở lên
- Cargo (đi kèm với Rust)

### Windows

- Windows 10 trở lên
- Visual Studio Build Tools

### macOS

- macOS 10.15 (Catalina) trở lên
- Xcode Command Line Tools

### Linux

- X11 hoặc Wayland
- GCC hoặc Clang

## 🚀 Cài Đặt

### Từ Source

```bash
# Clone repository
git clone https://github.com/yourusername/vikey.git
cd vikey

# Build dự án
cargo build --release

# Chạy
cargo run --release
```

### Từ Cargo

```bash
cargo install vikey
```

## 🔨 Build

```bash
# Build debug
cargo build

# Build release (tối ưu hóa)
cargo build --release

# Chạy tests
cargo test

# Chạy với logging
RUST_LOG=debug cargo run
```

## 📖 Sử Dụng

```rust
use vikey_core::{Engine, InputMethod};

fn main() {
    let mut engine = Engine::new(InputMethod::Telex);

    // Xử lý input
    let result = engine.process("tieng viet");
    println!("{}", result); // "tiếng việt"
}
```

## 🏗️ Kiến Trúc

```
vikey/
├── crates/
│   ├── vikey-core/       # Core processing engine
│   ├── vikey-vietnamese/ # Vietnamese input methods (Telex/VNI/VIQR)
│   ├── vikey-platform/   # Platform integrations (Windows/Mac/Linux)
│   └── vikey-config/     # Configuration management
├── docs/                 # Documentation
└── references/           # Reference implementations
```

Xem chi tiết tại [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## 🤝 Đóng Góp

Chúng tôi rất hoan nghênh mọi đóng góp! Vui lòng đọc [CONTRIBUTING.md](CONTRIBUTING.md) để biết thêm chi tiết.

### Quy Trình Đóng Góp

1. Fork dự án
2. Tạo branch cho tính năng (`git checkout -b feature/AmazingFeature`)
3. Commit thay đổi (`git commit -m 'Add some AmazingFeature'`)
4. Push lên branch (`git push origin feature/AmazingFeature`)
5. Mở Pull Request

## 📝 Giấy Phép

Dự án này được phân phối dưới giấy phép Apache 2.0. Xem file [LICENSE](LICENSE) để biết thêm chi tiết.

## 👥 Tác Giả

- Vikey Contributors

## 🙏 Cảm Ơn

- Cộng đồng Rust
- Các dự án bộ gõ tiếng Việt mã nguồn mở khác

## 📞 Liên Hệ

- Issues: [GitHub Issues](https://github.com/yourusername/vikey/issues)
- Discussions: [GitHub Discussions](https://github.com/yourusername/vikey/discussions)

---

Made with ❤️ by Vikey Contributors
