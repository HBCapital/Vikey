# Ý Tưởng Tính Năng Bộ Gõ Hiện Đại (Modern IME Features)

> Tài liệu brainstorm các tính năng nâng cao cho Vikey để trở thành bộ gõ hiện đại, vượt trội hơn các giải pháp truyền thống.

## 1. AI & Thông Minh Nhân Tạo

### 1.1 Context-Aware Autocomplete (Gợi ý theo ngữ cảnh)

- **Mô tả**: Không chỉ gợi ý từ tiếp theo, mà gợi ý cả cụm từ hoặc câu dựa trên ứng dụng đang dùng.
- **Ví dụ**:
  - Trong IDE (VS Code): Gợi ý biến, function name tiếng Việt không dấu -> có dấu.
  - Trong Email: Gợi ý văn phong trang trọng ("Trân trọng", "Kính gửi").
  - Trong Chat: Gợi ý văn phong thân mật, emoji.

### 1.2 AI Rewriting (Viết lại câu)

- **Mô tả**: Chọn một câu và yêu cầu AI viết lại theo style khác.
- **Styles**: Trang trọng, Ngắn gọn, Thân thiện, Tiếng Anh (dịch).
- **UX**: Menu ngữ cảnh hoặc phím tắt (ví dụ: `Ctrl+Space` sau khi bôi đen).

### 1.3 Grammar & Spell Check (Kiểm tra chính tả/ngữ pháp)

- **Mô tả**: Gạch chân đỏ dưới từ sai chính tả hoặc ngữ pháp tiếng Việt.
- **Real-time**: Kiểm tra ngay khi gõ.
- **Auto-fix**: Tự động sửa các lỗi phổ biến (ví dụ: "nghành" -> "ngành").

## 2. Developer-Centric Features (Dành cho Lập trình viên)

### 2.1 Vim Mode Integration

- **Mô tả**: Điều hướng trong cửa sổ gợi ý (candidate window) bằng `hjkl`.
- **Advanced**: Map phím tắt để di chuyển con trỏ trong văn bản mà không cần rời tay khỏi hàng phím cơ sở (Home row).

### 2.2 Snippet Expansion (Mở rộng từ tắt)

- **Mô tả**: Gõ từ tắt -> bung ra đoạn văn bản dài.
- **Ví dụ**:
  - `;date` -> `2025-12-05`
  - `;mail` -> `myemail@example.com`
  - `;sign` -> Chữ ký email đầy đủ.
- **Dynamic**: Hỗ trợ biến số, ngày giờ, clipboard content.

### 2.3 Case Conversion (Chuyển đổi kiểu chữ)

- **Mô tả**: Phím tắt để xoay vòng kiểu viết của từ hiện tại.
- **Cycle**: `camelCase` -> `snake_case` -> `kebab-case` -> `PascalCase` -> `UPPER_CASE`.
- **Lợi ích**: Cực kỳ hữu ích khi code.

### 2.4 ASCII/Math Input

- **Mô tả**: Gõ ký tự toán học hoặc ký tự đặc biệt bằng cú pháp giống LaTeX.
- **Ví dụ**: `\alpha` -> `α`, `\rightarrow` -> `→`, `\>=` -> `≥`.

## 3. Cloud & Sync (Đám mây & Đồng bộ)

### 3.1 Personal Dictionary Sync (Đồng bộ từ điển cá nhân)

- **Mô tả**: Đồng bộ từ gõ tắt, từ điển người dùng giữa các thiết bị (Windows, Mac, Linux).
- **Security**: Mã hóa đầu cuối (E2E Encryption), không ai đọc được dữ liệu của user.

### 3.2 Clipboard Manager (Quản lý Clipboard)

- **Mô tả**: Lưu lịch sử copy/paste.
- **Tính năng**: Tìm kiếm trong lịch sử, pin các mục thường dùng.
- **Privacy**: Tùy chọn tắt cho các ứng dụng nhạy cảm (Password manager).

## 4. UX/UI & Customization

### 4.1 Per-App Configuration (Cấu hình theo ứng dụng)

- **Mô tả**: Tự động chuyển đổi profile dựa trên cửa sổ đang active.
- **Ví dụ**:
  - VS Code, Terminal: Tự động tắt tiếng Việt (hoặc chuyển sang chế độ Dev).
  - Facebook, Word: Tự động bật Telex.
  - Photoshop: Tự động bật VNI (tránh phím tắt bị dính tiếng Việt).

### 4.2 Theming (Giao diện)

- **Mô tả**: Tùy chỉnh giao diện thanh trạng thái và cửa sổ gợi ý.
- **Styles**: Minimal, Cyberpunk, Glassmorphism, Native.

### 4.3 Sound Feedback (Phản hồi âm thanh)

- **Mô tả**: Âm thanh gõ phím cơ (Mechanical Keyboard Simulator) phát ra từ loa khi gõ.
- **Tùy chọn**: Cherry MX Blue, Brown, Red, Typewriter.

## 5. Privacy & Security (Riêng tư & Bảo mật)

### 5.1 Incognito Mode (Chế độ ẩn danh)

- **Mô tả**: Tự động tạm dừng học từ mới, không lưu lịch sử khi phát hiện trình duyệt đang ở chế độ Incognito hoặc khi gõ vào ô Password.

### 5.2 Local-Only Guarantee

- **Mô tả**: Cam kết và tùy chọn "Hard Switch" để ngắt hoàn toàn kết nối mạng của bộ gõ, đảm bảo an toàn tuyệt đối.

## 6. Accessibility (Hỗ trợ tiếp cận)

### 6.1 Eye-Tracking Support

- **Mô tả**: Tích hợp với thiết bị theo dõi mắt để gõ phím hoặc chọn từ gợi ý.

### 6.2 Morse Code Input

- **Mô tả**: Chế độ gõ bằng 1-2 phím (hoặc switch) cho người khuyết tật vận động.

---

## Đề Xuất Ưu Tiên Cho Vikey

1. **Snippet Expansion** (Dễ implement, giá trị cao cho dev)
2. **Per-App Configuration** (Giải quyết pain point lớn nhất của người dùng hiện tại)
3. **Case Conversion** (Tính năng "wow" cho dev)
4. **Cloud Sync** (Tính năng giữ chân người dùng)
