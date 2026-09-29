use crate::memory::similarity;
use crate::types::{Phenomenon, Relation, RelationKind};

#[derive(Clone, Debug, PartialEq)]
pub struct Concept {
    pub id: u32,
    pub prototype: Vec<f32>,
    pub confidence: f32,
    pub observations: u32,
    pub contradictions: u32,
    pub last_seen: u64,
}

#[derive(Clone, Debug)]
pub struct MeaningFormation {
    capacity: usize,
    next_id: u32,
    merge_threshold: f32,
    concepts: Vec<Concept>,
}

impl MeaningFormation {
    pub fn new(capacity: usize, first_id: u32) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            next_id: first_id,
            merge_threshold: 0.86,
            concepts: Vec::new(),
        }
    }

    pub fn observe(&mut self, p: &Phenomenon) -> u32 {
        if let Some((index, score)) = self.best_match(p) {
            if score >= self.merge_threshold {
                let c = &mut self.concepts[index];
                c.observations = c.observations.saturating_add(1);
                let rate = 1.0 / (c.observations.min(64) as f32);
                merge_prototype(&mut c.prototype, &p.features, rate);
                c.confidence =
                    (c.confidence * (1.0 - rate) + p.confidence * rate).clamp(0.0, 1.0);
                c.last_seen = p.timestamp;
                return c.id;
            }
        }

        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        if self.concepts.len() >= self.capacity {
            let index = self
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
            self.concepts.remove(index);
        }
        self.concepts.push(Concept {
            id,
            prototype: p.features.clone(),
            confidence: p.confidence,
            observations: 1,
            contradictions: 0,
            last_seen: p.timestamp,
        });
        id
    }

    pub fn contradict(&mut self, concept_id: u32, severity: f32) {
        if let Some(c) = self.concepts.iter_mut().find(|c| c.id == concept_id) {
            c.contradictions = c.contradictions.saturating_add(1);
            c.confidence =
                (c.confidence * (1.0 - 0.35 * severity.clamp(0.0, 1.0))).clamp(0.0, 1.0);
        }
    }

    pub fn infer_relation(
        &self,
        earlier: &Phenomenon,
        later: &Phenomenon,
        max_gap: u64,
    ) -> Option<Relation> {
        if later.timestamp < earlier.timestamp {
            return None;
        }
        let gap = later.timestamp - earlier.timestamp;
        if gap > max_gap {
            return None;
        }
        let proximity = 1.0 - gap as f32 / max_gap.max(1) as f32;
        let feature_affinity = similarity(&earlier.features, &later.features);
        let strength = (0.55 * proximity + 0.45 * feature_affinity).clamp(0.0, 1.0);
        if strength < 0.45 {
            return None;
        }
        Some(Relation {
            from: earlier.id,
            to: later.id,
            kind: RelationKind::Follows,
            strength,
            confidence: (earlier.confidence * later.confidence).sqrt(),
        })
    }

    pub fn concepts(&self) -> &[Concept] {
        &self.concepts
    }

    pub fn len(&self) -> usize {
        self.concepts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.concepts.is_empty()
    }

    fn best_match(&self, p: &Phenomenon) -> Option<(usize, f32)> {
        self.concepts
            .iter()
            .enumerate()
            .map(|(i, c)| (i, similarity(&c.prototype, &p.features)))
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    }
}

fn retention(c: &Concept) -> f32 {
    let contradiction_penalty = 1.0 / (1.0 + c.contradictions as f32);
    c.confidence * 0.6
        + (c.observations as f32).ln_1p() * 0.25
        + contradiction_penalty * 0.15
}

fn merge_prototype(dst: &mut Vec<f32>, src: &[f32], rate: f32) {
    if dst.len() < src.len() {
        dst.resize(src.len(), 0.0);
    }
    for (i, &x) in src.iter().enumerate() {
        dst[i] = dst[i] * (1.0 - rate) + x * rate;
    }
}
