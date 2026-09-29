use crate::semantic_embedding::{SemanticEncoder, SemanticVector};

#[derive(Clone, Debug, Default)]
pub struct ConceptComposer {
    encoder: SemanticEncoder,
}

impl ConceptComposer {
    pub fn compose(&self, parts: &[&str]) -> Option<SemanticVector> {
        if parts.is_empty() || parts.len() > 8 {
            return None;
        }
        let mut out = self.encoder.encode(parts[0]);
        for (i, part) in parts.iter().enumerate().skip(1) {
            let v = self.encoder.encode(part);
            out.blend(&v, 1.0 / (i as f32 + 1.0));
        }
        Some(out)
    }

    pub fn compositional_similarity(
        &self,
        parts: &[&str],
        target: &str,
    ) -> Option<f32> {
        let composed = self.compose(parts)?;
        let target = self.encoder.encode(target);
        Some(((composed.cosine(&target) + 1.0) * 0.5).clamp(0.0, 1.0))
    }
}
