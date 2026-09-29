#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(target_os = "android")]
pub mod android_ffi;
pub mod action;
pub mod action_queue;
pub mod adaptation;
pub mod budget;
pub mod capability;
pub mod continuity;
pub mod core;
pub mod dialogue;
pub mod curriculum;
pub mod four_matrix;
pub mod goals;
pub mod inference_matrix;
pub mod knowledge;
pub mod language;
pub mod meaning;
pub mod perception;
pub mod planner;
pub mod planning;
pub mod memory;
pub mod mobile;
pub mod persistence;
pub mod runtime;
pub mod token_stream;
pub mod types;
pub mod world;

pub use action::{ActionDecision, ActionProposal, Authority, CuTranPolicy};
pub use action_queue::ActionQueue;
pub use adaptation::{causal_credit, evaluate_delta, PromotionDecision, SkillDelta};
pub use budget::{middle_way, Budget, DeviceState};
pub use capability::{action_for_goal, encode_action, infer_device_action, DeviceAction, DeviceActionKind};
pub use continuity::{decode_continuity, encode_continuity, ContinuityState};
pub use core::{BiaDca, BiaDcaConfig};
pub use dialogue::{DialogueContext, DialogueTurn, Speaker};
pub use four_matrix::{
    classify_realm, encode_text_aggregates, AggregateVector, FourMatrixKernel,
    FourMatrixOutput, PerspectiveProjection, RealmBand, AGGREGATES,
};
pub use goals::{Goal, GoalStack, GoalStatus};
pub use curriculum::{score as score_curriculum, CurriculumDomain, CurriculumScore, TrialResult};
pub use inference_matrix::{f32_to_q15, q15_to_f32, MatrixDecision, MatrixLevel, MatrixSignal, TamThienMatrix, LANES};
pub use knowledge::{KnowledgeLedger, KnowledgeRecord, ProvenanceKind};
pub use language::{LanguageIntent, VietnameseGate};
pub use meaning::{Concept, MeaningFormation};
pub use perception::{MultiCanh, PerceptPacket};
pub use planner::{decompose_goal, Plan as DevicePlan};
pub use planning::{DeepQuan, Plan, PlanStep};
pub use memory::{Seed, SeedMemory};
pub use mobile::{MobileReply, OfflineMobileBia};
pub use persistence::{decode, encode, read_file, write_atomic, DharmaSnapshot, PersistenceError};
pub use runtime::{CapacityTier, RuntimeProfile, RuntimeTarget};
pub use token_stream::{InstantToken, InstantTokenEmitter};
pub use types::*;
pub use world::WorldGraph;

#[cfg(test)]
mod tests {
    use super::*;

    fn device() -> DeviceState {
        DeviceState {
            battery: 0.8,
            thermal: 0.2,
            load: 0.2,
            available_memory_mb: 512,
        }
    }

    #[test]
    fn bounded_world_and_seed_memory() {
        let mut bia = BiaDca::new(BiaDcaConfig {
            world_nodes: 3,
            world_edges: 4,
            seeds: 2,
            concepts: 2,
            active_causes: 2,
        });
        for i in 0..8 {
            bia.observe(Phenomenon::new(
                i,
                WorldLevel::TieuThien,
                SenseGate::Mind,
                i as u32,
                vec![i as f32, 1.0],
                0.8,
                0.5,
                i,
            ));
        }
        assert!(bia.world.len() <= 3);
        for i in 0..5 {
            let p = Phenomenon::new(
                100 + i,
                WorldLevel::TrungThien,
                SenseGate::System,
                i as u32,
                vec![1.0, i as f32 + 0.1],
                0.9,
                0.8,
                i,
            );
            bia.experience(&p, i as u32, 0.8, 0.1);
        }
        assert!(bia.memory.len() <= 2);
        assert!(bia.meaning.len() <= 2);
    }

