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


#[derive(Clone, Debug)]
pub struct V16GeneralizationReport {
    pub cases: usize,
    pub long_chain_passes: usize,
    pub distractor_passes: usize,
    pub counterfactual_passes: usize,
    pub reversal_passes: usize,
    pub persistence_passes: usize,
    pub elapsed: Duration,
}

impl V16GeneralizationReport {
    pub fn passed(&self) -> bool {
        self.long_chain_passes == self.cases
            && self.distractor_passes == self.cases
            && self.counterfactual_passes == self.cases
            && self.reversal_passes == self.cases
            && self.persistence_passes == self.cases
    }

    pub fn accuracy(&self) -> f32 {
        let denom = (self.cases * 5).max(1) as f32;
        (self.long_chain_passes
            + self.distractor_passes
            + self.counterfactual_passes
            + self.reversal_passes
            + self.persistence_passes) as f32
            / denom
    }
}

pub fn run_v16_generalization_evaluation() -> V16GeneralizationReport {
    use crate::reasoning::CausalReasoner;
    use crate::types::{Relation, RelationKind};
    use crate::world::WorldGraph;

    let start = Instant::now();
    let cases = 128usize;
    let mut long_chain = 0usize;
    let mut distractor = 0usize;
    let mut counterfactual = 0usize;
    let mut reversal = 0usize;
    let mut persistence = 0usize;

    for case in 0..cases {
        let base = case as u64 * 1000;
        let nodes = [base + 1, base + 2, base + 3, base + 4, base + 5, base + 6];

        let mut world = WorldGraph::new(128, 256);
        for i in 0..5 {
            world.relate(Relation {
                from: nodes[i],
                to: nodes[i + 1],
                kind: if i % 2 == 0 { RelationKind::Causes } else { RelationKind::Enables },
                strength: 0.95 - i as f32 * 0.03,
                confidence: 0.96 - i as f32 * 0.02,
            });
        }

        // Distractors with strong-looking but irrelevant edges.
        for d in 0..20u64 {
            world.relate(Relation {
                from: base + 100 + d,
                to: base + 200 + d,
                kind: RelationKind::Causes,
                strength: 1.0,
                confidence: 1.0,
            });
        }

        let reasoner = CausalReasoner::new(6, 16);
        let target = nodes[5];
        let factual = reasoner.infer(&world, target);

        if factual
            .best_path
            .as_ref()
            .is_some_and(|p| p.nodes == nodes.to_vec() && !p.inhibited)
            && factual.support > 0.5
        {
            long_chain += 1;
        }

        if factual
            .best_path
            .as_ref()
            .is_some_and(|p| p.nodes.iter().all(|n| *n < base + 100))
        {
            distractor += 1;
        }

        let cf = reasoner.counterfactual_without(&world, target, nodes[2]);
        if cf.support_delta > 0.20 && cf.counterfactual.support < cf.factual.support {
            counterfactual += 1;
        }

        world.relate(Relation {
            from: base + 900,
            to: target,
            kind: RelationKind::Inhibits,
            strength: 1.0,
            confidence: 1.0,
        });
        world.relate(Relation {
            from: base + 901,
            to: target,
            kind: RelationKind::Inhibits,
            strength: 1.0,
            confidence: 1.0,
        });
        let reversed = reasoner.infer(&world, target);
        if reversed.opposition > reversed.support && reversed.contradicted {
            reversal += 1;
        }

        // Simulate later-turn unrelated memory/world additions.
        let before_support = reversed.support;
        let before_opp = reversed.opposition;
        for t in 0..16u64 {
            world.relate(Relation {
                from: base + 500 + t,
                to: base + 600 + t,
                kind: RelationKind::Similar,
                strength: 0.9,
                confidence: 0.9,
            });
        }
        let after = reasoner.infer(&world, target);
        if (after.support - before_support).abs() < 1e-6
            && (after.opposition - before_opp).abs() < 1e-6
        {
            persistence += 1;
        }
    }

    V16GeneralizationReport {
        cases,
        long_chain_passes: long_chain,
        distractor_passes: distractor,
        counterfactual_passes: counterfactual,
        reversal_passes: reversal,
        persistence_passes: persistence,
        elapsed: start.elapsed(),
    }
}


#[derive(Clone, Debug)]
pub struct V18OpenReasoningReport {
    pub cases: usize,
    pub parse_passes: usize,
    pub composition_passes: usize,
    pub counterfactual_passes: usize,
    pub contradiction_passes: usize,
    pub paraphrase_passes: usize,
    pub elapsed: Duration,
}

impl V18OpenReasoningReport {
    pub fn passed(&self) -> bool {
        self.parse_passes == self.cases
            && self.composition_passes == self.cases
            && self.counterfactual_passes == self.cases
            && self.contradiction_passes == self.cases
            && self.paraphrase_passes == self.cases
    }

    pub fn accuracy(&self) -> f32 {
        let denom = (self.cases * 5).max(1) as f32;
        (self.parse_passes
            + self.composition_passes
            + self.counterfactual_passes
            + self.contradiction_passes
            + self.paraphrase_passes) as f32
            / denom
    }
}

