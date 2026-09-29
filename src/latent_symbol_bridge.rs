use crate::semantic_embedding::{SemanticEncoder, SemanticVector};

const MAX_SYMBOLS: usize = 96;

#[derive(Clone, Debug, PartialEq)]
pub struct SemanticSymbol {
    pub id: u64,
    pub label: String,
    pub vector: SemanticVector,
    pub confidence: f32,
}

#[derive(Clone, Debug, Default)]
pub struct LatentSymbolBridge {
    encoder: SemanticEncoder,
    symbols: Vec<SemanticSymbol>,
}

impl LatentSymbolBridge {
    pub fn bind(&mut self, id: u64, label: &str, confidence: f32) {
        let vector = self.encoder.encode(label);
        if let Some(s) = self.symbols.iter_mut().find(|s| s.id == id) {
            s.vector.blend(&vector, 0.20);
            s.label = label.chars().take(96).collect();
            s.confidence = (s.confidence * 0.8 + confidence.clamp(0.0,1.0) * 0.2).clamp(0.0,1.0);
            return;
        }
        if self.symbols.len() >= MAX_SYMBOLS {
            let idx = self.symbols
                .iter()
                .enumerate()
                .min_by(|(_,a),(_,b)| a.confidence.partial_cmp(&b.confidence).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i,_)| i).unwrap_or(0);
            self.symbols.remove(idx);
        }
        self.symbols.push(SemanticSymbol {
            id,
            label: label.chars().take(96).collect(),
            vector,
            confidence: confidence.clamp(0.0,1.0),
        });
    }

    pub fn nearest_symbol(&self, text: &str) -> Option<(SemanticSymbol, f32)> {
        let q = self.encoder.encode(text);
        self.symbols.iter().map(|s| {
            let score = ((q.cosine(&s.vector)+1.0)*0.5*s.confidence).clamp(0.0,1.0);
            (s.clone(), score)
        }).max_by(|a,b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .filter(|(_,score)| *score >= 0.50)
    }

    pub fn label(&self, id: u64) -> Option<&str> {
        self.symbols.iter().find(|s| s.id == id).map(|s| s.label.as_str())
    }

    pub fn len(&self) -> usize { self.symbols.len() }
    pub fn is_empty(&self) -> bool { self.symbols.is_empty() }
}
