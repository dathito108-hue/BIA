use crate::episodic::EpisodicMemory;
use crate::types::{Relation, RelationKind};
use crate::world::WorldGraph;

const MAX_RULES: usize = 32;

#[derive(Clone, Debug, PartialEq)]
pub struct SynthesizedRule {
    pub first: RelationKind,
    pub second: RelationKind,
    pub output: RelationKind,
    pub supports: usize,
    pub confidence: f32,
}

#[derive(Clone, Debug, Default)]
pub struct RuleSynthesizer {
    rules: Vec<SynthesizedRule>,
}

impl RuleSynthesizer {
    pub fn synthesize(&mut self, episodes: &EpisodicMemory) -> usize {
        let mut counts: Vec<(RelationKind, RelationKind, usize, f32)> = Vec::new();

        for episode in episodes.episodes() {
            for a in &episode.clauses {
                for b in &episode.clauses {
                    if a.to != b.from || !transferable(a.kind) || !transferable(b.kind) {
                        continue;
                    }
                    let weight = (a.confidence * b.confidence).sqrt();
                    if let Some(x) = counts.iter_mut().find(|x| x.0 == a.kind && x.1 == b.kind) {
                        x.2 += 1;
                        x.3 += weight;
                    } else {
                        counts.push((a.kind, b.kind, 1, weight));
                    }
                }
            }
        }

        let mut learned = 0usize;
        for (first, second, supports, score_sum) in counts {
            if supports < 2 {
                continue;
            }
            let confidence = (score_sum / supports as f32 * 0.9).clamp(0.0, 0.95);
            let output = compose(first, second);
            if let Some(existing) = self
                .rules
                .iter_mut()
                .find(|r| r.first == first && r.second == second)
            {
                existing.supports = supports;
                existing.confidence = confidence;
            } else if self.rules.len() < MAX_RULES {
                self.rules.push(SynthesizedRule {
                    first,
                    second,
                    output,
                    supports,
                    confidence,
                });
                learned += 1;
            }
        }
        learned
    }

    pub fn apply(&self, world: &mut WorldGraph) -> usize {
        let edges = world.edges().to_vec();
        let mut inferred = Vec::new();

        for rule in &self.rules {
            for a in edges.iter().filter(|e| e.kind == rule.first) {
                for b in edges
                    .iter()
                    .filter(|e| e.kind == rule.second && e.from == a.to)
                {
                    if a.from == b.to || world.edges().iter().any(|e| {
                        e.from == a.from && e.to == b.to && e.kind == rule.output
                    }) {
                        continue;
                    }
                    let confidence = (
                        a.strength
                            * a.confidence
                            * b.strength
                            * b.confidence
                            * rule.confidence
                    )
                        .sqrt()
                        .clamp(0.0, 0.95);
                    inferred.push(Relation {
                        from: a.from,
                        to: b.to,
                        kind: rule.output,
                        strength: confidence,
                        confidence,
                    });
                    if inferred.len() >= 32 {
                        break;
                    }
                }
            }
        }

        let n = inferred.len();
        for r in inferred {
            world.relate(r);
        }
        n
    }

    pub fn rules(&self) -> &[SynthesizedRule] {
        &self.rules
    }
}

fn compose(a: RelationKind, b: RelationKind) -> RelationKind {
    match (a, b) {
        (RelationKind::Inhibits, _) | (_, RelationKind::Inhibits) => RelationKind::Inhibits,
        (RelationKind::Causes, _) | (_, RelationKind::Causes) => RelationKind::Causes,
        _ => RelationKind::Enables,
    }
}

fn transferable(kind: RelationKind) -> bool {
    matches!(kind, RelationKind::Causes | RelationKind::Enables | RelationKind::Inhibits)
}
