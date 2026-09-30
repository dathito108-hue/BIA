use crate::conversation_continuity::RelationContinuity;
use crate::dialogue_goal_state::ImplicitDialogueGoal;
use crate::duyen_weave::DuyenWeave;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogueAction {
    Answer,
    Explain,
    Ground,
    Contrast,
    Qualify,
    Continue,
    Clarify,
    Invite,
    Conclude,
    Return,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ContextualDialoguePolicy {
    pub actions: Vec<DialogueAction>,
}

impl ContextualDialoguePolicy {
    pub fn select(
        goal: ImplicitDialogueGoal,
        continuity: RelationContinuity,
        weave: &DuyenWeave,
        uncertainty: f32,
        evidence: usize,
        repair_pending: bool,
    ) -> Self {
        let uncertainty = uncertainty.clamp(0.0, 1.0);
        let overlap = weave_overlap(weave);
        let mut scored: Vec<(DialogueAction, i32)> = vec![
            (DialogueAction::Answer, 100),
            (DialogueAction::Explain, 0),
            (DialogueAction::Ground, 0),
            (DialogueAction::Contrast, 0),
            (DialogueAction::Qualify, 0),
            (DialogueAction::Continue, 0),
            (DialogueAction::Clarify, 0),
            (DialogueAction::Invite, 0),
            (DialogueAction::Conclude, 0),
            (DialogueAction::Return, 0),
        ];

        for (action, score) in &mut scored {
            *score += match action {
                DialogueAction::Explain => (weave.max_depth as i32).min(3) * 8,
                DialogueAction::Ground => (evidence.min(4) as i32) * 12,
                DialogueAction::Contrast => (overlap * 40.0) as i32,
                DialogueAction::Qualify => (uncertainty * 50.0) as i32,
                DialogueAction::Continue => {
                    if goal == ImplicitDialogueGoal::Explore && continuity != RelationContinuity::Repeat { 35 } else { 0 }
                }
                DialogueAction::Clarify => if repair_pending || goal == ImplicitDialogueGoal::Challenge { 32 } else { 0 },
                DialogueAction::Invite => if goal == ImplicitDialogueGoal::Explore && overlap > 0.0 { 22 } else { 0 },
                DialogueAction::Conclude => if goal == ImplicitDialogueGoal::Conclude { 45 } else { 0 },
                DialogueAction::Return => if continuity == RelationContinuity::Return { 40 } else { 0 },
                DialogueAction::Answer => 0,
            };
        }

        if repair_pending {
            scored.retain(|(a, _)| matches!(a, DialogueAction::Clarify | DialogueAction::Answer));
        } else if uncertainty >= 0.35 {
            scored.retain(|(a, _)| !matches!(a, DialogueAction::Conclude));
        }

        scored.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| action_order(a.0).cmp(&action_order(b.0))));
        let mut actions = Vec::new();
        for (action, score) in scored {
            if score <= 0 && action != DialogueAction::Answer { continue; }
            if !actions.contains(&action) { actions.push(action); }
            if actions.len() >= 4 { break; }
        }
        if actions.is_empty() { actions.push(DialogueAction::Answer); }
        Self { actions }
    }

    pub fn primary(&self) -> DialogueAction {
        self.actions.first().copied().unwrap_or(DialogueAction::Answer)
    }

    pub fn has(&self, action: DialogueAction) -> bool {
        self.actions.contains(&action)
    }
}

fn action_order(action: DialogueAction) -> u8 {
    match action {
        DialogueAction::Answer => 0,
        DialogueAction::Explain => 1,
        DialogueAction::Ground => 2,
        DialogueAction::Contrast => 3,
        DialogueAction::Qualify => 4,
        DialogueAction::Continue => 5,
        DialogueAction::Clarify => 6,
        DialogueAction::Invite => 7,
        DialogueAction::Conclude => 8,
        DialogueAction::Return => 9,
    }
}

fn weave_overlap(weave: &DuyenWeave) -> f32 {
    weave.overlap_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uncertainty_prefers_qualification() {
        let w = DuyenWeave::default();
        let p = ContextualDialoguePolicy::select(
            ImplicitDialogueGoal::Verify,
            RelationContinuity::SameSubject,
            &w,
            0.8,
            1,
            false,
        );
        assert!(p.has(DialogueAction::Qualify));
    }

    #[test]
    fn repair_keeps_policy_bounded() {
        let w = DuyenWeave::default();
        let p = ContextualDialoguePolicy::select(
            ImplicitDialogueGoal::Explore,
            RelationContinuity::New,
            &w,
            0.1,
            0,
            true,
        );
        assert!(p.has(DialogueAction::Clarify));
        assert!(p.actions.len() <= 2);
    }

    #[test]
    fn policy_never_runs_unbounded() {
        let w = DuyenWeave::default();
        let p = ContextualDialoguePolicy::select(
            ImplicitDialogueGoal::Explore,
            RelationContinuity::SameObject,
            &w,
            0.5,
            4,
            false,
        );
        assert!(p.actions.len() <= 4);
    }
}
