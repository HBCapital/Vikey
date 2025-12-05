# References - Tài Liệu Tham Khảo

> ⚠️ **LƯU Ý QUAN TRỌNG**: Thư mục này chứa các dự án mã nguồn mở khác để **THAM KHẢO** và **PHÂN TÍCH** kiến trúc, không phải là code của dự án Vikey.

## Mục Đích

Thư mục `references/` được tạo ra để:

1. **Nghiên cứu**: Tìm hiểu cách các bộ gõ khác implement các tính năng
2. **Phân tích kiến trúc**: Học hỏi design patterns và best practices
3. **So sánh kỹ thuật**: Đánh giá các approaches khác nhau
4. **Tránh reinvent the wheel**: Hiểu những gì đã được giải quyết

## Các Dự Án Tham Khảo

### 1. OpenKey

- **Repository**: https://github.com/tuyenvm/OpenKey
- **Ngôn ngữ**: C++ (engine), Swift (macOS)
- **License**: GPL
- **Platform**: macOS (chính), Windows, Linux
- **Điểm mạnh**: Backspace technique, modern architecture
- **Tài liệu phân tích**: [`docs/analysis/openkey-analysis.md`](../docs/analysis/openkey-analysis.md)

### 2. UniKey

- **Repository**: Có trong `unikey/` folder
- **Ngôn ngữ**: C/C++
- **License**: GPL
- **Platform**: Windows (chính), Linux (x-unikey)
- **Điểm mạnh**: Mature, comprehensive features, 20+ years
- **Tài liệu phân tích**: [`docs/analysis/unikey-analysis.md`](../docs/analysis/unikey-analysis.md)

## Cấu Trúc

```
references/
├── README.md           # File này
├── openkey/           # Clone của OpenKey
└── unikey/            # Clone của UniKey
```

## Tài Liệu Phân Tích

Tất cả tài liệu phân tích đã được di chuyển vào `docs/`:

- **So sánh kiến trúc**: [`docs/analysis/architecture-comparison.md`](../docs/analysis/architecture-comparison.md)
- **Phân tích OpenKey**: [`docs/analysis/openkey-analysis.md`](../docs/analysis/openkey-analysis.md)
- **Phân tích UniKey**: [`docs/analysis/unikey-analysis.md`](../docs/analysis/unikey-analysis.md)
- **Input Methods**: [`docs/technical/input-methods.md`](../docs/technical/input-methods.md)
- **Platform APIs**: [`docs/technical/platform-apis.md`](../docs/technical/platform-apis.md)

## Quy Tắc

1. ✅ **Được phép**: Đọc, phân tích, học hỏi ý tưởng
2. ❌ **Không được**: Copy code trực tiếp mà không hiểu
3. ✅ **Được phép**: Implement lại ý tưởng bằng cách của riêng mình
4. ⚠️ **Chú ý**: Tôn trọng license của các dự án tham khảo

## Cách Sử Dụng

### Phân tích dự án

1. Đọc code trong `references/`
2. Viết tài liệu phân tích vào `docs/analysis/`
3. Cập nhật `docs/analysis/architecture-comparison.md`
4. Thiết kế implementation của Vikey dựa trên phân tích

### Clone thêm dự án

```bash
cd references/

# Clone dự án mới
git clone [URL] project-name
```

---

**Nhớ**: Học hỏi từ người khác, nhưng code của Vikey phải là của chúng ta! 🚀
