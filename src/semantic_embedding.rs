use crate::semantic::normalize;

pub const SEMANTIC_DIM: usize = 32;

#[derive(Clone, Debug, PartialEq)]
pub struct SemanticVector {
    pub values: [f32; SEMANTIC_DIM],
}

impl SemanticVector {
    pub fn cosine(&self, other: &Self) -> f32 {
        let mut dot = 0.0f32;
        let mut aa = 0.0f32;
        let mut bb = 0.0f32;
        for i in 0..SEMANTIC_DIM {
            dot += self.values[i] * other.values[i];
            aa += self.values[i] * self.values[i];
            bb += other.values[i] * other.values[i];
        }
        if aa <= 1e-9 || bb <= 1e-9 {
            return 0.0;
        }
        (dot / (aa.sqrt() * bb.sqrt())).clamp(-1.0, 1.0)
    }

    pub fn blend(&mut self, other: &Self, rate: f32) {
        let rate = rate.clamp(0.0, 1.0);
        for i in 0..SEMANTIC_DIM {
            self.values[i] = self.values[i] * (1.0 - rate) + other.values[i] * rate;
        }
        normalize_vec(&mut self.values);
    }
}

#[derive(Clone, Debug, Default)]
pub struct SemanticEncoder;

impl SemanticEncoder {
    pub fn encode(&self, text: &str) -> SemanticVector {
        let normalized = normalize(text);
        let bytes = normalized.as_bytes();
        let mut values = [0.0f32; SEMANTIC_DIM];

        for token in normalized.split_whitespace().take(64) {
            let h = hash64(token.as_bytes());
            let lane = (h as usize) & (SEMANTIC_DIM - 1);
            values[lane] += 1.0;
            let lane2 = ((h >> 11) as usize) & (SEMANTIC_DIM - 1);
            values[lane2] += 0.5;
        }

        for n in 2..=4usize {
            if bytes.len() < n {
                continue;
            }
            for gram in bytes.windows(n).take(192) {
                let h = hash64(gram);
                let lane = (h as usize) & (SEMANTIC_DIM - 1);
                let sign = if (h >> 63) == 0 { 1.0 } else { -1.0 };
                values[lane] += sign * (1.0 / n as f32);
            }
        }

        normalize_vec(&mut values);
        SemanticVector { values }
    }
}

fn normalize_vec(values: &mut [f32; SEMANTIC_DIM]) {
    let norm = values.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm > 1e-9 {
        for x in values {
            *x /= norm;
        }
    }
}

fn hash64(bytes: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in bytes {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
