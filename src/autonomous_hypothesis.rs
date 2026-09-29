use crate::competition::CandidateHypothesis;
use crate::types::{Relation, RelationKind};
use crate::world::WorldGraph;

#[derive(Clone, Debug)]
pub struct AutonomousHypothesisGenerator {
    max_candidates: usize,
    min_confidence: f32,
}

impl Default for AutonomousHypothesisGenerator {
    fn default() -> Self {
        Self {
            max_candidates: 24,
            min_confidence: 0.48,
        }
    }
}

impl AutonomousHypothesisGenerator {
    pub fn generate(&self, world: &WorldGraph) -> Vec<CandidateHypothesis> {
        let edges = world.edges();
        let mut out = Vec::new();

        for a in edges.iter().filter(|e| transferable(e.kind)) {
            for b in edges
                .iter()
                .filter(|e| transferable(e.kind) && e.from == a.to)
            {
                if a.from == b.to {
                    continue;
                }
                let kind = compose(a.kind, b.kind);
                if edges
                    .iter()
                    .any(|e| e.from == a.from && e.to == b.to && e.kind == kind)
                {
                    continue;
                }
                let confidence = (
                    a.strength * a.confidence * b.strength * b.confidence * 0.82
                )
                    .sqrt()
                    .clamp(0.0, 0.92);
                if confidence < self.min_confidence {
                    continue;
                }
                out.push(CandidateHypothesis {
                    relation: Relation {
                        from: a.from,
                        to: b.to,
                        kind,
                        strength: confidence,
                        confidence,
                    },
                    source: "autonomous-two-hop",
                    evidence: 2,
                });
                if out.len() >= self.max_candidates {
                    return out;
                }
            }
        }
        out
    }
}

fn compose(a: RelationKind, b: RelationKind) -> RelationKind {
    match (a, b) {
        (RelationKind::Inhibits, RelationKind::Inhibits) => RelationKind::Enables,
        (RelationKind::Inhibits, _) | (_, RelationKind::Inhibits) => RelationKind::Inhibits,
        (RelationKind::Causes, _) | (_, RelationKind::Causes) => RelationKind::Causes,
        _ => RelationKind::Enables,
    }
}

fn transferable(kind: RelationKind) -> bool {
    matches!(
        kind,
        RelationKind::Causes | RelationKind::Enables | RelationKind::Inhibits
    )
}
