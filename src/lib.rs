#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(target_os = "android")]
pub mod android_ffi;
pub mod abstraction;
pub mod active_evidence;
pub mod autonomous_hypothesis;
pub mod autonomous_cognitive_loop;
pub mod answer_critic;
pub mod analogy;
pub mod action;
pub mod action_queue;
pub mod adaptation;
pub mod budget;
pub mod calibration;
pub mod capability;
pub mod continuity;
pub mod hierarchy;
pub mod idle_cognition;
pub mod internal_questions;
pub mod hybrid_semantic;
pub mod competition;
pub mod concept_composition;
pub mod continual_semantics;
pub mod core;
pub mod dialogue;
pub mod deliberation;
pub mod discovery;
pub mod duyen_token;
pub mod episodic;
pub mod evidence_search;
pub mod integrated_cognition;
pub mod integrated_evaluation;
pub mod evidence_evaluation;
pub mod evaluation;
pub mod curriculum;
pub mod four_matrix;
pub mod generative_cognition;
pub mod goals;
pub mod induction;
pub mod inference_matrix;
pub mod knowledge;
pub mod knowledge_governor;
pub mod language;
pub mod latent_memory;
pub mod latent_symbol_bridge;
pub mod latent_relation;
pub mod meaning;
pub mod metacognition;
pub mod meta_rules;
pub mod perception;
pub mod planner;
pub mod planning;
pub mod memory;
pub mod mobile;
pub mod open_intelligence;
pub mod open_reasoning;
pub mod outcome_learning;
pub mod persistence;
pub mod reasoning;
pub mod recursive_deliberation;
pub mod rules;
pub mod retrieval;
pub mod semantic;
pub mod semantic_compression;
pub mod semantic_consolidation;
pub mod semantic_embedding;
pub mod runtime;
pub mod self_directed_compute;
pub mod skill_transfer;
pub mod token_stream;
pub mod types;
pub mod vector_retrieval;
pub mod world;
pub mod world_model;

