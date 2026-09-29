use crate::semantic_embedding::{SemanticEncoder, SemanticVector};

const MAX_LATENT_ITEMS: usize = 128;

#[derive(Clone, Debug, PartialEq)]
pub struct LatentItem {
    pub id: u64,
    pub text: String,
    pub vector: SemanticVector,
    pub confidence: f32,
    pub uses: u16,
}

#[derive(Clone, Debug, Default)]
pub struct LatentMemory {
    items: Vec<LatentItem>,
    encoder: SemanticEncoder,
}

impl LatentMemory {
    pub fn remember(&mut self, id: u64, text: &str, confidence: f32) {
        let vector = self.encoder.encode(text);
        if let Some(existing) = self.items.iter_mut().find(|x| x.id == id) {
            existing.vector.blend(&vector, 0.25);
            existing.confidence =
                (existing.confidence * 0.8 + confidence.clamp(0.0, 1.0) * 0.2).clamp(0.0, 1.0);
            existing.uses = existing.uses.saturating_add(1);
            existing.text = text.chars().take(192).collect();
            return;
        }

        if self.items.len() >= MAX_LATENT_ITEMS {
            let idx = self
                .items
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    retention(a)
                        .partial_cmp(&retention(b))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.items.remove(idx);
        }

        self.items.push(LatentItem {
            id,
            text: text.chars().take(192).collect(),
            vector,
            confidence: confidence.clamp(0.0, 1.0),
            uses: 1,
        });
    }

    pub fn nearest(&self, text: &str, limit: usize) -> Vec<(LatentItem, f32)> {
        if limit == 0 {
            return Vec::new();
        }
        let query = self.encoder.encode(text);
        let mut hits: Vec<(LatentItem, f32)> = self
            .items
            .iter()
            .map(|x| {
                let score = ((query.cosine(&x.vector) + 1.0) * 0.5 * x.confidence).clamp(0.0, 1.0);
                (x.clone(), score)
            })
            .filter(|(_, score)| *score > 0.20)
            .collect();
        hits.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        hits.truncate(limit.min(8));
        hits
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

fn retention(x: &LatentItem) -> f32 {
    x.confidence * 0.7 + (x.uses.min(16) as f32 / 16.0) * 0.3
}
