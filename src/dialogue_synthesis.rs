use crate::dialogue_goal_state::ImplicitDialogueGoal;
use crate::conversation_continuity::RelationContinuity;
use crate::duyen_weave::DuyenWeave;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogueMove {
    Answer,
    Ground,
    Contrast,
    Qualify,
    Continue,
    Close,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DialogueSynthesisPlan {
    pub moves: Vec<DialogueMove>,
    pub uncertainty: f32,
    pub overlap: f32,
}

impl DialogueSynthesisPlan {
    pub fn build(
        goal: Option<ImplicitDialogueGoal>,
        continuity: RelationContinuity,
        weave: &DuyenWeave,
        uncertainty: f32,
        evidence_count: usize,
    ) -> Self {
        let uncertainty = uncertainty.clamp(0.0, 1.0);
        let mut moves = vec![DialogueMove::Answer];

        if evidence_count > 0 || weave.max_depth > 0 {
            moves.push(DialogueMove::Ground);
        }
        if weave.opposing_paths > 0 {
            moves.push(DialogueMove::Contrast);
        }
        if uncertainty >= 0.25 || matches!(goal, Some(ImplicitDialogueGoal::Verify)) {
            moves.push(DialogueMove::Qualify);
        }
        if matches!(goal, Some(ImplicitDialogueGoal::Explore))
            && !matches!(continuity, RelationContinuity::Repeat)
        {
            moves.push(DialogueMove::Continue);
        }
        if matches!(goal, Some(ImplicitDialogueGoal::Conclude)) {
            moves.push(DialogueMove::Close);
        }

        moves.dedup();
        moves.truncate(5);
        Self {
            moves,
            uncertainty,
            overlap: weave.overlap_score,
        }
    }

    pub fn contains(&self, mv: DialogueMove) -> bool {
        self.moves.contains(&mv)
    }

    /// Adds only information selected by the current dialogue state.
    /// It never changes the underlying conclusion or invents evidence.
    pub fn enrich(
        &self,
        mut text: String,
        subject: &str,
        object: &str,
        evidence: &[String],
    ) -> String {
        if self.contains(DialogueMove::Contrast) && !text.contains("phản") {
            text.push_str(" Có cả nhánh ủng hộ và nhánh phản đối nên tôi giữ hai chiều thay vì ép thành một kết luận tuyệt đối.");
        }
        if self.contains(DialogueMove::Qualify)
            && self.uncertainty >= 0.45
            && !text.contains("chưa chắc")
            && !text.contains("chưa đủ")
        {
            text.push_str(" Độ chắc hiện tại còn hạn chế; phần này nên được xem là kết luận theo bằng chứng đang có.");
        }
        if self.contains(DialogueMove::Continue)
            && !text.contains("tiếp theo")
            && self.overlap > 0.0
        {
            text.push_str(&format!(
                " Mạch tiếp theo có thể xét quan hệ chồng lên giữa “{subject}” và “{object}” thay vì tách chúng thành hai vấn đề độc lập."
            ));
        }
        if self.contains(DialogueMove::Close) && !text.contains("kết luận") {
            text.push_str(" Kết luận ở lượt này được giữ theo đúng phạm vi bằng chứng hiện có.");
        }
        if self.contains(DialogueMove::Ground) && evidence.len() > 1 && !text.contains("nguồn") {
            text.push_str(&format!(" Có {} nguồn liên quan trong mạch hiện tại.", evidence.len()));
        }
        text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlaps_drive_grounded_multi_move_dialogue() {
        let weave = DuyenWeave {
            supporting_paths: 2,
            opposing_paths: 1,
            convergence_nodes: vec![1],
            shared_links: 1,
            max_depth: 3,
            overlap_score: 0.8,
        };
        let p = DialogueSynthesisPlan::build(
            Some(ImplicitDialogueGoal::Verify),
            RelationContinuity::SameSubject,
            &weave,
            0.55,
            2,
        );
        assert!(p.contains(DialogueMove::Answer));
        assert!(p.contains(DialogueMove::Ground));
        assert!(p.contains(DialogueMove::Contrast));
        assert!(p.contains(DialogueMove::Qualify));
    }

    #[test]
    fn conclusion_stays_bounded() {
        let p = DialogueSynthesisPlan::build(
            Some(ImplicitDialogueGoal::Conclude),
            RelationContinuity::Repeat,
            &DuyenWeave::default(),
            0.1,
            0,
        );
        assert!(p.contains(DialogueMove::Answer));
        assert!(p.contains(DialogueMove::Close));
        assert!(!p.contains(DialogueMove::Continue));
    }
}
