# Lộ Trình Tính Năng Vikey - Giai Đoạn 2

> Kế hoạch mở rộng tính năng cho bộ gõ Vikey

## Tổng Quan

Giai đoạn 2 tập trung vào mở rộng khả năng của Vikey để phục vụ cộng đồng rộng hơn, bao gồm:

1. Hỗ trợ ngôn ngữ dân tộc thiểu số và tiếng Việt cổ
2. Tích hợp voice-to-text (giọng nói sang văn bản)

---

## 1. Hỗ Trợ Ngôn Ngữ Dân Tộc Thiểu Số

### 1.1 Tiếng Việt Cổ & Chữ Nôm

#### Chữ Nôm (𡨸喃)

**Unicode Blocks:**

- CJK Unified Ideographs Extension B: U+20000–U+2A6DF
- CJK Unified Ideographs Extension C: U+2A700–U+2B73F
- CJK Unified Ideographs Extension D: U+2B740–U+2B81F

**Ví dụ:**

- 𡨸喃 (chữ Nôm)
- 𡦂 (người)
- 𢆥 (làm)

**Challenges:**

- Font support: Cần font HAN NOM A, B, C
- Input method: Cần bộ gõ riêng (Telex-Nôm hoặc tra cứu)
- Rendering: Một số ký tự rất phức tạp

**Implementation:**

```rust
// vikey-nom crate
pub struct NomTransformer {
    nom_dict: HashMap<String, char>, // Telex -> Nôm character
}

// Ví dụ: "nguoi" -> 𡦂
```

#### Tiếng Việt Cổ

**Đặc điểm:**

- Dấu thanh cổ (không còn dùng)
- Ký tự đặc biệt: ꞗ, ꞔ, ꞑ
- Chính tả cũ: "thường" thay vì "thường"

---

### 1.2 Ngôn Ngữ Dân Tộc

#### Chữ Thái (Tày, Nùng, Thái)

**Unicode Block:** Tai Viet (U+AA80–U+AADF)

**Ví dụ:**

- ꪀ ꪁ ꪂ ꪃ ꪄ (nguyên âm)
- ꪕ ꪖ ꪗ ꪘ (phụ âm)

**Input Method:**

- Telex-style cho Tai Viet
- Mapping: a → ꪀ, b → ꪕ, etc.

**Implementation:**

```rust
// vikey-tai crate
pub struct TaiVietTransformer {
    rules: HashMap<String, char>,
}
```

#### Chữ Mường

**Đặc điểm:**

- Dựa trên chữ Latinh
- Có thêm các ký tự đặc biệt: ư̆, ơ̆
- Dấu thanh tương tự tiếng Việt

**Input Method:**

- Mở rộng từ Telex tiếng Việt
- Thêm quy tắc cho ký tự đặc biệt

#### Chữ H'Mông

**Unicode Blocks:**

- Pahawh Hmong: U+16B00–U+16B8F
- Pollard Script: U+A4D0–U+A4FF

**Ví dụ Pahawh Hmong:**

- 𖬀 𖬁 𖬂 𖬃 𖬄

**Challenges:**

- Viết từ trái sang phải NHƯNG tone marks ở bên trái
- Cần xử lý đặc biệt cho rendering

#### Chữ Chăm

**Unicode Block:** Cham (U+AA00–U+AA5F)

**Ví dụ:**

- ꨀ ꨁ ꨂ ꨃ ꨄ (nguyên âm)
- ꨆ ꨇ ꨈ ꨉ (phụ âm)

**Đặc điểm:**

- Viết từ trái sang phải
- Có dấu thanh riêng
- Cần font hỗ trợ (Cham OI)

---

### 1.3 Kiến Trúc Đề Xuất

```
vikey/
├── crates/
│   ├── vikey-core/           # Core engine (không đổi)
│   ├── vikey-vietnamese/     # Tiếng Việt hiện đại
│   ├── vikey-nom/            # Chữ Nôm & Tiếng Việt cổ
│   ├── vikey-tai/            # Chữ Thái (Tày, Nùng, Thái)
│   ├── vikey-muong/          # Chữ Mường
│   ├── vikey-hmong/          # Chữ H'Mông
│   └── vikey-cham/           # Chữ Chăm
```