pub fn run_v18_open_reasoning_evaluation() -> V18OpenReasoningReport {
    use crate::open_reasoning::{OpenAnswer, SemanticReasoner};
    use crate::world::WorldGraph;

    let start = Instant::now();
    let cases = 128usize;
    let mut parse = 0usize;
    let mut composition = 0usize;
    let mut counterfactual = 0usize;
    let mut contradiction = 0usize;
    let mut paraphrase = 0usize;

    for i in 0..cases {
        let a = format!("nguon{i}");
        let b = format!("trung{i}");
        let c = format!("dich{i}");
        let d = format!("chan{i}");

        let reasoner = SemanticReasoner::default();

        let chain_text = format!(
            "{a} gây ra {b}. {b} dẫn đến {c}. {a} có gây ra {c} không?"
        );
        let mut world = WorldGraph::new(128, 256);
        let scene = reasoner.ingest(&mut world, &chain_text, i as u64 * 10);
        if scene.clauses.len() == 2 && scene.query.is_some() {
            parse += 1;
        }
        if matches!(
            reasoner.answer_scene(&world, &scene),
            OpenAnswer::Supported { path, .. } if path.len() == 3
        ) {
            composition += 1;
        }

        let cf_text = format!(
            "{a} gây ra {b}. {b} làm cho {c}. nếu bỏ {b} thì {c}?"
        );
        let mut cf_world = WorldGraph::new(128, 256);
        let cf_scene = reasoner.ingest(&mut cf_world, &cf_text, i as u64 * 10 + 1);
        if matches!(
            reasoner.answer_scene(&cf_world, &cf_scene),
            OpenAnswer::Counterfactual { support_delta, .. } if support_delta > 0.20
        ) {
            counterfactual += 1;
        }

        let conflict_text = format!(
            "{a} gây ra {c}. {d} ngăn {c}. {a} có gây ra {c} không?"
        );
        let mut conflict_world = WorldGraph::new(128, 256);
        let conflict_scene =
            reasoner.ingest(&mut conflict_world, &conflict_text, i as u64 * 10 + 2);
        if matches!(
            reasoner.answer_scene(&conflict_world, &conflict_scene),
            OpenAnswer::Contradicted { support, opposition }
                if support > 0.5 && opposition > 0.5
        ) {
            contradiction += 1;
        }

        let paraphrase_text = format!(
            "vì {a} nên {b}. {b} khiến {c}. {a} có dẫn đến {c} không?"
        );
        let mut paraphrase_world = WorldGraph::new(128, 256);
        let paraphrase_scene =
            reasoner.ingest(&mut paraphrase_world, &paraphrase_text, i as u64 * 10 + 3);
        if matches!(
            reasoner.answer_scene(&paraphrase_world, &paraphrase_scene),
            OpenAnswer::Supported { .. }
        ) {
            paraphrase += 1;
        }
    }

    V18OpenReasoningReport {
        cases,
        parse_passes: parse,
        composition_passes: composition,
        counterfactual_passes: counterfactual,
        contradiction_passes: contradiction,
        paraphrase_passes: paraphrase,
        elapsed: start.elapsed(),
    }
}


#[derive(Clone, Debug)]
pub struct V21DeepIntelligenceReport {
    pub cases: usize,
    pub abstraction_passes: usize,
    pub analogy_passes: usize,
    pub induction_passes: usize,
    pub compositional_passes: usize,
    pub elapsed: Duration,
}

impl V21DeepIntelligenceReport {
    pub fn passed(&self) -> bool {
        self.abstraction_passes == self.cases
            && self.analogy_passes == self.cases
            && self.induction_passes == self.cases
            && self.compositional_passes == self.cases
    }

    pub fn accuracy(&self) -> f32 {
        let denom = (self.cases * 4).max(1) as f32;
        (self.abstraction_passes
            + self.analogy_passes
            + self.induction_passes
            + self.compositional_passes) as f32
            / denom
    }
}

pub fn run_v21_deep_intelligence_evaluation() -> V21DeepIntelligenceReport {
    use crate::open_intelligence::OpenIntelligence;
    use crate::open_reasoning::OpenAnswer;
    use crate::semantic::concept_id;
    use crate::world::WorldGraph;

    let start = Instant::now();
    let cases = 128usize;
    let mut abstraction = 0usize;
    let mut analogy = 0usize;
    let mut induction = 0usize;
    let mut compositional = 0usize;

    for i in 0..cases {
        let canonical = format!("nguon_chinh_{i}");
        let alias = format!("ten_khac_{i}");
        let middle = format!("trung_gian_{i}");
        let target = format!("dich_{i}");

        let mut intelligence = OpenIntelligence::default();
        let mut world = WorldGraph::new(256, 512);

        let _ = intelligence.learn(
            &mut world,
            &format!("{canonical} còn gọi là {alias}."),
            i as u64 * 100,
        );
        let scene = intelligence.learn(
            &mut world,
            &format!(
                "{canonical} gây ra {middle}. {alias} có gây ra {middle} không?"
            ),
            i as u64 * 100 + 1,
        );
        if matches!(
            intelligence.answer_scene(&world, &scene),
            OpenAnswer::Supported { .. }
        ) {
            abstraction += 1;
        }

        let analog_from = format!("tuong_tu_nguon_{i}");
        let analog_to = format!("tuong_tu_dich_{i}");
        let scene = intelligence.learn(
            &mut world,
            &format!(
                "{canonical} gây ra {target}. {analog_from} giống {canonical}. {analog_to} giống {target}. {analog_from} có gây ra {analog_to} không?"
            ),
            i as u64 * 100 + 2,
        );
        if matches!(
            intelligence.answer_scene(&world, &scene),
            OpenAnswer::Supported { .. }
        ) {
            analogy += 1;
        }

        let a1 = format!("mau_a1_{i}");
        let a2 = format!("mau_a2_{i}");
        let b1 = format!("mau_b1_{i}");
        let b2 = format!("mau_b2_{i}");
        let x = format!("muc_tieu_x_{i}");
        let y = format!("muc_tieu_y_{i}");
        let _ = intelligence.learn(
            &mut world,
            &format!(
                "{a1} gây ra {b1}. {a2} gây ra {b2}. {x} giống {a1}. {x} giống {a2}. {y} giống {b1}. {y} giống {b2}."
            ),
            i as u64 * 100 + 3,
        );
        if intelligence
            .induce_between(&world, concept_id(&x), concept_id(&y))
            .is_some_and(|r| r.supports >= 2 && r.relation.confidence > 0.60)
        {
            induction += 1;
        }

        let scene = intelligence.learn(
            &mut world,
            &format!(
                "{canonical} gây ra {middle}. {middle} dẫn đến {target}. {alias} có gây ra {target} không?"
            ),
            i as u64 * 100 + 4,
        );
        if matches!(
            intelligence.answer_scene(&world, &scene),
            OpenAnswer::Supported { path, .. } if path.len() >= 3
        ) {
            compositional += 1;
        }
    }

    V21DeepIntelligenceReport {
        cases,
        abstraction_passes: abstraction,
        analogy_passes: analogy,
        induction_passes: induction,
        compositional_passes: compositional,
        elapsed: start.elapsed(),
    }
}


