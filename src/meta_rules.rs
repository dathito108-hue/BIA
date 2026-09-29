use crate::rules::SynthesizedRule;
use crate::types::{Relation, RelationKind};
use crate::world::WorldGraph;

const MAX_META_RULES: usize = 8;

#[derive(Clone, Debug, PartialEq)]
pub struct MetaRule {
    pub output: RelationKind,
    pub source_rules: usize,
    pub total_supports: usize,
    pub confidence: f32,
}

#[derive(Clone, Debug, Default)]
pub struct MetaRuleCompressor {
    meta: Vec<MetaRule>,
}

impl MetaRuleCompressor {
    pub fn compress(&mut self, rules: &[SynthesizedRule]) -> usize {
        let mut added = 0usize;
        for output in [
            RelationKind::Causes,
            RelationKind::Enables,
            RelationKind::Inhibits,
        ] {
            let matching: Vec<&SynthesizedRule> =
                rules.iter().filter(|r| r.output == output).collect();
            if matching.len() < 2 {
                continue;
            }
            let supports: usize = matching.iter().map(|r| r.supports).sum();
            let confidence = matching
                .iter()
                .map(|r| r.confidence)
                .sum::<f32>()
                / matching.len() as f32;
            let confidence = (confidence * 0.92).clamp(0.0, 0.93);

            if let Some(existing) = self.meta.iter_mut().find(|m| m.output == output) {
                existing.source_rules = matching.len();
                existing.total_supports = supports;
                existing.confidence = confidence;
            } else if self.meta.len() < MAX_META_RULES {
                self.meta.push(MetaRule {
                    output,
                    source_rules: matching.len(),
                    total_supports: supports,
                    confidence,
                });
                added += 1;
            }
        }
        added
    }

    pub fn apply(&self, world: &mut WorldGraph) -> usize {
        let edges = world.edges().to_vec();
        let mut inferred = Vec::new();
        for meta in &self.meta {
            for a in edges.iter().filter(|e| transferable(e.kind)) {
                for b in edges
                    .iter()
                    .filter(|e| transferable(e.kind) && e.from == a.to)
                {
                    if a.from == b.to || world.edges().iter().any(|e| {
                        e.from == a.from && e.to == b.to && e.kind == meta.output
                    }) {
                        continue;
                    }
                    let confidence = (
                        a.strength
                            * a.confidence
                            * b.strength
                            * b.confidence
                            * meta.confidence
                            * 0.80
                    )
                        .sqrt()
                        .clamp(0.0, 0.90);
                    inferred.push(Relation {
                        from: a.from,
                        to: b.to,
                        kind: meta.output,
                        strength: confidence,
                        confidence,
                    });
                    if inferred.len() >= 24 {
                        break;
                    }
                }
            }
        }
        let n = inferred.len();
        for relation in inferred {
            world.relate(relation);
        }
        n
    }

    pub fn rules(&self) -> &[MetaRule] {
        &self.meta
    }
}

fn transferable(kind: RelationKind) -> bool {
    matches!(
        kind,
        RelationKind::Causes | RelationKind::Enables | RelationKind::Inhibits
    )
}
