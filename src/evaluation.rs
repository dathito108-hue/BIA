use std::time::{Duration, Instant};

use crate::duyen_token::DuyenTokenDecoder;
use crate::four_matrix::{classify_realm, RealmBand};

#[derive(Clone, Debug)]
pub struct V11Report {
    pub cases: usize,
    pub semantic_passes: usize,
    pub determinism_passes: usize,
    pub realm_passes: usize,
    pub elapsed: Duration,
}

impl V11Report {
    pub fn semantic_accuracy(&self) -> f32 {
        self.semantic_passes as f32 / self.cases.max(1) as f32
    }
    pub fn deterministic_rate(&self) -> f32 {
        self.determinism_passes as f32 / self.cases.max(1) as f32
    }
    pub fn realm_accuracy(&self) -> f32 {
        self.realm_passes as f32 / self.cases.max(1) as f32
    }
    pub fn passed(&self) -> bool {
        self.semantic_accuracy() >= 0.90
            && self.deterministic_rate() >= 1.0
            && self.realm_accuracy() >= 0.90
    }
}

pub fn run_v11_evaluation() -> V11Report {
    let cases = [
        ("Mở YouTube", "Được", RealmBand::Mixed),
        ("Tìm web Phật giáo", "Được", RealmBand::Mixed),
        ("Làm tiếp việc này", "Được", RealmBand::Mixed),
        ("Tại sao trời mưa?", "Vì", RealmBand::Mixed),
        ("Không làm việc đó", "Không", RealmBand::Mixed),
        ("Mục tiêu: tìm tài liệu Rust", "Tôi", RealmBand::Mixed),
        ("Suy luận logic trừu tượng", "Tôi", RealmBand::Abstract),
        ("Phân tích thuật toán", "Tôi", RealmBand::Abstract),
        ("Đọc file trên điện thoại", "Tôi", RealmBand::Embodied),
        ("Phân tích ảnh camera", "Tôi", RealmBand::Embodied),
    ];

    let start = Instant::now();
    let mut semantic = 0;
    let mut deterministic = 0;
    let mut realm = 0;

    for (input, expected_first, expected_realm) in cases {
        let mut a = DuyenTokenDecoder::default();
        let mut b = DuyenTokenDecoder::default();
        let first_a = a.first_token(input);
        let first_b = b.first_token(input);
        if first_a == expected_first {
            semantic += 1;
        }
        if first_a == first_b {
            deterministic += 1;
        }
        if classify_realm(input) == expected_realm {
            realm += 1;
        }
    }

    V11Report {
        cases: cases.len(),
        semantic_passes: semantic,
        determinism_passes: deterministic,
        realm_passes: realm,
        elapsed: start.elapsed(),
    }
}


#[derive(Clone, Debug)]
pub struct V12Report {
    pub held_out_cases: usize,
    pub semantic_passes: usize,
    pub determinism_passes: usize,
    pub dynamic_vocab_learned: usize,
    pub dynamic_vocab_emitted: bool,
    pub elapsed: Duration,
}

impl V12Report {
    pub fn semantic_accuracy(&self) -> f32 {
        self.semantic_passes as f32 / self.held_out_cases.max(1) as f32
    }

    pub fn deterministic_rate(&self) -> f32 {
        self.determinism_passes as f32 / self.held_out_cases.max(1) as f32
    }

    pub fn passed(&self) -> bool {
        self.semantic_accuracy() >= 0.90
            && self.deterministic_rate() >= 1.0
            && self.dynamic_vocab_learned >= 3
            && self.dynamic_vocab_emitted
    }
}

pub fn run_v12_evaluation() -> V12Report {
    let held_out = [
        ("Hãy mở phần cài đặt", "Được"),
        ("Làm giúp tôi bước kế tiếp", "Được"),
        ("Tìm thông tin về kiến trúc Duyên khởi", "Được"),
        ("Tại sao cần trạng thái tái diễn?", "Vì"),
        ("Không mở ứng dụng đó", "Không"),
        ("Mục tiêu: nghiên cứu kiến trúc mới", "Tôi"),
        ("Chứng minh nguyên lý tính toán này", "Tôi"),
        ("Phân tích toán học của hệ thống", "Tôi"),
        ("Đọc tệp trên thiết bị", "Tôi"),
        ("Quan sát camera điện thoại", "Tôi"),
    ];

    let start = Instant::now();
    let mut semantic = 0usize;
    let mut deterministic = 0usize;

    for (input, expected) in held_out {
        let mut a = DuyenTokenDecoder::default();
        let mut b = DuyenTokenDecoder::default();
        let fa = a.first_token(input);
        let fb = b.first_token(input);
        if fa == expected {
            semantic += 1;
        }
        if fa == fb {
            deterministic += 1;
        }
    }

    let mut decoder = DuyenTokenDecoder::default();
    let learned = decoder.learn_text(
        "lotusgraph duyencore tructam provenancefield causallattice mobilekernel",
    );
    let generated = decoder.generate("Phân tích kiến trúc mới", 32);
    let dynamic_vocab_emitted = generated.tokens.iter().any(|token| {
        matches!(
            token.as_str(),
            "lotusgraph"
                | "duyencore"
                | "tructam"
                | "provenancefield"
                | "causallattice"
                | "mobilekernel"
        )
    });

    V12Report {
        held_out_cases: held_out.len(),
        semantic_passes: semantic,
        determinism_passes: deterministic,
        dynamic_vocab_learned: learned,
        dynamic_vocab_emitted,
        elapsed: start.elapsed(),
    }
}
