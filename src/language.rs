use crate::types::{CognitiveMoment, ComputeMode, Phenomenon, SenseGate, WorldLevel};

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageIntent {
    pub verb: u32,
    pub concepts: Vec<u32>,
    pub confidence: f32,
    pub verb_name: Option<&'static str>,
}

#[derive(Clone, Debug, Default)]
pub struct VietnameseGate;

impl VietnameseGate {
    pub fn perceive(&self, text: &str, timestamp: u64) -> Vec<Phenomenon> {
        normalize(text)
            .split_whitespace()
            .enumerate()
            .map(|(i, token)| {
                Phenomenon::new(
                    hash64(token) ^ timestamp.rotate_left((i % 31) as u32),
                    WorldLevel::TieuThien,
                    SenseGate::Mind,
                    stable_kind(token),
                    token_features(token),
                    0.85,
                    token_salience(token),
                    timestamp.saturating_add(i as u64),
                )
            })
            .collect()
    }

    pub fn infer_intent(&self, text: &str) -> LanguageIntent {
        let n = normalize(text);
        let words: Vec<&str> = n.split_whitespace().collect();
        let verb_name = words.iter().find_map(|w| canonical_verb(w));
        let verb = verb_name.map(stable_kind).unwrap_or(0);
        let concepts = words.iter().map(|w| stable_kind(w)).collect();
        LanguageIntent {
            verb,
            concepts,
            confidence: if verb == 0 { 0.48 } else { 0.86 },
            verb_name,
        }
    }

    pub fn respond(&self, input: &str, moment: &CognitiveMoment) -> String {
        let intent = self.infer_intent(input);
        let mode = match moment.mode {
            ComputeMode::Tinh => "Tĩnh",
            ComputeMode::Nhanh => "Nhanh",
            ComputeMode::Thuong => "Thường",
            ComputeMode::Sau => "Sâu",
        };

        if moment.uncertainty > 0.65 {
            return format!(
                "Tôi chưa đủ chắc để kết luận. Tôi đang ở chế độ {mode} và cần thêm dữ kiện hoặc một yêu cầu cụ thể hơn."
            );
        }

        if let Some(verb) = intent.verb_name {
            let action = match verb {
                "mo" => "mở",
                "tim" => "tìm",
                "gui" => "gửi",
                "doc" => "đọc",
                "luu" => "lưu",
                "tao" => "tạo",
                "xoa" => "xóa",
                "goi" => "gọi",
                _ => "thực hiện",
            };
            if moment.chosen.is_some() {
                return format!(
                    "Tôi đã nhận ra ý định {action}. Tôi đang quán các điều kiện liên quan trước khi chuyển thành hành động."
                );
            }
            return format!(
                "Tôi hiểu bạn muốn {action}, nhưng hiện chưa đủ duyên hoặc quyền hạn để hành động an toàn."
            );
        }

        if is_greeting(input) {
            return "Tôi đang ở đây. Bạn có thể nói điều bạn muốn tôi quan sát, ghi nhớ hoặc xử lý.".to_string();
        }

        if is_identity_question(input) {
            return "Tôi là BIA, một kiến trúc trí tuệ vận hành theo Cảnh–Duyên–Quán–Trí–Hành, chạy cục bộ trên thiết bị này.".to_string();
        }

        if moment.hypotheses.is_empty() {
            format!(
                "Tôi đã ghi nhận cảnh này và đang hình thành ý nghĩa từ kinh nghiệm. Chế độ nhận thức hiện tại: {mode}."
            )
        } else {
            format!(
                "Tôi đang thấy {} quan hệ có thể giải thích cảnh này. Tôi sẽ giữ chúng như giả thuyết thay vì coi một kết luận chưa chắc chắn là sự thật.",
                moment.hypotheses.len()
            )
        }
    }
}

fn canonical_verb(word: &str) -> Option<&'static str> {
    match word {
        "mo" => Some("mo"),
        "tim" => Some("tim"),
        "gui" => Some("gui"),
        "doc" => Some("doc"),
        "luu" | "nho" => Some("luu"),
        "tao" => Some("tao"),
        "xoa" => Some("xoa"),
        "goi" => Some("goi"),
        _ => None,
    }
}

fn is_greeting(input: &str) -> bool {
    let n = normalize(input);
    matches!(n.trim(), "xin chao" | "chao" | "hello" | "hi")
}

fn is_identity_question(input: &str) -> bool {
    let n = normalize(input);
    n.contains("ban la ai") || n.contains("bia la gi")
}

fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| match c {
            'à'|'á'|'ạ'|'ả'|'ã'|'â'|'ầ'|'ấ'|'ậ'|'ẩ'|'ẫ'|'ă'|'ằ'|'ắ'|'ặ'|'ẳ'|'ẵ' => 'a',
            'è'|'é'|'ẹ'|'ẻ'|'ẽ'|'ê'|'ề'|'ế'|'ệ'|'ể'|'ễ' => 'e',
            'ì'|'í'|'ị'|'ỉ'|'ĩ' => 'i',
            'ò'|'ó'|'ọ'|'ỏ'|'õ'|'ô'|'ồ'|'ố'|'ộ'|'ổ'|'ỗ'|'ơ'|'ờ'|'ớ'|'ợ'|'ở'|'ỡ' => 'o',
            'ù'|'ú'|'ụ'|'ủ'|'ũ'|'ư'|'ừ'|'ứ'|'ự'|'ử'|'ữ' => 'u',
            'ỳ'|'ý'|'ỵ'|'ỷ'|'ỹ' => 'y',
            'đ' => 'd',
            c if c.is_alphanumeric() || c.is_whitespace() => c,
            _ => ' ',
        })
        .collect()
}

fn hash64(s: &str) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in s.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn stable_kind(s: &str) -> u32 {
    (hash64(s) ^ (hash64(s) >> 32)) as u32
}

fn token_features(s: &str) -> Vec<f32> {
    let mut v = vec![0.0; 8];
    for (i, b) in s.bytes().enumerate() {
        v[i % 8] += f32::from(b) / 255.0;
    }
    let d = (s.len().max(1) as f32).sqrt();
    for x in &mut v {
        *x /= d;
    }
    v
}

fn token_salience(s: &str) -> f32 {
    if matches!(s, "khong" | "nguy" | "loi" | "xoa" | "dung") {
        0.9
    } else {
        0.55
    }
}