#[derive(Clone, Debug)]
pub struct V25EmergentIntelligenceReport {
    pub cases: usize,
    pub episodic_passes: usize,
    pub discovery_passes: usize,
    pub rule_passes: usize,
    pub competition_passes: usize,
    pub multidomain_passes: usize,
    pub elapsed: Duration,
}

impl V25EmergentIntelligenceReport {
    pub fn passed(&self) -> bool {
        self.episodic_passes == self.cases
            && self.discovery_passes == self.cases
            && self.rule_passes == self.cases
            && self.competition_passes == self.cases
            && self.multidomain_passes == self.cases
    }

    pub fn accuracy(&self) -> f32 {
        let denom = (self.cases * 5).max(1) as f32;
        (self.episodic_passes
            + self.discovery_passes
            + self.rule_passes
            + self.competition_passes
            + self.multidomain_passes) as f32
            / denom
    }
}

pub fn run_v25_emergent_intelligence_evaluation() -> V25EmergentIntelligenceReport {
    use crate::competition::{CandidateHypothesis, HypothesisCompetition};
    use crate::discovery::ContextDiscovery;
    use crate::episodic::EpisodicMemory;
    use crate::open_intelligence::OpenIntelligence;
    use crate::open_reasoning::OpenAnswer;
    use crate::rules::RuleSynthesizer;
    use crate::semantic::{concept_id, VietnameseSemanticParser};
    use crate::types::{Relation, RelationKind};
    use crate::world::WorldGraph;

    let start = Instant::now();
    let cases = 128usize;
    let mut episodic = 0usize;
    let mut discovery = 0usize;
    let mut rules = 0usize;
    let mut competition = 0usize;
    let mut multidomain = 0usize;

    for i in 0..cases {
        let parser = VietnameseSemanticParser;
        let mut memory = EpisodicMemory::default();

        let e1 = parser.parse(&format!(
            "nguon_a_{i} gây ra trung_a_{i}. trung_a_{i} cho phép dich_a_{i}."
        ));
        let e2 = parser.parse(&format!(
            "nguon_b_{i} gây ra trung_b_{i}. trung_b_{i} cho phép dich_b_{i}."
        ));
        memory.observe(&e1, 1, 0.7);
        memory.observe(&e2, 2, 0.8);
        if memory.len() == 2 && memory.episodes().iter().all(|e| e.clauses.len() == 2) {
            episodic += 1;
        }

        let mut world = WorldGraph::new(256, 512);
        let shared1 = concept_id(&format!("shared_x_{i}"));
        let shared2 = concept_id(&format!("shared_y_{i}"));
        let p = concept_id(&format!("pattern_p_{i}"));
        let q = concept_id(&format!("pattern_q_{i}"));
        for id in [shared1, shared2, p, q] {
            world.upsert(crate::types::Phenomenon::new(
                id,
                crate::types::WorldLevel::TrungThien,
                crate::types::SenseGate::Mind,
                id as u32,
                vec![0.5, 0.5],
                0.9,
                0.7,
                1,
            ));
        }
        for (from, to) in [(p, shared1), (q, shared1), (p, shared2), (q, shared2)] {
            world.relate(Relation {
                from,
                to,
                kind: RelationKind::Causes,
                strength: 0.9,
                confidence: 0.9,
            });
        }
        let discovered = ContextDiscovery::default().apply(&mut world);
        if discovered > 0
            && world.edges().iter().any(|e| {
                e.kind == RelationKind::Similar
                    && ((e.from == p && e.to == q) || (e.from == q && e.to == p))
            })
        {
            discovery += 1;
        }

        let mut synth = RuleSynthesizer::default();
        if synth.synthesize(&memory) > 0 {
            let novel_a = concept_id(&format!("novel_a_{i}"));
            let novel_b = concept_id(&format!("novel_b_{i}"));
            let novel_c = concept_id(&format!("novel_c_{i}"));
            for id in [novel_a, novel_b, novel_c] {
                world.upsert(crate::types::Phenomenon::new(
                    id,
                    crate::types::WorldLevel::TrungThien,
                    crate::types::SenseGate::Mind,
                    id as u32,
                    vec![0.4, 0.6],
                    0.9,
                    0.7,
                    2,
                ));
            }
            world.relate(Relation {
                from: novel_a,
                to: novel_b,
                kind: RelationKind::Causes,
                strength: 0.9,
                confidence: 0.9,
            });
            world.relate(Relation {
                from: novel_b,
                to: novel_c,
                kind: RelationKind::Enables,
                strength: 0.9,
                confidence: 0.9,
            });
            let applied = synth.apply(&mut world);
            if applied > 0
                && world.edges().iter().any(|e| {
                    e.from == novel_a && e.to == novel_c && e.kind == RelationKind::Causes
                })
            {
                rules += 1;
            }
        }

        let competition_engine = HypothesisCompetition;
        let winner_rel = Relation {
            from: 1,
            to: 2,
            kind: RelationKind::Causes,
            strength: 0.95,
            confidence: 0.95,
        };
        let weak_rel = Relation {
            from: 1,
            to: 2,
            kind: RelationKind::Enables,
            strength: 0.55,
            confidence: 0.70,
        };
        let clear = competition_engine.choose(&[
            CandidateHypothesis {
                relation: winner_rel.clone(),
                source: "causal",
                evidence: 4,
            },
            CandidateHypothesis {
                relation: weak_rel,
                source: "analogy",
                evidence: 1,
            },
        ]);
        let conflict = competition_engine.choose(&[
            CandidateHypothesis {
                relation: winner_rel,
                source: "causal",
                evidence: 2,
            },
            CandidateHypothesis {
                relation: Relation {
                    from: 1,
                    to: 2,
                    kind: RelationKind::Inhibits,
                    strength: 0.94,
                    confidence: 0.95,
                },
                source: "counter",
                evidence: 2,
            },
        ]);
        if clear.winner.is_some() && !clear.contradicted && conflict.winner.is_none() && conflict.contradicted {
            competition += 1;
        }

        let mut intelligence = OpenIntelligence::default();
        let mut domain_world = WorldGraph::new(256, 512);
        let domains = [
            ("pin", "giamxung", "latency"),
            ("mang", "matgoi", "lag"),
            ("game", "quatai", "giat"),
            ("gia", "bien_dong", "rui_ro"),
        ];
        let (a, b, c) = domains[i % domains.len()];
        let scene = intelligence.learn(
            &mut domain_world,
            &format!("{a}_{i} gây ra {b}_{i}. {b}_{i} dẫn đến {c}_{i}. {a}_{i} có gây ra {c}_{i} không?"),
            i as u64,
        );
        if matches!(
            intelligence.answer_scene(&domain_world, &scene),
            OpenAnswer::Supported { path, .. } if path.len() >= 3
        ) {
            multidomain += 1;
        }
    }

    V25EmergentIntelligenceReport {
        cases,
        episodic_passes: episodic,
        discovery_passes: discovery,
        rule_passes: rules,
        competition_passes: competition,
        multidomain_passes: multidomain,
        elapsed: start.elapsed(),
    }
}


