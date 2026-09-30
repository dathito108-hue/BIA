use crate::contextual_dialogue_policy::DialogueAction;
use crate::duyen_weave::DuyenWeave;

pub const MAX_STAGES: usize = 3;
pub const MAX_ACTIONS: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompositionMode {
    Sequential,
    Parallel,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompositionStage {
    pub mode: CompositionMode,
    pub actions: Vec<DialogueAction>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CausalDialogueComposition {
    pub stages: Vec<CompositionStage>,
}

impl CausalDialogueComposition {
    pub fn build(
        selected: &[DialogueAction],
        weave: &DuyenWeave,
        uncertainty: f32,
        evidence: usize,
    ) -> Self {
        let uncertainty = uncertainty.clamp(0.0, 1.0);
        let mut ordered = Vec::new();

        for action in selected.iter().copied() {
            if !ordered.contains(&action) {
                ordered.push(action);
            }
            if ordered.len() >= MAX_ACTIONS {
                break;
            }
        }

        if ordered.is_empty() {
            ordered.push(DialogueAction::Answer);
        }

        // The answer establishes the current relation. Other observations
        // read the same causal state and can therefore be composed in parallel.
        let mut sequential = Vec::new();
        let mut parallel = Vec::new();
        let mut tail = Vec::new();

        for action in ordered {
            match action {
                DialogueAction::Answer => sequential.push(action),
                DialogueAction::Explain
                | DialogueAction::Ground
                | DialogueAction::Contrast
                | DialogueAction::Qualify => parallel.push(action),
                DialogueAction::Continue
                | DialogueAction::Clarify
                | DialogueAction::Invite
                | DialogueAction::Conclude
                | DialogueAction::Return => tail.push(action),
            }
        }

        // Overlap is the causal signal that makes multi-branch composition
        // useful. Without it, keep only evidence/qualification branches that
        // have direct support in the current state.
        if weave.is_overlapping() {
            if evidence == 0 {
                parallel.retain(|a| *a != DialogueAction::Ground);
            }
        } else {
            parallel.retain(|a| {
                matches!(a, DialogueAction::Ground | DialogueAction::Qualify)
                    && (evidence > 0 || uncertainty >= 0.35)
            });
            if parallel.is_empty() && weave.max_depth > 1 {
                parallel.push(DialogueAction::Explain);
            }
        }

        let mut stages = Vec::new();
        if !sequential.is_empty() {
            stages.push(CompositionStage {
                mode: CompositionMode::Sequential,
                actions: sequential,
            });
        }
        if !parallel.is_empty() {
            stages.push(CompositionStage {
                mode: CompositionMode::Parallel,
                actions: parallel,
            });
        }
        if !tail.is_empty() {
            stages.push(CompositionStage {
                mode: CompositionMode::Sequential,
                actions: tail,
            });
        }

        stages.truncate(MAX_STAGES);
        Self { stages }
    }

    pub fn actions(&self) -> impl Iterator<Item = DialogueAction> + '_ {
        self.stages.iter().flat_map(|stage| stage.actions.iter().copied())
    }

    pub fn contains(&self, action: DialogueAction) -> bool {
        self.stages.iter().any(|stage| stage.actions.contains(&action))
    }

    pub fn parallel_actions(&self) -> usize {
        self.stages
            .iter()
            .filter(|stage| stage.mode == CompositionMode::Parallel)
            .map(|stage| stage.actions.len())
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn overlapping() -> DuyenWeave {
        DuyenWeave {
            supporting_paths: 2,
            opposing_paths: 1,
            convergence_nodes: vec![1, 2],
            shared_links: 1,
            max_depth: 3,
            overlap_score: 0.8,
        }
    }

    #[test]
    fn overlapping_branches_compose_answer_and_parallel_ground_contrast() {
        let selected = [
            DialogueAction::Answer,
            DialogueAction::Explain,
            DialogueAction::Ground,
            DialogueAction::Contrast,
            DialogueAction::Continue,
        ];
        let plan = CausalDialogueComposition::build(&selected, &overlapping(), 0.2, 2);
        assert!(plan.contains(DialogueAction::Answer));
        assert!(plan.contains(DialogueAction::Ground));
        assert!(plan.contains(DialogueAction::Contrast));
        assert!(plan.parallel_actions() >= 2);
        assert!(plan.stages.iter().any(|s| s.mode == CompositionMode::Parallel));
    }

    #[test]
    fn uncertainty_keeps_qualification_without_removing_answer() {
        let selected = [DialogueAction::Answer, DialogueAction::Qualify];
        let plan = CausalDialogueComposition::build(&selected, &DuyenWeave::default(), 0.8, 0);
        assert!(plan.contains(DialogueAction::Answer));
        assert!(plan.contains(DialogueAction::Qualify));
    }

    #[test]
    fn evidence_can_run_as_parallel_branch() {
        let selected = [DialogueAction::Answer, DialogueAction::Ground, DialogueAction::Explain];
        let plan = CausalDialogueComposition::build(&selected, &overlapping(), 0.1, 3);
        assert_eq!(plan.parallel_actions(), 2);
    }

    #[test]
    fn sequential_follow_up_is_preserved() {
        let selected = [DialogueAction::Answer, DialogueAction::Explain, DialogueAction::Continue];
        let plan = CausalDialogueComposition::build(&selected, &overlapping(), 0.1, 1);
        assert!(plan.contains(DialogueAction::Continue));
        assert_eq!(
            plan.stages.last().map(|s| s.mode),
            Some(CompositionMode::Sequential)
        );
    }

    #[test]
    fn no_context_falls_back_safely() {
        let plan = CausalDialogueComposition::build(&[], &DuyenWeave::default(), 0.0, 0);
        assert!(plan.contains(DialogueAction::Answer));
        assert!(plan.actions().count() <= MAX_ACTIONS);
    }

    #[test]
    fn composition_is_bounded() {
        let selected = [
            DialogueAction::Answer,
            DialogueAction::Explain,
            DialogueAction::Ground,
            DialogueAction::Contrast,
            DialogueAction::Qualify,
            DialogueAction::Continue,
            DialogueAction::Invite,
            DialogueAction::Return,
        ];
        let plan = CausalDialogueComposition::build(&selected, &overlapping(), 0.5, 4);
        assert!(plan.stages.len() <= MAX_STAGES);
        assert!(plan.actions().count() <= MAX_ACTIONS);
    }
}
