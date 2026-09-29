use crate::abstraction::ConceptAbstraction;
use crate::active_evidence::{ActiveEvidenceSeeker, EvidenceRequest};
use crate::budget::DeviceState;
use crate::calibration::SelfCalibration;
use crate::autonomous_hypothesis::AutonomousHypothesisGenerator;
use crate::competition::{CandidateHypothesis, CompetitionResult, HypothesisCompetition};
use crate::discovery::ContextDiscovery;
use crate::episodic::EpisodicMemory;
use crate::analogy::AnalogicalReasoner;
use crate::hierarchy::HierarchicalAbstraction;
use crate::induction::{InducedRelation, InductiveReasoner};
use crate::knowledge_governor::{KnowledgeAssessment, KnowledgeGovernor};
use crate::meta_rules::MetaRuleCompressor;
use crate::metacognition::{CognitiveAssessment, MetacognitiveController};
use crate::open_reasoning::{OpenAnswer, SemanticReasoner};
use crate::recursive_deliberation::{RecursiveDeliberator, RecursiveResult};
use crate::self_directed_compute::{ComputeRoute, SelfDirectedCompute};
use crate::skill_transfer::CrossDomainTransfer;
use crate::rules::RuleSynthesizer;
use crate::semantic::SemanticScene;
use crate::world::WorldGraph;

#[derive(Clone, Debug, Default)]
pub struct OpenIntelligence {
    abstraction: ConceptAbstraction,
    semantic: SemanticReasoner,
    analogy: AnalogicalReasoner,
    induction: InductiveReasoner,
    episodes: EpisodicMemory,
    discovery: ContextDiscovery,
    rules: RuleSynthesizer,
    competition: HypothesisCompetition,
    hierarchy: HierarchicalAbstraction,
    hypotheses: AutonomousHypothesisGenerator,
    meta_rules: MetaRuleCompressor,
    governor: KnowledgeGovernor,
    metacognition: MetacognitiveController,
    calibration: SelfCalibration,
    evidence: ActiveEvidenceSeeker,
    recursive: RecursiveDeliberator,
    compute: SelfDirectedCompute,
    transfer: CrossDomainTransfer,
}

impl OpenIntelligence {
    pub fn learn(&mut self, world: &mut WorldGraph, text: &str, timestamp: u64) -> SemanticScene {
        let _ = self.abstraction.learn_from_text(text);
        let canonical = self.abstraction.canonicalize_text(text);
        let scene = self.semantic.ingest(world, &canonical, timestamp);
        self.episodes.observe(&scene, timestamp, 0.5);
        let _ = self.discovery.apply(world);
        let _ = self.hierarchy.discover(world);
        let _ = self.rules.synthesize(&self.episodes);
        let _ = self.rules.apply(world);
        let _ = self.meta_rules.compress(self.rules.rules());
        let _ = self.meta_rules.apply(world);
        let _ = self.analogy.apply(world);

        let candidates = self.hypotheses.generate(world);
        for candidate in candidates.iter().take(8) {
            let _ = self.governor.promote_if_valid(world, candidate);
        }
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

    pub fn episodes(&self) -> &EpisodicMemory {
        &self.episodes
    }

    pub fn rules(&self) -> &RuleSynthesizer {
        &self.rules
    }

    pub fn hierarchy(&self) -> &HierarchicalAbstraction {
        &self.hierarchy
    }

    pub fn meta_rules(&self) -> &MetaRuleCompressor {
        &self.meta_rules
    }

    pub fn autonomous_hypotheses(&self, world: &WorldGraph) -> Vec<CandidateHypothesis> {
        self.hypotheses.generate(world)
    }

    pub fn assess_hypothesis(
        &self,
        world: &WorldGraph,
        candidate: &CandidateHypothesis,
    ) -> KnowledgeAssessment {
        self.governor.assess(world, candidate)
    }

    pub fn compete(&self, candidates: &[CandidateHypothesis]) -> CompetitionResult {
        self.competition.choose(candidates)
    }

    pub fn assess_cognition(
        &self,
        support: f32,
        opposition: f32,
        path_len: usize,
        evidence_count: usize,
    ) -> CognitiveAssessment {
        let raw = self
            .metacognition
            .assess(support, opposition, path_len, evidence_count);
        CognitiveAssessment {
            certainty: self.calibration.adjusted(raw.certainty),
            ..raw
        }
    }

    pub fn route_compute(
        &self,
        assessment: &CognitiveAssessment,
        device: DeviceState,
    ) -> ComputeRoute {
        self.compute.route(assessment, device)
    }

    pub fn evidence_request(
        &self,
        target: u64,
        assessment: &CognitiveAssessment,
        missing_link: bool,
    ) -> Option<EvidenceRequest> {
        self.evidence.request(target, assessment, missing_link)
    }

    pub fn recursive_refine<F>(&self, initial: f32, refine: F) -> RecursiveResult
    where
        F: FnMut(usize, f32) -> f32,
    {
        self.recursive.run(initial, refine)
    }

    pub fn observe_calibration(&mut self, predicted_confidence: f32, correct: bool) {
        self.calibration.observe(predicted_confidence, correct);
    }

    pub fn calibration_brier(&self) -> f32 {
        self.calibration.brier_score()
    }

    pub fn transfer_engine(&self) -> &CrossDomainTransfer {
        &self.transfer
    }

    pub fn analogy(&self) -> &AnalogicalReasoner {
        &self.analogy
    }

    pub fn induce_between(
        &self,
        world: &WorldGraph,
        from: u64,
        to: u64,
    ) -> Option<InducedRelation> {
        self.induction.infer_between(world, from, to)
    }

    pub fn apply_induction(
        &self,
        world: &mut WorldGraph,
        from: u64,
        to: u64,
    ) -> Option<InducedRelation> {
        self.induction.apply_between(world, from, to)
    }
}
