use crate::semantic::normalize;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscourseTurn {
    pub question: String,
    pub subject: String,
    pub object: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelationContinuity {
    New,
    Repeat,
    SameSubject,
    SameObject,
    Return,
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
        if self.turns.len() > 12 {
            let overflow = self.turns.len() - 12;
            self.turns.drain(0..overflow);
        }
    }

    pub fn last(&self) -> Option<&DiscourseTurn> {
        self.turns.last()
    }

    pub fn previous(&self) -> Option<&DiscourseTurn> {
        self.turns.len().checked_sub(2).and_then(|i| self.turns.get(i))
    }

    pub fn relation_continuity(&self, subject: &str, object: &str) -> RelationContinuity {
        let subject = normalize(subject);
        let object = normalize(object);
        let Some(last) = self.last() else {
            return RelationContinuity::New;
        };

        if last.subject == subject && last.object == object {
            return RelationContinuity::Repeat;
        }
        if last.subject == subject {
            return RelationContinuity::SameSubject;
        }
        if last.object == object {
            return RelationContinuity::SameObject;
        }
        if self
            .turns
            .iter()
            .rev()
            .skip(1)
            .any(|turn| turn.subject == subject && turn.object == object)
        {
            return RelationContinuity::Return;
        }
        RelationContinuity::New
    }

    pub fn relation_seen(&self, subject: &str, object: &str) -> bool {
        let subject = normalize(subject);
        let object = normalize(object);
        self.turns
            .iter()
            .any(|turn| turn.subject == subject && turn.object == object)
    }

    pub fn role_flags(&self, entity: &str) -> (bool, bool) {
        let entity = normalize(entity);
        let as_subject = self.turns.iter().any(|turn| turn.subject == entity);
        let as_object = self.turns.iter().any(|turn| turn.object == entity);
        (as_subject, as_object)
    }

    pub fn correct_last_entity(&mut self, replacement: &str, rejected: &str) -> Option<String> {
        let replacement = normalize(replacement).trim().to_string();
        let rejected = normalize(rejected).trim().to_string();
        if replacement.is_empty() || rejected.is_empty() || replacement == rejected {
            return None;
        }
        let last = self.turns.last_mut()?;
        if last.subject == rejected {
            last.subject = replacement.clone();
            last.question = format!("{} co gay ra {} khong", replacement, last.object);
            return Some(last.question.clone());
        }
        if last.object == rejected {
            last.object = replacement.clone();
            last.question = format!("{} co gay ra {} khong", last.subject, replacement);
            return Some(last.question.clone());
        }
        None
    }

    pub fn resolve_reference(&self, input: &str) -> Option<String> {
        let s = normalize(input);
        let last = self.last()?;

        if matches!(s.as_str(), "y vua roi" | "truong hop vua roi") {
            return Some(last.question.clone());
        }
        if matches!(s.as_str(), "y truoc" | "truong hop truoc" | "truong hop kia") {
            return self.previous().map(|turn| turn.question.clone()).or_else(|| Some(last.question.clone()));
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
        assert_eq!(
            c.correct_last_entity("duong tron", "duong uot").as_deref(),
            Some("mua co gay ra duong tron khong")
        );
    }

    #[test]
    fn classifies_relation_continuity_across_longer_context() {
        let mut c = ConversationContinuity::default();
        c.remember("a co gay ra b khong", "a", "b");
        assert_eq!(c.relation_continuity("a", "b"), RelationContinuity::Repeat);
        assert_eq!(c.relation_continuity("a", "c"), RelationContinuity::SameSubject);
        assert_eq!(c.relation_continuity("d", "b"), RelationContinuity::SameObject);
        c.remember("x co gay ra y khong", "x", "y");
        assert_eq!(c.relation_continuity("a", "b"), RelationContinuity::Return);
        assert!(c.relation_seen("a", "b"));
        assert_eq!(c.role_flags("a"), (true, false));
        assert_eq!(c.role_flags("b"), (false, true));

        for i in 0..20 {
            c.remember(&format!("q{i}"), &format!("s{i}"), &format!("o{i}"));
        }
        assert!(c.len() <= 12);
    }
}
