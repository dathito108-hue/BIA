use crate::memory::Seed;
use crate::types::{Phenomenon, Relation};

#[derive(Clone, Debug, PartialEq)]
pub struct SkillDelta {
    pub id:u64,
    pub added_relations:Vec<Relation>,
    pub added_seeds:Vec<Seed>,
    pub score_before:f32,
    pub score_after:f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PromotionDecision { Promote, Hold, Reject }

pub fn evaluate_delta(delta:&SkillDelta, min_gain:f32)->PromotionDecision {
    let gain=delta.score_after-delta.score_before;
    if !gain.is_finite() { return PromotionDecision::Reject; }
    if gain>=min_gain { PromotionDecision::Promote }
    else if gain>=0.0 { PromotionDecision::Hold }
    else { PromotionDecision::Reject }
}

pub fn causal_credit(event:&Phenomenon, benefit:f32, harm:f32)->f32 {
    let outcome=(benefit-harm).clamp(-1.0,1.0);
    (outcome*event.confidence*(0.5+0.5*event.salience)).clamp(-1.0,1.0)
}