#[derive(Clone, Debug)]
pub struct V31AutonomousKnowledgeReport {
    pub cases: usize,
    pub hierarchy_passes: usize,
    pub hypothesis_passes: usize,
    pub falsification_passes: usize,
    pub meta_rule_passes: usize,
    pub governance_passes: usize,
    pub elapsed: Duration,
}

impl V31AutonomousKnowledgeReport {
    pub fn passed(&self) -> bool {
        self.hierarchy_passes == self.cases
            && self.hypothesis_passes == self.cases
            && self.falsification_passes == self.cases
            && self.meta_rule_passes == self.cases
            && self.governance_passes == self.cases
    }

    pub fn accuracy(&self) -> f32 {
        let denom = (self.cases * 5).max(1) as f32;
        (self.hierarchy_passes
            + self.hypothesis_passes
            + self.falsification_passes
            + self.meta_rule_passes
            + self.governance_passes) as f32
            / denom
    }
}

pub fn run_v31_autonomous_knowledge_evaluation() -> V31AutonomousKnowledgeReport {
    use crate::autonomous_hypothesis::AutonomousHypothesisGenerator;
    use crate::competition::CandidateHypothesis;
    use crate::episodic::EpisodicMemory;
    use crate::hierarchy::HierarchicalAbstraction;
    use crate::knowledge_governor::{KnowledgeDecision, KnowledgeGovernor};
    use crate::meta_rules::MetaRuleCompressor;
    use crate::rules::RuleSynthesizer;
    use crate::semantic::VietnameseSemanticParser;
    use crate::types::{Phenomenon, Relation, RelationKind, SenseGate, WorldLevel};
    use crate::world::WorldGraph;

    let start = Instant::now();
    let cases = 128usize;
    let mut hierarchy = 0usize;
    let mut hypothesis = 0usize;
    let mut falsification = 0usize;
    let mut meta_rule = 0usize;
    let mut governance = 0usize;

    for i in 0..cases {
        let base = i as u64 * 10_000;

        let mut h_world = WorldGraph::new(64, 128);
        for id in [base + 1, base + 2, base + 3, base + 4] {
            h_world.upsert(Phenomenon::new(
                id,
                WorldLevel::TrungThien,
                SenseGate::Mind,
                id as u32,
                vec![0.5],
                0.9,
                0.7,
                1,
            ));
        }
        for (from, to) in [
            (base + 1, base + 3),
            (base + 1, base + 4),
            (base + 2, base + 3),
            (base + 2, base + 4),
        ] {
            h_world.relate(Relation {
                from,
                to,
                kind: RelationKind::Causes,
                strength: 0.9,
                confidence: 0.9,
            });
        }
        let mut abstraction = HierarchicalAbstraction::default();
        let _ = abstraction.discover(&h_world);
        if abstraction
            .concept_for(base + 1)
            .is_some_and(|c| c.members.contains(&(base + 2)))
        {
            hierarchy += 1;
        }

        let mut p_world = WorldGraph::new(32, 64);
        p_world.relate(Relation {
            from: base + 10,
            to: base + 11,
            kind: RelationKind::Causes,
            strength: 1.0,
            confidence: 1.0,
        });
        p_world.relate(Relation {
            from: base + 11,
            to: base + 12,
            kind: RelationKind::Enables,
            strength: 1.0,
            confidence: 1.0,
        });
        let generator = AutonomousHypothesisGenerator::default();
        let generated = generator.generate(&p_world);
        let candidate = generated.iter().find(|c| {
            c.relation.from == base + 10
                && c.relation.to == base + 12
                && c.relation.kind == RelationKind::Causes
        });
        if candidate.is_some() {
            hypothesis += 1;
        }

        let governor = KnowledgeGovernor::default();
        let contradictory = CandidateHypothesis {
            relation: Relation {
                from: base + 20,
                to: base + 21,
                kind: RelationKind::Causes,
                strength: 0.9,
                confidence: 0.9,
            },
            source: "candidate",
            evidence: 2,
        };
        let mut f_world = WorldGraph::new(16, 32);
        f_world.relate(Relation {
            from: base + 20,
            to: base + 21,
            kind: RelationKind::Inhibits,
            strength: 1.0,
            confidence: 1.0,
        });
        if governor.assess(&f_world, &contradictory).decision == KnowledgeDecision::Reject {
            falsification += 1;
        }

        let parser = VietnameseSemanticParser;
        let mut episodes = EpisodicMemory::default();
        for text in [
            format!("a1_{i} gây ra b1_{i}. b1_{i} cho phép c1_{i}."),
            format!("a2_{i} gây ra b2_{i}. b2_{i} cho phép c2_{i}."),
            format!("d1_{i} cho phép e1_{i}. e1_{i} gây ra f1_{i}."),
            format!("d2_{i} cho phép e2_{i}. e2_{i} gây ra f2_{i}."),
        ] {
            let scene = parser.parse(&text);
            episodes.observe(&scene, 1, 0.8);
        }
        let mut synth = RuleSynthesizer::default();
        let _ = synth.synthesize(&episodes);
        let mut meta = MetaRuleCompressor::default();
        let _ = meta.compress(synth.rules());
        if meta
            .rules()
            .iter()
            .any(|m| m.output == RelationKind::Causes && m.source_rules >= 2)
        {
            meta_rule += 1;
        }

        let promotable = CandidateHypothesis {
            relation: Relation {
                from: base + 30,
                to: base + 31,
                kind: RelationKind::Causes,
                strength: 1.0,
                confidence: 1.0,
            },
            source: "strong-two-hop",
            evidence: 3,
        };
        let mut g_world = WorldGraph::new(16, 32);
        let assessment = governor.promote_if_valid(&mut g_world, &promotable);
        if assessment.decision == KnowledgeDecision::Promote
            && g_world.edges().iter().any(|e| {
                e.from == base + 30
                    && e.to == base + 31
                    && e.kind == RelationKind::Causes
            })
        {
            governance += 1;
        }
    }

    V31AutonomousKnowledgeReport {
        cases,
        hierarchy_passes: hierarchy,
        hypothesis_passes: hypothesis,
        falsification_passes: falsification,
        meta_rule_passes: meta_rule,
        governance_passes: governance,
        elapsed: start.elapsed(),
    }
}


