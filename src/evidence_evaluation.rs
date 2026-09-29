use crate::budget::DeviceState;
use crate::evidence_search::CausalEvidenceSearch;
use crate::knowledge::{KnowledgeLedger, KnowledgeRecord, ProvenanceKind};
use crate::open_intelligence::OpenIntelligence;
use crate::open_reasoning::{OpenAnswer, SemanticReasoner};
use crate::retrieval::SemanticRetriever;
use crate::world::WorldGraph;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct EvidenceEvaluation {
    pub cases: usize,
    pub chain_passes: usize,
    pub contradiction_passes: usize,
    pub isolation_passes: usize,
    pub top_three_chain_passes: usize,
    pub elapsed: Duration,
}
impl EvidenceEvaluation {
    pub fn passed(&self) -> bool {
        self.chain_passes == self.cases
            && self.contradiction_passes == self.cases
            && self.isolation_passes == self.cases
    }
}

pub fn run_evidence_evaluation() -> EvidenceEvaluation {
    let start = Instant::now();
    let mut report = EvidenceEvaluation {
        cases: 128,
        chain_passes: 0,
        contradiction_passes: 0,
        isolation_passes: 0,
        top_three_chain_passes: 0,
        elapsed: Duration::ZERO,
    };
    let device = DeviceState {
        battery: 0.9,
        thermal: 0.1,
        load: 0.1,
        available_memory_mb: 1024,
    };
    for case in 0..report.cases {
        let depth = 2 + case % 5;
        let name = |i: usize| format!("nut{case}x{i}");
        let query = format!("{} có gây ra {} không?", name(0), name(depth));
        let intelligence = OpenIntelligence::default();
        let scene = intelligence.parse(&query);
        let world = WorldGraph::new(256, 512);
        let mut ledger = KnowledgeLedger::new(96);
        // Reversed order prevents document order from acting as an answer key.
        for step in (0..depth).rev() {
            ledger.add(KnowledgeRecord {
                id: step as u64 + 1,
                source: format!("doc-{step}"),
                kind: ProvenanceKind::LocalDocument,
                excerpt: format!("{} gây ra {}.", name(step), name(step + 1)),
                timestamp: step as u64,
                confidence: 0.75 + (case % 25) as f32 / 100.0,
            });
        }
        for noise in 0..24 {
            ledger.add(KnowledgeRecord {
                id: 100 + noise,
                source: "noise".into(),
                kind: ProvenanceKind::LocalDocument,
                excerpt: format!("nhieu{case}x{noise} gây ra {}.", name(depth)),
                timestamp: 0,
                confidence: 0.99,
            });
        }
        // Isolate retrieval policy: same parser/reasoner, no learning or mutation
        // of the shared fixture. This is a top-three baseline, not full V129.
        let mut baseline = world.clone();
        for hit in SemanticRetriever.recall(&ledger, &query, 3) {
            SemanticReasoner::default().ingest(&mut baseline, &hit.record.excerpt, 0);
        }
        if matches!(
            intelligence.answer_scene(&baseline, &scene),
            OpenAnswer::Supported { .. }
        ) {
            report.top_three_chain_passes += 1;
        }
        let result = CausalEvidenceSearch.search(&intelligence, &world, &ledger, &scene, device);
        if matches!(&result.answer, OpenAnswer::Supported { path, .. } if path.len() == depth + 1)
            && result.imported_clauses == depth
            && result.passes <= 6
        {
            report.chain_passes += 1;
        }
        ledger.add(KnowledgeRecord {
            id: 999,
            source: "counterevidence".into(),
            kind: ProvenanceKind::LocalDocument,
            excerpt: format!("{} ngăn {}.", name(0), name(depth)),
            timestamp: 9,
            confidence: 0.95,
        });
        let conflict = CausalEvidenceSearch.search(&intelligence, &world, &ledger, &scene, device);
        if matches!(conflict.answer, OpenAnswer::Contradicted { .. }) {
            report.contradiction_passes += 1;
        }
        if world.is_empty() && world.edges().is_empty() && ledger.len() == depth + 25 {
            report.isolation_passes += 1;
        }
    }
    report.elapsed = start.elapsed();
    report
}