### 1.4 Tính Năng Chung

1. **Language Switcher**: Chuyển đổi nhanh giữa các ngôn ngữ
2. **Dictionary**: Từ điển cho mỗi ngôn ngữ
3. **Font Fallback**: Tự động chọn font phù hợp
4. **Input Method Editor**: UI để chọn ký tự phức tạp

---

## 2. Voice-to-Text (Giọng Nói Sang Văn Bản)

### 2.1 Kiến Trúc Tổng Thể

```
┌─────────────┐
│ Microphone  │
└──────┬──────┘
       │
       ▼
┌─────────────────────┐
│ Audio Processing    │
│ - Noise Reduction   │
│ - VAD (Voice        │
│   Activity Detect)  │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Speech Recognition  │
│ - Whisper (Offline) │
│ - Cloud API (Online)│
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Language Model      │
│ - Vietnamese LM     │
│ - Accent Detection  │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Vikey Engine        │
│ - Unicode Normalize │
│ - Spell Check       │
│ - Punctuation       │
└──────┬──────────────┘
       │
       ▼
┌─────────────────────┐
│ Text Output         │
└─────────────────────┘
```

### 2.2 Speech Recognition Options

#### Option 1: Whisper (OpenAI) - Offline

**Pros:**

- ✅ Hoàn toàn offline (privacy)
- ✅ Hỗ trợ tiếng Việt tốt
- ✅ Open source

**Cons:**

- ⚠️ Cần GPU cho real-time (hoặc CPU mạnh)
- ⚠️ Model size: 75MB (tiny) đến 3GB (large)

**Models:**

- `tiny`: 75MB, nhanh nhưng kém chính xác
- `base`: 142MB, cân bằng
- `small`: 466MB, tốt cho production

**Implementation:**

```rust
// vikey-voice crate
use whisper_rs::WhisperContext;

pub struct VoiceRecognizer {
    whisper: WhisperContext,
    audio_buffer: Vec<f32>,
}

impl VoiceRecognizer {
    pub fn transcribe(&mut self, audio: &[f32]) -> Result<String> {
        let result = self.whisper.full(audio)?;
        Ok(result.text)
    }
}
```

#### Option 2: Cloud APIs - Online

**Google Speech-to-Text:**

- Độ chính xác cao
- Hỗ trợ giọng địa phương
- Cost: $0.006/15s

**Azure Speech:**

- Tương tự Google
- Tích hợp tốt với Windows

**Implementation:**

```rust
use reqwest::Client;

pub struct CloudRecognizer {
    client: Client,
    api_key: String,
}

impl CloudRecognizer {
    pub async fn transcribe(&self, audio: &[u8]) -> Result<String> {
        // Call cloud API
    }
}
```

#### Option 3: Hybrid

```rust
pub enum RecognitionMode {
    Offline,  // Whisper
    Online,   // Cloud API
    Hybrid,   // Offline first, fallback to cloud if confidence < threshold
}
```

### 2.3 Audio Processing

**Crate:** `cpal` (Cross-Platform Audio Library)

```rust
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct AudioCapture {
    stream: cpal::Stream,
    buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioCapture {
    pub fn start(&mut self) -> Result<()> {
        let buffer = self.buffer.clone();

        self.stream = device.build_input_stream(
            &config,
            move |data: &[f32], _: &_| {
                buffer.lock().unwrap().extend_from_slice(data);
            },
            |err| eprintln!("Error: {}", err),
        )?;

        self.stream.play()?;
        Ok(())
    }
}
```

### 2.4 Voice Activity Detection (VAD)

**Purpose:** Chỉ xử lý khi có giọng nói, tiết kiệm CPU/battery.

**Implementation:**

```rust
pub struct VAD {
    threshold: f32,
    window_size: usize,
}

impl VAD {
    pub fn is_speech(&self, audio: &[f32]) -> bool {
        let energy = audio.iter()
            .map(|&x| x * x)
            .sum::<f32>() / audio.len() as f32;

        energy > self.threshold
    }
}
```

