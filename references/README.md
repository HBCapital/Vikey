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

### 3. ibus-bamboo

- **Repository**: Có trong `ibus-bamboo/` folder
- **Ngôn ngữ**: Go
- **License**: GPL-3.0
- **Platform**: Linux (IBus framework)
- **Điểm mạnh**:
  - Modern Go implementation
  - IBus integration patterns
  - Vietnamese input optimization
  - Active development và community
- **Giá trị tham khảo**:
  - Cross-platform input method architecture
  - Linux platform integration
  - Performance optimization techniques

### 4. fcitx5-unikey

- **Repository**: Có trong `fcitx5-unikey/` folder
- **Ngôn ngữ**: C++
- **License**: GPL-2.0+
- **Platform**: Linux (Fcitx5 framework)
- **Điểm mạnh**:
  - Integration với Fcitx5 modern framework
  - UniKey engine adaptation
  - Qt-based configuration
- **Giá trị tham khảo**:
  - Fcitx5 plugin architecture
  - Config management patterns
  - Legacy code modernization

### 5. Afrim

- **Repository**: https://github.com/fodydev/afrim
- **Ngôn ngữ**: Rust
- **License**: MPL-2.0
- **Platform**: Cross-platform (Desktop, Web, Android)
- **Điểm mạnh**:
  - 🌐 Hỗ trợ sequential codification codes
  - 🎨 CLI interface dễ sử dụng
  - 📚 Customizable dictionary
  - 💻 Multi-platform: Desktop, Web, Android
  - 🤖 Rhai scripting language support
  - 📝 Auto-suggestion/correction/completion
  - ☁️ Full immersion mode (experimental)
