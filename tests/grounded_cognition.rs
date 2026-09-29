use bia_core::answer_critic::AnswerCritic;
use bia_core::autonomous_cognitive_loop::{AutonomousCognitiveLoop, LoopDecision};
use bia_core::metacognition::MetacognitiveController;
use bia_core::open_reasoning::{OpenAnswer, SemanticReasoner};
use bia_core::reasoning::CausalReasoner;
use bia_core::semantic::{QueryKind, SemanticEntity, SemanticQuery};
use bia_core::types::{Relation, RelationKind};
use bia_core::world::WorldGraph;
use bia_core::{BiaDca, BiaDcaConfig, DeviceState, OfflineMobileBia};

fn edge(world: &mut WorldGraph, from: u64, to: u64, kind: RelationKind, score: f32) {
    world.relate(Relation {
        from,
        to,
        kind,
        strength: score,
        confidence: 1.0,
    });
}
fn answer(world: &WorldGraph, from: u64, to: u64) -> OpenAnswer {
    SemanticReasoner::default().answer_query(
        world,
        &SemanticQuery {
            kind: QueryKind::Causal,
            subject: SemanticEntity {
                id: from,
                text: from.to_string(),
            },
            object: SemanticEntity {
                id: to,
                text: to.to_string(),
            },
        },
    )
}

#[test]
fn repeated_review_cannot_invent_evidence_or_resolve_conflict() {
    let candidate = OpenAnswer::Contradicted {
        support: 0.82,
        opposition: 0.76,
    };
    let assessment = MetacognitiveController.assess(0.82, 0.76, 4, 3);
    let expected = AnswerCritic.critique(&candidate, 0.45, 3);
    let mut reviewer = AutonomousCognitiveLoop::default();
    for _ in 0..100 {
        let result = reviewer.run(1, &candidate, &assessment, 0.45, 3);
        assert_eq!(result.critique, expected);
        assert_eq!(result.resolved_questions, 0);
        assert_eq!(result.decision, LoopDecision::GatherEvidence);
        assert!(result.final_confidence <= assessment.certainty);
        assert_eq!(result.passes, 1);
        assert!(reviewer.unresolved_questions() > 0);
    }
}

#[test]
fn unknown_stays_unknown_even_with_misleading_high_assessment() {
    let assessment = MetacognitiveController.assess(1.0, 0.0, 2, 8);
    let r = AutonomousCognitiveLoop::default().run(1, &OpenAnswer::Unknown, &assessment, 0.0, 8);
    assert_eq!(r.decision, LoopDecision::GatherEvidence);
    assert_eq!(r.final_confidence, 0.0);
    assert_eq!(r.resolved_questions, 0);
}

#[test]
fn stronger_real_evidence_allows_a_new_answer_review() {
    let mut reviewer = AutonomousCognitiveLoop::default();
    let weak = MetacognitiveController.assess(0.0, 0.0, 0, 0);
    assert_eq!(
        reviewer
            .run(1, &OpenAnswer::Unknown, &weak, 1.0, 0)
            .decision,
        LoopDecision::GatherEvidence
    );
    let known = OpenAnswer::Supported {
        confidence: 0.94,
        path: vec![1, 2],
    };
    let strong = MetacognitiveController.assess(0.94, 0.0, 2, 8);
    let r = reviewer.run(1, &known, &strong, 0.04, 8);
    assert_eq!(r.decision, LoopDecision::Answer);
    assert!(r.final_confidence <= 0.94);
    assert_eq!(r.internal_questions, 0);
    assert_eq!(reviewer.unresolved_questions(), 0);
}

#[test]
fn old_targets_do_not_starve_a_new_query_agenda() {
    let weak = MetacognitiveController.assess(0.0, 0.0, 0, 0);
    let mut reviewer = AutonomousCognitiveLoop::default();
    for target in 0..32 {
        let r = reviewer.run(target, &OpenAnswer::Unknown, &weak, 1.0, 0);
        assert_eq!(r.internal_questions, 2);
        assert_eq!(r.resolved_questions, 0);
    }
}

#[test]
fn uncertainty_and_missing_evidence_prevent_confident_answers() {
    let known = OpenAnswer::Supported {
        confidence: 0.99,
        path: vec![1, 2],
    };
    let assessment = MetacognitiveController.assess(0.99, 0.0, 2, 8);
    let mut reviewer = AutonomousCognitiveLoop::default();
    for uncertainty in [0.9, f32::NAN, f32::INFINITY] {
        let r = reviewer.run(1, &known, &assessment, uncertainty, 8);
        assert_ne!(r.decision, LoopDecision::Answer);
        assert!(r.final_confidence.is_finite() && r.final_confidence <= 0.11);
    }
    assert_eq!(
        reviewer.run(1, &known, &assessment, 0.0, 0).decision,
        LoopDecision::GatherEvidence
    );
}

