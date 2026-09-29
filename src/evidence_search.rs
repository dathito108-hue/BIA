//! Bounded, read-only causal evidence retrieval across local provenance records.
use std::collections::{BTreeMap, BTreeSet};

use crate::budget::{middle_way, DeviceState};
use crate::knowledge::KnowledgeLedger;
use crate::open_intelligence::OpenIntelligence;
use crate::open_reasoning::OpenAnswer;
use crate::semantic::{QueryKind, SemanticClause, SemanticScene, VietnameseSemanticParser};
use crate::types::RelationKind;
use crate::world::WorldGraph;

const MAX_RECORDS: usize = 96;
const MAX_CHARS: usize = 2048;
const MAX_CLAUSES_PER_RECORD: usize = 12;
const MAX_CANDIDATES: usize = 512;
const MAX_IMPORTED: usize = 128;
const MAX_NODES: usize = 256;
const MAX_PASSES: usize = 6;

#[derive(Clone, Debug, PartialEq)]
pub struct EvidenceSearchResult {
    pub answer: OpenAnswer,
    pub passes: usize,
    pub records_scanned: usize,
    /// Records actually imported, not a claim of independent corroboration.
    pub imported_record_ids: Vec<u64>,
    pub imported_clauses: usize,
    pub budget_limited: bool,
}

#[derive(Clone, Debug, Default)]
pub struct CausalEvidenceSearch;

struct Candidate {
    record_id: u64,
    timestamp: u64,
    clause: SemanticClause,
}

impl CausalEvidenceSearch {
    pub fn search(
        &self,
        intelligence: &OpenIntelligence,
        world: &WorldGraph,
        ledger: &KnowledgeLedger,
        scene: &SemanticScene,
        device: DeviceState,
    ) -> EvidenceSearchResult {
        let mut result = EvidenceSearchResult {
            answer: intelligence.answer_scene(world, scene),
            passes: 0,
            records_scanned: 0,
            imported_record_ids: Vec::new(),
            imported_clauses: 0,
            budget_limited: false,
        };
        let Some(query) = &scene.query else {
            return result;
        };
        if query.kind != QueryKind::Causal {
            return result;
        }
        let passes = middle_way(device, 0.9, 0.9)
            .contemplation_cycles
            .min(MAX_PASSES);
        if passes == 0 {
            result.budget_limited = !ledger.is_empty();
            return result;
        }

        // Parse each bounded excerpt once, then follow concept IDs rather than
        // repeatedly ranking unrelated documents by shared question words.
        let mut candidates: BTreeMap<u64, Vec<Candidate>> = BTreeMap::new();
        let mut count = 0;
        result.budget_limited = ledger.len() > MAX_RECORDS;
        for record in ledger.records().take(MAX_RECORDS) {
            result.records_scanned += 1;
            if !record.confidence.is_finite() || record.confidence < 0.5 {
                continue;
            }
            let mut chars = record.excerpt.chars();
            let excerpt: String = chars.by_ref().take(MAX_CHARS).collect();
            if chars.next().is_some() {
                // Do not turn a cut-off sentence into an asserted causal fact.
                result.budget_limited = true;
                continue;
            }
            let parsed = intelligence.parse(&excerpt);
            result.budget_limited |= parsed.clauses.len() > MAX_CLAUSES_PER_RECORD;
            for mut clause in parsed.clauses.into_iter().take(MAX_CLAUSES_PER_RECORD) {
                if !causal(clause.kind) {
                    continue;
                }
                if count >= MAX_CANDIDATES {
                    result.budget_limited = true;
                    break;
                }
                // Parser strength is discounted by the recorded source quality.
                // No source is promoted merely because it was retrieved.
                clause.confidence *= record.confidence.clamp(0.0, 1.0).sqrt();
                candidates
                    .entry(clause.subject.id)
                    .or_default()
                    .push(Candidate {
                        record_id: record.id,
                        timestamp: record.timestamp,
                        clause,
                    });
                count += 1;
            }
        }
        if candidates.is_empty() {
            return result;
        }
        // Extra scratch capacity prevents retrieval from evicting durable facts.
        let mut scratch = WorldGraph::from_parts(
            world.max_nodes().saturating_add(MAX_IMPORTED * 2),
            world.max_edges().saturating_add(MAX_IMPORTED),
            world.nodes().to_vec(),
            world.edges().to_vec(),
        );
        let mut frontier = BTreeSet::from([query.subject.id]);
        let mut visited = frontier.clone();
        let mut imported_ids = BTreeSet::new();
        for _ in 0..passes {
            result.passes += 1;
            for id in &frontier {
                for candidate in candidates.remove(id).unwrap_or_default() {
                    let clause = candidate.clause;
                    if scratch.edges().iter().any(|edge| {
                        edge.from == clause.subject.id
                            && edge.to == clause.object.id
                            && edge.kind == clause.kind
                    }) {
                        continue; // Re-reading a known edge must not reinforce it.
                    }
                    if result.imported_clauses >= MAX_IMPORTED {
                        result.budget_limited = true;
                        continue;
                    }
                    VietnameseSemanticParser.ingest(
                        &mut scratch,
                        &SemanticScene {
                            clauses: vec![clause],
                            query: None,
                        },
                        candidate.timestamp,
                    );
                    result.imported_clauses += 1;
                    imported_ids.insert(candidate.record_id);
                }
            }
            let mut next = BTreeSet::new();
            for edge in scratch
                .edges()
                .iter()
                .filter(|e| causal(e.kind) && frontier.contains(&e.from))
            {
                if visited.contains(&edge.to) {
                    continue;
                }
                if visited.len() >= MAX_NODES {
                    result.budget_limited = true;
                    continue;
                }
                visited.insert(edge.to);
                next.insert(edge.to);
            }
            frontier = next;
            if frontier.is_empty() {
                break;
            }
        }
        result.budget_limited |=
            !frontier.is_empty() && frontier.iter().any(|id| candidates.contains_key(id));
        result.imported_record_ids = imported_ids.into_iter().collect();
        result.answer = intelligence.answer_scene(&scratch, scene);
        result
    }
}

fn causal(kind: RelationKind) -> bool {
    matches!(
        kind,
        RelationKind::Causes
            | RelationKind::Enables
            | RelationKind::Inhibits
            | RelationKind::Follows
    )
}
