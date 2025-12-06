# Cấu trúc Menu - Vikey App

## Tổng quan

Menu của Vikey được thiết kế để hỗ trợ đa ngôn ngữ thiểu số Việt Nam, không chỉ giới hạn ở tiếng Việt hiện đại.

## Cấu trúc Menu Chính

```
├── ● Chữ Việt ▶
│   ├── ● Telex
│   ├── ○ VNI
│   └── ○ xxx
├── ○ Chữ Nôm
│   ├── ○ Ký âm
│   └── ○ Chiết tự
├── ○ Chữ Tây Nguyên
├── ○ Chữ xxx
├── ───────────
├── Tùy chọn ▶
│   ├── ☑ Tự động sửa lỗi chính tả
│   ├── ☑ Gõ tắt
│   └── ☑ Khởi động cùng Windows
├── ───────────
├── Hướng dẫn
└── Thoát
```

## Chi tiết các Menu Item

### 1. Chữ Việt (Submenu - Mặc định active)

**Mục đích:** Chọn phương pháp gõ tiếng Việt hiện đại

**Các tùy chọn:**

- **Telex** (mặc định) - Phương pháp phổ biến nhất
- **VNI** - Phương pháp sử dụng số
- **xxx** - Dự phòng cho các phương pháp khác (VIQR, Simple Telex, v.v.)

**Trạng thái:** Radio button (chỉ chọn 1)

### 2. Chữ Nôm (Submenu)

**Mục đích:** Hỗ trợ gõ chữ Nôm - hệ thống chữ viết cổ của Việt Nam

**Các tùy chọn:**

- **Ký âm** - Gõ theo âm đọc
- **Chiết tự** - Gõ theo cấu tạo chữ

**Trạng thái:** Radio button (chỉ chọn 1)

### 3. Chữ Tây Nguyên

**Mục đích:** Hỗ trợ các ngôn ngữ thiểu số Tây Nguyên (Ê Đê, Jarai, Bahnar, v.v.)

**Trạng thái:** Checkbox hoặc Submenu nếu có nhiều phương pháp

### 4. Chữ xxx

**Mục đích:** Dự phòng cho các hệ thống chữ viết khác của các dân tộc thiểu số

**Ví dụ:**

- Chữ Thái
- Chữ Mông
- Chữ Khmer
- v.v.

### 5. Tùy chọn (Submenu)

**Mục đích:** Cấu hình các tính năng bổ sung

**Các tùy chọn:**

- **☑ Tự động sửa lỗi chính tả** - Tự động sửa các lỗi gõ phổ biến
- **☑ Gõ tắt** - Hỗ trợ gõ tắt (ví dụ: `btw` → `by the way`)
- **☑ Khởi động cùng Windows** - Tự động chạy khi khởi động hệ thống

**Trạng thái:** Checkbox (có thể chọn nhiều)

### 6. Hướng dẫn

**Mục đích:** Mở tài liệu hướng dẫn sử dụng

**Hành động:** Mở file README.md hoặc link đến documentation

### 7. Thoát

**Mục đích:** Đóng ứng dụng

**Hành động:** Thoát hoàn toàn khỏi Vikey

## Nguyên tắc thiết kế

### 1. Phân cấp rõ ràng

- **Cấp 1:** Hệ thống chữ viết (Việt, Nôm, Tây Nguyên, v.v.)
- **Cấp 2:** Phương pháp gõ trong mỗi hệ thống
- **Cấp 3:** Tùy chọn bổ sung

### 2. Trạng thái hiển thị

- **●** (filled circle) = Active/Selected
- **○** (empty circle) = Inactive
- **☑** (checked) = Enabled
- **☐** (unchecked) = Disabled

### 3. Tương tác

- **Left-click trên tray icon:** Toggle ON/OFF toàn bộ bộ gõ
- **Right-click trên tray icon:** Hiển thị menu
- **Click vào menu item:** Thực hiện hành động tương ứng

### 4. Tooltip

Hiển thị trạng thái hiện tại:

- `Vikey - Chữ Việt (Telex) - ON`
- `Vikey - Chữ Nôm (Ký âm) - ON`
- `Vikey - OFF`

## Ghi chú triển khai

### Phase 1 (MVP - Hiện tại)

- [x] Chữ Việt - Telex
- [x] Chữ Việt - VNI
- [x] Thoát
- [x] Toggle ON/OFF (left-click)

### Phase 2 (Tiếp theo)

- [ ] Chữ Việt - VIQR
- [ ] Tùy chọn - Tự động sửa lỗi chính tả
- [ ] Tùy chọn - Khởi động cùng Windows
- [ ] Hướng dẫn

### Phase 3 (Tương lai)

- [ ] Chữ Nôm (Ký âm, Chiết tự)
- [ ] Chữ Tây Nguyên
- [ ] Tùy chọn - Gõ tắt
- [ ] Các hệ thống chữ viết khác

## Tham khảo

- [Unicode Han Nom](http://www.nomfoundation.org/)
- [Bộ gõ Unikey](https://www.unikey.org/)
- [Tiêu chuẩn Unicode cho tiếng Việt](https://unicode.org/charts/PDF/U1E00.pdf)
