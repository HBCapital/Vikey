# Quy Tắc Biến Đổi Telex & VNI

## 1. Quy Tắc Telex

### 1.1 Biến Đổi Nguyên Âm

| Input | Output | Ví Dụ Input | Ví Dụ Output |
| ----- | ------ | ----------- | ------------ |
| `aa`  | `â`    | `caan`      | `cân`        |
| `aw`  | `ă`    | `nawm`      | `năm`        |
| `ee`  | `ê`    | `teen`      | `tên`        |
| `oo`  | `ô`    | `moon`      | `môn`        |
| `ow`  | `ơ`    | `ow`        | `ơ`          |
| `w`   | `ư`    | `uw`        | `ư`          |
| `dd`  | `đ`    | `ddau`      | `đau`        |

### 1.2 Dấu Thanh

| Phím | Tên Dấu | Ký Hiệu | Ví Dụ Input | Ví Dụ Output |
| ---- | ------- | ------- | ----------- | ------------ |
| `s`  | Sắc     | ´       | `as`        | `á`          |
| `f`  | Huyền   | `       | `af`        | `à`          |
| `r`  | Hỏi     | ̉        | `ar`        | `ả`          |
| `x`  | Ngã     | ̃        | `ax`        | `ã`          |
| `j`  | Nặng    | ̣        | `aj`        | `ạ`          |

### 1.3 Quick Telex (Tính Năng Tùy Chọn)

| Input | Output | Ví Dụ Input | Ví Dụ Output |
| ----- | ------ | ----------- | ------------ |
| `cc`  | `ch`   | `ccao`      | `chao`       |
| `gg`  | `gi`   | `ggio`      | `gio`        |
| `kk`  | `kh`   | `kko`       | `kho`        |
| `nn`  | `ng`   | `nnon`      | `ngon`       |
| `qq`  | `qu`   | `qqan`      | `quan`       |
| `pp`  | `ph`   | `ppo`       | `pho`        |
| `tt`  | `th`   | `tthe`      | `the`        |

### 1.4 Các Trường Hợp Đặc Biệt (Edge Cases)

#### Trường Hợp 1: Undo Chữ Cái Kép

- Input: `aa` → Output: `â`
- Input: `aa` lần nữa → Output: `aa` (hoàn tác biến đổi)

#### Trường Hợp 2: Dấu Thanh Trên Nguyên Âm Đã Biến Đổi

- Input: `aa` → `â`
- Input: `s` → `ấ` (dấu thanh áp dụng lên nguyên âm đã biến đổi)

#### Trường Hợp 3: W Sau Nguyên Âm

- Input: `aw` → `ă`
- Input: `aw` lần nữa → `aw` (hoàn tác)
- Nhưng: `ow` → `ơ`, `oww` → `ơw` (w đơn trở thành ư chỉ trong ngữ cảnh cụ thể)

---

## 2. Quy Tắc VNI

### 2.1 Biến Đổi Nguyên Âm

| Input | Output | Ví Dụ Input | Ví Dụ Output |
| ----- | ------ | ----------- | ------------ |
| `a6`  | `ă`    | `na6m`      | `năm`        |
| `a7`  | `â`    | `ca7n`      | `cân`        |
| `e6`  | `ê`    | `te6n`      | `tên`        |
| `o6`  | `ô`    | `mo6n`      | `môn`        |
| `o7`  | `ơ`    | `o7`        | `ơ`          |
| `u7`  | `ư`    | `u7`        | `ư`          |
| `d9`  | `đ`    | `d9au`      | `đau`        |

### 2.2 Dấu Thanh

| Phím | Tên Dấu | Ký Hiệu | Ví Dụ Input | Ví Dụ Output |
| ---- | ------- | ------- | ----------- | ------------ |
| `1`  | Sắc     | ´       | `a1`        | `á`          |
| `2`  | Huyền   | `       | `a2`        | `à`          |
| `3`  | Hỏi     | ̉        | `a3`        | `ả`          |
| `4`  | Ngã     | ̃        | `a4`        | `ã`          |
| `5`  | Nặng    | ̣        | `a5`        | `ạ`          |

### 2.3 Biến Đổi Kết Hợp

VNI cho phép kết hợp biến đổi nguyên âm và dấu thanh trong một bước:

| Input | Output | Giải Thích |
| ----- | ------ | ---------- |
| `a61` | `ắ`    | ă + sắc    |
| `a71` | `ấ`    | â + sắc    |
| `e62` | `ề`    | ê + huyền  |
| `o65` | `ộ`    | ô + nặng   |