#[test]
fn source_specific_claim_ignores_stronger_unrelated_causes_and_conflicts() {
    // Vary identities, depth and strength rather than rerunning one fixture.
    for case in 0..128_u64 {
        let root = case * 100 + 1;
        let depth = 1 + case % 6;
        let score = 0.25 + (case % 60) as f32 / 100.0;
        let target = root + depth;
        let mut world = WorldGraph::new(100, 256);
        for step in 0..depth {
            edge(
                &mut world,
                root + step,
                root + step + 1,
                RelationKind::Causes,
                score,
            );
        }
        // More unrelated parents than the old backwards beam could retain.
        for noise in 20..60 {
            edge(
                &mut world,
                root + noise,
                target,
                RelationKind::Inhibits,
                0.99,
            );
        }
        match answer(&world, root, target) {
            OpenAnswer::Supported { confidence, path } => {
                assert!((confidence - score).abs() < 0.0001);
                assert_eq!(path.first(), Some(&root));
                assert_eq!(path.last(), Some(&target));
                assert_eq!(path.len(), depth as usize + 1);
            }
            other => panic!("case {case}: {other:?}"),
        }
        assert_eq!(answer(&world, root + 90, target), OpenAnswer::Unknown);
    }
}

#[test]
fn query_can_start_at_an_internal_node() {
    let mut world = WorldGraph::new(16, 16);
    edge(&mut world, 1, 2, RelationKind::Inhibits, 0.99);
    edge(&mut world, 2, 3, RelationKind::Causes, 0.8);
    assert!(matches!(answer(&world, 2, 3), OpenAnswer::Supported { .. }));
    assert!(matches!(answer(&world, 1, 3), OpenAnswer::Opposed { .. }));
}

#[test]
fn shared_paths_do_not_amplify_confidence() {
    let mut world = WorldGraph::new(16, 32);
    edge(&mut world, 1, 2, RelationKind::Causes, 0.6);
    for via in 3..10 {
        edge(&mut world, 2, via, RelationKind::Enables, 0.9);
        edge(&mut world, via, 10, RelationKind::Causes, 0.9);
    }
    match answer(&world, 1, 10) {
        OpenAnswer::Supported { confidence, .. } => assert!((confidence - 0.6).abs() < 0.0001),
        other => panic!("{other:?}"),
    }
}

#[test]
fn new_opposing_evidence_changes_the_actual_verdict() {
    let mut world = WorldGraph::new(16, 16);
    edge(&mut world, 1, 2, RelationKind::Causes, 0.8);
    assert!(matches!(answer(&world, 1, 2), OpenAnswer::Supported { .. }));
    edge(&mut world, 1, 2, RelationKind::Inhibits, 0.9);
    assert!(matches!(
        answer(&world, 1, 2),
        OpenAnswer::Contradicted { .. }
    ));
}

#[test]
fn cycles_and_depth_limits_do_not_create_a_self_cause() {
    let mut world = WorldGraph::new(16, 16);
    edge(&mut world, 1, 2, RelationKind::Causes, 0.8);
    edge(&mut world, 2, 1, RelationKind::Causes, 0.8);
    edge(&mut world, 2, 3, RelationKind::Causes, 0.8);
    assert_eq!(answer(&world, 1, 1), OpenAnswer::Unknown);
    assert!(CausalReasoner::new(1, 1)
        .infer_between(&world, 1, 3)
        .best_path
        .is_none());
    assert!(matches!(answer(&world, 1, 3), OpenAnswer::Supported { .. }));
}

#[test]
fn mobile_unknown_query_explicitly_requests_evidence() {
    let mut app = OfflineMobileBia::new(BiaDca::new(BiaDcaConfig::default()));
    let device = DeviceState {
        battery: 0.9,
        thermal: 0.1,
        load: 0.1,
        available_memory_mb: 1024,
    };
    let reply = app.converse("alpha có gây ra beta không?", 1, device).unwrap();
    assert!(reply.text.contains("bằng chứng"), "{}", reply.text);
    assert!(reply.pending_action.is_none());
    assert_eq!(app.queue_len(), 0);
}
