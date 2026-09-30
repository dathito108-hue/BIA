use crate::semantic::normalize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogueThreadMove {
    ExplainCause,
    GroundEvidence,
    Compare,
    Continue,
    Counterfactual,
    Return,
    Clarify,
}

pub fn parse(input: &str) -> Option<DialogueThreadMove> {
    if input.chars().count() > 1024 {
        return None;
    }
    let s = normalize(input).trim().trim_end_matches(['?', '!', '.']).trim().to_string();

    if s.starts_with("tai sao ") || s.starts_with("vi sao ") || s == "tai sao" || s == "vi sao" {
        return Some(DialogueThreadMove::ExplainCause);
    }
    if s.contains("dua vao dau") || s.contains("nguon nao") || s.contains("bang chung nao")
        || s == "can cu vao dau" {
        return Some(DialogueThreadMove::GroundEvidence);
    }
    if s.contains("so voi") || s.contains("khac gi") || s.starts_with("con truong hop ") {
        return Some(DialogueThreadMove::Compare);
    }
    if s.starts_with("neu ") || s.starts_with("gia su ") || s.starts_with("trong truong hop nguoc lai") {
        return Some(DialogueThreadMove::Counterfactual);
    }
    if s == "quay lai" || s == "tro lai" || s == "xet lai cai truoc" {
        return Some(DialogueThreadMove::Return);
    }
    if s.starts_with("tiep ") || s == "tiep theo" || s == "con gi nua" || s == "roi sao" {
        return Some(DialogueThreadMove::Continue);
    }
    if s.starts_with("y la ") || s.starts_with("nghia la ") || s.starts_with("ban dang noi ") {
        return Some(DialogueThreadMove::Clarify);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_natural_follow_up_forms() {
        assert_eq!(parse("Tại sao?"), Some(DialogueThreadMove::ExplainCause));
        assert_eq!(parse("Bạn dựa vào đâu?"), Some(DialogueThreadMove::GroundEvidence));
        assert_eq!(parse("So với trường hợp trước thì sao?"), Some(DialogueThreadMove::Compare));
        assert_eq!(parse("Nếu độ ẩm giảm thì sao?"), Some(DialogueThreadMove::Counterfactual));
        assert_eq!(parse("Tiếp theo?"), Some(DialogueThreadMove::Continue));
        assert_eq!(parse("Quay lại"), Some(DialogueThreadMove::Return));
    }
}