#[derive(Clone, Debug)]
pub struct V37DeliberationReport {
    pub cases: usize,
    pub prediction_passes: usize,
    pub planning_passes: usize,
    pub avoidance_passes: usize,
    pub replan_passes: usize,
    pub bounded_passes: usize,
    pub elapsed: Duration,
}

impl V37DeliberationReport {
    pub fn passed(&self) -> bool {
        self.prediction_passes == self.cases
            && self.planning_passes == self.cases
            && self.avoidance_passes == self.cases
            && self.replan_passes == self.cases
            && self.bounded_passes == self.cases
    }

    pub fn accuracy(&self) -> f32 {
        let denom = (self.cases * 5).max(1) as f32;
        (self.prediction_passes
            + self.planning_passes
            + self.avoidance_passes
            + self.replan_passes
            + self.bounded_passes) as f32
            / denom
    }
}

pub fn run_v37_deliberation_evaluation() -> V37DeliberationReport {
    use crate::deliberation::{DeliberativePlanner, GoalSpec};
    use crate::outcome_learning::OutcomeLearner;
    use crate::world_model::{SimState, TransitionModel, WorldModel};

    let start_time = Instant::now();
    let cases = 128usize;
    let mut prediction = 0usize;
    let mut planning = 0usize;
    let mut avoidance = 0usize;
    let mut replan = 0usize;
    let mut bounded = 0usize;

    for i in 0..cases {
        let base = i as u64 * 100;
        let ready = base + 1;
        let prepared = base + 2;
        let done = base + 3;
        let danger = base + 4;
        let fallback = base + 5;

        let a_prepare = base + 10;
        let a_finish = base + 11;
        let a_risky = base + 12;
        let a_recover = base + 13;

        let mut model = WorldModel::default();
        model.add_transition(TransitionModel {
            action: a_prepare,
            requires: vec![ready],
            adds: vec![prepared],
            removes: vec![],
            utility: 0.25,
            cost: 0.05,
            confidence: 0.98,
        });
        model.add_transition(TransitionModel {
            action: a_finish,
            requires: vec![prepared],
            adds: vec![done],
            removes: vec![],
            utility: 1.0,
            cost: 0.10,
            confidence: 0.97,
        });
        model.add_transition(TransitionModel {
            action: a_risky,
            requires: vec![ready],
            adds: vec![done, danger],
            removes: vec![],
            utility: 1.2,
            cost: 0.01,
            confidence: 0.95,
        });
        model.add_transition(TransitionModel {
            action: a_recover,
            requires: vec![fallback],
            adds: vec![prepared],
            removes: vec![fallback],
            utility: 0.15,
            cost: 0.05,
            confidence: 0.95,
        });

        let initial = SimState::new([ready]);
        if model
            .simulate(&initial, a_prepare)
            .is_some_and(|s| s.contains(prepared) && s.contains(ready))
        {
            prediction += 1;
        }

        let goal = GoalSpec {
            desired: vec![done],
            avoid: vec![danger],
        };
        let planner = DeliberativePlanner;
        let plan = planner.plan(&model, &initial, &goal);
        if plan
            .as_ref()
            .is_some_and(|p| p.reached_goal && p.actions == vec![a_prepare, a_finish])
        {
            planning += 1;
        }
        if plan
            .as_ref()
            .is_some_and(|p| !p.final_state.contains(danger))
        {
            avoidance += 1;
        }

        if let Some(planned) = plan {
            let predicted_after_first = model
                .simulate(&initial, planned.actions[0])
                .expect("prediction");
            let observed = SimState::new([fallback]);
            let learner = OutcomeLearner;
            if learner.audit(&predicted_after_first, &observed).replan_required
                && learner
                    .replan_if_needed(&model, &goal, &predicted_after_first, &observed)
                    .is_some_and(|p| {
                        p.actions.first() == Some(&a_recover)
                            && p.actions.contains(&a_finish)
                            && p.reached_goal
                    })
            {
                replan += 1;
            }
        }

        if model.len() <= 64
            && planner
                .plan(&model, &initial, &goal)
                .is_some_and(|p| p.actions.len() <= 4)
        {
            bounded += 1;
        }
    }

    V37DeliberationReport {
        cases,
        prediction_passes: prediction,
        planning_passes: planning,
        avoidance_passes: avoidance,
        replan_passes: replan,
        bounded_passes: bounded,
        elapsed: start_time.elapsed(),
    }
}


