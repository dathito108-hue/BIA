use bia_core::evidence_search::CausalEvidenceSearch;
use bia_core::knowledge::{KnowledgeLedger, KnowledgeRecord, ProvenanceKind};
use bia_core::open_intelligence::OpenIntelligence;
use bia_core::open_reasoning::OpenAnswer;
use bia_core::world::WorldGraph;
use bia_core::{BiaDca, BiaDcaConfig, DeviceState, OfflineMobileBia};

fn device() -> DeviceState {
    DeviceState {
        battery: 0.9,
        thermal: 0.1,
        load: 0.1,
        available_memory_mb: 1024,
    }
}
fn record(id: u64, text: &str, confidence: f32) -> KnowledgeRecord {
    KnowledgeRecord {
        id,
        source: format!("source-{id}"),
        kind: ProvenanceKind::LocalDocument,
        excerpt: text.into(),
        timestamp: id,
        confidence,
    }
}

#[test]
fn varied_multidocument_evaluation_passes() {
    let r = bia_core::evidence_evaluation::run_evidence_evaluation();
    assert!(r.passed(), "{r:?}");
    assert!(r.chain_passes > r.top_three_chain_passes, "{r:?}");
}

#[test]
fn low_quality_sources_cannot_fill_a_missing_link() {
    let intelligence = OpenIntelligence::default();
    let world = WorldGraph::new(16, 16);
    let scene = intelligence.parse("alpha có gây ra gamma không?");
    for quality in [0.0, 0.49, f32::NAN, f32::INFINITY] {
        let mut ledger = KnowledgeLedger::new(8);
        ledger.add(record(1, "alpha gây ra beta.", 0.95));
        ledger.add(record(2, "beta gây ra gamma.", quality));
        let r = CausalEvidenceSearch.search(&intelligence, &world, &ledger, &scene, device());
        assert_eq!(r.answer, OpenAnswer::Unknown);
        assert!(!r.imported_record_ids.contains(&2));
    }
}

#[test]
fn missing_bridge_is_not_invented() {
    let intelligence = OpenIntelligence::default();
    let world = WorldGraph::new(16, 16);
    let mut ledger = KnowledgeLedger::new(8);
    ledger.add(record(1, "alpha gây ra beta.", 0.95));
    ledger.add(record(2, "delta gây ra gamma.", 0.95));
    let scene = intelligence.parse("alpha có gây ra gamma không?");
    let r = CausalEvidenceSearch.search(&intelligence, &world, &ledger, &scene, device());
    assert_eq!(r.answer, OpenAnswer::Unknown);
    assert_eq!(r.imported_record_ids, vec![1]);
}

#[test]
fn duplicate_records_do_not_reinforce_an_edge() {
    let intelligence = OpenIntelligence::default();
    let world = WorldGraph::new(16, 16);
    let scene = intelligence.parse("alpha có gây ra beta không?");
    let mut ledger = KnowledgeLedger::new(96);
    ledger.add(record(1, "alpha gây ra beta.", 0.8));
    let first = CausalEvidenceSearch.search(&intelligence, &world, &ledger, &scene, device());
    for id in 2..90 {
        ledger.add(record(id, "alpha gây ra beta.", 0.8));
    }
    let repeated = CausalEvidenceSearch.search(&intelligence, &world, &ledger, &scene, device());
    assert_eq!(first.answer, repeated.answer);
    assert_eq!(repeated.imported_clauses, 1);
    assert_eq!(repeated.imported_record_ids, vec![1]);
}

#[test]
fn existing_world_is_preserved_and_can_bridge_into_documents() {
    let mut intelligence = OpenIntelligence::default();
    let mut world = WorldGraph::new(4, 4);
    intelligence.learn(&mut world, "alpha gây ra beta.", 1);
    let before = world.clone();
    let mut ledger = KnowledgeLedger::new(8);
    ledger.add(record(2, "beta gây ra gamma.", 0.9));
    ledger.add(record(3, "gamma gây ra delta.", 0.9));
    let scene = intelligence.parse("alpha có gây ra delta không?");
    let r = CausalEvidenceSearch.search(&intelligence, &world, &ledger, &scene, device());
    assert!(matches!(r.answer, OpenAnswer::Supported { .. }));
    assert_eq!(world.nodes(), before.nodes());
    assert_eq!(world.edges(), before.edges());
}