### 2.5 Accent Support

**Giọng Miền:**

- **Bắc**: Chuẩn, dễ nhận dạng nhất
- **Trung**: Đặc trưng: d/gi, r/g
- **Nam**: Đặc trưng: không phân biệt s/x, ch/tr

**Language Model Adaptation:**

```rust
pub enum Accent {
    North,
    Central,
    South,
}

pub struct AccentAdapter {
    accent: Accent,
    rules: HashMap<String, String>,
}

impl AccentAdapter {
    pub fn adapt(&self, text: &str) -> String {
        match self.accent {
            Accent::South => {
                // "sáng" có thể được nghe thành "xáng"
                text.replace("x", "s")
            }
            // ...
        }
    }
}
```

### 2.6 Punctuation & Post-processing

```rust
pub struct PostProcessor {
    vikey_engine: Engine,
}

impl PostProcessor {
    pub fn process(&self, text: &str) -> String {
        let mut result = text.to_string();

        // 1. Thêm dấu câu (ML model hoặc rules)
        result = self.add_punctuation(&result);

        // 2. Chuẩn hóa Unicode (NFC)
        result = result.nfc().collect();

        // 3. Spell check
        result = self.spell_check(&result);

        result
    }
}
```

### 2.7 Voice Commands

**Ví dụ:**

- "bật tiếng việt" → Enable Vietnamese input
- "chuyển sang telex" → Switch to Telex
- "tắt bộ gõ" → Disable IME

```rust
pub struct VoiceCommand {
    commands: HashMap<String, Command>,
}

enum Command {
    EnableVietnamese,
    SwitchInputMethod(InputMethod),
    Disable,
}
```

---

## 3. Implementation Roadmap

### Phase 2.1: Minority Languages (3-6 tháng)

**Month 1-2: Research & Design**

- [ ] Nghiên cứu Unicode blocks
- [ ] Thu thập font và tài liệu
- [ ] Thiết kế input methods

**Month 3-4: Implementation**

- [ ] Implement vikey-nom
- [ ] Implement vikey-tai
- [ ] Implement vikey-muong

**Month 5-6: Testing & Polish**

- [ ] Testing với native speakers
- [ ] Documentation
- [ ] Font packaging

### Phase 2.2: Voice-to-Text (4-6 tháng)

**Month 1-2: Audio Infrastructure**

- [ ] Audio capture với `cpal`
- [ ] VAD implementation
- [ ] Audio preprocessing

**Month 3-4: Speech Recognition**

- [ ] Whisper integration (offline)
- [ ] Cloud API integration (online)
- [ ] Hybrid mode

**Month 5-6: Post-processing & UX**

- [ ] Punctuation model
- [ ] Accent adaptation
- [ ] Voice commands
- [ ] UI/UX for voice input

---

## 4. Technical Challenges

### Minority Languages

| Challenge           | Solution                           |
| ------------------- | ---------------------------------- |
| Font availability   | Bundle fonts, fallback mechanism   |
| Complex rendering   | Use HarfBuzz for text shaping      |
| Limited resources   | Community collaboration            |
| Input method design | User research with native speakers |

### Voice-to-Text

| Challenge      | Solution                               |
| -------------- | -------------------------------------- |
| Latency        | Use streaming recognition, VAD         |
| Accuracy       | Hybrid mode, language model adaptation |
| Privacy        | Prioritize offline mode                |
| Resource usage | Model quantization, efficient VAD      |
| Accents        | Accent-specific models or adaptation   |

---

## 5. Tài Liệu Tham Khảo

**Unicode:**

- Unicode Standard: https://unicode.org/
- Tai Viet: https://unicode.org/charts/PDF/UAA80.pdf
- Cham: https://unicode.org/charts/PDF/UAA00.pdf

**Speech Recognition:**

- Whisper: https://github.com/openai/whisper
- Google Speech-to-Text: https://cloud.google.com/speech-to-text
- Vosk: https://alphacephei.com/vosk/

**Audio Processing:**

- cpal: https://github.com/RustAudio/cpal
- HarfBuzz: https://harfbuzz.github.io/

---

**Last Updated**: 2025-12-05
