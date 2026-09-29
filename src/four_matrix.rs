//! Four-matrix cognitive kernel.
//!
//! Engineering inspiration:
//! 1) Realm Mask Matrix: five bounded "aggregate" channels are selectively gated.
//! 2) Dependent Origination Matrix: recurrent state is updated by conditioned,
//!    element-wise Q15 interactions.
//! 3) Perception Projection Matrix: one conditioned state is summarized into
//!    three cheap perspectives without re-running inference.
//! 4) Zero-State Matrix: temporary scratch is centered/pruned, while durable
//!    memory remains elsewhere in BIA.
//!
//! The Buddhist names are architectural inspiration, not claims that the
//! religious concepts are literal numerical mechanisms.

pub const AGGREGATES: usize = 5;
const Q: i32 = 32767;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RealmBand {
    Embodied,
    Mixed,
    Abstract,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AggregateVector {
    pub rupa: i16,
    pub vedana: i16,
    pub sanna: i16,
    pub sankhara: i16,
    pub vinnana: i16,
}

impl AggregateVector {
    pub fn as_array(self) -> [i16; AGGREGATES] {
        [
            self.rupa,
            self.vedana,
            self.sanna,
            self.sankhara,
            self.vinnana,
        ]
    }

    pub fn from_array(v: [i16; AGGREGATES]) -> Self {
        Self {
            rupa: v[0],
            vedana: v[1],
            sanna: v[2],
            sankhara: v[3],
            vinnana: v[4],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerspectiveProjection {
    pub technical: i16,
    pub affective: i16,
    pub global: i16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FourMatrixOutput {
    pub masked: AggregateVector,
    pub conditioned: AggregateVector,
    pub projected: PerspectiveProjection,
    pub zeroed: AggregateVector,
}

#[derive(Clone, Debug)]
pub struct FourMatrixKernel {
    state: [i16; AGGREGATES],
    decay_q15: i16,
    input_gain_q15: i16,
    neighbor_gain_q15: i16,
    prune_threshold_q15: i16,
}

impl Default for FourMatrixKernel {
    fn default() -> Self {
        Self {
            state: [0; AGGREGATES],
            decay_q15: 24576,          // 0.75
            input_gain_q15: 22937,     // 0.70
            neighbor_gain_q15: 8192,   // 0.25
            prune_threshold_q15: 983,  // ~0.03
        }
    }
}

impl FourMatrixKernel {
    pub fn state(&self) -> AggregateVector {
        AggregateVector::from_array(self.state)
    }

    pub fn process(
        &mut self,
        input: AggregateVector,
        realm: RealmBand,
    ) -> FourMatrixOutput {
        let masked = AggregateVector::from_array(adaptive_realm_mask(input.as_array(), realm));
        let conditioned = AggregateVector::from_array(self.update_dependent(masked.as_array()));
        let projected = project(conditioned.as_array());
        let zeroed = AggregateVector::from_array(zero_state(
            conditioned.as_array(),
            self.prune_threshold_q15,
        ));

        FourMatrixOutput {
            masked,
            conditioned,
            projected,
            zeroed,
        }
    }

    fn update_dependent(
        &mut self,
        input: [i16; AGGREGATES],
    ) -> [i16; AGGREGATES] {
        let previous = self.state;
        let mut next = [0i16; AGGREGATES];

        for i in 0..AGGREGATES {
            let prev = previous[i];
            let left = previous[(i + AGGREGATES - 1) % AGGREGATES];
            let right = previous[(i + 1) % AGGREGATES];
            let neighbor = sat_i16((left as i32 + right as i32) / 2);

            let retained = qmul(prev, self.decay_q15);
            let incoming = qmul(input[i], self.input_gain_q15);
            let conditioned_neighbor = qmul(neighbor, self.neighbor_gain_q15);

            next[i] = sat_add(retained, sat_add(incoming, conditioned_neighbor));
        }

        self.state = next;
        next
    }
}

pub fn encode_text_aggregates(text: &str) -> AggregateVector {
    let mut bytes = 0usize;
    let mut alphabetic = 0usize;
    let mut punctuation = 0usize;
    let mut directive = 0usize;
    let mut uncertainty = 0usize;

    let lower = text.to_lowercase();
    for c in lower.chars().take(256) {
        bytes += c.len_utf8();
        if c.is_alphabetic() {
            alphabetic += 1;
        }
        if c.is_ascii_punctuation() {
            punctuation += 1;
        }
    }

    for marker in ["mở", "mo ", "tìm", "tim ", "làm", "lam ", "hãy", "hay "] {
        if lower.contains(marker) {
            directive += 1;
        }
    }
    for marker in ["không chắc", "khong chac", "có thể", "co the", "hay là", "hay la"] {
        if lower.contains(marker) {
            uncertainty += 1;
        }
    }

    let len = lower.chars().count().max(1) as f32;
    AggregateVector {
        // "Rupa": amount of concrete incoming form/signal.
        rupa: f32_to_q15((bytes as f32 / 256.0).min(1.0)),
        // "Vedana": urgency/salience proxy from punctuation/directive force.
        vedana: f32_to_q15(((punctuation + directive * 2) as f32 / 12.0).min(1.0)),
        // "Sanna": recognizable symbolic density.
        sanna: f32_to_q15((alphabetic as f32 / len).clamp(0.0, 1.0)),
        // "Sankhara": action/formational pressure.
        sankhara: f32_to_q15((directive as f32 / 3.0).min(1.0)),
        // "Vinnana": current coherence proxy, reduced by explicit uncertainty.
        vinnana: f32_to_q15((1.0 - uncertainty as f32 * 0.25).clamp(0.0, 1.0)),
    }
}

pub fn classify_realm(text: &str) -> RealmBand {
    let lower = text.to_lowercase();
    let embodied = [
        "ảnh", "hình", "camera", "màn hình", "âm thanh", "giọng", "cảm biến",
        "file", "tệp", "thiết bị", "điện thoại",
    ]
    .iter()
    .any(|m| lower.contains(m));

    let abstracted = [
        "logic", "chứng minh", "suy luận", "khái niệm", "trừu tượng",
        "nguyên lý", "thuật toán", "toán",
    ]
    .iter()
    .any(|m| lower.contains(m));

    match (embodied, abstracted) {
        (true, false) => RealmBand::Embodied,
        (false, true) => RealmBand::Abstract,
        _ => RealmBand::Mixed,
    }
}

pub fn adaptive_realm_weights(
    input: [i16; AGGREGATES],
    realm: RealmBand,
) -> [i16; AGGREGATES] {
    let base = match realm {
        RealmBand::Embodied => [32767, 28672, 24576, 24576, 28672],
        RealmBand::Mixed => [24576, 24576, 28672, 28672, 28672],
        RealmBand::Abstract => [4096, 12288, 32767, 32767, 32767],
    };

    let total = input.iter().map(|v| v.unsigned_abs() as i32).sum::<i32>().max(1);
    let mut weights = base;
    for i in 0..AGGREGATES {
        let share_q15 = ((input[i].unsigned_abs() as i32 * Q) / total).clamp(0, Q);
        // Up to +12.5% adaptive gain for a channel strongly represented in this event.
        let bonus = share_q15 / 8;
        weights[i] = sat_i16((base[i] as i32 + bonus).min(Q));
        // Never fully mask a channel; minimum keeps contradictory/safety evidence available.
        weights[i] = weights[i].max(2048);
    }
    weights
}

fn adaptive_realm_mask(
    input: [i16; AGGREGATES],
    realm: RealmBand,
) -> [i16; AGGREGATES] {
    let weights = adaptive_realm_weights(input, realm);
    let mut out = [0i16; AGGREGATES];
    for i in 0..AGGREGATES {
        out[i] = qmul(input[i], weights[i]);
    }
    out
}

fn project(v: [i16; AGGREGATES]) -> PerspectiveProjection {
    // Technical emphasizes recognition, formation, awareness.
    let technical = weighted_sum(
        v,
        [4096, 2048, 10923, 8192, 7509],
    );
    // Affective emphasizes salience/feeling plus awareness.
    let affective = weighted_sum(
        v,
        [2048, 16384, 4096, 2048, 8192],
    );
    // Global remains balanced.
    let global = weighted_sum(
        v,
        [6553, 6553, 6553, 6553, 6555],
    );

    PerspectiveProjection {
        technical,
        affective,
        global,
    }
}

fn zero_state(
    mut v: [i16; AGGREGATES],
    threshold: i16,
) -> [i16; AGGREGATES] {
    let mean = v.iter().map(|&x| x as i32).sum::<i32>() / AGGREGATES as i32;
    for x in &mut v {
        let centered = sat_i16(*x as i32 - mean);
        *x = if centered.unsigned_abs() < threshold.unsigned_abs() {
            0
        } else {
            centered
        };
    }
    v
}

fn weighted_sum(
    v: [i16; AGGREGATES],
    w: [i16; AGGREGATES],
) -> i16 {
    let mut acc = 0i32;
    for i in 0..AGGREGATES {
        acc += qmul(v[i], w[i]) as i32;
    }
    sat_i16(acc)
}

pub fn f32_to_q15(value: f32) -> i16 {
    sat_i16((value.clamp(-1.0, 1.0) * Q as f32).round() as i32)
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
