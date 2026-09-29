use crate::continual_semantics::ContinualSemanticLearner;

#[derive(Clone, Debug, PartialEq)]
pub struct ConsolidationReport {
    pub anchored: usize,
    pub restored: usize,
    pub retained: usize,
}

#[derive(Clone, Debug, Default)]
pub struct SemanticConsolidator {
    passes: u32,
}

impl SemanticConsolidator {
    pub fn consolidate(
        &mut self,
        learner: &mut ContinualSemanticLearner,
        important_ids: &[u64],
    ) -> ConsolidationReport {
        let mut anchored = 0usize;
        for id in important_ids.iter().copied().take(24) {
            if learner.anchor(id) {
                anchored += 1;
            }
        }
        let restored = learner.restore_anchors(0.18);
        self.passes = self.passes.saturating_add(1);
        ConsolidationReport {
            anchored,
            restored,
            retained: learner.len(),
        }
    }

    pub fn passes(&self) -> u32 { self.passes }
}
