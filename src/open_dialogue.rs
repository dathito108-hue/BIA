use crate::dialogue_synthesis::{DialogueMove, DialogueSynthesisPlan, ResponseDepth};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenDialogueMove { Answer, Explain, Clarify, Relate, Challenge, Continue, Ask }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpenDialoguePlan {
    pub moves: Vec<OpenDialogueMove>,
    pub depth: ResponseDepth,
}

impl OpenDialoguePlan {
    pub fn from_synthesis(s: &DialogueSynthesisPlan) -> Self {
        let mut moves = Vec::with_capacity(6);
        for mv in &s.moves {
            let mapped = match mv {
                DialogueMove::Answer => OpenDialogueMove::Answer,
                DialogueMove::Ground | DialogueMove::Qualify => OpenDialogueMove::Explain,
                DialogueMove::Contrast | DialogueMove::Clarify => OpenDialogueMove::Challenge,
                DialogueMove::Continue => OpenDialogueMove::Continue,
                DialogueMove::Close => OpenDialogueMove::Relate,
                DialogueMove::Invite => OpenDialogueMove::Ask,
            };
            if !moves.contains(&mapped) { moves.push(mapped); }
        }
        Self { moves, depth: s.depth }
    }

    pub fn has(&self, mv: OpenDialogueMove) -> bool { self.moves.contains(&mv) }

    pub fn compose(&self, base: String, subject: &str, object: &str) -> String {
        let mut out = base;
        if self.has(OpenDialogueMove::Challenge) && !out.contains("hai chiều") {
            out.push_str(" Tôi giữ các khả năng đang cạnh tranh thay vì tự chọn một nhánh chưa đủ căn cứ.");
        }
        if self.has(OpenDialogueMove::Continue) && self.depth != ResponseDepth::Brief
            && !out.contains("có thể xét") {
            out.push_str(&format!(" Có thể xét tiếp quan hệ giữa “{subject}” và “{object}” theo nhánh nhân hoặc quả."));
        }
        if self.has(OpenDialogueMove::Ask) && self.depth == ResponseDepth::Deep
            && !out.contains("nếu bạn muốn") {
            out.push_str(" Nếu bạn muốn đi sâu hơn, hãy đưa thêm một quan hệ hoặc nguồn; tôi sẽ nối nó vào mạch hiện tại.");
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_plan_maps_multiple_dialogue_moves() {
        let s = DialogueSynthesisPlan {
            moves: vec![DialogueMove::Answer, DialogueMove::Contrast, DialogueMove::Continue, DialogueMove::Invite],
            uncertainty: 0.5, overlap: 0.8, depth: ResponseDepth::Deep,
        };
        let p = OpenDialoguePlan::from_synthesis(&s);
        assert!(p.has(OpenDialogueMove::Answer));
        assert!(p.has(OpenDialogueMove::Challenge));
        assert!(p.has(OpenDialogueMove::Continue));
        assert!(p.has(OpenDialogueMove::Ask));
    }
}