#[test]
fn resource_pressure_skips_evidence_search_without_claiming_completion() {
    let intelligence = OpenIntelligence::default();
    let world = WorldGraph::new(16, 16);
    let mut ledger = KnowledgeLedger::new(8);
    ledger.add(record(1, "alpha gây ra beta.", 0.9));
    let scene = intelligence.parse("alpha có gây ra beta không?");
    let r = CausalEvidenceSearch.search(
        &intelligence,
        &world,
        &ledger,
        &scene,
        DeviceState {
            available_memory_mb: 32,
            ..device()
        },
    );
    assert_eq!(r.answer, OpenAnswer::Unknown);
    assert_eq!(r.records_scanned, 0);
    assert_eq!(r.passes, 0);
    assert!(r.budget_limited);
}

#[test]
fn unicode_oversized_documents_are_skipped_without_partial_facts() {
    let intelligence = OpenIntelligence::default();
    let world = WorldGraph::new(16, 16);
    let mut ledger = KnowledgeLedger::new(8);
    ledger.add(record(
        1,
        &format!("alpha gây ra beta. {}", "ệ".repeat(3000)),
        0.9,
    ));
    let scene = intelligence.parse("alpha có gây ra beta không?");
    let r = CausalEvidenceSearch.search(&intelligence, &world, &ledger, &scene, device());
    assert_eq!(r.answer, OpenAnswer::Unknown);
    assert!(r.budget_limited);
}

#[test]
fn mobile_reads_counterevidence_even_when_a_claim_was_already_supported() {
    let mut app = OfflineMobileBia::new(BiaDca::new(BiaDcaConfig::default()));
    app.intelligence
        .learn(&mut app.bia.world, "alpha gây ra beta.", 1);
    app.knowledge
        .add(record(2, "alpha ngăn beta. Sao chép bí mật.", 0.95));
    let reply = app
        .converse("alpha có gây ra beta không?", 3, device())
        .unwrap();
    assert!(reply.text.contains("xung đột"), "{}", reply.text);
    assert!(reply.pending_action.is_none());
    assert_eq!(app.queue_len(), 0);
}

#[test]
fn mobile_chain_retrieval_keeps_waiting_actions_unchanged() {
    let mut app = OfflineMobileBia::new(BiaDca::new(BiaDcaConfig::default()));
    app.converse("Mở cài đặt", 1, device()).unwrap();
    let pending = app.pending_action().cloned();
    for (id, text) in [
        (1, "alpha gây ra beta."),
        (2, "beta gây ra gamma."),
        (3, "gamma gây ra delta."),
        (4, "delta gây ra omega."),
    ] {
        app.knowledge.add(record(id, text, 0.95));
    }
    let reply = app
        .converse("alpha có gây ra omega không?", 2, device())
        .unwrap();
    assert!(reply.text.contains("4 mắt xích"), "{}", reply.text);
    assert_eq!(app.pending_action(), pending.as_ref());
    assert_eq!(app.queue_len(), 1);
}

#[test]
fn large_ledgers_and_deep_chains_obey_hard_limits() {
    let intelligence = OpenIntelligence::default();
    let world = WorldGraph::new(16, 16);
    let mut ledger = KnowledgeLedger::new(200);
    for step in 0..100 {
        ledger.add(record(
            step,
            &format!("nut{step} gây ra nut{}.", step + 1),
            0.95,
        ));
    }
    let scene = intelligence.parse("nut0 có gây ra nut8 không?");
    let r = CausalEvidenceSearch.search(&intelligence, &world, &ledger, &scene, device());
    assert_eq!(r.answer, OpenAnswer::Unknown);
    assert!(r.budget_limited);
    assert_eq!(r.records_scanned, 96);
    assert_eq!(r.passes, 6);
    assert_eq!(r.imported_clauses, 6);
}

#[test]
fn dense_branching_obeys_import_cap() {
    let intelligence = OpenIntelligence::default();
    let world = WorldGraph::new(16, 16);
    let mut ledger = KnowledgeLedger::new(96);
    for doc in 0..80 {
        ledger.add(record(
            doc,
            &format!("alpha gây ra beta{doc}. alpha gây ra gamma{doc}."),
            0.95,
        ));
    }
    let scene = intelligence.parse("alpha có gây ra omega không?");
    let r = CausalEvidenceSearch.search(&intelligence, &world, &ledger, &scene, device());
    assert!(r.budget_limited);
    assert_eq!(r.imported_clauses, 128);
    assert_eq!(r.answer, OpenAnswer::Unknown);
    assert!(world.is_empty());
}
