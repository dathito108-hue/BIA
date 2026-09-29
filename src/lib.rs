#![forbid(unsafe_code)]

/// BIA-KSANA-1: dependency-free reference core.
///
/// This implementation is intentionally small and explicit. It demonstrates
/// the state contract and causal ordering of a BIA cognitive moment; optimized
/// quantized kernels arrive in later milestones.

#[derive(Clone, Debug)]
pub struct BiaConfig {
    pub width: usize,
    pub active_conditions: usize,
    pub santati_decay: f32,
}

impl Default for BiaConfig {
    fn default() -> Self {
        Self {
            width: 256,
            active_conditions: 4,
            santati_decay: 0.94,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BiaState {
    /// Continuity stream. This is transient process state, not an identity.
    pub santati: Vec<f32>,
    /// Revisable self-model kept separate from the core continuity stream.
    pub self_model: Vec<f32>,
    pub step: u64,
}

impl BiaState {
    pub fn new(width: usize) -> Self {
        Self {
            santati: vec![0.0; width],
            self_model: vec![0.0; width],
            step: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ksana {
    pub rupa: Vec<f32>,
    pub vedana: Vec<f32>,
    pub sanna: Vec<f32>,
    pub sankhara: Vec<f32>,
    pub vinnana: Vec<f32>,
    pub active_conditions: Vec<usize>,
}

#[derive(Clone, Debug)]
pub struct BiaCore {
    config: BiaConfig,
}

impl BiaCore {
    pub fn new(config: BiaConfig) -> Self {
        assert!(config.width > 0);
        assert!(config.active_conditions > 0);
        assert!((0.0..=1.0).contains(&config.santati_decay));
        Self { config }
    }

    pub fn config(&self) -> &BiaConfig {
        &self.config
    }

    /// Execute one bounded cognitive moment.
    ///
    /// Input may be any already-encoded modality vector. Values are folded
    /// into the configured width, allowing streaming adapters to remain
    /// independent from the process core.
    pub fn step(&self, input: &[f32], state: &mut BiaState) -> Ksana {
        assert_eq!(state.santati.len(), self.config.width);
        assert_eq!(state.self_model.len(), self.config.width);

        let rupa = fold_input(input, self.config.width);

        // Vedana is a relevance/novelty signal, not an emotion claim.
        let vedana = combine(&rupa, &state.santati, |r, h| (r - h).tanh());

        // Sanna binds current form with continuity for recognition.
        let sanna = combine(&rupa, &state.santati, |r, h| (0.70 * r + 0.30 * h).tanh());

        // Sankhara forms candidate transformations / intentions.
        let sankhara = combine(&sanna, &vedana, |n, v| (n + 0.50 * v).tanh());

        // Vinnana is only the transient integration of this moment.
        let vinnana = zip4(&rupa, &vedana, &sanna, &sankhara, |r, v, n, k| {
            (0.20 * r + 0.15 * v + 0.30 * n + 0.35 * k).tanh()
        });

        let active_conditions = top_k_abs(&vedana, self.config.active_conditions);

        // Sparse dependent update: only dimensions selected by conditions
        // receive the stronger event write. All dimensions still decay.
        for i in 0..self.config.width {
            state.santati[i] *= self.config.santati_decay;
        }
        for &i in &active_conditions {
            state.santati[i] +=
                (1.0 - self.config.santati_decay) * vinnana[i];
        }

        // A self-model is maintained as a weak, revisable hypothesis.
        // It cannot dominate the process state in this reference contract.
        for i in 0..self.config.width {
            state.self_model[i] =
                (0.98 * state.self_model[i] + 0.02 * state.santati[i]).tanh();
        }

        state.step = state.step.saturating_add(1);

        Ksana {
            rupa,
            vedana,
            sanna,
            sankhara,
            vinnana,
            active_conditions,
        }
    }
}

fn fold_input(input: &[f32], width: usize) -> Vec<f32> {
    let mut out = vec![0.0; width];
    if input.is_empty() {
        return out;
    }
    let mut counts = vec![0u32; width];
    for (i, &x) in input.iter().enumerate() {
        let j = i % width;
        out[j] += x;
        counts[j] += 1;
    }
    for i in 0..width {
        if counts[i] > 0 {
            out[i] = (out[i] / counts[i] as f32).tanh();
        }
    }
    out
}

fn combine<F>(a: &[f32], b: &[f32], f: F) -> Vec<f32>
where
    F: Fn(f32, f32) -> f32,
{
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).map(|(&x, &y)| f(x, y)).collect()
}

fn zip4<F>(a: &[f32], b: &[f32], c: &[f32], d: &[f32], f: F) -> Vec<f32>
where
    F: Fn(f32, f32, f32, f32) -> f32,
{
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), c.len());
    assert_eq!(a.len(), d.len());
    (0..a.len()).map(|i| f(a[i], b[i], c[i], d[i])).collect()
}

fn top_k_abs(values: &[f32], k: usize) -> Vec<usize> {
    let mut idx: Vec<usize> = (0..values.len()).collect();
    idx.sort_unstable_by(|&a, &b| {
        values[b]
            .abs()
            .partial_cmp(&values[a].abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    idx.truncate(k.min(values.len()));
    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ksana_is_bounded_and_advances_stream() {
        let core = BiaCore::new(BiaConfig {
            width: 8,
            active_conditions: 2,
            santati_decay: 0.9,
        });
        let mut state = BiaState::new(8);

        let event = core.step(&[1.0, -0.5, 0.25], &mut state);

        assert_eq!(event.rupa.len(), 8);
        assert_eq!(event.vinnana.len(), 8);
        assert_eq!(event.active_conditions.len(), 2);
        assert_eq!(state.santati.len(), 8);
        assert_eq!(state.self_model.len(), 8);
        assert_eq!(state.step, 1);
        assert!(state.santati.iter().any(|x| *x != 0.0));
    }

    #[test]
    fn empty_input_is_valid_stream_event() {
        let core = BiaCore::new(BiaConfig {
            width: 4,
            active_conditions: 1,
            santati_decay: 0.95,
        });
        let mut state = BiaState::new(4);
        let event = core.step(&[], &mut state);

        assert_eq!(event.rupa, vec![0.0; 4]);
        assert_eq!(state.step, 1);
    }
}
