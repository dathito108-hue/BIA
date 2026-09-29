#![forbid(unsafe_code)]

pub mod action;
pub mod adaptation;
pub mod budget;
pub mod core;
pub mod curriculum;
pub mod language;
pub mod meaning;
pub mod perception;
pub mod planning;
pub mod memory;
pub mod mobile;
pub mod persistence;
pub mod runtime;
pub mod types;
pub mod world;

pub use action::{ActionDecision, ActionProposal, Authority, CuTranPolicy};
pub use adaptation::{causal_credit, evaluate_delta, PromotionDecision, SkillDelta};
pub use budget::{middle_way, Budget, DeviceState};
pub use core::{BiaDca, BiaDcaConfig};
pub use curriculum::{score as score_curriculum, CurriculumDomain, CurriculumScore, TrialResult};
pub use language::{LanguageIntent, VietnameseGate};
pub use meaning::{Concept, MeaningFormation};
pub use perception::{MultiCanh, PerceptPacket};
pub use planning::{DeepQuan, Plan, PlanStep};
pub use memory::{Seed, SeedMemory};
pub use mobile::{MobileReply, OfflineMobileBia};
pub use persistence::{decode, encode, read_file, write_atomic, DharmaSnapshot, PersistenceError};
pub use runtime::{CapacityTier, RuntimeProfile, RuntimeTarget};
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

}
