use crate::semantic::{normalize, VietnameseSemanticParser};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroundingMove {
    CorrectRelation { question: String },
    ReplaceSubject { rejected: String, replacement: String },
    ReplaceObject { rejected: String, replacement: String },
    Clarify { prompt: String },
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct GroundingState {
    repairs: u8,
    unresolved: bool,
}

impl GroundingState {
    pub fn note_repair(&mut self) {
        self.repairs = self.repairs.saturating_add(1);
        self.unresolved = false;
    }

    pub fn note_unresolved(&mut self) {
        self.unresolved = true;
    }

    pub fn repairs(&self) -> u8 {
        self.repairs
    }

    pub fn unresolved(&self) -> bool {
        self.unresolved
    }

    pub fn clear_unresolved(&mut self) {
        self.unresolved = false;
    }
}

pub fn parse(input: &str, current_question: Option<&str>) -> Option<GroundingMove> {
    if input.chars().count() > 1024 {
        return None;
    }

    let normalized = normalize(input);
    let bare = normalized
        .trim()
        .trim_end_matches(['?', '!', '.'])
        .trim();

    for prefix in [
        "khong, y toi la ",
        "khong y toi la ",
        "khong, y minh la ",
        "khong y minh la ",
        "y toi la ",
        "y minh la ",
        "toi muon noi la ",
        "toi dang noi ve ",
    ] {
        if let Some(rest) = bare.strip_prefix(prefix) {
            if let Some(question) = relation_question(rest) {
                return Some(GroundingMove::CorrectRelation { question });
            }
        }
    }

    let current = current_question?;
    let scene = VietnameseSemanticParser.parse(current);
    let query = scene.query?;
    let subject = normalize(&query.subject.text);
    let object = normalize(&query.object.text);

    for marker in ["khong phai ", "khong phai la "] {
        if let Some(rest) = bare.strip_prefix(marker) {
            if let Some((rejected, replacement)) = split_correction(rest) {
                return role_correction(&subject, &object, &rejected, &replacement);
            }
        }
    }

    for marker in [
        "khong phai ",
        "toi hoi ",
        "toi noi ",
        "toi dang noi ",
    ] {
        if let Some(rest) = bare.strip_prefix(marker) {
            if let Some((rejected, replacement)) = split_chu_y(rest) {
                return role_correction(&subject, &object, &rejected, &replacement);
            }
        }
    }

    None
}

fn relation_question(text: &str) -> Option<String> {
    let candidate = text.trim();
    if candidate.is_empty() || candidate.chars().count() > 240 {
        return None;
    }

    let parsed = VietnameseSemanticParser.parse(candidate);
    if let Some(query) = parsed.query {
        return Some(format!(
            "{} co gay ra {} khong",
            query.subject.text, query.object.text
        ));
    }

    if parsed.clauses.len() == 1 {
        let clause = &parsed.clauses[0];
        return Some(format!(
            "{} co gay ra {} khong",
            clause.subject.text, clause.object.text
        ));
    }

    None
}

fn split_correction(rest: &str) -> Option<(String, String)> {
    for marker in [", ma la ", ", ma ", ", la ", " ma la ", " ma "] {
        if let Some((left, right)) = rest.split_once(marker) {
            let rejected = clean_entity(left)?;
            let replacement = clean_entity(right)?;
            return Some((rejected, replacement));
        }
    }
    None
}

fn split_chu_y(rest: &str) -> Option<(String, String)> {
    for marker in [" chu khong phai ", " chu khong phai la ", " thay vi "] {
        if let Some((replacement, rejected)) = rest.split_once(marker) {
            let replacement = clean_entity(replacement)?;
            let rejected = clean_entity(rejected)?;
            return Some((rejected, replacement));
        }
    }
    None
}

fn clean_entity(text: &str) -> Option<String> {
    let mut value = text.trim().trim_matches([',', ' ']);
    for prefix in ["nguyen nhan ", "ket qua ", "doi tuong "] {
        if let Some(rest) = value.strip_prefix(prefix) {
            value = rest.trim().trim_matches([',', ' ']);
            break;
        }
    }
    if value.is_empty()
        || value.chars().count() > 80
        || value.contains([':', ';', '?'])
        || matches!(value, "gi" | "sao" | "the nao")
    {
        return None;
    }
    Some(value.to_string())
}

fn role_correction(
    subject: &str,
    object: &str,
    rejected: &str,
    replacement: &str,
) -> Option<GroundingMove> {
    let subject_match = normalize(rejected) == subject;
    let object_match = normalize(rejected) == object;

    match (subject_match, object_match) {
        (true, false) => Some(GroundingMove::ReplaceSubject {
            rejected: rejected.to_string(),
            replacement: replacement.to_string(),
        }),
        (false, true) => Some(GroundingMove::ReplaceObject {
            rejected: rejected.to_string(),
            replacement: replacement.to_string(),
        }),
        (true, true) => Some(GroundingMove::Clarify {
            prompt: format!(
                "“{rejected}” đang xuất hiện ở cả hai vai trong quan hệ hiện tại. Bạn muốn thay ở vai nguyên nhân hay kết quả?"
            ),
        }),
        (false, false) => Some(GroundingMove::Clarify {
            prompt: format!(
                "Tôi hiểu bạn đang sửa “{rejected}”, nhưng nó chưa khớp rõ với nguyên nhân “{subject}” hay kết quả “{object}”. Bạn muốn sửa bên nào?"
            ),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_explicit_relation_repair() {
        assert_eq!(
            parse(
                "Không, ý tôi là gió gây ra sóng.",
                Some("mưa co gay ra duong tron khong")
            ),
            Some(GroundingMove::CorrectRelation {
                question: "gio co gay ra song khong".into()
            })
        );
    }

    #[test]
    fn replaces_subject_from_explicit_correction() {
        assert_eq!(
            parse(
                "Không phải mưa mà gió.",
                Some("mưa co gay ra duong tron khong")
            ),
            Some(GroundingMove::ReplaceSubject {
                rejected: "mua".into(),
                replacement: "gio".into()
            })
        );
    }

    #[test]
    fn replaces_object_and_preserves_role() {
        assert_eq!(
            parse(
                "Không phải đường ướt, mà là đường trơn.",
                Some("mưa co gay ra duong uot khong")
            ),
            Some(GroundingMove::ReplaceObject {
                rejected: "duong uot".into(),
                replacement: "duong tron".into()
            })
        );
    }

    #[test]
    fn parses_repair_from_chu_khong_phai_form() {
        assert_eq!(
            parse(
                "Tôi hỏi gió chứ không phải mưa.",
                Some("mưa co gay ra duong tron khong")
            ),
            Some(GroundingMove::ReplaceSubject {
                rejected: "mua".into(),
                replacement: "gio".into()
            })
        );
    }

    #[test]
    fn asks_for_role_when_repair_is_not_grounded() {
        let move_ = parse(
            "Không phải X mà Y.",
            Some("mưa co gay ra duong uot khong"),
        );
        assert!(matches!(move_, Some(GroundingMove::Clarify { .. })));
    }

    #[test]
    fn bounded_state_records_repair_without_creating_knowledge() {
        let mut state = GroundingState::default();
        assert_eq!(state.repairs(), 0);
        state.note_repair();
        state.note_unresolved();
        assert_eq!(state.repairs(), 1);
        assert!(state.unresolved());
        state.clear_unresolved();
        assert!(!state.unresolved());
    }
}