#[derive(Clone, Debug)]
pub struct V45MaxIntelligenceReport {
    pub cases: usize,
    pub metacognition_passes: usize,
    pub calibration_passes: usize,
    pub evidence_passes: usize,
    pub recursive_passes: usize,
    pub compute_routing_passes: usize,
    pub transfer_passes: usize,
    pub elapsed: Duration,
}

impl V45MaxIntelligenceReport {
    pub fn passed(&self) -> bool {
        self.metacognition_passes == self.cases
            && self.calibration_passes == self.cases
            && self.evidence_passes == self.cases
            && self.recursive_passes == self.cases
            && self.compute_routing_passes == self.cases
            && self.transfer_passes == self.cases
    }

    pub fn accuracy(&self) -> f32 {
        let denom = (self.cases * 6).max(1) as f32;
        (self.metacognition_passes
            + self.calibration_passes
            + self.evidence_passes
            + self.recursive_passes
            + self.compute_routing_passes
            + self.transfer_passes) as f32
            / denom
    }
}

pub fn run_v45_max_intelligence_evaluation() -> V45MaxIntelligenceReport {
    use crate::active_evidence::{ActiveEvidenceSeeker, EvidenceRequestKind};
    use crate::budget::DeviceState;
    use crate::calibration::SelfCalibration;
    use crate::metacognition::{CognitiveDecision, MetacognitiveController};
    use crate::recursive_deliberation::RecursiveDeliberator;
    use crate::self_directed_compute::{ReasoningTier, SelfDirectedCompute};
    use crate::skill_transfer::CrossDomainTransfer;
    use crate::world_model::{TransitionModel, WorldModel};

    let start = Instant::now();
    let cases = 128usize;
    let mut metacognition = 0usize;
    let mut calibration = 0usize;
    let mut evidence = 0usize;
    let mut recursive = 0usize;
    let mut routing = 0usize;
    let mut transfer = 0usize;

    for i in 0..cases {
        let controller = MetacognitiveController;
        let confident = controller.assess(0.96, 0.02, 2, 8);
        let conflicted = controller.assess(0.82, 0.76, 4, 4);
        if confident.decision == CognitiveDecision::Answer
            && confident.certainty > 0.72
            && conflicted.decision == CognitiveDecision::SeekEvidence
            && conflicted.conflict > 0.45
        {
            metacognition += 1;
        }

        let mut self_cal = SelfCalibration::default();
        for _ in 0..8 {
            self_cal.observe(0.90, false);
        }
        for _ in 0..8 {
            self_cal.observe(0.80, true);
        }
        let before = 0.82f32;
        let adjusted = self_cal.adjusted(before);
        if self_cal.len() == 16
            && self_cal.bias() > 0.0
            && adjusted < before
            && self_cal.brier_score() > 0.0
        {
            calibration += 1;
        }

        let seeker = ActiveEvidenceSeeker;
        if seeker
            .request(i as u64 + 1, &conflicted, false)
            .is_some_and(|r| {
                r.kind == EvidenceRequestKind::Opposing
                    && r.priority > 0.30
            })
        {
            evidence += 1;
        }

        let deliberator = RecursiveDeliberator::default();
        let rr = deliberator.run(0.40, |depth, score| {
            if depth <= 2 {
                score + 0.12
            } else {
                score + 0.01
            }
        });
        if rr.final_score > 0.60
            && rr.passes.len() <= 4
            && rr.stopped_early
        {
            recursive += 1;
        }

        let compute = SelfDirectedCompute;
        let hard = controller.assess(0.58, 0.08, 7, 4);
        let deep_route = compute.route(
            &hard,
            DeviceState {
                battery: 0.9,
                thermal: 0.1,
                load: 0.1,
                available_memory_mb: 1024,
            },
        );
        let pressure_route = compute.route(
            &hard,
            DeviceState {
                battery: 0.08,
                thermal: 0.95,
                load: 0.92,
                available_memory_mb: 96,
            },
        );
        if deep_route.tier == ReasoningTier::Deep
            && deep_route.reasoning_passes >= 3
            && pressure_route.tier == ReasoningTier::Instant
            && pressure_route.reasoning_passes == 1
        {
            routing += 1;
        }

        let base = i as u64 * 1000;
        let mut source = WorldModel::default();
        source.add_transition(TransitionModel {
            action: base + 1,
            requires: vec![],
            adds: vec![base + 100],
            removes: vec![],
            utility: 0.5,
            cost: 0.1,
            confidence: 0.95,
        });
        source.add_transition(TransitionModel {
            action: base + 2,
            requires: vec![],
            adds: vec![base + 101, base + 102],
            removes: vec![],
            utility: 0.8,
            cost: 0.1,
            confidence: 0.95,
        });

        let xfer = CrossDomainTransfer;
        if let Some(pattern) = xfer.extract(&source, &[base + 1, base + 2]) {
            let mut target = WorldModel::default();
            target.add_transition(TransitionModel {
                action: base + 11,
                requires: vec![],
                adds: vec![base + 201],
                removes: vec![],
                utility: 0.4,
                cost: 0.05,
                confidence: 0.9,
            });
            target.add_transition(TransitionModel {
                action: base + 12,
                requires: vec![],
                adds: vec![base + 202, base + 203],
                removes: vec![],
                utility: 0.9,
                cost: 0.1,
                confidence: 0.9,
            });
            target.add_transition(TransitionModel {
                action: base + 13,
                requires: vec![],
                adds: vec![base + 204, base + 205, base + 206],
                removes: vec![],
                utility: -0.3,
                cost: 0.2,
                confidence: 0.9,
            });
            if xfer
                .match_actions(&target, &pattern)
                .is_some_and(|actions| actions == vec![base + 11, base + 12])
            {
                transfer += 1;
            }
        }
    }

    V45MaxIntelligenceReport {
        cases,
        metacognition_passes: metacognition,
        calibration_passes: calibration,
        evidence_passes: evidence,
        recursive_passes: recursive,
        compute_routing_passes: routing,
        transfer_passes: transfer,
        elapsed: start.elapsed(),
    }
}


