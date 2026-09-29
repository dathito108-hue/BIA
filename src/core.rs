use crate::budget::{middle_way, Budget, DeviceState};
use crate::meaning::MeaningFormation;
use crate::memory::SeedMemory;
use crate::persistence::DharmaSnapshot;
use crate::types::{CognitiveMoment, Hypothesis, Intention, Phenomenon, Relation, RelationKind};
use crate::world::WorldGraph;

#[derive(Clone, Debug)]
pub struct BiaDcaConfig {
    pub world_nodes: usize,
    pub world_edges: usize,
    pub seeds: usize,
    pub concepts: usize,
    pub active_causes: usize,
}

impl Default for BiaDcaConfig {
    fn default() -> Self {
        Self {
            world_nodes: 4096,
            world_edges: 16384,
            seeds: 4096,
            concepts: 2048,
            active_causes: 12,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BiaDca {
    pub world: WorldGraph,
    pub memory: SeedMemory,
    pub meaning: MeaningFormation,
    active_causes: usize,
    cycle: u64,
    previous_observation: Option<Phenomenon>,
}

impl BiaDca {
    pub fn new(cfg: BiaDcaConfig) -> Self {
        Self {
            world: WorldGraph::new(cfg.world_nodes, cfg.world_edges),
            memory: SeedMemory::new(cfg.seeds),
            meaning: MeaningFormation::new(cfg.concepts, 1_000_000),
            active_causes: cfg.active_causes,
            cycle: 0,
            previous_observation: None,
        }
    }

    pub fn from_snapshot(cfg: BiaDcaConfig, snapshot: DharmaSnapshot) -> Self {
        Self {
            world: snapshot.world,
            memory: snapshot.memory,
            meaning: MeaningFormation::new(cfg.concepts, 1_000_000),
            active_causes: cfg.active_causes,
            cycle: 0,
            previous_observation: None,
        }
    }

    pub fn snapshot(&self) -> DharmaSnapshot {
        DharmaSnapshot {
            world: self.world.clone(),
            memory: self.memory.clone(),
        }
    }

    pub fn observe(&mut self, p: Phenomenon) -> u32 {
        let concept = self.meaning.observe(&p);

        if let Some(previous) = &self.previous_observation {
            if let Some(relation) = self.meaning.infer_relation(previous, &p, 64) {
                self.world.relate(relation);
            }
        }

        self.previous_observation = Some(p.clone());
        self.world.upsert(p);
        concept
    }

    pub fn relate(&mut self, r: Relation) {
        self.world.relate(r);
    }

    pub fn contemplate(
        &mut self,
        focus: Phenomenon,
        device: DeviceState,
        importance: f32,
    ) -> CognitiveMoment {
        let emergent_concept = self.observe(focus.clone());
        let uncertainty = (1.0 - focus.confidence).clamp(0.0, 1.0);
        let budget = middle_way(device, importance, uncertainty);
        let active = self.world.activate(&focus, budget.world_limit);
        let causes = self
            .world
            .causes_for(focus.id, self.active_causes.min(budget.world_limit));
        let memories = self.memory.recall(&focus, budget.memory_limit);

        let mut recognition: Vec<u32> = vec![emergent_concept];
        for p in &active {
            if !recognition.contains(&p.kind) {
                recognition.push(p.kind);
            }
        }
        for s in &memories {
            if !recognition.contains(&s.meaning) {
                recognition.push(s.meaning);
            }
        }

        let feeling =
            (focus.salience * (0.5 + 0.5 * importance) - uncertainty * 0.35).clamp(-1.0, 1.0);
        let mut hypotheses = Vec::new();
        for c in &causes {
            let score = (c.strength * c.confidence * focus.confidence).clamp(0.0, 1.0);
            hypotheses.push(Hypothesis {
                source: c.from,
                target: c.to,
                score,
                support: vec![c.from, c.to],
            });
        }
        for _ in 0..budget.contemplation_cycles {
            reinforce_consistent(&mut hypotheses, &causes);
        }
        hypotheses.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let chosen = choose_intention(&focus, &causes, &memories, budget);
        self.cycle = self.cycle.saturating_add(1);

        CognitiveMoment {
            observed: active.iter().map(|p| p.id).collect(),
            active_causes: causes,
            feeling,
            recognition,
            hypotheses,
            chosen,
            mode: budget.mode,
            uncertainty,
        }
    }

    pub fn experience(&mut self, p: &Phenomenon, meaning: u32, benefit: f32, harm: f32) {
        let utility = (benefit - harm).clamp(-1.0, 1.0);
        self.memory.imprint(p, meaning, utility);
    }

    pub fn contradict(&mut self, concept_id: u32, severity: f32) {
        self.meaning.contradict(concept_id, severity);
    }

    pub fn cycle(&self) -> u64 {
        self.cycle
    }
}

fn reinforce_consistent(h: &mut [Hypothesis], causes: &[Relation]) {
    for x in h {
        let support = causes
            .iter()
            .filter(|r| r.from == x.source || r.to == x.target)
            .count() as f32;
        x.score = (x.score + 0.015 * support.min(4.0)).clamp(0.0, 1.0);
    }
}

fn choose_intention(
    focus: &Phenomenon,
    causes: &[Relation],
    memories: &[crate::memory::Seed],
    budget: Budget,
) -> Option<Intention> {
    if matches!(budget.mode, crate::types::ComputeMode::Tinh) {
        return None;
    }

    let enabling = causes
        .iter()
        .filter(|r| matches!(r.kind, RelationKind::Enables | RelationKind::Causes))
        .map(|r| r.strength * r.confidence)
        .fold(0.0_f32, f32::max);
    let inhibiting = causes
        .iter()
        .filter(|r| matches!(r.kind, RelationKind::Inhibits))
        .map(|r| r.strength * r.confidence)
        .fold(0.0_f32, f32::max);
    let remembered = memories
        .iter()
        .map(|s| s.utility * s.strength)
        .fold(0.0_f32, f32::max);

    let benefit =
        (0.45 * focus.salience + 0.30 * enabling + 0.25 * remembered.max(0.0)).clamp(0.0, 1.0);
    let harm = (0.65 * inhibiting + 0.35 * (1.0 - focus.confidence)).clamp(0.0, 1.0);
    if benefit <= harm {
        return None;
    }

    Some(Intention {
        action: focus.kind,
        target: Some(focus.id),
        expected_benefit: benefit,
        expected_harm: harm,
        reversibility: (1.0 - harm * 0.7).clamp(0.0, 1.0),
        confidence: (focus.confidence * (1.0 - harm)).clamp(0.0, 1.0),
    })
}
