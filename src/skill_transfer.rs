use crate::world_model::{TransitionModel, WorldModel};

#[derive(Clone, Debug, PartialEq)]
pub struct SkillPattern {
    pub length: usize,
    pub utility_signs: Vec<i8>,
    pub effect_counts: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct CrossDomainTransfer;

impl CrossDomainTransfer {
    pub fn extract(&self, model: &WorldModel, actions: &[u64]) -> Option<SkillPattern> {
        if actions.is_empty() || actions.len() > 4 {
            return None;
        }
        let mut utility_signs = Vec::new();
        let mut effect_counts = Vec::new();
        for action in actions {
            let t = model.transitions().iter().find(|t| t.action == *action)?;
            utility_signs.push(sign(t.utility - t.cost));
            effect_counts.push(t.adds.len().min(8) as u8);
        }
        Some(SkillPattern {
            length: actions.len(),
            utility_signs,
            effect_counts,
        })
    }

    pub fn match_actions(
        &self,
        model: &WorldModel,
        pattern: &SkillPattern,
    ) -> Option<Vec<u64>> {
        let candidates: Vec<&TransitionModel> = model.transitions().iter().collect();
        if pattern.length == 0 || candidates.len() < pattern.length {
            return None;
        }

        let mut chosen = Vec::new();
        let mut used = Vec::new();
        for i in 0..pattern.length {
            let target_sign = pattern.utility_signs[i];
            let target_effects = pattern.effect_counts[i];
            let best = candidates
                .iter()
                .filter(|t| !used.contains(&t.action))
                .min_by_key(|t| {
                    let s = sign(t.utility - t.cost);
                    let sign_penalty = if s == target_sign { 0 } else { 10 };
                    sign_penalty + (t.adds.len() as i32 - target_effects as i32).unsigned_abs()
                })?;
            chosen.push(best.action);
            used.push(best.action);
        }
        Some(chosen)
    }
}

fn sign(v: f32) -> i8 {
    if v > 0.05 {
        1
    } else if v < -0.05 {
        -1
    } else {
        0
    }
}
