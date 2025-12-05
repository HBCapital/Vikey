# Phương Pháp Gõ Tiếng Việt

> Tài liệu phân tích chi tiết về các phương pháp gõ tiếng Việt

## Tổng Quan

Có 3 phương pháp gõ tiếng Việt phổ biến:

1. **Telex** - Phổ biến nhất tại Việt Nam
2. **VNI** - Sử dụng số để đánh dấu
3. **VIQR** - Chuẩn RFC 1456

## 1. Telex

### Quy Tắc

#### Nguyên Âm

- `aa` → `â`
- `aw` → `ă`
- `ee` → `ê`
- `oo` → `ô`
- `ow` → `ơ`
- `w` → `ư`

#### Dấu Thanh

- `s` → dấu sắc (´)
- `f` → dấu huyền (`)
- `r` → dấu hỏi (?)
- `x` → dấu ngã (~)
- `j` → dấu nặng (.)

#### Dấu Đ

- `dd` → `đ`

### Ví Dụ

```
tieng viet → tiếng việt
viet nam → việt nam
```

### Ưu Điểm

- [ ] Dễ nhớ, trực quan
- [ ] Phổ biến nhất
- [ ] Ít xung đột với từ tiếng Anh

### Nhược Điểm

- [ ] Cần gõ 2 ký tự cho một số chữ cái

---

## 2. VNI

### Quy Tắc

#### Nguyên Âm

- `a6` → `ă`
- `a7` → `â`
- `e6` → `ê`
- `o6` → `ô`
- `o7` → `ơ`
- `u7` → `ư`

#### Dấu Thanh

- `1` → dấu sắc (´)
- `2` → dấu huyền (`)
- `3` → dấu hỏi (?)
- `4` → dấu ngã (~)
- `5` → dấu nặng (.)

#### Dấu Đ

- `d9` → `đ`

### Ví Dụ

```
tie61ng vie65t → tiếng việt
vie65t nam → việt nam
```

### Ưu Điểm

- [ ] Compact, ít ký tự hơn
- [ ] Dễ implement

### Nhược Điểm

- [ ] Khó nhớ với người mới
- [ ] Xung đột với số trong văn bản

---

## 3. VIQR (RFC 1456)

### Quy Tắc

#### Nguyên Âm

- `a(` → `ă`
- `a^` → `â`
- `e^` → `ê`
- `o^` → `ô`
- `o+` → `ơ`
- `u+` → `ư`

#### Dấu Thanh

- `'` → dấu sắc (´)
- `` ` `` → dấu huyền (`)
- `?` → dấu hỏi (?)
- `~` → dấu ngã (~)
- `.` → dấu nặng (.)

#### Dấu Đ

- `dd` hoặc `d-` → `đ`

### Ví Dụ

```
tie^'ng vie^.t → tiếng việt
vie^.t nam → việt nam
```

### Ưu Điểm

- [ ] Chuẩn quốc tế (RFC)
- [ ] Logic, dễ hiểu

### Nhược Điểm

- [ ] Ít phổ biến tại VN
- [ ] Xung đột với ký tự đặc biệt

---

## So Sánh

| Aspect      | Telex      | VNI      | VIQR   |
| ----------- | ---------- | -------- | ------ |
| Độ phổ biến | ⭐⭐⭐⭐⭐ | ⭐⭐⭐   | ⭐⭐   |
| Dễ học      | ⭐⭐⭐⭐   | ⭐⭐     | ⭐⭐⭐ |
| Tốc độ gõ   | ⭐⭐⭐     | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| Xung đột    | ⭐⭐⭐⭐   | ⭐⭐     | ⭐⭐⭐ |

## Implementation Notes

### State Machine

Cần thiết kế state machine để:

1. Track các ký tự đã gõ
2. Detect patterns (aa, aw, etc.)
3. Apply transformations
4. Handle backspace/undo

### Edge Cases

- [ ] Từ tiếng Anh có chứa pattern (e.g., "book")
- [ ] Gõ nhầm và backspace
- [ ] Dấu thanh ở vị trí khác nhau
- [ ] Từ ghép

### Performance

- [ ] Lookup table vs. algorithm
- [ ] Memory usage
- [ ] Latency requirements

## Quyết Định Cho Vikey

### Priority

1. Telex (must-have)
2. VNI (should-have)
3. VIQR (nice-to-have)

### Implementation Strategy

- [ ] Shared state machine
- [ ] Pluggable transformation rules
- [ ] User-configurable

## Tài Liệu Tham Khảo

- RFC 1456: https://tools.ietf.org/html/rfc1456
- Vietnamese Unicode: https://unicode.org/charts/PDF/U1EA0.pdf
