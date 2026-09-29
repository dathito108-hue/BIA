use crate::types::{Relation, RelationKind};
use crate::world::WorldGraph;

#[derive(Clone, Debug, PartialEq)]
pub struct AnalogicalHypothesis {
    pub relation: Relation,
    pub source_relation: Relation,
    pub mapping_confidence: f32,
}

#[derive(Clone, Debug)]
pub struct AnalogicalReasoner {
    max_hypotheses: usize,
    threshold: f32,
}

impl Default for AnalogicalReasoner {
    fn default() -> Self {
        Self {
            max_hypotheses: 16,
            threshold: 0.55,
        }
    }
}

impl AnalogicalReasoner {
    pub fn infer(&self, world: &WorldGraph) -> Vec<AnalogicalHypothesis> {
        let edges = world.edges();
        let mut out = Vec::new();

        for source in edges.iter().filter(|e| transferable(e.kind)) {
            let from_maps: Vec<&Relation> = edges
                .iter()
                .filter(|e| {
                    e.kind == RelationKind::Similar
                        && (e.from == source.from || e.to == source.from)
                })
                .take(8)
                .collect();
            let to_maps: Vec<&Relation> = edges
                .iter()
                .filter(|e| {
                    e.kind == RelationKind::Similar
                        && (e.from == source.to || e.to == source.to)
                })
                .take(8)
                .collect();

            for fm in &from_maps {
                let mapped_from = other_end(fm, source.from);
                for tm in &to_maps {
                    let mapped_to = other_end(tm, source.to);
                    if mapped_from == mapped_to
                        || world.edges().iter().any(|e| {
                            e.from == mapped_from
                                && e.to == mapped_to
                                && e.kind == source.kind
                        })
                    {
                        continue;
                    }

                    let mapping_confidence = (
                        fm.strength
                            * fm.confidence
                            * tm.strength
                            * tm.confidence
                    )
                        .sqrt()
                        .clamp(0.0, 1.0);
                    let confidence = (
                        source.strength
                            * source.confidence
                            * mapping_confidence
                            * 0.85
                    )
                        .clamp(0.0, 1.0);

                    if confidence >= self.threshold {
                        out.push(AnalogicalHypothesis {
                            relation: Relation {
                                from: mapped_from,
                                to: mapped_to,
                                kind: source.kind,
                                strength: confidence,
                                confidence,
                            },
                            source_relation: source.clone(),
                            mapping_confidence,
                        });
                        if out.len() >= self.max_hypotheses {
                            return out;
                        }
                    }
                }
            }
        }
        out
    }

    pub fn apply(&self, world: &mut WorldGraph) -> usize {
        let hypotheses = self.infer(world);
        let count = hypotheses.len();
        for h in hypotheses {
            world.relate(h.relation);
        }
        count
    }
}

fn other_end(r: &Relation, node: u64) -> u64 {
    if r.from == node { r.to } else { r.from }
}

fn transferable(kind: RelationKind) -> bool {
    matches!(
        kind,
        RelationKind::Causes | RelationKind::Enables | RelationKind::Inhibits
    )
}
