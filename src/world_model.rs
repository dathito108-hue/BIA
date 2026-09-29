const MAX_FACTS: usize = 64;
const MAX_TRANSITIONS: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub struct SimState {
    pub facts: Vec<u64>,
    pub value: f32,
    pub confidence: f32,
}

impl SimState {
    pub fn new(facts: impl IntoIterator<Item = u64>) -> Self {
        let mut facts: Vec<u64> = facts.into_iter().take(MAX_FACTS).collect();
        facts.sort_unstable();
        facts.dedup();
        Self {
            facts,
            value: 0.0,
            confidence: 1.0,
        }
    }

    pub fn contains(&self, fact: u64) -> bool {
        self.facts.binary_search(&fact).is_ok()
    }

    fn insert(&mut self, fact: u64) {
        if self.contains(fact) || self.facts.len() >= MAX_FACTS {
            return;
        }
        self.facts.push(fact);
        self.facts.sort_unstable();
    }

    fn remove(&mut self, fact: u64) {
        if let Ok(i) = self.facts.binary_search(&fact) {
            self.facts.remove(i);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransitionModel {
    pub action: u64,
    pub requires: Vec<u64>,
    pub adds: Vec<u64>,
    pub removes: Vec<u64>,
    pub utility: f32,
    pub cost: f32,
    pub confidence: f32,
}

#[derive(Clone, Debug, Default)]
pub struct WorldModel {
    transitions: Vec<TransitionModel>,
}

impl WorldModel {
    pub fn add_transition(&mut self, transition: TransitionModel) {
        if self.transitions.len() >= MAX_TRANSITIONS {
            let idx = self
                .transitions
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    a.confidence
                        .partial_cmp(&b.confidence)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.transitions.remove(idx);
        }
        self.transitions.push(transition);
    }

    pub fn applicable<'a>(
        &'a self,
        state: &'a SimState,
    ) -> impl Iterator<Item = &'a TransitionModel> {
        self.transitions
            .iter()
            .filter(move |t| t.requires.iter().all(|r| state.contains(*r)))
    }

    pub fn simulate(&self, state: &SimState, action: u64) -> Option<SimState> {
        let transition = self
            .transitions
            .iter()
            .filter(|t| t.action == action)
            .filter(|t| t.requires.iter().all(|r| state.contains(*r)))
            .max_by(|a, b| {
                a.confidence
                    .partial_cmp(&b.confidence)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?;

        let mut next = state.clone();
        for fact in transition.removes.iter().take(16) {
            next.remove(*fact);
        }
        for fact in transition.adds.iter().take(16) {
            next.insert(*fact);
        }
        next.value += transition.utility - transition.cost;
        next.confidence *= transition.confidence.clamp(0.0, 1.0);
        Some(next)
    }

    pub fn transitions(&self) -> &[TransitionModel] {
        &self.transitions
    }

    pub fn len(&self) -> usize {
        self.transitions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transitions.is_empty()
    }
}
