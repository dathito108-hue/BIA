use crate::latent_relation::LatentRelationLearner;
use crate::semantic::{concept_id, normalize};
use crate::types::{Relation, RelationKind};
use crate::world::WorldGraph;

#[derive(Clone, Debug, PartialEq)]
pub struct LatentInference {
    pub from: u64,
    pub to: u64,
    pub kind: RelationKind,
    pub confidence: f32,
}

#[derive(Clone, Debug, Default)]
pub struct HybridSemanticReasoner;

impl HybridSemanticReasoner {
    pub fn infer_clause(
        &self,
        learner: &LatentRelationLearner,
        sentence: &str,
    ) -> Option<LatentInference> {
        let normalized = normalize(sentence);
        let words: Vec<&str> = normalized.split_whitespace().collect();
        if words.len() < 3 {
            return None;
        }

        let split = choose_split(&words);
        let left = words[..split].join(" ");
        let right = words[split..].join(" ");
        let (kind, confidence) = learner.classify(&normalized)?;

        Some(LatentInference {
            from: concept_id(&left),
            to: concept_id(&right),
            kind,
            confidence: (confidence * 0.88).clamp(0.0, 0.90),
        })
    }

    pub fn apply(&self, world: &mut WorldGraph, inference: &LatentInference) {
        world.relate(Relation {
            from: inference.from,
            to: inference.to,
            kind: inference.kind,
            strength: inference.confidence,
            confidence: inference.confidence,
        });
    }
}

fn choose_split(words: &[&str]) -> usize {
    let mid = words.len() / 2;
    mid.clamp(1, words.len().saturating_sub(1))
}
