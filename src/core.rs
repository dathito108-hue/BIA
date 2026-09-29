use crate::budget::{middle_way, Budget, DeviceState};
use crate::memory::SeedMemory;
use crate::types::{CognitiveMoment, Hypothesis, Intention, Phenomenon, Relation, RelationKind};
use crate::world::WorldGraph;

#[derive(Clone, Debug)]
pub struct BiaDcaConfig {
    pub world_nodes:usize, pub world_edges:usize, pub seeds:usize,
    pub active_causes:usize,
}
impl Default for BiaDcaConfig {
    fn default()->Self{Self{world_nodes:4096,world_edges:16384,seeds:4096,active_causes:12}}
}

#[derive(Clone, Debug)]
pub struct BiaDca {
    pub world:WorldGraph,
    pub memory:SeedMemory,
    active_causes:usize,
    cycle:u64,
}
impl BiaDca {
    pub fn new(cfg:BiaDcaConfig)->Self{
        Self{world:WorldGraph::new(cfg.world_nodes,cfg.world_edges),memory:SeedMemory::new(cfg.seeds),active_causes:cfg.active_causes,cycle:0}
    }

    pub fn observe(&mut self,p:Phenomenon){ self.world.upsert(p); }
    pub fn relate(&mut self,r:Relation){ self.world.relate(r); }

    pub fn contemplate(&mut self, focus:Phenomenon, device:DeviceState, importance:f32)->CognitiveMoment{
        self.observe(focus.clone());
        let uncertainty=(1.0-focus.confidence).clamp(0.0,1.0);
        let budget=middle_way(device,importance,uncertainty);
        let active=self.world.activate(&focus,budget.world_limit);
        let causes=self.world.causes_for(focus.id,self.active_causes.min(budget.world_limit));
        let memories=self.memory.recall(&focus,budget.memory_limit);

        let mut recognition:Vec<u32>=active.iter().map(|p|p.kind).collect();
        for s in &memories { if !recognition.contains(&s.meaning){ recognition.push(s.meaning); } }

        let feeling=(focus.salience*(0.5+0.5*importance)-uncertainty*0.35).clamp(-1.0,1.0);
        let mut hypotheses=Vec::new();
        for c in &causes {
            let score=(c.strength*c.confidence*focus.confidence).clamp(0.0,1.0);
            hypotheses.push(Hypothesis{source:c.from,target:c.to,score,support:vec![c.from,c.to]});
        }
        for _ in 0..budget.contemplation_cycles {
            reinforce_consistent(&mut hypotheses,&causes);
        }
        hypotheses.sort_by(|a,b|b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        let chosen=choose_intention(&focus,&causes,&memories,budget);
        self.cycle=self.cycle.saturating_add(1);

        CognitiveMoment{
            observed:active.iter().map(|p|p.id).collect(),
            active_causes:causes,
            feeling,
            recognition,
            hypotheses,
            chosen,
            mode:budget.mode,
            uncertainty,
        }
    }

    pub fn experience(&mut self,p:&Phenomenon,meaning:u32,benefit:f32,harm:f32){
        let utility=(benefit-harm).clamp(-1.0,1.0);
        self.memory.imprint(p,meaning,utility);
    }

    pub fn cycle(&self)->u64{self.cycle}
}

fn reinforce_consistent(h:&mut [Hypothesis],causes:&[Relation]){
    for x in h {
        let support=causes.iter().filter(|r|r.from==x.source||r.to==x.target).count() as f32;
        x.score=(x.score+0.015*support.min(4.0)).clamp(0.0,1.0);
    }
}

fn choose_intention(
    focus:&Phenomenon,
    causes:&[Relation],
    memories:&[crate::memory::Seed],
    budget:Budget,
)->Option<Intention>{
    if matches!(budget.mode,crate::types::ComputeMode::Tinh){return None}
    let enabling=causes.iter().filter(|r|matches!(r.kind,RelationKind::Enables|RelationKind::Causes)).map(|r|r.strength*r.confidence).fold(0.0_f32,f32::max);
    let inhibiting=causes.iter().filter(|r|matches!(r.kind,RelationKind::Inhibits)).map(|r|r.strength*r.confidence).fold(0.0_f32,f32::max);
    let remembered=memories.iter().map(|s|s.utility*s.strength).fold(0.0_f32,f32::max);
    let benefit=(0.45*focus.salience+0.30*enabling+0.25*remembered.max(0.0)).clamp(0.0,1.0);
    let harm=(0.65*inhibiting+0.35*(1.0-focus.confidence)).clamp(0.0,1.0);
    if benefit<=harm{return None}
    Some(Intention{
        action:focus.kind,
        target:Some(focus.id),
        expected_benefit:benefit,
        expected_harm:harm,
        reversibility:(1.0-harm*0.7).clamp(0.0,1.0),
        confidence:(focus.confidence*(1.0-harm)).clamp(0.0,1.0),
    })
}
