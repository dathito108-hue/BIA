use crate::world_model::{SimState, WorldModel};

const MAX_DEPTH: usize = 4;
const MAX_BRANCH: usize = 8;
const MAX_BEAM: usize = 12;

#[derive(Clone, Debug, PartialEq)]
pub struct GoalSpec {
    pub desired: Vec<u64>,
    pub avoid: Vec<u64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanCandidate {
    pub actions: Vec<u64>,
    pub final_state: SimState,
    pub score: f32,
    pub reached_goal: bool,
}

#[derive(Clone, Debug, Default)]
pub struct DeliberativePlanner;

impl DeliberativePlanner {
    pub fn plan(
        &self,
        model: &WorldModel,
        start: &SimState,
        goal: &GoalSpec,
    ) -> Option<PlanCandidate> {
        let mut beam = vec![PlanCandidate {
            actions: Vec::new(),
            final_state: start.clone(),
            score: evaluate(start, goal),
            reached_goal: reached(start, goal),
        }];
        let mut best = beam[0].clone();

        for _ in 0..MAX_DEPTH {
            let mut next = Vec::new();
            for candidate in &beam {
                if candidate.reached_goal {
                    if candidate.score > best.score {
                        best = candidate.clone();
                    }
                    continue;
                }

                for transition in model
                    .applicable(&candidate.final_state)
                    .take(MAX_BRANCH)
                {
                    if candidate.actions.contains(&transition.action) {
                        continue;
                    }
                    if let Some(state) = model.simulate(&candidate.final_state, transition.action) {
                        let mut actions = candidate.actions.clone();
                        actions.push(transition.action);
                        let goal_score = evaluate(&state, goal);
                        let score = goal_score
                            + state.value
                            + state.confidence * 0.25
                            - actions.len() as f32 * 0.04;
                        let reached_goal = reached(&state, goal);
                        let item = PlanCandidate {
                            actions,
                            final_state: state,
                            score,
                            reached_goal,
                        };
                        if item.score > best.score
                            || (item.reached_goal && !best.reached_goal)
                        {
                            best = item.clone();
                        }
                        next.push(item);
                    }
                }
            }

            if next.is_empty() {
                break;
            }
            next.sort_by(|a, b| {
                b.reached_goal
                    .cmp(&a.reached_goal)
                    .then_with(|| {
                        b.score
                            .partial_cmp(&a.score)
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
            });
            next.truncate(MAX_BEAM);
            beam = next;
            if best.reached_goal {
                break;
            }
        }

        (!best.actions.is_empty() || best.reached_goal).then_some(best)
    }
}

fn reached(state: &SimState, goal: &GoalSpec) -> bool {
    goal.desired.iter().all(|f| state.contains(*f))
        && goal.avoid.iter().all(|f| !state.contains(*f))
}

fn evaluate(state: &SimState, goal: &GoalSpec) -> f32 {
    let desired = goal
        .desired
        .iter()
        .filter(|f| state.contains(**f))
        .count() as f32;
    let avoided = goal
        .avoid
        .iter()
        .filter(|f| !state.contains(**f))
        .count() as f32;
    let bad = goal
        .avoid
        .iter()
        .filter(|f| state.contains(**f))
        .count() as f32;
    desired * 1.5 + avoided * 0.5 - bad * 2.0
}
