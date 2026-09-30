use crate::semantic::normalize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImplicitDialogueGoal {
    Explore,
    Verify,
    Challenge,
    Conclude,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DialogueGoalState {
    relation: Option<String>,
    current: Option<ImplicitDialogueGoal>,
    trail: Vec<ImplicitDialogueGoal>,
}

impl DialogueGoalState {
    pub fn bind_relation(&mut self, question: &str) {
        let relation=normalize(question).trim().to_string();
        if relation.is_empty() {
            return;
        }
        if self.relation.as_deref()!=Some(relation.as_str()) {
            self.relation=Some(relation);
            self.current=Some(ImplicitDialogueGoal::Explore);
            self.trail.clear();
            self.trail.push(ImplicitDialogueGoal::Explore);
        } else if self.current.is_none() {
            self.current=Some(ImplicitDialogueGoal::Explore);
            self.push(ImplicitDialogueGoal::Explore);
        }
    }

    pub fn advance(&mut self, goal: ImplicitDialogueGoal) -> bool {
        if self.relation.is_none() {
            return false;
        }
        self.current=Some(goal);
        self.push(goal);
        true
    }

    pub fn current(&self) -> Option<ImplicitDialogueGoal> {
        self.current
    }

    pub fn has_visited(&self, goal: ImplicitDialogueGoal) -> bool {
        self.trail.contains(&goal)
    }

    pub fn relation(&self) -> Option<&str> {
        self.relation.as_deref()
    }

    pub fn clear_active(&mut self) {
        self.relation=None;
        self.current=None;
        self.trail.clear();
    }

    pub fn trail_len(&self) -> usize {
        self.trail.len()
    }

    fn push(&mut self, goal: ImplicitDialogueGoal) {
        if self.trail.last().copied()!=Some(goal) {
            self.trail.push(goal);
        }
        if self.trail.len()>8 {
            let overflow=self.trail.len()-8;
            self.trail.drain(0..overflow);
        }
    }
}

pub fn parse_goal(input: &str) -> Option<ImplicitDialogueGoal> {
    if input.chars().count()>1024 {
        return None;
    }
    let s=normalize(input);
    let bare=s.trim().trim_end_matches(['?','!','.']).trim();

    if [
        "co phan chung khong",
        "tim phan chung",
        "phan chung dau",
        "co bang chung phan doi khong",
        "co gi phan doi khong",
        "tim diem phan doi",
        "xem phan chung",
    ].contains(&bare) {
        return Some(ImplicitDialogueGoal::Challenge);
    }

    if [
        "vay ket luan the nao",
        "ket luan the nao",
        "chot lai",
        "chot lai di",
        "ket luan cuoi",
        "chot ket luan",
        "vay chot lai di",
    ].iter().any(|x|bare==*x) {
        return Some(ImplicitDialogueGoal::Conclude);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_bounded_goal_transitions_for_one_relation() {
        let mut state=DialogueGoalState::default();
        state.bind_relation("mua co gay ra duong tron khong");
        assert_eq!(state.current(),Some(ImplicitDialogueGoal::Explore));
        assert!(state.advance(ImplicitDialogueGoal::Verify));
        assert!(state.advance(ImplicitDialogueGoal::Challenge));
        assert!(state.has_visited(ImplicitDialogueGoal::Verify));
        assert!(state.has_visited(ImplicitDialogueGoal::Challenge));
        for _ in 0..12 {
            state.advance(ImplicitDialogueGoal::Explore);
            state.advance(ImplicitDialogueGoal::Verify);
        }
        assert!(state.trail_len()<=8);
    }

    #[test]
    fn new_relation_reanchors_the_goal_state() {
        let mut state=DialogueGoalState::default();
        state.bind_relation("mua co gay ra duong tron khong");
        state.advance(ImplicitDialogueGoal::Challenge);
        state.bind_relation("gio co gay ra song khong");
        assert_eq!(state.current(),Some(ImplicitDialogueGoal::Explore));
        assert!(!state.has_visited(ImplicitDialogueGoal::Challenge));
        assert_eq!(state.relation(),Some("gio co gay ra song khong"));
    }

    #[test]
    fn parses_implicit_challenge_and_conclusion_goals() {
        assert_eq!(parse_goal("Có phản chứng không?"),Some(ImplicitDialogueGoal::Challenge));
        assert_eq!(parse_goal("Vậy kết luận thế nào?"),Some(ImplicitDialogueGoal::Conclude));
    }
}
