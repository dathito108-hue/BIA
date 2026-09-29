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


#[derive(Clone, Debug)]
pub struct V14StressReport {
    pub iterations: usize,
    pub bounded_token_passes: usize,
    pub finite_state_passes: usize,
    pub noise_passes: usize,
    pub deterministic_replay_passes: usize,
    pub elapsed: Duration,
}

impl V14StressReport {
    pub fn passed(&self) -> bool {
        self.bounded_token_passes == self.iterations
            && self.finite_state_passes == self.iterations
            && self.noise_passes == self.iterations
            && self.deterministic_replay_passes == self.iterations
    }

    pub fn ns_per_iteration(&self) -> u128 {
        self.elapsed.as_nanos() / self.iterations.max(1) as u128
    }
}

pub fn run_v14_stress(iterations: usize) -> V14StressReport {
    let iterations = iterations.clamp(100, 50_000);
    let corpus = [
        "Mở ứng dụng rồi tìm tài liệu",
        "Tại sao trạng thái cần được giới hạn?",
        "Không thực thi nếu chưa xác nhận",
        "Suy luận logic trừu tượng với dữ kiện mới",
        "Đọc tệp trên điện thoại và ghi nhớ provenance",
        "###@@@ nhiễu 12345 ??? tiếng Việt vẫn phải chạy",
        "😀🙏📱 kiểm tra unicode và biểu tượng",
        "mục tiêu: phân tích hệ thống theo nhiều góc nhìn",
    ];

    let start = Instant::now();
    let mut bounded = 0usize;
    let mut finite = 0usize;
    let mut noise = 0usize;
    let mut deterministic = 0usize;

    let mut decoder = DuyenTokenDecoder::default();
    let _ = decoder.learn_text(
        "tamthien duyenkhoi nguuan tinhkhong trungdao thienhanh provenance causality",
    );

    for i in 0..iterations {
        let input = corpus[i % corpus.len()];
        let before = decoder.state();
        let seq = decoder.generate(input, 48);
        if !seq.tokens.is_empty() && seq.tokens.len() <= 48 {
            bounded += 1;
        }

        let state = decoder.state();
        if state.iter().all(|v| *v != i16::MIN && *v != i16::MAX) {
            finite += 1;
        }

        if seq.tokens.iter().all(|t| !t.is_empty() && t.chars().count() <= 32) {
            noise += 1;
        }

        let mut a = DuyenTokenDecoder::default();
        let mut b = DuyenTokenDecoder::default();
        let _ = a.learn_text("tamthien duyenkhoi nguuan tinhkhong");
        let _ = b.learn_text("tamthien duyenkhoi nguuan tinhkhong");
        if a.generate(input, 16) == b.generate(input, 16) {
            deterministic += 1;
        }

        // The recurrent state must remain bounded but must also be able to evolve.
        if i == 0 {
            debug_assert_ne!(before, state);
        }
    }

    V14StressReport {
        iterations,
        bounded_token_passes: bounded,
        finite_state_passes: finite,
        noise_passes: noise,
        deterministic_replay_passes: deterministic,
        elapsed: start.elapsed(),
    }
}


#[derive(Clone, Debug)]
pub struct V15ReasoningReport {
    pub cases: usize,
    pub multi_hop_passes: usize,
    pub contradiction_passes: usize,
    pub revision_passes: usize,
    pub causal_persistence_passes: usize,
    pub elapsed: Duration,
}

impl V15ReasoningReport {
    pub fn passed(&self) -> bool {
        self.multi_hop_passes == self.cases
            && self.contradiction_passes == self.cases
            && self.revision_passes == self.cases
            && self.causal_persistence_passes == self.cases
    }

    pub fn accuracy(&self) -> f32 {
        let denom = (self.cases * 4).max(1) as f32;
        (self.multi_hop_passes
            + self.contradiction_passes
            + self.revision_passes
            + self.causal_persistence_passes) as f32
            / denom
    }
}

pub fn run_v15_reasoning_evaluation() -> V15ReasoningReport {
    use crate::reasoning::CausalReasoner;
    use crate::types::{Relation, RelationKind};
    use crate::world::WorldGraph;

    let start = Instant::now();
    let cases = 64usize;
    let mut multi_hop = 0usize;
    let mut contradiction = 0usize;
    let mut revision = 0usize;
    let mut persistence = 0usize;

    for case in 0..cases {
        let base = case as u64 * 100;
        let a = base + 1;
        let b = base + 2;
        let c = base + 3;
        let d = base + 4;
        let e = base + 5;

        let mut world = WorldGraph::new(64, 128);
        world.relate(Relation {
            from: a,
            to: b,
            kind: RelationKind::Causes,
            strength: 0.95,
            confidence: 0.95,
        });
        world.relate(Relation {
            from: b,
            to: c,
            kind: RelationKind::Enables,
            strength: 0.90,
            confidence: 0.90,
        });

        let reasoner = CausalReasoner::new(4, 12);
        let initial = reasoner.infer(&world, c);
        if initial
            .best_path
            .as_ref()
            .is_some_and(|p| p.nodes == vec![a, b, c] && !p.inhibited)
            && initial.support > 0.60
        {
            multi_hop += 1;
        }

        world.relate(Relation {
            from: d,
            to: c,
            kind: RelationKind::Inhibits,
            strength: 0.88,
            confidence: 0.92,
        });
        let conflicted = reasoner.infer(&world, c);
        if conflicted.contradicted
            && conflicted.support > 0.50
            && conflicted.opposition > 0.50
            && conflicted.confidence < initial.confidence
        {
            contradiction += 1;
        }

        let revised = reasoner.revise_with_relation(
            &mut world,
            Relation {
                from: e,
                to: c,
                kind: RelationKind::Inhibits,
                strength: 1.0,
                confidence: 1.0,
            },
            c,
        );
        if revised.opposition > conflicted.opposition
            && revised.opposition > revised.support
            && revised.contradicted
        {
            revision += 1;
        }

        world.relate(Relation {
            from: base + 40,
            to: base + 41,
            kind: RelationKind::Similar,
            strength: 1.0,
            confidence: 1.0,
        });
        let after_unrelated = reasoner.infer(&world, c);
        if (after_unrelated.support - revised.support).abs() < 1e-6
            && (after_unrelated.opposition - revised.opposition).abs() < 1e-6
        {
            persistence += 1;
        }
    }

    V15ReasoningReport {
        cases,
        multi_hop_passes: multi_hop,
        contradiction_passes: contradiction,
        revision_passes: revision,
        causal_persistence_passes: persistence,
        elapsed: start.elapsed(),
    }
}
