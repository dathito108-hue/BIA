use crate::semantic_embedding::{SemanticEncoder, SemanticVector};

const MAX_CONCEPTS: usize = 96;
const MAX_ANCHORS: usize = 24;

#[derive(Clone, Debug, PartialEq)]
pub struct ContinualConcept {
    pub id: u64,
    pub vector: SemanticVector,
    pub observations: u16,
    pub stability: f32,
}

#[derive(Clone, Debug, Default)]
pub struct ContinualSemanticLearner {
    encoder: SemanticEncoder,
    concepts: Vec<ContinualConcept>,
    anchors: Vec<(u64, SemanticVector)>,
}

impl ContinualSemanticLearner {
    pub fn observe(&mut self, id: u64, text: &str) {
        let v = self.encoder.encode(text);
        if let Some(c) = self.concepts.iter_mut().find(|c| c.id == id) {
            let old = c.vector.clone();
            let rate = (0.35 / (c.observations.saturating_add(1) as f32).sqrt()).clamp(0.05, 0.25);
            c.vector.blend(&v, rate);
            c.observations = c.observations.saturating_add(1);
            c.stability = ((old.cosine(&c.vector) + 1.0) * 0.5).clamp(0.0, 1.0);
            return;
        }

        if self.concepts.len() >= MAX_CONCEPTS {
            let idx = self
                .concepts
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    retention(a)
                        .partial_cmp(&retention(b))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.concepts.remove(idx);
        }
        self.concepts.push(ContinualConcept {
            id,
            vector: v,
            observations: 1,
            stability: 1.0,
        });
    }

    pub fn anchor(&mut self, id: u64) -> bool {
        let Some(c) = self.concepts.iter().find(|c| c.id == id) else {
            return false;
        };
        if let Some(a) = self.anchors.iter_mut().find(|a| a.0 == id) {
            a.1 = c.vector.clone();
            return true;
        }
        if self.anchors.len() >= MAX_ANCHORS {
            self.anchors.remove(0);
        }
        self.anchors.push((id, c.vector.clone()));
        true
    }

    pub fn restore_anchors(&mut self, max_drift: f32) -> usize {
        let mut restored = 0;
        for (id, anchor) in &self.anchors {
            if let Some(c) = self.concepts.iter_mut().find(|c| c.id == *id) {
                let similarity = ((c.vector.cosine(anchor) + 1.0) * 0.5).clamp(0.0, 1.0);
                let drift = 1.0 - similarity;
                let rate = if drift > max_drift.clamp(0.0, 1.0) {
                    0.35
                } else {
                    0.05
                };
                c.vector.blend(anchor, rate);
                c.stability = ((c.vector.cosine(anchor) + 1.0) * 0.5).clamp(0.0, 1.0);
                restored += 1;
            }
        }
        restored
    }

    pub fn similarity(&self, a: u64, b: u64) -> Option<f32> {
        let a = self.concepts.iter().find(|c| c.id == a)?;
        let b = self.concepts.iter().find(|c| c.id == b)?;
        Some(((a.vector.cosine(&b.vector) + 1.0) * 0.5).clamp(0.0, 1.0))
    }

    pub fn concept(&self, id: u64) -> Option<&ContinualConcept> {
        self.concepts.iter().find(|c| c.id == id)
    }

    pub fn len(&self) -> usize { self.concepts.len() }
    pub fn is_empty(&self) -> bool { self.concepts.is_empty() }
}

fn retention(c: &ContinualConcept) -> f32 {
    c.stability * 0.65 + (c.observations.min(16) as f32 / 16.0) * 0.35
}
