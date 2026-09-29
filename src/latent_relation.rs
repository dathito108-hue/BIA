use crate::semantic_embedding::{SemanticEncoder, SemanticVector};
use crate::types::RelationKind;

#[derive(Clone, Debug, PartialEq)]
pub struct RelationPrototype {
    pub kind: RelationKind,
    pub vector: SemanticVector,
    pub examples: u16,
    pub confidence: f32,
}

#[derive(Clone, Debug, Default)]
pub struct LatentRelationLearner {
    encoder: SemanticEncoder,
    prototypes: Vec<RelationPrototype>,
}

impl LatentRelationLearner {
    pub fn observe(&mut self, text: &str, kind: RelationKind, confidence: f32) {
        if !transferable(kind) {
            return;
        }
        let vector = self.encoder.encode(text);
        if let Some(p) = self.prototypes.iter_mut().find(|p| p.kind == kind) {
            let rate = (1.0 / (p.examples.saturating_add(1).min(16) as f32)).max(0.08);
            p.vector.blend(&vector, rate);
            p.examples = p.examples.saturating_add(1);
            p.confidence =
                (p.confidence * 0.85 + confidence.clamp(0.0, 1.0) * 0.15).clamp(0.0, 1.0);
        } else {
            self.prototypes.push(RelationPrototype {
                kind,
                vector,
                examples: 1,
                confidence: confidence.clamp(0.0, 1.0),
            });
        }
    }

    pub fn classify(&self, text: &str) -> Option<(RelationKind, f32)> {
        let q = self.encoder.encode(text);
        let mut best: Option<(RelationKind, f32)> = None;
        for p in &self.prototypes {
            if p.examples < 2 {
                continue;
            }
            let similarity = ((q.cosine(&p.vector) + 1.0) * 0.5).clamp(0.0, 1.0);
            let score = similarity * p.confidence * support_gain(p.examples);
            if best.is_none_or(|(_, s)| score > s) {
                best = Some((p.kind, score));
            }
        }
        best.filter(|(_, score)| *score >= 0.58)
    }

    pub fn prototypes(&self) -> &[RelationPrototype] {
        &self.prototypes
    }
}

fn support_gain(examples: u16) -> f32 {
    (0.70 + examples.min(8) as f32 * 0.035).min(0.98)
}

fn transferable(kind: RelationKind) -> bool {
    matches!(
        kind,
        RelationKind::Causes | RelationKind::Enables | RelationKind::Inhibits
    )
}
