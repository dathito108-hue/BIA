use crate::types::{Intention, Relation, RelationKind};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanStep {
    pub action:u32,
    pub target:Option<u64>,
    pub confidence:f32,
    pub reversible:bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    pub steps:Vec<PlanStep>,
    pub confidence:f32,
    pub requires_observation:bool,
}

#[derive(Clone, Debug)]
pub struct DeepQuan {
    max_depth:usize,
    branch_limit:usize,
}

impl DeepQuan {
    pub fn new(max_depth:usize, branch_limit:usize)->Self{
        assert!(max_depth>0 && branch_limit>0);
        Self{max_depth,branch_limit}
    }

    pub fn plan(&self, goal:u32, target:Option<u64>, relations:&[Relation], uncertainty:f32)->Plan{
        let mut candidates:Vec<(f32,u32,Option<u64>)>=relations.iter()
            .filter(|r|matches!(r.kind,RelationKind::Enables|RelationKind::Causes|RelationKind::GoalRelevant))
            .map(|r|(r.strength*r.confidence, goal ^ (r.from as u32).rotate_left(3), Some(r.to)))
            .collect();
        candidates.sort_by(|a,b|b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        candidates.truncate(self.branch_limit);

        let mut steps=Vec::new();
        for (score,action,tgt) in candidates.into_iter().take(self.max_depth.saturating_sub(1)){
            steps.push(PlanStep{action,target:tgt,confidence:score,reversible:score<0.92});
        }
        steps.push(PlanStep{action:goal,target,confidence:(1.0-uncertainty).clamp(0.0,1.0),reversible:uncertainty>0.1});
        let confidence=if steps.is_empty(){0.0}else{steps.iter().map(|s|s.confidence).sum::<f32>()/steps.len() as f32};
        Plan{steps,confidence,requires_observation:uncertainty>0.35}
    }

    pub fn counterfactual_score(&self, intention:&Intention, blocking:&[Relation])->f32{
        let inhibit=blocking.iter().filter(|r|matches!(r.kind,RelationKind::Inhibits))
            .map(|r|r.strength*r.confidence).fold(0.0_f32,f32::max);
        (intention.expected_benefit-intention.expected_harm-inhibit*0.5).clamp(-1.0,1.0)
    }
}
