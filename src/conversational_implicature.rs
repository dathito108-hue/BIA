use crate::semantic::normalize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConversationAct {
    Acknowledge,
    Confirm,
    Doubt,
    Expand,
    NewOnly,
}

pub fn parse(input: &str) -> Option<ConversationAct> {
    if input.chars().count() > 1024 {
        return None;
    }
    let normalized = normalize(input);
    let bare = normalized
        .trim()
        .trim_end_matches(['?', '!', '.'])
        .trim();

    match bare {
        "hieu roi" | "toi hieu roi" | "ok hieu roi" | "duoc toi hieu" => {
            Some(ConversationAct::Acknowledge)
        }
        "vay la dung" | "vay la dung roi" | "dung chu" | "dung vay khong"
        | "co phai vay khong" => Some(ConversationAct::Confirm),
        "co chac khong" | "chac khong" | "that khong" | "co chac vay khong"
        | "co dang tin khong" => Some(ConversationAct::Doubt),
        "noi them" | "phan tich them" | "dao sau hon" | "giai thich sau hon"
        | "con gi nua" => Some(ConversationAct::Expand),
        "chi noi phan moi" | "phan moi thoi" | "chi noi diem moi" | "co gi moi"
        | "khac gi so voi truoc" => Some(ConversationAct::NewOnly),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_common_conversational_implicatures() {
        assert_eq!(parse("Hiểu rồi"), Some(ConversationAct::Acknowledge));
        assert_eq!(parse("Đúng chứ?"), Some(ConversationAct::Confirm));
        assert_eq!(parse("Có chắc không?"), Some(ConversationAct::Doubt));
        assert_eq!(parse("Nói thêm"), Some(ConversationAct::Expand));
        assert_eq!(parse("Chỉ nói phần mới"), Some(ConversationAct::NewOnly));
    }
}