#[derive(Clone, Debug)]
pub struct V61LearnedSemanticReport {
    pub cases: usize,
    pub embedding_passes: usize,
    pub latent_memory_passes: usize,
    pub relation_passes: usize,
    pub vector_retrieval_passes: usize,
    pub compression_passes: usize,
    pub hybrid_passes: usize,
    pub elapsed: Duration,
}

impl V61LearnedSemanticReport {
    pub fn passed(&self) -> bool {
        self.embedding_passes == self.cases
            && self.latent_memory_passes == self.cases
            && self.relation_passes == self.cases
            && self.vector_retrieval_passes == self.cases
            && self.compression_passes == self.cases
            && self.hybrid_passes == self.cases
    }

    pub fn accuracy(&self) -> f32 {
        let denom = (self.cases * 6).max(1) as f32;
        (self.embedding_passes
            + self.latent_memory_passes
            + self.relation_passes
            + self.vector_retrieval_passes
            + self.compression_passes
            + self.hybrid_passes) as f32
            / denom
    }
}

pub fn run_v61_learned_semantic_evaluation() -> V61LearnedSemanticReport {
    use crate::hybrid_semantic::HybridSemanticReasoner;
    use crate::knowledge::{KnowledgeLedger, KnowledgeRecord, ProvenanceKind};
    use crate::latent_memory::LatentMemory;
    use crate::latent_relation::LatentRelationLearner;
    use crate::semantic_compression::SemanticCompressor;
    use crate::semantic_embedding::SemanticEncoder;
    use crate::types::RelationKind;
    use crate::vector_retrieval::VectorSemanticRetriever;

    let started = Instant::now();
    let cases = 128usize;
    let mut embedding = 0usize;
    let mut latent_memory = 0usize;
    let mut relation = 0usize;
    let mut vector_retrieval = 0usize;
    let mut compression = 0usize;
    let mut hybrid = 0usize;

    for i in 0..cases {
        let encoder = SemanticEncoder;
        let a = encoder.encode(&format!("pin yeu gay ra hieu nang cham {i}"));
        let b = encoder.encode(&format!("pin yeu gay nen may cham {i}"));
        let unrelated = encoder.encode(&format!("hoa sen no buoi sang {i}"));
        if a.cosine(&b) > a.cosine(&unrelated) {
            embedding += 1;
        }

        let mut memory = LatentMemory::default();
        memory.remember(
            i as u64 + 1,
            &format!("nhiet cao gay ra throttling {i}"),
            0.95,
        );
        memory.remember(
            i as u64 + 10_000,
            &format!("hoa sen no buoi sang {i}"),
            0.95,
        );
        if memory
            .nearest(&format!("nhiet cao gay nen throttling {i}"), 1)
            .first()
            .is_some_and(|(item, _)| item.id == i as u64 + 1)
        {
            latent_memory += 1;
        }

        let mut learner = LatentRelationLearner::default();
        for text in [
            format!("a{i} gay ra b{i}"),
            format!("c{i} gay ra d{i}"),
            format!("e{i} dan den f{i}"),
        ] {
            learner.observe(&text, RelationKind::Causes, 0.96);
        }
        for text in [
            format!("g{i} cho phep h{i}"),
            format!("j{i} ho tro k{i}"),
            format!("m{i} tao dieu kien cho n{i}"),
        ] {
            learner.observe(&text, RelationKind::Enables, 0.96);
        }
        for text in [
            format!("p{i} ngan q{i}"),
            format!("r{i} can tro s{i}"),
            format!("t{i} ngan can u{i}"),
        ] {
            learner.observe(&text, RelationKind::Inhibits, 0.96);
        }

        if learner
            .classify(&format!("x{i} gay nen y{i}"))
            .is_some_and(|(kind, score)| kind == RelationKind::Causes && score >= 0.58)
        {
            relation += 1;
        }

        let mut ledger = KnowledgeLedger::new(8);
        ledger.add(KnowledgeRecord {
            id: 1,
            source: "thermal".to_string(),
            kind: ProvenanceKind::LocalDocument,
            excerpt: format!("nhiet cao gay ra throttling va lam may cham {i}"),
            timestamp: 1,
            confidence: 0.95,
        });
        ledger.add(KnowledgeRecord {
            id: 2,
            source: "lotus".to_string(),
            kind: ProvenanceKind::LocalDocument,
            excerpt: format!("hoa sen no vao buoi sang {i}"),
            timestamp: 2,
            confidence: 0.95,
        });
        if VectorSemanticRetriever::default()
            .recall(&ledger, &format!("nhiet cao gay nen may cham {i}"), 1)
            .first()
            .is_some_and(|h| h.record.id == 1)
        {
            vector_retrieval += 1;
        }

        let mut compressor = SemanticCompressor::default();
        let c1 = compressor.observe(&format!("pin yeu gay ra may cham {i}"), 0.9);
        let c2 = compressor.observe(&format!("pin yeu gay nen may cham {i}"), 0.9);
        if c1 == c2 && compressor.len() == 1 {
            compression += 1;
        }

        let reasoner = HybridSemanticReasoner;
        if reasoner
            .infer_clause(&learner, &format!("nguon{i} gay nen dich{i}"))
            .is_some_and(|x| x.kind == RelationKind::Causes && x.confidence > 0.45)
        {
            hybrid += 1;
        }
    }

    V61LearnedSemanticReport {
        cases,
        embedding_passes: embedding,
        latent_memory_passes: latent_memory,
        relation_passes: relation,
        vector_retrieval_passes: vector_retrieval,
        compression_passes: compression,
        hybrid_passes: hybrid,
        elapsed: started.elapsed(),
    }
}


