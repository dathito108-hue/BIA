use crate::semantic::normalize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DialogueGoal {
    ExplainCurrent,
    ComparePrevious,
    Summarize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompositeDialoguePlan {
    pub goals: Vec<DialogueGoal>,
}

impl CompositeDialoguePlan {
    pub fn parse(input: &str) -> Option<Self> {
        let s = normalize(input);
        let mut goals = Vec::new();

        if s.contains("giai thich") || s.contains("noi ro") || s.contains("phan tich") {
            goals.push(DialogueGoal::ExplainCurrent);
        }
        if s.contains("so sanh") && (s.contains("truong hop truoc") || s.contains("y truoc") || s.contains("truong hop kia")) {
            goals.push(DialogueGoal::ComparePrevious);
        }
        if s.contains("tom tat") || s.contains("noi ngan gon") || s.contains("rut gon") {
            goals.push(DialogueGoal::Summarize);
        }

        goals.dedup();
        if goals.len() >= 2 {
            Some(Self { goals })
        } else {
            None
        }
    }

    pub fn contains(&self, goal: DialogueGoal) -> bool {
        self.goals.contains(&goal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_bounded_multi_goal_request() {
        let p = CompositeDialoguePlan::parse(
            "Giải thích rõ, so sánh với trường hợp trước rồi tóm tắt ngắn gọn",
        )
        .expect("plan");
        assert!(p.contains(DialogueGoal::ExplainCurrent));
        assert!(p.contains(DialogueGoal::ComparePrevious));
        assert!(p.contains(DialogueGoal::Summarize));
    }

    #[test]
    fn ignores_single_goal_request() {
        assert!(CompositeDialoguePlan::parse("Giải thích kỹ hơn").is_none());
    }
}