### 2.4 Các Trường Hợp Đặc Biệt

#### Trường Hợp 1: Số Trong Văn Bản

- Input: `2024` → KHÔNG biến đổi
- Giải pháp: Chỉ biến đổi khi đi trước bởi một nguyên âm hợp lệ

#### Trường Hợp 2: Nhiều Chữ Số

- Input: `a612` → `ắ2` (chỉ chữ số đầu tiên là dấu thanh)

---

## 3. Thuật Toán Đặt Dấu Thanh

### 3.1 Nguyên Âm Đơn

Đặt dấu trực tiếp lên nguyên âm.

- `a` + `s` → `á`
- `ê` + `f` → `ề`

### 3.2 Hai Nguyên Âm (Nguyên Âm Đôi)

#### Quy Tắc 1: Ưu Tiên Nguyên Âm Chính

- `oa` → dấu trên `o` (kiểu cũ) hoặc `a` (kiểu mới)
- `oe` → dấu trên `o` (kiểu cũ) hoặc `e` (kiểu mới)
- `uy` → dấu trên `u` (kiểu cũ) hoặc `y` (kiểu mới)

#### Quy Tắc 2: Trường Hợp Đặc Biệt

- `qu` + dấu → dấu trên nguyên âm sau (ví dụ: `quá`)
- `gi` + dấu → dấu trên nguyên âm sau (ví dụ: `giá`)

### 3.3 Ba Nguyên Âm (Nguyên Âm Ba)

Đặt dấu trên **nguyên âm giữa**:

- `oai` → `oái` (dấu trên `a`)
- `uôi` → `uối` (dấu trên `ô`)
- `uyê` → `uyế` (dấu trên `ê`)

### 3.4 Kiểu Mới vs Kiểu Cũ

| Từ   | Kiểu Cũ | Kiểu Mới | Mặc Định Vikey |
| ---- | ------- | -------- | -------------- |
| hòa  | hoà     | hòa      | Mới (hòa)      |
| thủy | thuỷ    | thủy     | Mới (thủy)     |
| quả  | quả     | quả      | Giống nhau     |

---

## 4. Ghi Chú Implementation

### 4.1 Máy Trạng Thái (State Machine)

```
INITIAL → BUFFERING → PATTERN_MATCH → TRANSFORM → OUTPUT
   ↓          ↓            ↓             ↓          ↓
 RESET    ADD_CHAR    DETECT_RULE   APPLY_RULE  SEND_KEYS
```

### 4.2 Quản Lý Buffer

- **Kích Thước Buffer Tối Đa**: 32 ký tự (đủ cho từ tiếng Việt dài nhất)
- **Điều Kiện Xóa**:
  - Space, Enter, Tab
  - Dấu câu (., !, ?, v.v.)
  - Các phím mũi tên, Escape
  - Các tổ hợp Ctrl/Alt/Cmd

### 4.3 Xử Lý Backspace

Khi người dùng nhấn Backspace:

1. Xóa ký tự cuối cùng khỏi buffer
2. Nếu ký tự cuối đã được biến đổi, gửi nhiều backspace (cho UTF-8 multi-byte)
3. Đánh giá lại trạng thái buffer

### 4.4 Tối Ưu Hóa Hiệu Năng

- Sử dụng bảng tra cứu (lookup tables) cho pattern matching O(1)
- Tính toán trước các kết hợp dấu + nguyên âm
- Cache các biến đổi thường dùng

---

## 5. Test Cases

### 5.1 Telex Cơ Bản

```
Input:  "tieng viet"
Output: "tiếng việt"

Input:  "viet nam"
Output: "việt nam"

Input:  "xin chao"
Output: "xin chào"
```

### 5.2 VNI Cơ Bản

```
Input:  "tie61ng vie65t"
Output: "tiếng việt"

Input:  "vie65t nam"
Output: "việt nam"
```

### 5.3 Edge Cases

```
Input:  "aa" sau đó "a"
Output: "â" sau đó "aa" (undo)

Input:  "book" (từ tiếng Anh)
Output: "book" (không biến đổi)

Input:  "2024"
Output: "2024" (giữ nguyên số)
```

---

## 6. Tài Liệu Tham Khảo

- **Chuẩn Telex**: Quy ước cộng đồng, không có spec chính thức
- **Chuẩn VNI**: Phát triển bởi VNI Software
- **Unicode Vietnamese**: https://unicode.org/charts/PDF/U1EA0.pdf