#[derive(Clone, Debug)]
pub struct V81ContinualGenerativeReport {
    pub cases: usize,
    pub continual_passes: usize,
    pub anchor_passes: usize,
    pub composition_passes: usize,
    pub symbol_passes: usize,
    pub consolidation_passes: usize,
    pub generation_passes: usize,
    pub elapsed: Duration,
}

impl V81ContinualGenerativeReport {
    pub fn passed(&self) -> bool {
        self.continual_passes == self.cases
            && self.anchor_passes == self.cases
            && self.composition_passes == self.cases
            && self.symbol_passes == self.cases
            && self.consolidation_passes == self.cases
            && self.generation_passes == self.cases
    }

    pub fn accuracy(&self) -> f32 {
        let denom = (self.cases * 6).max(1) as f32;
        (self.continual_passes
            + self.anchor_passes
            + self.composition_passes
            + self.symbol_passes
            + self.consolidation_passes
            + self.generation_passes) as f32
            / denom
    }
}

pub fn run_v81_continual_generative_evaluation() -> V81ContinualGenerativeReport {
    use crate::concept_composition::ConceptComposer;
    use crate::continual_semantics::ContinualSemanticLearner;
    use crate::generative_cognition::{GenerativeCognition, ResponseStance};
    use crate::latent_symbol_bridge::LatentSymbolBridge;
    use crate::open_reasoning::OpenAnswer;
    use crate::semantic::concept_id;
    use crate::semantic_consolidation::SemanticConsolidator;

    let started = Instant::now();
    let cases = 128usize;
    let mut continual = 0usize;
    let mut anchor = 0usize;
    let mut composition = 0usize;
    let mut symbol = 0usize;
    let mut consolidation = 0usize;
    let mut generation = 0usize;

    for i in 0..cases {
        let stable_id = concept_id(&format!("pin yeu may cham {i}"));
        let near_id = concept_id(&format!("pin yeu thiet bi cham {i}"));
        let mut learner = ContinualSemanticLearner::default();
        learner.observe(stable_id, &format!("pin yeu gay ra may cham {i}"));
        learner.observe(near_id, &format!("pin yeu gay nen thiet bi cham {i}"));
        let before = learner.similarity(stable_id, near_id).unwrap_or(0.0);

        for j in 0..48u64 {
            learner.observe(
                10_000 + i as u64 * 100 + j,
                &format!("khai niem nhieu {i} {j} khac biet"),
            );
        }
        let after = learner.similarity(stable_id, near_id).unwrap_or(0.0);
        if before > 0.65 && after > 0.60 {
            continual += 1;
        }

        let _ = learner.anchor(stable_id);
        for k in 0..16 {
            learner.observe(
                stable_id,
                &format!("pin yeu bien the rat khac {i} {k}"),
            );
        }
        let restored = learner.restore_anchors(0.08);
        if restored > 0
            && learner
                .concept(stable_id)
                .is_some_and(|x| x.stability > 0.55)
        {
            anchor += 1;
        }

        let composer = ConceptComposer::default();
        let related = composer
            .compositional_similarity(
                &["pin yeu", "may cham"],
                "pin yeu gay ra may cham",
            )
            .unwrap_or(0.0);
        let unrelated = composer
            .compositional_similarity(
                &["pin yeu", "may cham"],
                "hoa sen no buoi sang",
            )
            .unwrap_or(1.0);
        if related > unrelated {
            composition += 1;
        }

        let mut bridge = LatentSymbolBridge::default();
        bridge.bind(1, &format!("nhiet cao throttling {i}"), 0.95);
        bridge.bind(2, &format!("hoa sen buoi sang {i}"), 0.95);
        if bridge
            .nearest_symbol(&format!("nhiet cao lam throttling {i}"))
            .is_some_and(|(s, score)| s.id == 1 && score >= 0.50)
        {
            symbol += 1;
        }

        let mut consolidator = SemanticConsolidator::default();
        let report = consolidator.consolidate(&mut learner, &[stable_id]);
        if report.anchored == 1
            && report.retained <= 96
            && consolidator.passes() == 1
        {
            consolidation += 1;
        }

        let generator = GenerativeCognition;
        let strong = generator.render(
            &OpenAnswer::Supported {
                confidence: 0.95,
                path: vec![1, 2, 3],
            },
            0.05,
        );
        let weak = generator.render(
            &OpenAnswer::Supported {
                confidence: 0.62,
                path: vec![1, 2, 3],
            },
            0.45,
        );
        let conflict = generator.render(
            &OpenAnswer::Contradicted {
                support: 0.82,
                opposition: 0.78,
            },
            0.4,
        );
        let cf = generator.render(
            &OpenAnswer::Counterfactual {
                support_delta: 0.48,
                factual_support: 0.90,
                counterfactual_support: 0.42,
            },
            0.2,
        );
        if strong.stance == ResponseStance::Certain
            && weak.stance == ResponseStance::Cautious
            && conflict.stance == ResponseStance::Contradictory
            && cf.stance == ResponseStance::Counterfactual
            && strong.text != weak.text
            && conflict.text != cf.text
        {
            generation += 1;
        }
    }

    V81ContinualGenerativeReport {
        cases,
        continual_passes: continual,
        anchor_passes: anchor,
        composition_passes: composition,
        symbol_passes: symbol,
        consolidation_passes: consolidation,
        generation_passes: generation,
        elapsed: started.elapsed(),
    }
}
