use crate::dialogue_goal_state::ImplicitDialogueGoal;
use crate::conversation_continuity::RelationContinuity;
use crate::duyen_weave::DuyenWeave;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogueMove {
    Answer, Ground, Contrast, Qualify, Continue, Close, Clarify, Invite,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResponseDepth { Brief, Standard, Deep }

#[derive(Clone, Debug, PartialEq)]
pub struct DialogueSynthesisPlan {
    pub moves: Vec<DialogueMove>,
    pub uncertainty: f32,
    pub overlap: f32,
    pub depth: ResponseDepth,
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
        if evidence_count > 0 || weave.max_depth > 0 { moves.push(DialogueMove::Ground); }
        if weave.opposing_paths > 0 { moves.push(DialogueMove::Contrast); }
        if uncertainty >= 0.25 || matches!(goal, Some(ImplicitDialogueGoal::Verify)) {
            moves.push(DialogueMove::Qualify);
        }
        if matches!(goal, Some(ImplicitDialogueGoal::Explore))
            && !matches!(continuity, RelationContinuity::Repeat) { moves.push(DialogueMove::Continue); }
        if matches!(goal, Some(ImplicitDialogueGoal::Conclude)) { moves.push(DialogueMove::Close); }
        if matches!(goal, Some(ImplicitDialogueGoal::Challenge)) { moves.push(DialogueMove::Clarify); }
        if matches!(goal, Some(ImplicitDialogueGoal::Explore))
            && weave.overlap_score > 0.0 { moves.push(DialogueMove::Invite); }
        moves.dedup();
        moves.truncate(6);
        let depth = if matches!(goal, Some(ImplicitDialogueGoal::Conclude))
            || matches!(continuity, RelationContinuity::Repeat) { ResponseDepth::Brief }
            else if weave.max_depth >= 3 || weave.overlap_score >= 0.5
                || uncertainty >= 0.45 { ResponseDepth::Deep }
            else { ResponseDepth::Standard };
        Self { moves, uncertainty, overlap: weave.overlap_score, depth }
    }

    pub fn contains(&self, mv: DialogueMove) -> bool { self.moves.contains(&mv) }

    pub fn render(
        &self,
        mut text: String,
        subject: &str,
        object: &str,
        evidence: &[String],
    ) -> String {
        if self.contains(DialogueMove::Contrast) && !text.contains("phản") {
            text.push_str(" Có cả nhánh ủng hộ và phản đối nên tôi giữ hai chiều thay vì ép thành một kết luận tuyệt đối.");
        }
        if self.contains(DialogueMove::Qualify) && self.uncertainty >= 0.45
            && !text.contains("chưa chắc") && !text.contains("chưa đủ") {
            text.push_str(" Độ chắc hiện tại còn hạn chế; phần này được giữ theo bằng chứng đang có.");
        }
        if self.contains(DialogueMove::Continue) && self.overlap > 0.0 && !text.contains("tiếp theo") {
            text.push_str(&format!(" Có thể tiếp tục xét quan hệ chồng lên giữa “{subject}” và “{object}”."));
        }
        if self.contains(DialogueMove::Invite) && !text.contains("nếu bạn muốn") {
            text.push_str(" Nếu bạn muốn, ta có thể xét nhánh nhân hoặc quả tiếp theo.");
        }
        if self.contains(DialogueMove::Close) && !text.contains("kết luận") {
            text.push_str(" Kết luận ở lượt này giữ đúng phạm vi bằng chứng hiện có.");
        }
        if self.contains(DialogueMove::Ground) && evidence.len() > 1 && !text.contains("nguồn") {
            text.push_str(&format!(" Có {} nguồn liên quan trong mạch hiện tại.", evidence.len()));
        }
        text
    }

    pub fn cap_for(&self, requested: crate::expression_style::ExpressionStyle) -> ResponseDepth {
        match requested {
            crate::expression_style::ExpressionStyle::Brief => ResponseDepth::Brief,
            crate::expression_style::ExpressionStyle::Deep => ResponseDepth::Deep,
            crate::expression_style::ExpressionStyle::Standard => self.depth,
        }
    }
}
