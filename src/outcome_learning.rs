use crate::deliberation::{DeliberativePlanner, GoalSpec, PlanCandidate};
use crate::world_model::{SimState, WorldModel};

#[derive(Clone, Debug, PartialEq)]
pub struct PredictionAudit {
    pub expected_facts: usize,
    pub matched_facts: usize,
    pub unexpected_facts: usize,
    pub accuracy: f32,
    pub replan_required: bool,
}

#[derive(Clone, Debug, Default)]
pub struct OutcomeLearner;

impl OutcomeLearner {
    pub fn audit(&self, predicted: &SimState, observed: &SimState) -> PredictionAudit {
        let matched = predicted
            .facts
            .iter()
            .filter(|f| observed.contains(**f))
            .count();
        let unexpected = observed
            .facts
            .iter()
            .filter(|f| !predicted.contains(**f))
            .count();
        let expected = predicted.facts.len();
        let accuracy = matched as f32 / expected.max(1) as f32;
        PredictionAudit {
            expected_facts: expected,
            matched_facts: matched,
            unexpected_facts: unexpected,
            accuracy,
            replan_required: accuracy < 0.70 || unexpected > expected / 2 + 1,
        }
    }

    pub fn replan_if_needed(
        &self,
        model: &WorldModel,
        goal: &GoalSpec,
        predicted: &SimState,
        observed: &SimState,
    ) -> Option<PlanCandidate> {
        let audit = self.audit(predicted, observed);
        audit
            .replan_required
            .then(|| DeliberativePlanner.plan(model, observed, goal))
            .flatten()
    }
}