    #[test]
    fn dependent_causes_form_hypotheses() {
        let mut bia = BiaDca::new(BiaDcaConfig::default());
        let rain = Phenomenon::new(
            1,
            WorldLevel::TrungThien,
            SenseGate::Sight,
            10,
            vec![1.0, 0.2],
            0.95,
            0.8,
            1,
        );
        let wet = Phenomenon::new(
            2,
            WorldLevel::TieuThien,
            SenseGate::Touch,
            11,
            vec![0.9, 0.2],
            0.9,
            0.9,
            2,
        );
        bia.observe(rain);
        bia.observe(wet.clone());
        bia.relate(Relation {
            from: 1,
            to: 2,
            kind: RelationKind::Causes,
            strength: 0.9,
            confidence: 0.95,
        });
        let moment = bia.contemplate(wet, device(), 0.8);
        assert!(!moment.hypotheses.is_empty());
        assert_eq!(moment.hypotheses[0].source, 1);
        assert_eq!(bia.cycle(), 1);
    }

    #[test]
    fn middle_way_enters_stillness_under_pressure() {
        let b = middle_way(
            DeviceState {
                battery: 0.05,
                thermal: 0.98,
                load: 0.95,
                available_memory_mb: 64,
            },
            1.0,
            1.0,
        );
        assert_eq!(b.mode, ComputeMode::Tinh);
        assert_eq!(b.contemplation_cycles, 0);
    }

    #[test]
    fn experience_changes_recall_without_model_weights() {
        let mut bia = BiaDca::new(BiaDcaConfig::default());
        let p = Phenomenon::new(
            7,
            WorldLevel::TieuThien,
            SenseGate::Mind,
            77,
            vec![0.3, 0.8, 0.1],
            0.9,
            0.7,
            1,
        );
        bia.experience(&p, 900, 0.9, 0.0);
        let recalled = bia.memory.recall(&p, 4);
        assert_eq!(recalled[0].meaning, 900);
    }

    #[test]
    fn repeated_phenomena_form_one_concept() {
        let mut m = MeaningFormation::new(8, 1000);
        let a = Phenomenon::new(
            1,
            WorldLevel::TieuThien,
            SenseGate::Sight,
            1,
            vec![1.0, 0.05, 0.0],
            0.9,
            0.8,
            1,
        );
        let b = Phenomenon::new(
            2,
            WorldLevel::TieuThien,
            SenseGate::Sight,
            9,
            vec![0.98, 0.08, 0.0],
            0.95,
            0.7,
            2,
        );
        let ca = m.observe(&a);
        let cb = m.observe(&b);
        assert_eq!(ca, cb);
        assert_eq!(m.len(), 1);
        assert_eq!(m.concepts()[0].observations, 2);
    }

    #[test]
    fn contradiction_lowers_concept_confidence() {
        let mut m = MeaningFormation::new(4, 2000);
        let p = Phenomenon::new(
            1,
            WorldLevel::TieuThien,
            SenseGate::Mind,
            3,
            vec![0.2, 0.7],
            0.9,
            0.5,
            1,
        );
        let id = m.observe(&p);
        let before = m.concepts()[0].confidence;
        m.contradict(id, 1.0);
        assert!(m.concepts()[0].confidence < before);
    }

    #[test]
    fn dharma_snapshot_roundtrip_and_corruption_detection() {
        let mut bia = BiaDca::new(BiaDcaConfig::default());
        let p = Phenomenon::new(
            42,
            WorldLevel::DaiThien,
            SenseGate::System,
            5,
            vec![0.1, 0.2, 0.3],
            0.88,
            0.66,
            123,
        );
        bia.observe(p.clone());
        bia.experience(&p, 555, 0.7, 0.1);

        let bytes = encode(&bia.snapshot());
        let restored = decode(&bytes).expect("snapshot should decode");
        assert_eq!(restored.world.len(), 1);
        assert_eq!(restored.memory.len(), 1);
        assert_eq!(restored.world.node(42).unwrap().features, p.features);

        let mut damaged = bytes;
        let last = damaged.len() - 1;
        damaged[last] ^= 0x55;
        assert!(matches!(
            decode(&damaged),
            Err(PersistenceError::ChecksumMismatch)
        ));
    }
    #[test]
    fn vietnamese_gate_produces_phenomena_and_intent() {
        let gate = VietnameseGate;
        let ps = gate.perceive("Mở ứng dụng và tìm nhạc", 10);
        assert!(!ps.is_empty());
        let intent = gate.infer_intent("Mở ứng dụng");
        assert_ne!(intent.verb, 0);
    }

