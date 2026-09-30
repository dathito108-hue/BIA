use crate::semantic::normalize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RepairKind { Clarify, CorrectEntity, CorrectRelation, Reanchor }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialogueRepair {
    pub kind: RepairKind,
    pub original: String,
    pub repaired: String,
    pub reason: String,
}

pub fn detect(input: &str, current: Option<&str>) -> Option<DialogueRepair> {
    if input.chars().count()>1024 { return None; }
    let s=normalize(input);
    let t=s.trim().trim_end_matches(['?','!','.','。','！','？']).trim();
    if t.is_empty() { return None; }

    if let Some(rest)=t.strip_prefix("khong phai ") {
        let corrected=rest.trim();
        if corrected.is_empty() { return None; }
        return Some(DialogueRepair{
            kind:RepairKind::CorrectEntity,
            original:current.unwrap_or("").to_string(),
            repaired:corrected.to_string(),
            reason:"người dùng phủ định phần diễn giải trước và cung cấp phần thay thế".into()
        });
    }

    if ["y toi la","y minh la","toi muon noi","khong, y la","khong phai y do"]
        .iter().any(|p|t.starts_with(p)) {
        let repaired=t.split_once(' ').map(|(_,r)|r.trim()).unwrap_or(t);
        return Some(DialogueRepair{
            kind:RepairKind::Clarify,
            original:current.unwrap_or("").to_string(),
            repaired:repaired.to_string(),
            reason:"người dùng báo hiệu ý định sửa hoặc làm rõ".into()
        });
    }

    if ["ban hieu sai","hieu sai roi","khong dung y toi","sai y roi"]
        .iter().any(|p| t.starts_with(p)) {
        return Some(DialogueRepair{
            kind:RepairKind::Clarify,
            original:current.unwrap_or("").to_string(),
            repaired:String::new(),
            reason:"người dùng xác nhận diễn giải hiện tại không đúng".into()
        });
    }
    None
}

pub fn repair_ack(repair:&DialogueRepair)->String {
    match repair.kind {
        RepairKind::Clarify | RepairKind::CorrectEntity =>
            if repair.repaired.is_empty() {
                "Tôi đã nhận ra điểm lệch; hãy nêu lại phần bạn muốn tôi hiểu.".into()
            } else {
                format!("Tôi sửa mạch hiểu theo phần bạn vừa làm rõ: {}.", repair.repaired)
            },
        RepairKind::CorrectRelation =>
            format!("Tôi neo lại quan hệ theo phần sửa: {}.",repair.repaired),
        RepairKind::Reanchor =>
            "Tôi đã neo lại mạch hội thoại theo thông tin mới.".into(),
    }
}

#[cfg(test)]
mod tests {
 use super::*;
 #[test] fn detects_explicit_repair(){let r=detect("Bạn hiểu sai rồi",Some("mua gay duong tron")).unwrap();assert_eq!(r.kind,RepairKind::Clarify);}
 #[test] fn detects_replacement(){let r=detect("Không phải gió",Some("mưa")).unwrap();assert_eq!(r.kind,RepairKind::CorrectEntity);assert_eq!(r.repaired,"gio");}
 #[test] fn ignores_empty_and_long_input(){assert!(detect("",None).is_none());assert!(detect(&"x".repeat(1025),None).is_none());}
}
