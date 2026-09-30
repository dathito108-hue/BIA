use crate::semantic::normalize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscourseTurn {
    pub question: String,
    pub subject: String,
    pub object: String,
}

#[derive(Clone, Debug, Default)]
pub struct ConversationContinuity {
    turns: Vec<DiscourseTurn>,
}

impl ConversationContinuity {
    pub fn remember(&mut self, question: &str, subject: &str, object: &str) {
        self.turns.push(DiscourseTurn {
            question: question.to_string(),
            subject: subject.to_string(),
            object: object.to_string(),
        });
        if self.turns.len() > 6 {
            let overflow = self.turns.len() - 6;
            self.turns.drain(0..overflow);
        }
    }

    pub fn last(&self) -> Option<&DiscourseTurn> {
        self.turns.last()
    }

    pub fn resolve_reference(&self, input: &str) -> Option<String> {
        let s = normalize(input);
        let last = self.last()?;

        if matches!(
            s.as_str(),
            "y truoc" | "y vua roi" | "truong hop truoc" | "truong hop vua roi"
        ) {
            return Some(last.question.clone());
        }

        if matches!(s.as_str(), "cai thu hai" | "doi tuong thu hai") {
            return Some(last.object.clone());
        }

        if matches!(s.as_str(), "cai thu nhat" | "doi tuong thu nhat") {
            return Some(last.subject.clone());
        }

        None
    }

    pub fn len(&self) -> usize {
        self.turns.len()
    }

    pub fn is_empty(&self) -> bool {
        self.turns.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remembers_bounded_turns_and_resolves_ordinals() {
        let mut c = ConversationContinuity::default();
        c.remember("mua co gay ra duong uot khong", "mua", "duong uot");
        assert_eq!(c.resolve_reference("cái thứ nhất").as_deref(), Some("mua"));
        assert_eq!(c.resolve_reference("cái thứ hai").as_deref(), Some("duong uot"));
        assert_eq!(
            c.resolve_reference("ý vừa rồi").as_deref(),
            Some("mua co gay ra duong uot khong")
        );
    }
}
