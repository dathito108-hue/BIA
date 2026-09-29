//! Tam-Thien inference matrix.
//!
//! This is an engineering analogy to a three-scale Buddhist cosmology:
//! - TieuThien: immediate local phenomena
//! - TrungThien: conditioned relation bundles
//! - DaiThien: global decision / meaning selection
//!
//! The hot path uses fixed-size Q15 integer arithmetic. It is not an LLM,
//! Transformer, SSM, or dense learned matrix multiplication.

pub const LANES: usize = 16;
pub const Q: i32 = 32767;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatrixLevel {
    TieuThien,
    TrungThien,
    DaiThien,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MatrixSignal {
    pub lane: u8,
    pub value_q15: i16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatrixDecision {
    pub tieu: [i16; LANES],
    pub trung: [i16; LANES],
    pub dai: [i16; LANES],
    pub winner: u8,
    pub confidence_q15: i16,
    pub early_ready: bool,
}

#[derive(Clone, Debug)]
pub struct TamThienMatrix {
    local_decay_q15: i16,
    relation_gain_q15: i16,
    global_gain_q15: i16,
    early_threshold_q15: i16,
}

impl Default for TamThienMatrix {
    fn default() -> Self {
        Self {
            local_decay_q15: 24576,      // 0.75
            relation_gain_q15: 19661,    // 0.60
            global_gain_q15: 22937,      // 0.70
            early_threshold_q15: 21299,  // 0.65
        }
    }
}

impl TamThienMatrix {
    pub fn infer(&self, signals: &[MatrixSignal]) -> MatrixDecision {
        let mut tieu = [0i16; LANES];
        // Sparse local accumulation: only supplied lanes are touched.
        for signal in signals.iter().take(LANES * 2) {
            let lane = signal.lane as usize % LANES;
            tieu[lane] = sat_add(tieu[lane], signal.value_q15);
        }

        // Local normalization/decay is bounded O(LANES).
        for x in &mut tieu {
            *x = qmul(*x, self.local_decay_q15);
        }

        let (local_winner, local_best, local_second) = top2(&tieu);
        let early_ready = local_best >= self.early_threshold_q15
            && local_best.saturating_sub(local_second) >= 3277; // >= 0.10 margin

        // TrungThien only mixes nearest causal neighborhoods.
        let mut trung = [0i16; LANES];
        for i in 0..LANES {
            let left = tieu[(i + LANES - 1) % LANES];
            let self_v = tieu[i];
            let right = tieu[(i + 1) % LANES];
            let neighborhood = avg3(left, self_v, right);
            trung[i] = sat_add(self_v, qmul(neighborhood, self.relation_gain_q15));
        }

        // DaiThien uses four coarse sectors rather than all-to-all interaction.
        let mut dai = [0i16; LANES];
        for sector in 0..4 {
            let base = sector * 4;
            let coarse = avg4(
                trung[base],
                trung[base + 1],
                trung[base + 2],
                trung[base + 3],
            );
            let boosted = qmul(coarse, self.global_gain_q15);
            for i in 0..4 {
                dai[base + i] = sat_add(trung[base + i], boosted);
            }
        }

        let (winner, best, second) = top2(&dai);
        let margin = best.saturating_sub(second).max(0);
        let confidence_q15 = if best <= 0 {
            0
        } else {
            sat_i16(((margin as i32 * Q) / (best as i32).max(1)).clamp(0, Q))
        };

        MatrixDecision {
            tieu,
            trung,
            dai,
            winner: if early_ready { local_winner } else { winner },
            confidence_q15,
            early_ready,
        }
    }

    pub fn infer_early(&self, signals: &[MatrixSignal]) -> Option<(u8, i16)> {
        let mut local = [0i16; LANES];
        for signal in signals.iter().take(LANES * 2) {
            let lane = signal.lane as usize % LANES;
            local[lane] = sat_add(local[lane], signal.value_q15);
        }
        for x in &mut local {
            *x = qmul(*x, self.local_decay_q15);
        }
        let (winner, best, second) = top2(&local);
        if best >= self.early_threshold_q15 && best.saturating_sub(second) >= 3277 {
            Some((winner, best))
        } else {
            None
        }
    }
}

pub fn f32_to_q15(value: f32) -> i16 {
    sat_i16((value.clamp(-1.0, 1.0) * Q as f32).round() as i32)
}

pub fn q15_to_f32(value: i16) -> f32 {
    value as f32 / Q as f32
}

fn qmul(a: i16, b: i16) -> i16 {
    sat_i16(((a as i32 * b as i32) + (1 << 14)) >> 15)
}

fn sat_add(a: i16, b: i16) -> i16 {
    sat_i16(a as i32 + b as i32)
}

fn sat_i16(v: i32) -> i16 {
    v.clamp(i16::MIN as i32, i16::MAX as i32) as i16
}

fn avg3(a: i16, b: i16, c: i16) -> i16 {
    sat_i16((a as i32 + b as i32 + c as i32) / 3)
}

fn avg4(a: i16, b: i16, c: i16, d: i16) -> i16 {
    sat_i16((a as i32 + b as i32 + c as i32 + d as i32) / 4)
}

fn top2(values: &[i16; LANES]) -> (u8, i16, i16) {
    let mut best_i = 0usize;
    let mut best = i16::MIN;
    let mut second = i16::MIN;
    for (i, &v) in values.iter().enumerate() {
        if v > best {
            second = best;
            best = v;
            best_i = i;
        } else if v > second {
            second = v;
        }
    }
    (best_i as u8, best.max(0), second.max(0))
}
