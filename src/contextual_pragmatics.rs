use crate::semantic::normalize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PragmaticMove {
    EllipticEntity(String),
    TopicShift(String),
}

pub fn parse(input: &str) -> Option<PragmaticMove> {
    if input.chars().count() > 1024 {
        return None;
    }
    let normalized = normalize(input);
    let bare = normalized
        .trim()
        .trim_end_matches(['?', '!', '.'])
        .trim();

    for prefix in [
        "chuyen chu de sang ",
        "doi chu de sang ",
        "chuyen sang chu de ",
        "gio chuyen sang ",
    ] {
        if let Some(topic) = bare.strip_prefix(prefix) {
            let topic = topic.trim();
            if valid_fragment(topic) {
                return Some(PragmaticMove::TopicShift(topic.to_string()));
            }
        }
    }

    for prefix in ["the con ", "gio con ", "con "] {
        if let Some(rest) = bare.strip_prefix(prefix) {
            let entity = rest.strip_suffix(" thi sao").unwrap_or(rest).trim();
            if valid_entity(entity) {
                return Some(PragmaticMove::EllipticEntity(entity.to_string()));
            }
        }
    }

    None
}

fn valid_fragment(text: &str) -> bool {
    !text.is_empty()
        && text.chars().count() <= 80
        && !text.contains([':', ';'])
}

fn valid_entity(text: &str) -> bool {
    valid_fragment(text)
        && text.split_whitespace().count() <= 8
        && !matches!(
            text,
            "gi"
                | "gi nua"
                | "sao"
                | "the nao"
                | "cai nay"
                | "cai do"
                | "dieu nay"
                | "dieu do"
                | "no"
        )
        && !text.contains("nguyen nhan ")
        && !text.contains("ket qua ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_bounded_ellipsis_and_topic_shift() {
        assert_eq!(
            parse("Thế còn bùn thì sao?"),
            Some(PragmaticMove::EllipticEntity("bun".into()))
        );
        assert_eq!(
            parse("Còn gió?"),
            Some(PragmaticMove::EllipticEntity("gio".into()))
        );
        assert_eq!(
            parse("Chuyển chủ đề sang âm nhạc"),
            Some(PragmaticMove::TopicShift("am nhac".into()))
        );
    }

    #[test]
    fn ambiguous_pronouns_are_not_resolved_here() {
        assert_eq!(parse("Còn nó thì sao?"), None);
        assert_eq!(parse("Thế còn cái đó?"), None);
    }
}
