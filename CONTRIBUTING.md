# Đóng Góp Cho Vikey

Cảm ơn bạn đã quan tâm đến việc đóng góp cho Vikey! 🎉

## Quy Trình Đóng Góp

### 1. Fork Repository

Fork dự án về tài khoản GitHub của bạn.

### 2. Clone Repository

```bash
git clone https://github.com/your-username/vikey.git
cd vikey
```

### 3. Tạo Branch Mới

```bash
git checkout -b feature/amazing-feature
```

Quy tắc đặt tên branch:

- `feature/` - Tính năng mới
- `fix/` - Sửa lỗi
- `docs/` - Cập nhật tài liệu
- `refactor/` - Refactor code
- `test/` - Thêm tests

### 4. Phát Triển

#### Setup Development Environment

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build project
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt
```

#### Coding Standards

- **Rust Style**: Tuân theo [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
- **Format**: Sử dụng `cargo fmt` trước khi commit
- **Lint**: Đảm bảo `cargo clippy` không có warnings
- **Tests**: Viết tests cho code mới
- **Documentation**: Thêm doc comments cho public APIs

### 5. Commit Changes

```bash
git add .
git commit -m "feat: add amazing feature"
```

#### Commit Message Format

Sử dụng [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:

- `feat`: Tính năng mới
- `fix`: Sửa lỗi
- `docs`: Cập nhật tài liệu
- `style`: Format code
- `refactor`: Refactor code
- `test`: Thêm tests
- `chore`: Maintenance tasks

**Examples**:

```
feat(engine): add VNI input method support
fix(input): resolve keyboard hook memory leak
docs(readme): update installation instructions
```

### 6. Push Changes

```bash
git push origin feature/amazing-feature
```

### 7. Tạo Pull Request

1. Mở Pull Request trên GitHub
2. Điền template PR đầy đủ
3. Link đến issues liên quan
4. Chờ review

## Code Review Process

1. **Automated Checks**: CI/CD sẽ chạy tests và lints
2. **Peer Review**: Maintainers sẽ review code
3. **Feedback**: Thực hiện changes nếu được yêu cầu
4. **Merge**: PR được merge sau khi approved

## Development Guidelines

### Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run benchmarks
cargo bench
```

### Documentation

```bash
# Generate docs
cargo doc --open

# Check doc links
cargo doc --no-deps
```

### Platform-Specific Development

#### Windows

```bash
# Build for Windows
cargo build --target x86_64-pc-windows-msvc
```

#### macOS

```bash
# Build for macOS
cargo build --target x86_64-apple-darwin
```

#### Linux

```bash
# Build for Linux
cargo build --target x86_64-unknown-linux-gnu
```

## Báo Cáo Lỗi

Sử dụng [GitHub Issues](https://github.com/yourusername/vikey/issues) để báo cáo lỗi.

**Template**:

```markdown
**Mô tả lỗi**
Mô tả ngắn gọn về lỗi.

**Cách tái hiện**

1. Bước 1
2. Bước 2
3. ...

**Kết quả mong đợi**
Mô tả kết quả mong đợi.

**Kết quả thực tế**
Mô tả kết quả thực tế.

**Môi trường**

- OS: [e.g., Windows 11]
- Rust version: [e.g., 1.70]
- Vikey version: [e.g., 0.1.0]
```

## Đề Xuất Tính Năng

Sử dụng [GitHub Discussions](https://github.com/yourusername/vikey/discussions) để đề xuất tính năng mới.

## Câu Hỏi

Nếu có câu hỏi, vui lòng:

1. Kiểm tra [Documentation](docs/)
2. Tìm trong [Issues](https://github.com/yourusername/vikey/issues)
3. Tạo [Discussion](https://github.com/yourusername/vikey/discussions)

## License

Bằng việc đóng góp, bạn đồng ý rằng contributions của bạn sẽ được licensed dưới Apache License 2.0.

---

Cảm ơn bạn đã đóng góp! 🙏
