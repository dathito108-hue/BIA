#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Speaker {
    User,
    Bia,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DialogueTurn {
    pub speaker: Speaker,
    pub text: String,
    pub timestamp: u64,
}

#[derive(Clone, Debug)]
pub struct DialogueContext {
    capacity: usize,
    turns: Vec<DialogueTurn>,
}

impl DialogueContext {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            turns: Vec::new(),
        }
    }

    pub fn push(&mut self, turn: DialogueTurn) {
        if self.turns.len() >= self.capacity {
            self.turns.remove(0);
        }
        self.turns.push(turn);
    }

    pub fn recent_user_text(&self) -> Option<&str> {
        self.turns
            .iter()
            .rev()
            .find(|t| t.speaker == Speaker::User)
            .map(|t| t.text.as_str())
    }

    pub fn len(&self) -> usize {
        self.turns.len()
    }
}
