use crate::knowledge::{KnowledgeLedger, KnowledgeRecord};
use crate::semantic_embedding::SemanticEncoder;

#[derive(Clone, Debug, PartialEq)]
pub struct VectorKnowledgeHit {
    pub record: KnowledgeRecord,
    pub score: f32,
}

#[derive(Clone, Debug, Default)]
pub struct VectorSemanticRetriever {
    encoder: SemanticEncoder,
}

impl VectorSemanticRetriever {
    pub fn recall(
        &self,
        ledger: &KnowledgeLedger,
        query: &str,
        limit: usize,
    ) -> Vec<VectorKnowledgeHit> {
        if limit == 0 {
            return Vec::new();
        }
        let q = self.encoder.encode(query);
        let mut hits: Vec<VectorKnowledgeHit> = ledger
            .records()
            .map(|record| {
                let rv = self.encoder.encode(&record.excerpt);
                let semantic = ((q.cosine(&rv) + 1.0) * 0.5).clamp(0.0, 1.0);
                VectorKnowledgeHit {
                    record: record.clone(),
                    score: (semantic * 0.85 + record.confidence.clamp(0.0, 1.0) * 0.15)
                        .clamp(0.0, 1.0),
                }
            })
            .filter(|h| h.score >= 0.42)
            .collect();

        hits.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        hits.truncate(limit.min(8));
        hits
    }
}
