use crate::four_matrix::{classify_realm, encode_text_aggregates, FourMatrixKernel};
use crate::inference_matrix::{MatrixSignal, TamThienMatrix};

const VOCAB: [&str; 32] = [
    "Tôi","đã","hiểu","yêu","cầu","của","bạn","và","sẽ","xử","lý","theo","mục","tiêu",
    "hiện","tại","cần","thêm","dữ","kiện","để","kết","luận","chính","xác","hơn","Được",
    "Không","Vì","Theo","Hiện","này"
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedSequence {
    pub tokens: Vec<String>,
    pub stopped: bool,
}

#[derive(Clone, Debug, Default)]
pub struct DuyenTokenDecoder {
    four: FourMatrixKernel,
    tam: TamThienMatrix,
    recurrent: [i16; 8],
    learned_vocab: Vec<String>,
}

impl DuyenTokenDecoder {
    pub fn learn_text(&mut self, text: &str) -> usize {
        let mut added = 0usize;
        for raw in text.split_whitespace().take(256) {
            let token = sanitize_token(raw);
            if token.chars().count() < 2 || token.chars().count() > 32 {
                continue;
            }
            if VOCAB.iter().any(|v| v.eq_ignore_ascii_case(&token))
                || self.learned_vocab.iter().any(|v| v.eq_ignore_ascii_case(&token))
            {
                continue;
            }
            if self.learned_vocab.len() >= 128 {
                self.learned_vocab.remove(0);
            }
            self.learned_vocab.push(token);
            added += 1;
        }
        added
    }

    pub fn learned_vocab_len(&self) -> usize {
        self.learned_vocab.len()
    }

    pub fn generate(&mut self, input: &str, max_tokens: usize) -> GeneratedSequence {
        let limit = max_tokens.clamp(1, 48);
        let mut out = Vec::with_capacity(limit);
        let mut previous = 0usize;

        for step in 0..limit {
            let aggregates = encode_text_aggregates(input);
            let realm = classify_realm(input);
            let fm = self.four.process(aggregates, realm);

            let mut signals = [MatrixSignal { lane: 0, value_q15: 0 }; 24];
            let mut count = 0usize;
            for (i, v) in fm.conditioned.as_array().iter().enumerate() {
                signals[count] = MatrixSignal { lane: i as u8, value_q15: *v };
                count += 1;
            }
            for (lane, value) in [
                (8u8, fm.projected.technical),
                (9u8, fm.projected.affective),
                (10u8, fm.projected.global),
            ] {
                signals[count] = MatrixSignal { lane, value_q15: value };
                count += 1;
            }

            for (i, v) in self.recurrent.iter().enumerate() {
                if *v != 0 && count < signals.len() {
                    signals[count] = MatrixSignal { lane: (i + 4) as u8, value_q15: *v };
                    count += 1;
                }
            }

            let decision = self.tam.infer(&signals[..count]);
            let intent_bias = intent_class(input) as usize;
            let idx = ((decision.winner as usize * 7)
                .wrapping_add(intent_bias * 5)
                .wrapping_add(step * 3)
                .wrapping_add(previous))
                % VOCAB.len();

            let token = if step >= 4 && !self.learned_vocab.is_empty() && (decision.winner as usize + step) % 3 == 0 {
                let learned_idx = ((decision.winner as usize * 11)
                    .wrapping_add(previous)
                    .wrapping_add(step))
                    % self.learned_vocab.len();
                self.learned_vocab[learned_idx].clone()
            } else {
                constrained_token(input, step, idx).to_string()
            };
            previous = token_index_dynamic(&token, &self.learned_vocab);
            out.push(token);
            self.feedback(previous, decision.confidence_q15);

            if should_stop(input, &out, step) {
                return GeneratedSequence { tokens: out, stopped: true };
            }
        }

        GeneratedSequence { tokens: out, stopped: false }
    }

    pub fn first_token(&mut self, input: &str) -> String {
        self.generate(input, 1)
            .tokens
            .into_iter()
            .next()
            .unwrap_or_else(|| "Tôi".to_string())
    }

    fn feedback(&mut self, token: usize, confidence: i16) {
        let lane = token & 7;
        let retained = (self.recurrent[lane] as i32 * 3) / 4;
        let injected = (confidence as i32 / 4).max(512);
        self.recurrent[lane] = (retained + injected)
            .clamp(i16::MIN as i32, i16::MAX as i32) as i16;
        for i in 0..self.recurrent.len() {
            if i != lane {
                self.recurrent[i] = ((self.recurrent[i] as i32 * 7) / 8) as i16;
            }
        }
    }

    pub fn state(&self) -> [i16; 8] {
        self.recurrent
    }
}

fn intent_class(input: &str) -> u8 {
    let n = normalize(input);
    if n.contains("tai sao") { 1 }
    else if n.contains("khong") { 2 }
    else if n.contains("mo ") || n.contains("tim ") || n.contains("lam ") { 3 }
    else if n.contains("muc tieu") { 4 }
    else { 0 }
}

fn constrained_token(input: &str, step: usize, fallback: usize) -> &'static str {
    let intent = intent_class(input);
    match (intent, step) {
        (1, 0) => "Vì",
        (2, 0) => "Không",
        (3, 0) => "Được",
        (4, 0) => "Tôi",
        (_, 0) => "Tôi",
        (3, 1) => "sẽ",
        (3, 2) => "xử",
        (3, 3) => "lý",
        (4, 1) => "đã",
        (4, 2) => "hiểu",
        _ => VOCAB[fallback],
    }
}

fn should_stop(input: &str, tokens: &[String], step: usize) -> bool {
    let intent = intent_class(input);
    let min_len = match intent {
        3 => 6,
        4 => 7,
        _ => 8,
    };
    step + 1 >= min_len && (tokens.last().map(String::as_str) == Some("hơn") || step >= 15)
}

fn token_index_dynamic(token: &str, learned: &[String]) -> usize {
    if let Some(i) = VOCAB.iter().position(|t| *t == token) {
        return i;
    }
    VOCAB.len()
        + learned
            .iter()
            .position(|t| t == token)
            .unwrap_or(0)
}

fn sanitize_token(raw: &str) -> String {
    raw.trim_matches(|c: char| !c.is_alphanumeric() && c != '_' && c != '-')
        .chars()
        .take(32)
        .collect()
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
            x => x,
        })
        .collect()
}