pub use abstraction::{ConceptAbstraction, ConceptGroup};
pub use active_evidence::{ActiveEvidenceSeeker, EvidenceRequest, EvidenceRequestKind};
pub use autonomous_hypothesis::AutonomousHypothesisGenerator;
pub use autonomous_cognitive_loop::{AutonomousCognitiveLoop, CognitiveLoopInput, CognitiveLoopResult, LoopDecision};
pub use answer_critic::{AnswerCritic, AnswerCritique};
pub use action::{ActionDecision, ActionProposal, Authority, CuTranPolicy};
pub use analogy::{AnalogicalHypothesis, AnalogicalReasoner};
pub use action_queue::ActionQueue;
pub use adaptation::{causal_credit, evaluate_delta, PromotionDecision, SkillDelta};
pub use budget::{middle_way, Budget, DeviceState};
pub use calibration::{CalibrationEvent, SelfCalibration};
pub use capability::{action_for_goal, encode_action, infer_device_action, DeviceAction, DeviceActionKind};
pub use continuity::{decode_continuity, encode_continuity, ContinuityState};
pub use competition::{CandidateHypothesis, CompetitionResult, HypothesisCompetition};
pub use concept_composition::ConceptComposer;
pub use continual_semantics::{ContinualConcept, ContinualSemanticLearner};
pub use core::{BiaDca, BiaDcaConfig};
pub use dialogue::{DialogueContext, DialogueTurn, Speaker};
pub use deliberation::{DeliberativePlanner, GoalSpec, PlanCandidate};
pub use discovery::{ContextDiscovery, DiscoveredSimilarity};
pub use duyen_token::{DuyenTokenDecoder, GeneratedSequence};
pub use episodic::{Episode, EpisodeClause, EpisodicMemory};
pub use evaluation::{run_v11_evaluation, run_v12_evaluation, run_v14_stress, run_v15_reasoning_evaluation, run_v16_generalization_evaluation, run_v18_open_reasoning_evaluation, run_v21_deep_intelligence_evaluation, run_v25_emergent_intelligence_evaluation, run_v31_autonomous_knowledge_evaluation, run_v37_deliberation_evaluation, run_v45_max_intelligence_evaluation, run_v61_learned_semantic_evaluation, run_v81_continual_generative_evaluation, run_v101_autonomous_loop_evaluation, V11Report, V12Report, V14StressReport, V15ReasoningReport, V16GeneralizationReport, V18OpenReasoningReport, V21DeepIntelligenceReport, V25EmergentIntelligenceReport, V31AutonomousKnowledgeReport, V37DeliberationReport, V45MaxIntelligenceReport, V61LearnedSemanticReport, V81ContinualGenerativeReport, V101AutonomousLoopReport};
pub use four_matrix::{
    adaptive_realm_weights, classify_realm, encode_text_aggregates, AggregateVector, FourMatrixKernel,
    FourMatrixOutput, PerspectiveProjection, RealmBand, AGGREGATES,
};
pub use generative_cognition::{GeneratedThought, GenerativeCognition, ResponseStance};
pub use goals::{Goal, GoalStack, GoalStatus};
pub use hierarchy::{AbstractConcept, HierarchicalAbstraction, RoleSignature};
pub use idle_cognition::{IdleCognitionScheduler, IdleCognitiveTask};
pub use internal_questions::{CognitiveAgenda, InternalQuestion, InternalQuestionKind};
pub use hybrid_semantic::{HybridSemanticReasoner, LatentInference};
pub use curriculum::{score as score_curriculum, CurriculumDomain, CurriculumScore, TrialResult};
pub use induction::{InducedRelation, InductiveReasoner};
pub use inference_matrix::{f32_to_q15, q15_to_f32, MatrixDecision, MatrixLevel, MatrixSignal, TamThienMatrix, LANES};
pub use knowledge::{KnowledgeLedger, KnowledgeRecord, ProvenanceKind};
pub use knowledge_governor::{KnowledgeAssessment, KnowledgeDecision, KnowledgeGovernor};
pub use language::{LanguageIntent, VietnameseGate};
pub use latent_memory::{LatentItem, LatentMemory};
pub use latent_symbol_bridge::{LatentSymbolBridge, SemanticSymbol};
pub use latent_relation::{LatentRelationLearner, RelationPrototype};
pub use meaning::{Concept, MeaningFormation};
pub use metacognition::{CognitiveAssessment, CognitiveDecision, MetacognitiveController};
pub use meta_rules::{MetaRule, MetaRuleCompressor};
pub use perception::{MultiCanh, PerceptPacket};
pub use planner::{decompose_goal, Plan as DevicePlan};
pub use planning::{DeepQuan, Plan, PlanStep};
pub use memory::{Seed, SeedMemory};
pub use mobile::{MobileReply, OfflineMobileBia};
pub use open_intelligence::OpenIntelligence;
pub use open_reasoning::{OpenAnswer, SemanticReasoner};
pub use outcome_learning::{OutcomeLearner, PredictionAudit};
pub use persistence::{decode, encode, read_file, write_atomic, DharmaSnapshot, PersistenceError};
pub use reasoning::{CausalPath, CausalReasoner, CounterfactualVerdict, ReasoningVerdict};
pub use recursive_deliberation::{RecursiveDeliberator, RecursivePass, RecursiveResult};
pub use rules::{RuleSynthesizer, SynthesizedRule};
pub use retrieval::{KnowledgeHit, SemanticRetriever};
pub use semantic::{concept_id, QueryKind, SemanticClause, SemanticEntity, SemanticQuery, SemanticScene, VietnameseSemanticParser};
pub use semantic_compression::{SemanticCentroid, SemanticCompressor};
pub use semantic_consolidation::{ConsolidationReport, SemanticConsolidator};
pub use semantic_embedding::{SemanticEncoder, SemanticVector, SEMANTIC_DIM};
pub use runtime::{CapacityTier, RuntimeProfile, RuntimeTarget};
pub use self_directed_compute::{ComputeRoute, ReasoningTier, SelfDirectedCompute};
pub use skill_transfer::{CrossDomainTransfer, SkillPattern};
pub use token_stream::{InstantToken, InstantTokenEmitter};
pub use types::*;
pub use vector_retrieval::{VectorKnowledgeHit, VectorSemanticRetriever};
pub use world::WorldGraph;
pub use world_model::{SimState, TransitionModel, WorldModel};

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
        let mut bia = BiaDca::new(BiaDcaConfig::default());
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

    #[test]
    fn adaptive_realm_mask_changes_with_event_distribution() {
        let a = [30000, 1000, 1000, 1000, 1000];
        let b = [1000, 1000, 30000, 1000, 1000];
        let wa = adaptive_realm_weights(a, RealmBand::Mixed);
        let wb = adaptive_realm_weights(b, RealmBand::Mixed);
        assert!(wa[0] > wb[0]);
        assert!(wb[2] > wa[2]);
    }

    #[test]
    fn duyen_token_feedback_changes_recurrent_state() {
        let mut decoder = DuyenTokenDecoder::default();
        let before = decoder.state();
        let seq = decoder.generate("Mở YouTube", 8);
        assert!(!seq.tokens.is_empty());
        assert_ne!(decoder.state(), before);
    }

    #[test]
    fn v11_evaluation_meets_acceptance_thresholds() {
        let report = run_v11_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.determinism_passes, report.cases);
    }

    #[test]
    fn learned_vocab_is_bounded_and_emittable() {
        let mut decoder = DuyenTokenDecoder::default();
        for i in 0..200 {
            let _ = decoder.learn_text(&format!("term{i}"));
        }
        assert_eq!(decoder.learned_vocab_len(), 128);
        let seq = decoder.generate("Phân tích kiến trúc mới", 32);
        assert!(seq.tokens.iter().any(|t| t.starts_with("term")));
    }

    #[test]
    fn v12_held_out_and_dynamic_vocab_meet_thresholds() {
        let report = run_v12_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.determinism_passes, report.held_out_cases);
    }

    #[test]
    fn v14_stress_keeps_decoder_bounded_and_deterministic() {
        let report = run_v14_stress(5_000);
        assert!(report.passed(), "report={report:?}");
    }

    #[test]
    fn decoder_survives_max_vocab_and_unicode_noise() {
        let mut decoder = DuyenTokenDecoder::default();
        for i in 0..300 {
            let _ = decoder.learn_text(&format!("khainiem{i}"));
        }
        assert_eq!(decoder.learned_vocab_len(), 128);
        for input in [
            "😀🙏📱⚡",
            "###@@@???",
            "中文 русский العربية tiếng Việt",
            &"a".repeat(10000),
        ] {
            let seq = decoder.generate(input, 48);
            assert!(!seq.tokens.is_empty());
            assert!(seq.tokens.len() <= 48);
        }
    }

    #[test]
    fn multi_hop_reasoning_finds_three_node_chain() {
        let mut world = WorldGraph::new(16, 32);
        world.relate(Relation {
            from: 1,
            to: 2,
            kind: RelationKind::Causes,
            strength: 0.95,
            confidence: 0.95,
        });
        world.relate(Relation {
            from: 2,
            to: 3,
            kind: RelationKind::Enables,
            strength: 0.9,
            confidence: 0.9,
        });
        let verdict = CausalReasoner::default().infer(&world, 3);
        assert!(verdict.support > 0.6);
        assert_eq!(
            verdict.best_path.as_ref().map(|p| p.nodes.as_slice()),
            Some([1, 2, 3].as_slice())
        );
    }

    #[test]
    fn contradiction_reduces_reasoning_confidence() {
        let mut world = WorldGraph::new(16, 32);
        world.relate(Relation {
            from: 1,
            to: 3,
            kind: RelationKind::Causes,
            strength: 0.9,
            confidence: 0.9,
        });
        let reasoner = CausalReasoner::default();
        let before = reasoner.infer(&world, 3);
        world.relate(Relation {
            from: 2,
            to: 3,
            kind: RelationKind::Inhibits,
            strength: 0.9,
            confidence: 0.9,
        });
        let after = reasoner.infer(&world, 3);
        assert!(after.contradicted);
        assert!(after.confidence < before.confidence);
    }

    #[test]
    fn new_evidence_revises_hypothesis_direction() {
        let mut world = WorldGraph::new(16, 32);
        world.relate(Relation {
            from: 1,
            to: 3,
            kind: RelationKind::Causes,
            strength: 0.7,
            confidence: 0.8,
        });
        let reasoner = CausalReasoner::default();
        let before = reasoner.infer(&world, 3);
        let after = reasoner.revise_with_relation(
            &mut world,
            Relation {
                from: 2,
                to: 3,
                kind: RelationKind::Inhibits,
                strength: 1.0,
                confidence: 1.0,
            },
            3,
        );
        assert!(before.support > before.opposition);
        assert!(after.opposition > after.support);
    }

    #[test]
    fn v15_reasoning_quality_suite_passes() {
        let report = run_v15_reasoning_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.accuracy(), 1.0);
    }

    #[test]
    fn counterfactual_removal_weakens_required_chain() {
        let mut world = WorldGraph::new(32, 64);
        for (from, to) in [(1,2),(2,3),(3,4),(4,5)] {
            world.relate(Relation {
                from,
                to,
                kind: RelationKind::Causes,
                strength: 0.95,
                confidence: 0.95,
            });
        }
        let reasoner = CausalReasoner::new(6, 16);
        let cf = reasoner.counterfactual_without(&world, 5, 3);
        assert!(cf.support_delta > 0.2);
        assert!(cf.counterfactual.support < cf.factual.support);
    }

    #[test]
    fn v16_generalization_suite_passes() {
        let report = run_v16_generalization_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.accuracy(), 1.0);
    }

    #[test]
    fn semantic_parser_builds_causal_scene_from_vietnamese() {
        let parser = VietnameseSemanticParser;
        let scene = parser.parse(
            "Pin yếu gây ra giảm xung. Giảm xung dẫn đến suy luận chậm. Pin yếu có gây ra suy luận chậm không?"
        );
        assert_eq!(scene.clauses.len(), 2);
        assert!(scene.query.is_some());
    }

    #[test]
    fn semantic_reasoner_composes_unseen_language_chain() {
        let reasoner = SemanticReasoner::default();
        let mut world = WorldGraph::new(64, 128);
        let scene = reasoner.ingest(
            &mut world,
            "nhietcao gây ra throttling. throttling làm cho latencycao. nhietcao có gây ra latencycao không?",
            1000,
        );
        assert!(matches!(
            reasoner.answer_scene(&world, &scene),
            OpenAnswer::Supported { path, .. } if path.len() == 3
        ));
    }

    #[test]
    fn semantic_counterfactual_removes_middle_condition() {
        let reasoner = SemanticReasoner::default();
        let mut world = WorldGraph::new(64, 128);
        let scene = reasoner.ingest(
            &mut world,
            "a gây ra b. b dẫn đến c. nếu bỏ b thì c?",
            2000,
        );
        assert!(matches!(
            reasoner.answer_scene(&world, &scene),
            OpenAnswer::Counterfactual { support_delta, .. } if support_delta > 0.20
        ));
    }

    #[test]
    fn v18_open_reasoning_suite_passes() {
        let report = run_v18_open_reasoning_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.accuracy(), 1.0);
    }

    #[test]
    fn semantic_retrieval_recovers_relevant_provenance() {
        let mut ledger = KnowledgeLedger::new(8);
        ledger.add(KnowledgeRecord {
            id: 1,
            source: "doc-a".to_string(),
            kind: ProvenanceKind::LocalDocument,
            excerpt: "pin yếu gây ra giảm xung; giảm xung dẫn đến suy luận chậm".to_string(),
            timestamp: 1,
            confidence: 0.9,
        });
        ledger.add(KnowledgeRecord {
            id: 2,
            source: "doc-b".to_string(),
            kind: ProvenanceKind::LocalDocument,
            excerpt: "hoa sen nở vào buổi sáng".to_string(),
            timestamp: 2,
            confidence: 0.9,
        });
        let hits = SemanticRetriever.recall(&ledger, "pin yếu có gây suy luận chậm không", 2);
        assert!(!hits.is_empty());
        assert_eq!(hits[0].record.source, "doc-a");
    }

    #[test]
    fn mobile_reasons_over_ingested_knowledge() {
        let mut app = OfflineMobileBia::new(BiaDca::new(BiaDcaConfig::default()));
        app.ingest_content(
            "doc",
            ProvenanceKind::LocalDocument,
            "pin yếu gây ra giảm xung. giảm xung dẫn đến suy luận chậm.",
            10,
            0.95,
        );
        let reply = app
            .converse(
                "pin yếu có gây ra suy luận chậm không?",
                20,
                device(),
            )
            .expect("reply");
        assert!(reply.text.contains("ủng hộ"));
    }

    #[test]
    fn abstraction_unifies_learned_synonym() {
        let mut abstraction = ConceptAbstraction::default();
        assert_eq!(abstraction.learn_from_text("pin yếu còn gọi là low battery."), 1);
        assert_eq!(
            abstraction.canonical_phrase("low battery"),
            abstraction.canonical_phrase("pin yếu")
        );
    }

    #[test]
    fn analogy_transfers_relation_across_similar_entities() {
        let mut intelligence = OpenIntelligence::default();
        let mut world = WorldGraph::new(64, 128);
        let scene = intelligence.learn(
            &mut world,
            "mưa lớn gây ra đường ướt. mưa nhẹ giống mưa lớn. sân ẩm giống đường ướt. mưa nhẹ có gây ra sân ẩm không?",
            1,
        );
        assert!(matches!(
            intelligence.answer_scene(&world, &scene),
            OpenAnswer::Supported { .. }
        ));
    }

    #[test]
    fn induction_requires_multiple_structural_examples() {
        let mut intelligence = OpenIntelligence::default();
        let mut world = WorldGraph::new(64, 128);
        let _ = intelligence.learn(
            &mut world,
            "a1 gây ra b1. a2 gây ra b2. x giống a1. x giống a2. y giống b1. y giống b2.",
            1,
        );
        let induced = intelligence
            .induce_between(&world, concept_id("x"), concept_id("y"))
            .expect("induced relation");
        assert!(induced.supports >= 2);
        assert_eq!(induced.relation.kind, RelationKind::Causes);
    }

    #[test]
    fn v21_deep_intelligence_suite_passes() {
        let report = run_v21_deep_intelligence_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.accuracy(), 1.0);
    }

    #[test]
    fn episodic_memory_is_bounded() {
        let parser = VietnameseSemanticParser;
        let mut memory = EpisodicMemory::default();
        for i in 0..200u64 {
            let scene = parser.parse(&format!("a{i} gây ra b{i}."));
            memory.observe(&scene, i, 0.5);
        }
        assert!(memory.len() <= 128);
    }

    #[test]
    fn context_discovery_finds_shared_causal_role() {
        let mut world = WorldGraph::new(32, 64);
        for id in 1..=4u64 {
            world.upsert(Phenomenon::new(
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
        for (from, to) in [(1, 3), (2, 3), (1, 4), (2, 4)] {
            world.relate(Relation {
                from,
                to,
                kind: RelationKind::Causes,
                strength: 0.9,
                confidence: 0.9,
            });
        }
        assert!(ContextDiscovery::default().apply(&mut world) > 0);
        assert!(world.edges().iter().any(|e| {
            e.kind == RelationKind::Similar
                && ((e.from == 1 && e.to == 2) || (e.from == 2 && e.to == 1))
        }));
    }

    #[test]
    fn rule_synthesis_transfers_two_step_pattern() {
        let parser = VietnameseSemanticParser;
        let mut memory = EpisodicMemory::default();
        for text in [
            "a1 gây ra b1. b1 cho phép c1.",
            "a2 gây ra b2. b2 cho phép c2.",
        ] {
            let scene = parser.parse(text);
            memory.observe(&scene, 1, 0.8);
        }
        let mut synth = RuleSynthesizer::default();
        assert!(synth.synthesize(&memory) > 0);

        let mut world = WorldGraph::new(16, 32);
        world.relate(Relation {
            from: 10,
            to: 11,
            kind: RelationKind::Causes,
            strength: 0.9,
            confidence: 0.9,
        });
        world.relate(Relation {
            from: 11,
            to: 12,
            kind: RelationKind::Enables,
            strength: 0.9,
            confidence: 0.9,
        });
        assert!(synth.apply(&mut world) > 0);
        assert!(world.edges().iter().any(|e| {
            e.from == 10 && e.to == 12 && e.kind == RelationKind::Causes
        }));
    }

    #[test]
    fn hypothesis_competition_rejects_near_equal_opposites() {
        let engine = HypothesisCompetition;
        let result = engine.choose(&[
            CandidateHypothesis {
                relation: Relation {
                    from: 1,
                    to: 2,
                    kind: RelationKind::Causes,
                    strength: 0.95,
                    confidence: 0.95,
                },
                source: "support",
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
                source: "oppose",
                evidence: 2,
            },
        ]);
        assert!(result.contradicted);
        assert!(result.winner.is_none());
    }

    #[test]
    fn v25_emergent_intelligence_suite_passes() {
        let report = run_v25_emergent_intelligence_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.accuracy(), 1.0);
    }

    #[test]
    fn hierarchy_forms_superconcept_from_shared_roles() {
        let mut world = WorldGraph::new(16, 32);
        for id in 1..=4u64 {
            world.upsert(Phenomenon::new(
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
        for (from, to) in [(1,3),(1,4),(2,3),(2,4)] {
            world.relate(Relation {
                from,
                to,
                kind: RelationKind::Causes,
                strength: 0.9,
                confidence: 0.9,
            });
        }
        let mut hierarchy = HierarchicalAbstraction::default();
        assert!(hierarchy.discover(&world) > 0);
        assert!(hierarchy
            .concept_for(1)
            .is_some_and(|c| c.members.contains(&2)));
    }

    #[test]
    fn autonomous_hypothesis_is_generated_from_unclosed_chain() {
        let mut world = WorldGraph::new(16, 32);
        world.relate(Relation {
            from: 1,
            to: 2,
            kind: RelationKind::Causes,
            strength: 1.0,
            confidence: 1.0,
        });
        world.relate(Relation {
            from: 2,
            to: 3,
            kind: RelationKind::Enables,
            strength: 1.0,
            confidence: 1.0,
        });
        let xs = AutonomousHypothesisGenerator::default().generate(&world);
        assert!(xs.iter().any(|c| {
            c.relation.from == 1
                && c.relation.to == 3
                && c.relation.kind == RelationKind::Causes
        }));
    }

    #[test]
    fn knowledge_governor_rejects_strong_counterevidence() {
        let candidate = CandidateHypothesis {
            relation: Relation {
                from: 1,
                to: 3,
                kind: RelationKind::Causes,
                strength: 0.9,
                confidence: 0.9,
            },
            source: "test",
            evidence: 2,
        };
        let mut world = WorldGraph::new(8, 16);
        world.relate(Relation {
            from: 1,
            to: 3,
            kind: RelationKind::Inhibits,
            strength: 1.0,
            confidence: 1.0,
        });
        let assessment = KnowledgeGovernor::default().assess(&world, &candidate);
        assert_eq!(assessment.decision, KnowledgeDecision::Reject);
    }

    #[test]
    fn meta_rule_compresses_multiple_specific_rules() {
        let parser = VietnameseSemanticParser;
        let mut memory = EpisodicMemory::default();
        for text in [
            "a1 gây ra b1. b1 cho phép c1.",
            "a2 gây ra b2. b2 cho phép c2.",
            "d1 cho phép e1. e1 gây ra f1.",
            "d2 cho phép e2. e2 gây ra f2.",
        ] {
            memory.observe(&parser.parse(text), 1, 0.8);
        }
        let mut synth = RuleSynthesizer::default();
        let _ = synth.synthesize(&memory);
        let mut meta = MetaRuleCompressor::default();
        assert!(meta.compress(synth.rules()) > 0);
        assert!(meta
            .rules()
            .iter()
            .any(|r| r.output == RelationKind::Causes && r.source_rules >= 2));
    }

    #[test]
    fn v31_autonomous_knowledge_suite_passes() {
        let report = run_v31_autonomous_knowledge_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.accuracy(), 1.0);
    }

    #[test]
    fn world_model_predicts_transition_effects() {
        let mut model = WorldModel::default();
        model.add_transition(TransitionModel {
            action: 10,
            requires: vec![1],
            adds: vec![2],
            removes: vec![],
            utility: 0.5,
            cost: 0.1,
            confidence: 0.9,
        });
        let next = model.simulate(&SimState::new([1]), 10).expect("next");
        assert!(next.contains(1));
        assert!(next.contains(2));
        assert!(next.value > 0.0);
    }

    #[test]
    fn deliberation_prefers_safe_goal_reaching_plan() {
        let mut model = WorldModel::default();
        model.add_transition(TransitionModel {
            action: 10,
            requires: vec![1],
            adds: vec![2],
            removes: vec![],
            utility: 0.2,
            cost: 0.05,
            confidence: 1.0,
        });
        model.add_transition(TransitionModel {
            action: 11,
            requires: vec![2],
            adds: vec![3],
            removes: vec![],
            utility: 1.0,
            cost: 0.1,
            confidence: 1.0,
        });
        model.add_transition(TransitionModel {
            action: 12,
            requires: vec![1],
            adds: vec![3, 4],
            removes: vec![],
            utility: 1.2,
            cost: 0.0,
            confidence: 1.0,
        });
        let plan = DeliberativePlanner
            .plan(
                &model,
                &SimState::new([1]),
                &GoalSpec {
                    desired: vec![3],
                    avoid: vec![4],
                },
            )
            .expect("plan");
        assert_eq!(plan.actions, vec![10, 11]);
        assert!(!plan.final_state.contains(4));
    }

    #[test]
    fn outcome_mismatch_triggers_replan() {
        let mut model = WorldModel::default();
        model.add_transition(TransitionModel {
            action: 10,
            requires: vec![1],
            adds: vec![2],
            removes: vec![],
            utility: 0.2,
            cost: 0.0,
            confidence: 1.0,
        });
        model.add_transition(TransitionModel {
            action: 11,
            requires: vec![2],
            adds: vec![3],
            removes: vec![],
            utility: 1.0,
            cost: 0.0,
            confidence: 1.0,
        });
        model.add_transition(TransitionModel {
            action: 12,
            requires: vec![5],
            adds: vec![2],
            removes: vec![5],
            utility: 0.1,
            cost: 0.0,
            confidence: 1.0,
        });
        let predicted = model.simulate(&SimState::new([1]), 10).expect("prediction");
        let observed = SimState::new([5]);
        let goal = GoalSpec {
            desired: vec![3],
            avoid: vec![],
        };
        let learner = OutcomeLearner;
        assert!(learner.audit(&predicted, &observed).replan_required);
        let replanned = learner
            .replan_if_needed(&model, &goal, &predicted, &observed)
            .expect("replan");
        assert_eq!(replanned.actions, vec![12, 11]);
    }

    #[test]
    fn v37_deliberation_suite_passes() {
        let report = run_v37_deliberation_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.accuracy(), 1.0);
    }

    #[test]
    fn metacognition_seeks_evidence_under_conflict() {
        let controller = MetacognitiveController;
        let a = controller.assess(0.82, 0.76, 4, 4);
        assert_eq!(a.decision, CognitiveDecision::SeekEvidence);
        assert!(a.conflict > 0.45);
    }

    #[test]
    fn calibration_reduces_overconfidence_after_errors() {
        let mut cal = SelfCalibration::default();
        for _ in 0..8 {
            cal.observe(0.9, false);
        }
        for _ in 0..8 {
            cal.observe(0.8, true);
        }
        assert!(cal.bias() > 0.0);
        assert!(cal.adjusted(0.82) < 0.82);
    }

    #[test]
    fn recursive_deliberation_stops_when_gain_collapses() {
        let result = RecursiveDeliberator::default().run(0.4, |depth, score| {
            if depth <= 2 { score + 0.12 } else { score + 0.01 }
        });
        assert!(result.stopped_early);
        assert!(result.passes.len() <= 4);
        assert!(result.final_score > 0.6);
    }

    #[test]
    fn self_directed_compute_degrades_under_device_pressure() {
        let controller = MetacognitiveController;
        let assessment = controller.assess(0.58, 0.08, 7, 4);
        let router = SelfDirectedCompute;
        let deep = router.route(
            &assessment,
            DeviceState {
                battery: 0.9,
                thermal: 0.1,
                load: 0.1,
                available_memory_mb: 1024,
            },
        );
        let pressured = router.route(
            &assessment,
            DeviceState {
                battery: 0.08,
                thermal: 0.95,
                load: 0.92,
                available_memory_mb: 96,
            },
        );
        assert_eq!(deep.tier, ReasoningTier::Deep);
        assert_eq!(pressured.tier, ReasoningTier::Instant);
    }

    #[test]
    fn v45_max_intelligence_suite_passes() {
        let report = run_v45_max_intelligence_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.accuracy(), 1.0);
    }

    #[test]
    fn semantic_embedding_prefers_related_surface_form() {
        let encoder = SemanticEncoder;
        let a = encoder.encode("pin yeu gay ra may cham");
        let b = encoder.encode("pin yeu gay nen may cham");
        let c = encoder.encode("hoa sen no buoi sang");
        assert!(a.cosine(&b) > a.cosine(&c));
    }

    #[test]
    fn latent_relation_learns_unseen_causal_paraphrase() {
        let mut learner = LatentRelationLearner::default();
        for text in ["a gay ra b", "c gay ra d", "e dan den f"] {
            learner.observe(text, RelationKind::Causes, 0.96);
        }
        assert!(learner
            .classify("x gay nen y")
            .is_some_and(|(kind, score)| kind == RelationKind::Causes && score >= 0.58));
    }

    #[test]
    fn vector_retrieval_finds_semantically_related_record() {
        let mut ledger = KnowledgeLedger::new(8);
        ledger.add(KnowledgeRecord {
            id: 1,
            source: "thermal".to_string(),
            kind: ProvenanceKind::LocalDocument,
            excerpt: "nhiet cao gay ra throttling va lam may cham".to_string(),
            timestamp: 1,
            confidence: 0.95,
        });
        ledger.add(KnowledgeRecord {
            id: 2,
            source: "lotus".to_string(),
            kind: ProvenanceKind::LocalDocument,
            excerpt: "hoa sen no vao buoi sang".to_string(),
            timestamp: 2,
            confidence: 0.95,
        });
        let hits = VectorSemanticRetriever::default()
            .recall(&ledger, "nhiet cao gay nen may cham", 1);
        assert_eq!(hits.first().map(|h| h.record.id), Some(1));
    }

    #[test]
    fn semantic_compressor_merges_close_phrases() {
        let mut compressor = SemanticCompressor::default();
        let a = compressor.observe("pin yeu gay ra may cham", 0.9);
        let b = compressor.observe("pin yeu gay nen may cham", 0.9);
        assert_eq!(a, b);
        assert_eq!(compressor.len(), 1);
    }

    #[test]
    fn v61_learned_semantic_suite_passes() {
        let report = run_v61_learned_semantic_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.accuracy(), 1.0);
    }

    #[test]
    fn continual_semantics_retains_related_concepts_under_interference() {
        let a = concept_id("pin yeu gay ra may cham");
        let b = concept_id("pin yeu gay nen thiet bi cham");
        let mut learner = ContinualSemanticLearner::default();
        learner.observe(a, "pin yeu gay ra may cham");
        learner.observe(b, "pin yeu gay nen thiet bi cham");
        let before = learner.similarity(a, b).unwrap();
        for i in 0..48u64 {
            learner.observe(10_000 + i, &format!("khai niem nhieu {i} khac biet"));
        }
        let after = learner.similarity(a, b).unwrap();
        assert!(before > 0.65);
        assert!(after > 0.60);
    }

    #[test]
    fn anchored_concept_can_be_consolidated_after_drift() {
        let id = concept_id("pin yeu");
        let mut learner = ContinualSemanticLearner::default();
        learner.observe(id, "pin yeu");
        assert!(learner.anchor(id));
        for i in 0..16 {
            learner.observe(id, &format!("pin yeu bien the rat khac {i}"));
        }
        assert!(learner.restore_anchors(0.08) > 0);
    }

    #[test]
    fn generative_cognition_changes_surface_with_reasoning_state() {
        let g = GenerativeCognition;
        let strong = g.render(
            &OpenAnswer::Supported { confidence: 0.95, path: vec![1,2,3] },
            0.05,
        );
        let weak = g.render(
            &OpenAnswer::Supported { confidence: 0.62, path: vec![1,2,3] },
            0.45,
        );
        assert_eq!(strong.stance, ResponseStance::Certain);
        assert_eq!(weak.stance, ResponseStance::Cautious);
        assert_ne!(strong.text, weak.text);
    }

    #[test]
    fn v81_continual_generative_suite_passes() {
        let report = run_v81_continual_generative_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.accuracy(), 1.0);
    }

    #[test]
    fn cognitive_agenda_forms_counter_questions_under_conflict() {
        let assessment = MetacognitiveController.assess(0.82, 0.76, 4, 3);
        let answer = OpenAnswer::Contradicted {
            support: 0.82,
            opposition: 0.76,
        };
        let mut agenda = CognitiveAgenda::default();
        assert!(agenda.formulate(42, &assessment, &answer) >= 2);
        assert!(agenda.unresolved() >= 2);
    }

    #[test]
    fn answer_critic_revises_conflicted_but_accepts_strong_supported() {
        let strong = OpenAnswer::Supported {
            confidence: 0.95,
            path: vec![1, 2, 3, 4],
        };
        let conflict = OpenAnswer::Contradicted {
            support: 0.82,
            opposition: 0.78,
        };
        let critic = AnswerCritic;
        assert!(!critic.critique(&strong, 0.05, 7).revise);
        assert!(critic.critique(&conflict, 0.4, 2).revise);
    }

    #[test]
    fn autonomous_loop_is_bounded_and_authority_safe() {
        let assessment = MetacognitiveController.assess(0.82, 0.76, 4, 3);
        let answer = OpenAnswer::Contradicted {
            support: 0.82,
            opposition: 0.76,
        };
        let mut loop_controller = AutonomousCognitiveLoop::default();
        let result = loop_controller.run(7, &answer, &assessment, 0.45, 2);
        assert!(result.passes <= 4);
        assert!(result.stopped_bounded);

        let mut idle = IdleCognitionScheduler::default();
        assert_eq!(
            idle.choose(device(), true, 2),
            IdleCognitiveTask::None
        );
    }

    #[test]
    fn v101_autonomous_loop_suite_passes() {
        let report = run_v101_autonomous_loop_evaluation();
        assert!(report.passed(), "report={report:?}");
        assert_eq!(report.accuracy(), 1.0);
    }

}