    #[test]
    fn multicanh_binds_modalities() {
        let binder = MultiCanh;
        let p = binder.bind(
            &[
                PerceptPacket {
                    gate: SenseGate::Sight,
                    source: 1,
                    values: vec![1.0, 0.2],
                    confidence: 0.9,
                    timestamp: 4,
                },
                PerceptPacket {
                    gate: SenseGate::Sound,
                    source: 2,
                    values: vec![0.4, 0.7],
                    confidence: 0.8,
                    timestamp: 5,
                },
            ],
            99,
        );
        assert_eq!(p.id, 99);
        assert_eq!(p.level, WorldLevel::TrungThien);
    }

    #[test]
    fn deep_quan_builds_bounded_plan() {
        let q = DeepQuan::new(4, 3);
        let rel = Relation {
            from: 1,
            to: 2,
            kind: RelationKind::Enables,
            strength: 0.9,
            confidence: 0.9,
        };
        let plan = q.plan(7, Some(2), &[rel], 0.2);
        assert!(!plan.steps.is_empty());
        assert!(plan.steps.len() <= 4);
    }

    #[test]
    fn cutran_denies_unauthorized_irreversible_action() {
        let p = ActionProposal {
            step: PlanStep {
                action: 1,
                target: None,
                confidence: 0.9,
                reversible: false,
            },
            authority: Authority::Irreversible,
            rationale_confidence: 0.9,
        };
        assert!(matches!(
            CuTranPolicy::default().evaluate(p),
            ActionDecision::Denied(_)
        ));
    }

    #[test]
    fn adaptation_promotes_only_measured_gain() {
        let d = SkillDelta {
            id: 1,
            added_relations: vec![],
            added_seeds: vec![],
            score_before: 0.5,
            score_after: 0.7,
        };
        assert_eq!(evaluate_delta(&d, 0.1), PromotionDecision::Promote);
    }

    #[test]
    fn runtime_scales_capacity_not_architecture() {
        let tiny = RuntimeProfile::for_memory_mb(RuntimeTarget::AndroidArm64, 128);
        let mobile = RuntimeProfile::for_memory_mb(RuntimeTarget::AndroidArm64, 1024);
        assert_eq!(tiny.tier, CapacityTier::Tiny);
        assert_eq!(mobile.tier, CapacityTier::Mobile);
        assert!(mobile.max_world_nodes > tiny.max_world_nodes);
    }

    #[test]
    fn curriculum_scores_accuracy_and_calibration() {
        let s = score_curriculum(
            &[
                TrialResult {
                    domain: CurriculumDomain::Causality,
                    correct: true,
                    confidence: 0.9,
                    latency_ms: 20,
                    memory_kb: 200,
                },
                TrialResult {
                    domain: CurriculumDomain::Planning,
                    correct: true,
                    confidence: 0.8,
                    latency_ms: 30,
                    memory_kb: 220,
                },
            ],
            0.8,
        );
        assert!(s.passed);
    }

    #[test]
    fn offline_mobile_conversation_runs_through_bia_core() {
        let bia = BiaDca::new(BiaDcaConfig::default());
        let mut app = OfflineMobileBia::new(bia);
        let reply = app
            .converse(
                "Mở nhạc",
                1,
                DeviceState {
                    battery: 0.8,
                    thermal: 0.2,
                    load: 0.2,
                    available_memory_mb: 512,
                },
            )
            .expect("reply");
        assert!(!reply.text.is_empty());
        assert!(app.bia.cycle() > 0);
    }

    #[test]
    fn vietnamese_reply_is_human_readable_not_concept_ids() {
        let bia = BiaDca::new(BiaDcaConfig::default());
        let mut app = OfflineMobileBia::new(bia);
        let reply = app
            .converse("Xin chào", 1, device())
            .expect("reply");
        assert!(reply.text.contains("Tôi"));
        assert!(!reply.text.contains("khái-niệm-"));
    }

