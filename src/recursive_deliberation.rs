#[derive(Clone, Debug, PartialEq)]
pub struct RecursivePass {
    pub depth: usize,
    pub score: f32,
    pub gain: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RecursiveResult {
    pub passes: Vec<RecursivePass>,
    pub final_score: f32,
    pub stopped_early: bool,
}

#[derive(Clone, Debug)]
pub struct RecursiveDeliberator {
    max_passes: usize,
    min_gain: f32,
}

impl Default for RecursiveDeliberator {
    fn default() -> Self {
        Self {
            max_passes: 4,
            min_gain: 0.025,
        }
    }
}

impl RecursiveDeliberator {
    pub fn run<F>(&self, initial: f32, mut refine: F) -> RecursiveResult
    where
        F: FnMut(usize, f32) -> f32,
    {
        let mut score = initial.clamp(0.0, 1.0);
        let mut passes = Vec::new();
        let mut stopped_early = false;

        for depth in 1..=self.max_passes {
            let next = refine(depth, score).clamp(0.0, 1.0);
            let gain = next - score;
            passes.push(RecursivePass {
                depth,
                score: next,
                gain,
            });
            score = next;

            if gain < self.min_gain {
                stopped_early = true;
                break;
            }
        }

        RecursiveResult {
            passes,
            final_score: score,
            stopped_early,
        }
    }
}
