use crate::abstraction::ConceptAbstraction;
use crate::analogy::AnalogicalReasoner;
use crate::open_reasoning::{OpenAnswer, SemanticReasoner};
use crate::semantic::SemanticScene;
use crate::world::WorldGraph;

#[derive(Clone, Debug, Default)]
pub struct OpenIntelligence {
    abstraction: ConceptAbstraction,
    semantic: SemanticReasoner,
    analogy: AnalogicalReasoner,
}

impl OpenIntelligence {
    pub fn learn(&mut self, world: &mut WorldGraph, text: &str, timestamp: u64) -> SemanticScene {
        let _ = self.abstraction.learn_from_text(text);
        let canonical = self.abstraction.canonicalize_text(text);
        let scene = self.semantic.ingest(world, &canonical, timestamp);
        let _ = self.analogy.apply(world);
        scene
    }

    pub fn parse(&self, text: &str) -> SemanticScene {
        self.semantic
            .parse(&self.abstraction.canonicalize_text(text))
    }

    pub fn answer_scene(&self, world: &WorldGraph, scene: &SemanticScene) -> OpenAnswer {
        self.semantic.answer_scene(world, scene)
    }

    pub fn learn_and_answer(
        &mut self,
        world: &mut WorldGraph,
        text: &str,
        timestamp: u64,
    ) -> OpenAnswer {
        let scene = self.learn(world, text, timestamp);
        self.answer_scene(world, &scene)
    }

    pub fn canonicalize(&self, text: &str) -> String {
        self.abstraction.canonicalize_text(text)
    }

    pub fn abstraction(&self) -> &ConceptAbstraction {
        &self.abstraction
    }

    pub fn analogy(&self) -> &AnalogicalReasoner {
        &self.analogy
    }
}
