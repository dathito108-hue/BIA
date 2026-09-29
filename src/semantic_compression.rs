use crate::semantic_embedding::{SemanticEncoder, SemanticVector};

const MAX_CENTROIDS: usize = 32;

#[derive(Clone, Debug, PartialEq)]
pub struct SemanticCentroid {
    pub id: u64,
    pub vector: SemanticVector,
    pub members: u16,
    pub confidence: f32,
}

#[derive(Clone, Debug, Default)]
pub struct SemanticCompressor {
    encoder: SemanticEncoder,
    centroids: Vec<SemanticCentroid>,
    next_id: u64,
}

impl SemanticCompressor {
    pub fn observe(&mut self, text: &str, confidence: f32) -> u64 {
        let vector = self.encoder.encode(text);
        let best = self
            .centroids
            .iter()
            .enumerate()
            .map(|(i, c)| (i, c.vector.cosine(&vector)))
            .max_by(|a, b| {
                a.1.partial_cmp(&b.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

        if let Some((idx, similarity)) = best {
            if similarity >= 0.78 {
                let c = &mut self.centroids[idx];
                let rate = (1.0 / c.members.saturating_add(1).min(16) as f32).max(0.08);
                c.vector.blend(&vector, rate);
                c.members = c.members.saturating_add(1);
                c.confidence =
                    (c.confidence * 0.85 + confidence.clamp(0.0, 1.0) * 0.15).clamp(0.0, 1.0);
                return c.id;
            }
        }

        self.next_id = self.next_id.saturating_add(1);
        let id = self.next_id;
        if self.centroids.len() >= MAX_CENTROIDS {
            let idx = self
                .centroids
                .iter()
                .enumerate()
                .min_by_key(|(_, c)| c.members)
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.centroids.remove(idx);
        }
        self.centroids.push(SemanticCentroid {
            id,
            vector,
            members: 1,
            confidence: confidence.clamp(0.0, 1.0),
        });
        id
    }

    pub fn centroids(&self) -> &[SemanticCentroid] {
        &self.centroids
    }

    pub fn len(&self) -> usize {
        self.centroids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.centroids.is_empty()
    }
}
