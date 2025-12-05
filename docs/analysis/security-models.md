# Phân Tích: Mô Hình Bảo Mật Modern OS & Kiến Trúc Input Method

## 1. Giới Thiệu

Các Hệ điều hành hiện đại (Modern Operating Systems) đang chuyển dịch sang hướng cô lập nghiêm ngặt (Sandboxing) và phân tách đặc quyền. Điều này ảnh hưởng lớn đến các bộ gõ (IME) truyền thống vốn dựa vào global hooks hoặc quyền truy cập hệ thống không hạn chế.

## 2. Mô Hình Bảo Mật Theo Từng Nền Tảng

### 2.1 Linux (Wayland)

- **Vấn đề**: Không có tọa độ hệ thống toàn cục (global system coordinates) cho client. Client không thể "theo dõi" (spy) các cửa sổ khác.
- **Ảnh hưởng**: IME không thể định vị "Candidate Window" (cửa sổ gợi ý) một cách dễ dàng.
- **Giải pháp**:
  - **Protocol**: Sử dụng `zwp_input_method_v2` kết hợp `zwp_input_popup_surface_v2`.
  - **Cơ chế**: Compositor sẽ chịu trách nhiệm đặt popup gần con trỏ chuột (do Compositor nắm quyền cao nhất).
  - **Fallback**: `text-input-v3` cho phép đồng bộ hình chữ nhật con trỏ (cursor rectangle).

### 2.2 macOS (InputMethodKit)

- **Vấn đề**: **Secure Input Mode**.
  - Được kích hoạt bởi các trường Password hoặc ứng dụng Terminal (ví dụ: Terminal.app khi gõ mật khẩu sudo).
  - **Blocking**: Khi active, **TOÀN BỘ** IME sẽ bị chặn không nhận được sự kiện phím.
- **Ảnh hưởng**: Vikey sẽ ngừng hoạt động trong các trường này. Đây là tính năng "By Design" của macOS.
- **Hardened Runtime**: Vikey bắt buộc phải được công chứng (notarized) và khả năng cao cần các entitlements cụ thể (`com.apple.security.input-method`).

### 2.3 Windows (UIPI & AppContainer)

- **Vấn đề**: **User Interface Privilege Isolation (UIPI)**.
  - Các IME chạy ở mức integrity thấp không thể gửi input vào các App chạy mức integrity cao (ví dụ: Run as Admin).
  - **AppContainer**: Các ứng dụng UWP/Store chạy trong môi trường sandbox.
- **Giải pháp**:
  - Vikey bắt buộc phải hỗ trợ **TSF (Text Services Framework)**.
  - Để tương tác với Admin apps, tiến trình broker của Vikey cần có quyền `uiAccess=true` trong manifest và phải được ký số, cài đặt trong `Program Files`.
  - **Filesystem**: Các file từ điển phải nằm ở vị trí có thể truy cập được (ví dụ: `Program Files/Common Files`) với ACLs phù hợp.

### 2.4 Android/iOS (Mobile Sandbox)

- **Vấn đề**: Sandbox nghiêm ngặt & Cô lập mạng.
- **Ảnh hưởng**: Các IME mặc định thường KHÔNG có quyền truy cập mạng để đảm bảo riêng tư.
- **Ràng buộc**: Các tính năng Cloud (đồng bộ/gợi ý từ server) yêu cầu quyền `INTERNET` rõ ràng, điều này sẽ kích hoạt cảnh báo quyền riêng tư cho người dùng.
- **Định danh**: Mỗi IME chạy dưới một user ID riêng biệt.

## 3. Kiến Trúc Đề Xuất Cho Vikey

### 3.1 Thiết Kế Module IPC (Mẫu "Broker")

Thay vì một monolithic engine nhúng thẳng vào client apps:

```mermaid
graph TD
    A[App 1 (Sandbox)] -->|TSF/Protocol| B(Vikey Broker / Engine)
    C[App 2 (Admin)] -->|Protocol| B
    B -->|Graphics| D[Candidate Window (Overlay)]

    style B fill:#f9f,stroke:#333
```

- **Broker Process**: Một tiến trình (hoặc Service) riêng biệt, có đặc quyền cao, chịu trách nhiệm xử lý logic `vikey-core`.
- **Lightweight Clients**: Các "cầu nối" (bridges/DLLs/bundles) nhẹ dành riêng cho từng platform, chỉ làm nhiệm vụ chuyển tiếp keystrokes tới Broker.
- **Lợi ích**:
  - Giải quyết vấn đề đồng bộ Từ điển (Broker giữ state).
  - Giải quyết vấn đề UIPI (Broker có đủ quyền cần thiết).
  - Giải quyết vấn đề Wayland (Broker giao tiếp trực tiếp với Compositor).

### 3.2 Nhận Biết Secure Mode

- Vikey phải lắng nghe các sự kiện "Secure Input" (macOS) hoặc cờ "Password Field" (Windows/Wayland).
- **Hành vi**: Tự động chuyển về chế độ "English/Passthrough" ngay lập tức. Không cố gắng xử lý input.

### 3.3 Chiến Lược Mạng (Network Strategy)

- **Mặc định**: Offline-first. Không yêu cầu mạng để tra cứu định nghĩa cơ bản.
- **Tùy chọn**: Cloud Sync là tính năng **opt-in** (người dùng phải chủ động bật), có thể thực hiện thông qua một background agent riêng biệt để tránh việc process chính của IME bị flag là "Internet-connected".