    #[test]
    fn restored_snapshot_keeps_native_world_and_memory() {
        let mut bia = BiaDca::new(BiaDcaConfig::default());
        let p = Phenomenon::new(
            808,
            WorldLevel::TrungThien,
            SenseGate::Mind,
            88,
            vec![0.8, 0.08],
            0.9,
            0.7,
            9,
        );
        bia.observe(p.clone());
        bia.experience(&p, 8080, 0.9, 0.0);
        let snapshot = decode(&encode(&bia.snapshot())).expect("snapshot");
        let restored = BiaDca::from_snapshot(BiaDcaConfig::default(), snapshot);
        assert!(restored.world.node(808).is_some());
        assert_eq!(restored.memory.len(), 1);
    }

    #[test]
    fn taught_memory_is_imprinted_without_global_retraining() {
        let bia = BiaDca::new(BiaDcaConfig::default());
        let mut app = OfflineMobileBia::new(bia);
        let before = app.bia.memory.len();
        let reply = app
            .converse("Nhớ rằng sen là biểu tượng tôi đang nói tới", 10, device())
            .expect("reply");
        assert!(reply.text.contains("ghi nhận"));
        assert!(app.bia.memory.len() > before);
    }

    #[test]
    fn goal_is_retained_across_turns_in_runtime() {
        let bia = BiaDca::new(BiaDcaConfig::default());
        let mut app = OfflineMobileBia::new(bia);
        app.converse("Mục tiêu: tìm tài liệu học Rust", 20, device())
            .expect("goal reply");
        assert_eq!(
            app.goals.active().map(|g| g.description.as_str()),
            Some("tìm tài liệu học Rust")
        );
        app.converse("tiếp tục", 21, device()).expect("follow-up");
        assert!(app.dialogue.len() >= 4);
    }

    #[test]
    fn device_action_is_structured_and_requires_ui_execution() {
        let bia = BiaDca::new(BiaDcaConfig::default());
        let mut app = OfflineMobileBia::new(bia);
        let reply = app
            .converse("Tìm web Phật giáo Trúc Lâm", 30, device())
            .expect("reply");
        let action = reply.pending_action.expect("pending action");
        assert_eq!(action.kind, DeviceActionKind::SearchWeb);
        assert!(action.payload.contains("Phật giáo"));
    }

    #[test]
    fn action_outcome_becomes_experience() {
        let bia = BiaDca::new(BiaDcaConfig::default());
        let mut app = OfflineMobileBia::new(bia);
        app.converse("Mở cài đặt", 40, device()).expect("reply");
        let before = app.bia.memory.len();
        app.resolve_pending_action(true, 41);
        assert!(app.bia.memory.len() > before);
        assert!(app.pending_action().is_none());
    }

    #[test]
    fn continuity_roundtrip_restores_goal_and_action_queue() {
        let bia = BiaDca::new(BiaDcaConfig::default());
        let mut app = OfflineMobileBia::new(bia);
        app.converse("Mục tiêu: tìm tài liệu Rust", 100, device())
            .expect("goal");
        app.converse("Tiếp tục", 101, device()).expect("continue");
        assert!(app.pending_action().is_some());

        let state = app.continuity_export();
        let mut restored = OfflineMobileBia::new(BiaDca::new(BiaDcaConfig::default()));
        assert!(restored.continuity_import(&state));
        assert!(restored.goals.active().is_some());
        assert!(restored.pending_action().is_some());
    }

    #[test]
    fn failed_action_clears_remaining_queue_and_blocks_goal() {
        let bia = BiaDca::new(BiaDcaConfig::default());
        let mut app = OfflineMobileBia::new(bia);
        app.converse("Mục tiêu: tìm tài liệu Rust", 110, device())
            .expect("goal");
        app.converse("Tiếp tục", 111, device()).expect("continue");
        assert!(app.queue_len() > 0);
        app.resolve_pending_action(false, 112);
        assert_eq!(app.queue_len(), 0);
        assert_eq!(
            app.goals.active().map(|g| g.status),
            None
        );
    }

    #[test]
    fn planner_decomposes_explicit_multi_step_goal() {
        let plan = decompose_goal(
            "Tìm web Phật giáo Trúc Lâm rồi mở https://example.com",
            700,
        );
        assert_eq!(plan.steps.len(), 2);
    }