- **Frontends**:
  - [afrim-wish](https://github.com/fodydev/afrim-wish): Desktop environment
  - [afrim-web](https://github.com/fodydev/afrim-web): Web platform (MIT license)
  - [afrim-keyboard](https://github.com/fodydev/afrim-keyboard): Android
- **Giá trị tham khảo**:
  - **Rust architecture**: Modern input method framework design
  - **Multi-platform strategy**: Cách tiếp cận cross-platform với Rust
  - **Plugin system**: Scripting integration với Rhai
  - **Dictionary management**: Customizable data structures
  - **Web integration**: WebAssembly patterns cho web platform
  - **Auto-suggestion engine**: ML/AI integration patterns
- **Tương đồng với Vikey**:
  - Cùng viết bằng Rust
  - Cùng mục tiêu cross-platform
  - Cùng focus vào extensibility
  - Có web platform (quan trọng cho Vikey roadmap)

---

## Dự Án Tham Khảo Cho Chữ Nôm & CJK

> 💡 **Quan trọng**: Để hỗ trợ **chữ Nôm** (𡨸喃) trong roadmap Phase 2, cần tham khảo các bộ gõ tiếng Trung (CJK) vì chữ Nôm sử dụng CJK Unicode blocks.

### 6. RIME (中州韻輸入法引擎)

- **Repository**: https://github.com/rime/librime
- **Ngôn ngữ**: C++
- **License**: BSD-3-Clause
- **Platform**: Cross-platform (Windows, macOS, Linux, Android)
- **Mô tả**: Modular, extensible input method engine - **QUAN TRỌNG NHẤT** cho chữ Nôm
- **Điểm mạnh**:
  - 🏗️ Modular architecture với plugin system
  - 📝 Schema DSL (YAML) để define input methods
  - 🌏 Native support cho Traditional Chinese, có OpenCC conversion
  - 🎯 Spelling Algebra cho Chinese dialects
  - 🔌 Extensible với custom schemas
- **Frontends**:
  - [Weasel](https://github.com/rime/weasel) (小狼毫): Windows
  - [Squirrel](https://github.com/rime/squirrel) (鼠鬚管): macOS
  - [ibus-rime](https://github.com/rime/ibus-rime): Linux/IBus
  - [fcitx5-rime](https://github.com/fcitx/fcitx5-rime): Linux/Fcitx5
  - [Trime](https://github.com/osfans/trime) (同文): Android
- **Giá trị tham khảo**:
  - **Schema system**: YAML-based DSL cho input method definition
  - **Dictionary format**: Efficient trie-based dictionary structure
  - **Modular design**: Plugin architecture patterns
  - **Multi-platform**: Cross-platform C++ implementation
  - **OpenCC integration**: Traditional ↔ Simplified conversion
- **Tương đồng với Vikey**:
  - Cùng mục tiêu: Extensible IME framework
  - Schema-based approach (RIME dùng YAML, Vikey có thể dùng Rust configs)
  - Cross-platform architecture

### 7. RIME Vietnamese & Hán Nôm Schemas

#### 7.1 rime-vietnamese

- **Repository**: https://github.com/gkovacs/rime-vietnamese
- **License**: MIT
- **Mô tả**: RIME schema cho tiếng Việt - **HỖ TRỢ CẢ CHỮ NÔM**
- **Tính năng**:
  - ✅ Hỗ trợ chữ Quốc ngữ (國語字 / 𡨸國語)
  - ✅ Hỗ trợ chữ Nôm (喃字 / 𡨸喃)
  - 🎯 Telex input method
- **Giá trị tham khảo**:
  - **Nôm implementation**: Cách map Telex → chữ Nôm
  - **Dictionary structure**: Vietnamese + Nôm character mappings
  - **Schema design**: RIME schema patterns cho Vietnamese
- **Quan trọng cho Vikey**: Đây là reference implementation cho `vikey-nom` crate

#### 7.2 rime-ime-han-nom-data

- **Repository**: https://github.com/miketvo/rime-ime-han-nom-data
- **License**: GPL-3.0
- **Mô tả**: Hán Nôm Telex schema & dictionary cho RIME Weasel/Squirrel
- **Tính năng**:
  - 📚 Dictionary data từ Wiktionary
  - 🎯 Accurate Telex schema cho chữ Nôm
  - 🔤 Mapping: Vietnamese spelling → Nôm characters
- **Giá trị tham khảo**:
  - **Dictionary data**: Comprehensive Nôm character database
  - **Telex mapping**: Proven Telex → Nôm conversion rules
  - **Data format**: Dictionary file structure

#### 7.3 Weasel_HanNom_Keyboard

- **Repository**: https://github.com/Liu2k5/Weasel_HanNom_Keyboard
- **License**: GPL
- **Mô tả**: Modified Weasel Hán Nôm Keyboard (no-diacritics input)
- **Tính năng**:
  - 🔤 Pinyin-style input (no diacritics)
  - 📝 Common compound words cho faster typing
  - 🎯 Based on Ủy ban Phục sinh Hán Nôm Việt Nam
- **Giá trị tham khảo**:
  - **Alternative input method**: Pinyin-style thay vì Telex
  - **Compound words**: Optimization patterns
  - **User experience**: Different approach to Nôm input

### 8. Chinese Input Methods (Phonetic)

#### 8.1 libchewing

- **Repository**: https://github.com/chewing/libchewing
- **Ngôn ngữ**: C
- **License**: LGPL-2.1
- **Platform**: Cross-platform
- **Mô tả**: Intelligent phonetic (Zhuyin/Bopomofo) input method library
- **Điểm mạnh**:
  - 🧠 Intelligent character selection
  - 🎯 Zhuyin (Bopomofo) và Hanyu Pinyin support
  - 📝 User phrase learning
  - 🔌 Modular library design
- **Frontends**:
  - [ibus-chewing](https://github.com/chewing/ibus-chewing): Linux/IBus
  - [windows-chewing-tsf](https://github.com/chewing/windows-chewing-tsf): Windows TSF
- **Giá trị tham khảo**:
  - **Intelligent suggestion**: Algorithm cho character selection
  - **Phrase learning**: User dictionary adaptation
  - **Phonetic input**: Patterns cho pronunciation-based input
  - **Library design**: Clean API separation
- **Liên quan đến Vikey**: Patterns cho intelligent suggestion trong `vikey-nom`

#### 8.2 vChewing

- **Repository**: https://github.com/vChewing/vChewing-macOS
- **Ngôn ngữ**: Swift (macOS), C++ (core)
- **License**: MIT
- **Platform**: macOS (chính), Windows (experimental)
- **Mô tả**: Modern phonabet-based input method cho Mandarin Chinese
- **Điểm mạnh**:
  - 🎨 Modern Swift architecture
  - 🧠 Advanced suggestion engine
  - ⚡ Performance optimization
  - 🎯 Traditional Chinese focus
- **Giá trị tham khảo**:
  - **Modern architecture**: Swift patterns cho macOS IME
  - **Suggestion engine**: Advanced ranking algorithms
  - **Performance**: Optimization techniques
  - **UI/UX**: Modern input method interface

### 9. Rust Crates cho CJK

> 🦀 **Rust implementations** - Có thể integrate trực tiếp vào Vikey

#### 9.1 librustpinyin

- **Repository**: https://github.com/phreer/librustpinyin
- **License**: MIT
- **Mô tả**: Rust library để parse Pinyin và output Chinese characters
- **Tính năng**:
  - 🔤 Pinyin parsing
  - 🇨🇳 Chinese character output
  - 🔌 C-compatible API
  - 🎯 Designed cho IME development
- **Giá trị tham khảo**:
  - **Rust IME patterns**: Cách implement IME bằng Rust
  - **Pinyin algorithm**: Core conversion logic
  - **C FFI**: Patterns cho platform integration
- **Use case cho Vikey**: Reference cho `vikey-nom` Rust implementation

#### 9.2 opencc-rust

- **Repository**: https://github.com/magiclen/opencc-rust
- **License**: MIT
- **Mô tả**: Rust binding cho Open Chinese Convert (OpenCC)
- **Tính năng**:
  - 🔄 Traditional Chinese ↔ Simplified Chinese
  - 🌏 Regional variants (Taiwan, Hong Kong, Mainland)
  - ⚡ Fast conversion
  - 🦀 Pure Rust API
- **Giá trị tham khảo**:
  - **Text conversion**: Patterns cho character variant conversion
  - **Dictionary-based**: Efficient lookup structures
  - **Rust bindings**: FFI patterns
- **Use case cho Vikey**: Có thể cần cho Hán Việt ↔ Nôm conversion

#### 9.3 cjk crate

- **Repository**: https://crates.io/crates/cjk
- **License**: MIT/Apache-2.0
- **Mô tả**: Utilities cho CJK characters
- **Tính năng**:
  - 🔍 Identify CJK characters
  - 🇨🇳 Check Simplified vs Traditional
  - 🔤 Chinese → Pinyin conversion
  - 🇯🇵 Japanese → Romaji conversion
- **Giá trị tham khảo**:
  - **Character utilities**: Helper functions cho CJK
  - **Unicode handling**: CJK Unicode block processing
  - **Text analysis**: Character classification
- **Use case cho Vikey**: Utilities cho `vikey-nom` character handling

### 10. Latin & Diacritics Handling Patterns

> 💡 **Ý tưởng**: Tham khảo các ngôn ngữ Latin khác (Pinyin, African) về cách xử lý dấu.

#### 10.1 Pinyin Tone Placement

- **Logic**: Quy tắc đặt dấu thanh (Tone marks) trong Pinyin có điểm tương đồng với tiếng Việt (ưu tiên nguyên âm chính).
- **Quy tắc cốt lõi**: `a > o > e > i > u > ü`
- **Repos tham khảo**:
  - [pinyin-tone](https://github.com/hotoo/pinyin-tone): JS library convert number-to-mark `hao3` -> `hǎo`
  - [pinyinify](https://github.com/b_k/pinyinify): Lightweight converter
- **Giá trị cho Vikey**:
  - Thuật toán xác định vị trí đặt dấu (Tone placement algorithm)
  - Logic xử lý "vowel clusters" (như `oa`, `uy`, `iê`)

#### 10.2 Rust Normalization & Diacritics

Các thư viện Rust xử lý Unicode và dấu:

- **[unicode-normalization](https://github.com/unicode-rs/unicode-normalization)**:
  - **Quan trọng nhất**: Xử lý NFC (Precomposed) vs NFD (Decomposed).
  - Vikey bắt buộc phải dùng để chuẩn hóa input trước khi xử lý.
- **[unaccent](https://github.com/irevoire/unaccent)**:
  - Remove accents (dùng cho search/fuzzy matching).
  - Ví dụ: `Tiếng Việt` -> `Tieng Viet`.
- **[diacritics](https://github.com/vslifes/diacritics)**:
  - Lightweight crate để remove diacritics.

---

- Lightweight crate để remove diacritics.

### 11. Các Kỹ Thuật Latin Input Đặc Thù

> 💡 **Kỹ thuật hay**: Các model xử lý input đặc biệt từ các ngôn ngữ Latin khác.

#### 11.1 Esperanto "x-system"

- **Vấn đề**: Cần nhập các ký tự mũ (`ĉ`, `ĝ`, `ĥ`, `ĵ`, `ŝ`, `ŭ`) mà không có trên bàn phím chuẩn.
- **Giải pháp**: Dùng hậu tố `x` (là ký tự không dùng trong tiếng Esperanto) để làm "dead key" sau.
  - Ví dụ: `cx` -> `ĉ`.
- **Điểm hay**: Dùng 1 ký tự **không tồn tại** trong ngôn ngữ làm phím chức năng -> Tránh ambiguity hoàn toàn (khác với Telex dùng `s`, `f`, `r` là các phím có nghĩa).
- **Áp dụng cho Vikey**: Có thể dùng kỹ thuật này cho các chế độ "Quick Type" hoặc "Safe Mode" để tránh conflict với tiếng Anh.

#### 11.2 Keyman Engine Architecture

- **Repository**: https://github.com/keymanapp/keyman
- **Mô tả**: "The WordPress of Input Methods" - hỗ trợ hơn 2000 ngôn ngữ.
- **Kỹ thuật**:
  - Dùng **Keyboard Programming Language** (`.kmn` files) để define logic thay vì hardcode.
  - Context-aware transformation rule: `store(vowels) "aeiou" + "'" > index(vowels, 1) U+0301`
- **Áp dụng cho Vikey**: Thiết kế config file cho user tự define kiểu gõ (như Vikey Configuration Language).

#### 11.3 Linux Compose Key Logic

- **Mô tả**: Standard way trên Linux để nhập ký tự đặc biệt.
- **Logic**: `Compose` + `Sequence` = `Character`.
  - `Compose` + `'` + `e` -> `é`
  - `Compose` + `/` + `o` -> `ø`
- **File Format**: `.XCompose` file format rất dễ đọc và clear.
- **Áp dụng**: Hỗ trợ Compose Mode cho Vikey để gõ các ký tự Latin mở rộng (European languages).

---

- `Compose` + `/` + `o` -> `ø`
- **File Format**: `.XCompose` file format rất dễ đọc và clear.
- **Áp dụng**: Hỗ trợ Compose Mode cho Vikey để gõ các ký tự Latin mở rộng (European languages).

### 12. Wayland Architecture (Future Linux Support)

> ⚠️ **Challenge**: Wayland không cho phép global key sniffing như X11/Windows.

#### 12.1 Protocols

- **text-input-v3**: App <-> Compositor.
- **input-method-v2**: IME Vikey <-> Compositor (Protocol quan trọng nhất cho Vikey).

#### 12.2 Rust Ecology for Wayland

- **[smithay-client-toolkit](https://github.com/Smithay/client-toolkit)**: Toolkit viết Wayland client bằng Rust. Có `seat::input_method` support.
- **[zwp-input-method-service](https://crates.io/crates/zwp-input-method-service)**: Crate chuyên biệt implement `zwp_input_method_v2`.

#### 12.3 Reference Strategy: Fcitx5

- Fcitx5 dùng chiến lược **Hybrid**: Native Wayland protocol + Legacy Modules (`GTK_IM_MODULE`).
- Xem chi tiết phân tích tại: [`docs/analysis/wayland-analysis.md`](../docs/analysis/wayland-analysis.md)

### 13. Modern OS Security Models

> 🛡️ **Challenge**: Hệ điều hành hiện đại chặn việc IMe truy cập tự do vào hệ thống.

- **Wayland**:
  - Vấn đề: Candidate Window positioning (không có global coords).
  - Giải pháp: Protocol `zwp_input_popup_surface_v2`.
- **macOS (InputMethodKit)**:
  - Vấn đề: `Secure Input Mode` (trong password field/Terminal) chặn hoàn toàn IME.
  - Note: Vikey phải handle state này để tránh app bị treo.
- **Windows (UIPI)**:
  - Vấn đề: App thường không thể gõ vào App Admin.
  - Giải pháp: IME broker process cần `uiAccess=true` và ký số.
- **Android/iOS**:
  - Vấn đề: Sandbox mạng (IME thường không được cấp quyền Internet).
- **Xem chi tiết phân tích**: [`docs/analysis/security-models.md`](../docs/analysis/security-models.md)
- **Phân tích chuyên sâu (Senior Architect)**: [`docs/analysis/security-deep-dive.md`](../docs/analysis/security-deep-dive.md)

---

## Cấu Trúc

```
references/
├── README.md              # File này
├── openkey/              # Clone của OpenKey
├── unikey/               # Clone của UniKey
├── ibus-bamboo/          # Clone của ibus-bamboo
├── fcitx5-unikey/        # Clone của fcitx5-unikey
├── afrim/                # Clone của Afrim (sẽ thêm)
├── librime/              # Clone của RIME core (sẽ thêm)
├── rime-vietnamese/      # Clone của rime-vietnamese (sẽ thêm)
├── rime-han-nom-data/    # Clone của rime-ime-han-nom-data (sẽ thêm)
├── libchewing/           # Clone của libchewing (sẽ thêm)
├── vChewing-macOS/       # Clone của vChewing (sẽ thêm)
└── keyman/               # Clone của Keyman (sẽ thêm - reference architecture)
```

> 📝 **Lưu ý**: Rust crates (librustpinyin, opencc-rust, cjk) không cần clone, chỉ cần thêm vào `Cargo.toml` khi cần.

## Tài Liệu Phân Tích

Tất cả tài liệu phân tích đã được di chuyển vào `docs/`:

### Vietnamese IME

- **So sánh kiến trúc**: [`docs/analysis/architecture-comparison.md`](../docs/analysis/architecture-comparison.md)
- **Phân tích OpenKey**: [`docs/analysis/openkey-analysis.md`](../docs/analysis/openkey-analysis.md)
- **Phân tích UniKey**: [`docs/analysis/unikey-analysis.md`](../docs/analysis/unikey-analysis.md)
- **Phân tích ibus-bamboo**: Sẽ được tạo khi cần thiết
- **Phân tích fcitx5-unikey**: Sẽ được tạo khi cần thiết
- **Phân tích Afrim**: Sẽ được tạo khi cần thiết

### CJK & Nôm IME

- **Phân tích RIME**: Sẽ được tạo khi implement `vikey-nom`
- **Phân tích rime-vietnamese**: Sẽ được tạo khi implement `vikey-nom`
- **Phân tích libchewing**: Sẽ được tạo khi cần intelligent suggestion
- **Phân tích vChewing**: Sẽ được tạo khi cần modern architecture patterns

### Technical Documentation

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

#### Vietnamese IME

```bash
cd references/

# Clone Afrim (nếu chưa có)
git clone https://github.com/fodydev/afrim.git afrim
```

#### RIME & Nôm Support

```bash
cd references/

# Clone RIME core engine (QUAN TRỌNG cho chữ Nôm)
git clone https://github.com/rime/librime.git librime

# Clone Vietnamese + Nôm schema
git clone https://github.com/gkovacs/rime-vietnamese.git rime-vietnamese

# Clone Hán Nôm dictionary data
git clone https://github.com/miketvo/rime-ime-han-nom-data.git rime-han-nom-data

# Clone modified Weasel Nôm keyboard (optional)
git clone https://github.com/Liu2k5/Weasel_HanNom_Keyboard.git weasel-hannom
```

#### Chinese IME (cho patterns)

```bash
cd references/

# Clone libchewing (intelligent suggestion patterns)
git clone https://github.com/chewing/libchewing.git libchewing

# Clone vChewing (modern architecture)
git clone https://github.com/vChewing/vChewing-macOS.git vChewing-macOS
```

#### Rust Crates

> 📦 Không cần clone, chỉ cần add vào `Cargo.toml` khi implement `vikey-nom`:

```toml
[dependencies]
# Pinyin processing
librustpinyin = "0.1"

# Traditional ↔ Simplified conversion
opencc-rust = "1.1"

# CJK utilities
cjk = "0.2"
```

---

**Nhớ**: Học hỏi từ người khác, nhưng code của Vikey phải là của chúng ta! 🚀
