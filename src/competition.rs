use crate::types::{Relation, RelationKind};

#[derive(Clone, Debug, PartialEq)]
pub struct CandidateHypothesis {
    pub relation: Relation,
    pub source: &'static str,
    pub evidence: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CompetitionResult {
    pub winner: Option<CandidateHypothesis>,
    pub contradicted: bool,
    pub margin: f32,
}

#[derive(Clone, Debug, Default)]
pub struct HypothesisCompetition;

impl HypothesisCompetition {
    pub fn choose(&self, candidates: &[CandidateHypothesis]) -> CompetitionResult {
        if candidates.is_empty() {
            return CompetitionResult {
                winner: None,
                contradicted: false,
                margin: 0.0,
            };
        }

        let mut ranked = candidates.to_vec();
        ranked.sort_by(|a, b| {
            score(b)
                .partial_cmp(&score(a))
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let winner = ranked[0].clone();
        let winner_score = score(&winner);
        let second_score = ranked.get(1).map(score).unwrap_or(0.0);
        let contradicted = ranked.iter().skip(1).any(|c| {
            opposite(winner.relation.kind, c.relation.kind)
                && score(c) >= winner_score * 0.75
        });

        CompetitionResult {
            winner: (!contradicted).then_some(winner),
            contradicted,
            margin: (winner_score - second_score).max(0.0),
        }
    }
}

fn score(c: &CandidateHypothesis) -> f32 {
    let evidence_gain = 1.0 + (c.evidence.min(8) as f32).ln_1p() * 0.15;
    (c.relation.strength * c.relation.confidence * evidence_gain).clamp(0.0, 1.25)
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