    #[test]
    fn provenance_ingestion_becomes_knowledge_and_memory() {
        let bia = BiaDca::new(BiaDcaConfig::default());
        let mut app = OfflineMobileBia::new(bia);
        let before = app.bia.memory.len();
        let count = app.ingest_content(
            "note.txt",
            ProvenanceKind::LocalDocument,
            "Trúc Lâm là nguồn tài liệu thử nghiệm cho BIA.",
            800,
            0.9,
        );
        assert!(count > 0);
        assert_eq!(app.knowledge.len(), 1);
        assert!(app.bia.memory.len() > before);
    }

    #[test]
    fn continuity_v2_keeps_provenance_records() {
        let bia = BiaDca::new(BiaDcaConfig::default());
        let mut app = OfflineMobileBia::new(bia);
        app.ingest_content(
            "shared",
            ProvenanceKind::SharedText,
            "nội dung có nguồn",
            900,
            0.8,
        );
        let state = app.continuity_export();
        let mut restored = OfflineMobileBia::new(BiaDca::new(BiaDcaConfig::default()));
        assert!(restored.continuity_import(&state));
        assert_eq!(restored.knowledge.len(), 1);
        assert_eq!(
            restored.knowledge.recent().map(|r| r.source.as_str()),
            Some("shared")
        );
    }

    #[test]
    fn tam_thien_matrix_is_bounded_and_deterministic() {
        let matrix = TamThienMatrix::default();
        let signals = vec![
            MatrixSignal { lane: 1, value_q15: 28000 },
            MatrixSignal { lane: 1, value_q15: 4000 },
            MatrixSignal { lane: 5, value_q15: 12000 },
        ];
        let a = matrix.infer(&signals);
        let b = matrix.infer(&signals);
        assert_eq!(a, b);
        assert!(a.winner < LANES as u8);
        assert_eq!(a.tieu.len(), LANES);
        assert_eq!(a.trung.len(), LANES);
        assert_eq!(a.dai.len(), LANES);
    }

    #[test]
    fn instant_token_path_is_bounded_for_long_input() {
        let mut emitter = InstantTokenEmitter::default();
        let long = "abc ".repeat(10_000);
        let signals = emitter.signals_from_text(&long);
        assert!(signals.len() <= LANES);
        let tokens = emitter.emit_immediate("làm tiếp");
        assert!(tokens.len() <= 1);
    }

    #[test]
    fn core_fast_matrix_runs_without_world_scan() {
        let bia = BiaDca::new(BiaDcaConfig::default());
        let focus = Phenomenon::new(
            1,
            WorldLevel::TieuThien,
            SenseGate::Mind,
            7,
            vec![0.9, 0.1, 0.7, 0.2],
            0.95,
            0.8,
            1,
        );
        let decision = bia.fast_matrix(&focus);
        assert!(decision.winner < LANES as u8);
    }

    #[test]
    fn four_matrix_realm_mask_suppresses_unneeded_form_channel() {
        let input = encode_text_aggregates("logic suy luận trừu tượng");
        let mut kernel = FourMatrixKernel::default();
        let abstract_out = kernel.process(input, RealmBand::Abstract);
        let mut embodied_kernel = FourMatrixKernel::default();
        let embodied_out = embodied_kernel.process(input, RealmBand::Embodied);
        assert!(abstract_out.masked.rupa < embodied_out.masked.rupa);
    }

    #[test]
    fn dependent_origin_state_changes_incrementally() {
        let mut kernel = FourMatrixKernel::default();
        let input = AggregateVector {
            rupa: 12000,
            vedana: 8000,
            sanna: 24000,
            sankhara: 16000,
            vinnana: 26000,
        };
        let first = kernel.process(input, RealmBand::Mixed);
        let second = kernel.process(input, RealmBand::Mixed);
        assert_ne!(first.conditioned, second.conditioned);
        assert_eq!(kernel.state(), second.conditioned);
    }

    #[test]
    fn zero_state_is_scratch_only_not_seed_memory() {
        let mut bia = BiaDca::new(BiaDcaConfig::default());
        let p = Phenomenon::new(
            999,
            WorldLevel::TieuThien,
            SenseGate::Mind,
            9,
            vec![0.8, 0.3],
            0.9,
            0.7,
            1,
        );
        bia.experience(&p, 77, 0.9, 0.0);
        let before = bia.memory.len();
        let _ = bia.fast_matrix(&p);
        assert_eq!(bia.memory.len(), before);
    }

}
