use crate::competition::CandidateHypothesis;
use crate::types::RelationKind;
use crate::world::WorldGraph;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KnowledgeDecision {
    Promote,
    Hold,
    Reject,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KnowledgeAssessment {
    pub decision: KnowledgeDecision,
    pub support: f32,
    pub opposition: f32,
    pub confidence: f32,
}

#[derive(Clone, Debug)]
pub struct KnowledgeGovernor {
    promote_threshold: f32,
    reject_threshold: f32,
}

impl Default for KnowledgeGovernor {
    fn default() -> Self {
        Self {
            promote_threshold: 0.66,
            reject_threshold: 0.72,
        }
    }
}

impl KnowledgeGovernor {
    pub fn assess(
        &self,
        world: &WorldGraph,
        candidate: &CandidateHypothesis,
    ) -> KnowledgeAssessment {
        let base = candidate.relation.strength * candidate.relation.confidence;
        let mut support = base;
        let mut opposition = 0.0f32;

        for e in world.edges().iter().filter(|e| {
            e.from == candidate.relation.from && e.to == candidate.relation.to
        }) {
            let score = e.strength * e.confidence;
            if compatible(e.kind, candidate.relation.kind) {
                support = noisy_or(support, score);
            } else if opposite(e.kind, candidate.relation.kind) {
                opposition = noisy_or(opposition, score);
            }
        }

        let confidence = (support * (1.0 - opposition)).clamp(0.0, 1.0);
        let decision = if opposition >= self.reject_threshold && opposition > support {
            KnowledgeDecision::Reject
        } else if support >= self.promote_threshold && opposition < 0.45 {
            KnowledgeDecision::Promote
        } else {
            KnowledgeDecision::Hold
        };

        KnowledgeAssessment {
            decision,
            support,
            opposition,
            confidence,
        }
    }

    pub fn promote_if_valid(
        &self,
        world: &mut WorldGraph,
        candidate: &CandidateHypothesis,
    ) -> KnowledgeAssessment {
        let assessment = self.assess(world, candidate);
        if assessment.decision == KnowledgeDecision::Promote {
            world.relate(candidate.relation.clone());
        }
        assessment
    }
}

fn compatible(a: RelationKind, b: RelationKind) -> bool {
    a == b
        || matches!(
            (a, b),
            (RelationKind::Causes, RelationKind::Enables)
                | (RelationKind::Enables, RelationKind::Causes)
        )
}

fn opposite(a: RelationKind, b: RelationKind) -> bool {
    matches!(
        (a, b),
        (RelationKind::Inhibits, RelationKind::Causes)
            | (RelationKind::Inhibits, RelationKind::Enables)
            | (RelationKind::Causes, RelationKind::Inhibits)
            | (RelationKind::Enables, RelationKind::Inhibits)
    )
}

fn noisy_or(current: f32, evidence: f32) -> f32 {
    (1.0 - (1.0 - current) * (1.0 - evidence)).clamp(0.0, 1.0)
}
