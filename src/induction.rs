use crate::types::{Relation, RelationKind};
use crate::world::WorldGraph;

#[derive(Clone, Debug, PartialEq)]
pub struct InducedRelation {
    pub relation: Relation,
    pub supports: usize,
}

#[derive(Clone, Debug)]
pub struct InductiveReasoner {
    min_supports: usize,
    max_sources: usize,
}

impl Default for InductiveReasoner {
    fn default() -> Self {
        Self {
            min_supports: 2,
            max_sources: 64,
        }
    }
}

impl InductiveReasoner {
    pub fn infer_between(
        &self,
        world: &WorldGraph,
        target_from: u64,
        target_to: u64,
    ) -> Option<InducedRelation> {
        let mut buckets = [
            (RelationKind::Causes, 0usize, 0.0f32),
            (RelationKind::Enables, 0usize, 0.0f32),
            (RelationKind::Inhibits, 0usize, 0.0f32),
        ];

        for source in world
            .edges()
            .iter()
            .filter(|e| transferable(e.kind))
            .take(self.max_sources)
        {
            let from_sim = similarity(world, source.from, target_from);
            let to_sim = similarity(world, source.to, target_to);
            if from_sim <= 0.0 || to_sim <= 0.0 {
                continue;
            }
            let evidence = (
                source.strength
                    * source.confidence
                    * from_sim
                    * to_sim
            )
                .sqrt()
                .clamp(0.0, 1.0);

            if let Some(bucket) = buckets.iter_mut().find(|b| b.0 == source.kind) {
                bucket.1 += 1;
                bucket.2 = noisy_or(bucket.2, evidence);
            }
        }

        let (kind, supports, confidence) = buckets
            .into_iter()
            .max_by(|a, b| {
                a.2.partial_cmp(&b.2)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?;

        if supports < self.min_supports || confidence < 0.60 {
            return None;
        }

        Some(InducedRelation {
            relation: Relation {
                from: target_from,
                to: target_to,
                kind,
                strength: confidence,
                confidence,
            },
            supports,
        })
    }

    pub fn apply_between(
        &self,
        world: &mut WorldGraph,
        target_from: u64,
        target_to: u64,
    ) -> Option<InducedRelation> {
        let induced = self.infer_between(world, target_from, target_to)?;
        world.relate(induced.relation.clone());
        Some(induced)
    }
}

fn similarity(world: &WorldGraph, a: u64, b: u64) -> f32 {
    if a == b {
        return 1.0;
    }
    world
        .edges()
        .iter()
        .filter(|e| {
            e.kind == RelationKind::Similar
                && ((e.from == a && e.to == b) || (e.from == b && e.to == a))
        })
        .map(|e| e.strength * e.confidence)
        .fold(0.0f32, f32::max)
}

fn transferable(kind: RelationKind) -> bool {
    matches!(
        kind,
        RelationKind::Causes | RelationKind::Enables | RelationKind::Inhibits
    )
}

fn noisy_or(current: f32, evidence: f32) -> f32 {
    (1.0 - (1.0 - current) * (1.0 - evidence)).clamp(0.0, 1.0)
}
