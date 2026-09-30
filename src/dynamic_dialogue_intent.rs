use crate::semantic::normalize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DynamicIntent {
    Acknowledge,
    Confirm,
    Doubt,
    Expand,
    NewOnly,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DynamicDialoguePlan {
    pub intents: Vec<DynamicIntent>,
}

impl DynamicDialoguePlan {
    pub fn parse(input: &str) -> Option<Self> {
        if input.chars().count() > 1024 {
            return None;
        }
        let s = normalize(input);
        let mut intents = Vec::new();

        if contains_any(
            &s,
            &["hieu roi", "toi hieu roi", "ok hieu roi", "duoc toi hieu"],
        ) {
            intents.push(DynamicIntent::Acknowledge);
        }
        if contains_any(
            &s,
            &[
                "vay la dung",
                "vay la dung roi",
                "dung chu",
                "dung vay khong",
                "co phai vay khong",
            ],
        ) {
            intents.push(DynamicIntent::Confirm);
        }
        if contains_any(
            &s,
            &[
                "co chac khong",
                "chac khong",
                "that khong",
                "co chac vay khong",
                "co dang tin khong",
            ],
        ) {
            intents.push(DynamicIntent::Doubt);
        }
        if contains_any(
            &s,
            &[
                "noi them",
                "phan tich them",
                "dao sau hon",
                "giai thich sau hon",
                "con gi nua",
            ],
        ) {
            intents.push(DynamicIntent::Expand);
        }
        if contains_any(
            &s,
            &[
                "chi noi phan moi",
                "phan moi thoi",
                "chi noi diem moi",
                "co gi moi",
                "khac gi so voi truoc",
            ],
        ) {
            intents.push(DynamicIntent::NewOnly);
        }

        intents.dedup();
        if intents.contains(&DynamicIntent::Doubt) {
            intents.retain(|intent| *intent != DynamicIntent::Confirm);
        }

        (intents.len() >= 2).then_some(Self { intents })
    }

    pub fn contains(&self, intent: DynamicIntent) -> bool {
        self.intents.contains(&intent)
    }

    pub fn wants_deep_review(&self) -> bool {
        self.contains(DynamicIntent::Doubt) || self.contains(DynamicIntent::Expand)
    }
}

fn contains_any(s: &str, needles: &[&str]) -> bool {
    needles.iter().any(|needle| s.contains(needle))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn composes_acknowledge_doubt_and_new_only() {
        let p = DynamicDialoguePlan::parse(
            "Tôi hiểu rồi, nhưng có chắc không, nếu có gì mới thì chỉ nói phần mới thôi",
        )
        .expect("plan");
        assert!(p.contains(DynamicIntent::Acknowledge));
        assert!(p.contains(DynamicIntent::Doubt));
        assert!(p.contains(DynamicIntent::NewOnly));
        assert!(p.wants_deep_review());
    }

    #[test]
    fn doubt_supersedes_confirmation_but_keeps_other_intents() {
        let p = DynamicDialoguePlan::parse(
            "Đúng chứ, nhưng có chắc không, nói thêm một chút",
        )
        .expect("plan");
        assert!(!p.contains(DynamicIntent::Confirm));
        assert!(p.contains(DynamicIntent::Doubt));
        assert!(p.contains(DynamicIntent::Expand));
    }

    #[test]
    fn single_intent_is_left_to_existing_v159_path() {
        assert!(DynamicDialoguePlan::parse("Có chắc không?").is_none());
        assert!(DynamicDialoguePlan::parse("Hiểu rồi").is_none());
    }
}
