# Tài Liệu Kỹ Thuật Unicode Tiếng Việt

## 1. Chuẩn Hóa Unicode (Unicode Normalization)

Tiếng Việt có thể được biểu diễn dưới hai dạng chính trong Unicode:

- **NFC (Normalization Form C)**: Ký tự dựng sẵn (Pre-composed). Được ưu tiên cho hầu hết các ứng dụng (Web, Windows).
  - Ví dụ: `á` (U+00E1)
- **NFD (Normalization Form D)**: Ký tự tổ hợp (Decomposed). Ký tự gốc + Dấu kết hợp. Phổ biến trong hệ thống file macOS (HFS+).
  - Ví dụ: `a` (U+0061) + `´` (U+0301)

### Khuyến Nghị Cho Vikey

Luôn output **NFC** mặc định để đảm bảo tính tương thích tối đa.

## 2. Dấu Thanh (Tone Marks)

| Tên Dấu | Tiếng Việt | Ký Tự Kết Hợp | Ví Dụ Dựng Sẵn (a) |
| ------- | ---------- | ------------- | ------------------ |
| Ngang   | Không dấu  | N/A           | a (U+0061)         |
| Huyền   | Huyền      | U+0300        | à (U+00E0)         |
| Sắc     | Sắc        | U+0301        | á (U+00E1)         |
| Hỏi     | Hỏi        | U+0309        | ả (U+1EA3)         |
| Ngã     | Ngã        | U+0303        | ã (U+00E3)         |
| Nặng    | Nặng       | U+0323        | ạ (U+1EA1)         |

## 3. Ký Tự Đặc Biệt (Dấu Mũ / Dấu Phụ)

| Tên        | Ký Tự   | Ký Tự Kết Hợp | Ví Dụ      |
| ---------- | ------- | ------------- | ---------- |
| Breve      | Ă       | U+0306        | ă (U+0103) |
| Circumflex | Â, Ê, Ô | U+0302        | â (U+00E2) |
| Horn       | Ơ, Ư    | U+031B        | ơ (U+01A1) |
| Stroke     | Đ       | N/A (Riêng)   | đ (U+0111) |

## 4. Bảng Mã Ký Tự Dựng Sẵn (NFC)

### Nguyên Âm Có Dấu

#### A

| Gốc | Huyền | Sắc | Hỏi | Ngã | Nặng |
| --- | ----- | --- | --- | --- | ---- |
| a   | à     | á   | ả   | ã   | ạ    |
| ă   | ằ     | ắ   | ẳ   | ẵ   | ặ    |
| â   | ầ     | ấ   | ẩ   | ẫ   | ậ    |

#### E

| Gốc | Huyền | Sắc | Hỏi | Ngã | Nặng |
| --- | ----- | --- | --- | --- | ---- |
| e   | è     | é   | ẻ   | ẽ   | ẹ    |
| ê   | ề     | ế   | ể   | ễ   | ệ    |

#### I

| Gốc | Huyền | Sắc | Hỏi | Ngã | Nặng |
| --- | ----- | --- | --- | --- | ---- |
| i   | ì     | í   | ỉ   | ĩ   | ị    |

#### O

| Gốc | Huyền | Sắc | Hỏi | Ngã | Nặng |
| --- | ----- | --- | --- | --- | ---- |
| o   | ò     | ó   | ỏ   | õ   | ọ    |
| ô   | ồ     | ố   | ổ   | ỗ   | ộ    |
| ơ   | ờ     | ớ   | ở   | ỡ   | ợ    |

#### U

| Gốc | Huyền | Sắc | Hỏi | Ngã | Nặng |
| --- | ----- | --- | --- | --- | ---- |
| u   | ù     | ú   | ủ   | ũ   | ụ    |
| ư   | ừ     | ứ   | ử   | ữ   | ự    |

#### Y

| Gốc | Huyền | Sắc | Hỏi | Ngã | Nặng |
| --- | ----- | --- | --- | --- | ---- |
| y   | ỳ     | ý   | ỷ   | ỹ   | ỵ    |

## 5. Quy Tắc Implementation

1.  **Đặt Dấu Thanh (Tone Placement)**:

    - Nếu là nguyên âm đơn: Đặt dấu lên nguyên âm đó.
    - Nếu là nguyên âm đôi/ba: Tuân theo "Kiểu Mới" (ví dụ: `hòa`, `thủy`) hoặc "Kiểu Cũ" (`hoà`, `thuỷ`).
    - **Mặc định Vikey**: Kiểu Mới (đặt dấu lên nguyên âm chính).

2.  **Logic Input**:

    - Phát hiện ký tự gốc.
    - Áp dụng biến đổi (mũ/móc).
    - Áp dụng dấu thanh.
    - Chuẩn hóa về NFC.

3.  **Xử Lý Backspace**:
    - Ký tự NFC là 1 code point nhưng có thể là 2-3 bytes trong UTF-8.
    - Phải xóa toàn bộ code point, không chỉ byte cuối cùng.
