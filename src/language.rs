use crate::types::{Phenomenon, SenseGate, WorldLevel};

#[derive(Clone, Debug, PartialEq)]
pub struct LanguageIntent {
    pub verb: u32,
    pub concepts: Vec<u32>,
    pub confidence: f32,
}

#[derive(Clone, Debug, Default)]
pub struct VietnameseGate;

impl VietnameseGate {
    pub fn perceive(&self, text: &str, timestamp: u64) -> Vec<Phenomenon> {
        normalize(text)
            .split_whitespace()
            .enumerate()
            .map(|(i, token)| Phenomenon::new(
                hash64(token) ^ timestamp.rotate_left((i % 31) as u32),
                WorldLevel::TieuThien,
                SenseGate::Mind,
                stable_kind(token),
                token_features(token),
                0.85,
                token_salience(token),
                timestamp.saturating_add(i as u64),
            ))
            .collect()
    }

    pub fn infer_intent(&self, text: &str) -> LanguageIntent {
        let n = normalize(text);
        let words: Vec<&str> = n.split_whitespace().collect();
        let verb = words
            .iter()
            .find(|w| matches!(**w, "mo" | "tim" | "gui" | "doc" | "luu" | "tao" | "xoa" | "goi"))
            .map(|w| stable_kind(w))
            .unwrap_or(0);
        let concepts = words.iter().map(|w| stable_kind(w)).collect();
        LanguageIntent {
            verb,
            concepts,
            confidence: if verb == 0 { 0.45 } else { 0.82 },
        }
    }

    pub fn express(&self, concepts: &[u32]) -> String {
        if concepts.is_empty() {
            return "Tôi chưa có đủ duyên để kết luận.".to_string();
        }
        let body = concepts
            .iter()
            .take(8)
            .map(|x| format!("khái-niệm-{x}"))
            .collect::<Vec<_>>()
            .join(", ");
        format!("Tôi đang nhận thấy: {body}.")
    }
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

fn hash64(s:&str)->u64 {
    let mut h=0xcbf29ce484222325u64;
    for b in s.as_bytes(){ h^=u64::from(*b); h=h.wrapping_mul(0x100000001b3); }
    h
}
fn stable_kind(s:&str)->u32 { (hash64(s) ^ (hash64(s)>>32)) as u32 }
fn token_features(s:&str)->Vec<f32>{
    let mut v=vec![0.0;8];
    for (i,b) in s.bytes().enumerate(){ v[i%8]+=f32::from(b)/255.0; }
    let d=(s.len().max(1) as f32).sqrt();
    for x in &mut v { *x/=d; }
    v
}
fn token_salience(s:&str)->f32 {
    if matches!(s,"khong"|"nguy"|"loi"|"xoa"|"dung"){0.9}else{0.55}
}
