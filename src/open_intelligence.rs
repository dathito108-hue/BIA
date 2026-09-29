use crate::abstraction::ConceptAbstraction;
use crate::autonomous_cognitive_loop::{AutonomousCognitiveLoop, CognitiveLoopInput, CognitiveLoopResult};
use crate::active_evidence::{ActiveEvidenceSeeker, EvidenceRequest};
use crate::budget::DeviceState;
use crate::calibration::SelfCalibration;
use crate::autonomous_hypothesis::AutonomousHypothesisGenerator;
use crate::competition::{CandidateHypothesis, CompetitionResult, HypothesisCompetition};
use crate::concept_composition::ConceptComposer;
use crate::continual_semantics::ContinualSemanticLearner;
use crate::discovery::ContextDiscovery;
use crate::episodic::EpisodicMemory;
use crate::analogy::AnalogicalReasoner;
use crate::generative_cognition::{GeneratedThought, GenerativeCognition};
use crate::idle_cognition::{IdleCognitionScheduler, IdleCognitiveTask};
use crate::hierarchy::HierarchicalAbstraction;
use crate::hybrid_semantic::{HybridSemanticReasoner, LatentInference};
use crate::latent_memory::LatentMemory;
use crate::latent_symbol_bridge::LatentSymbolBridge;
use crate::latent_relation::LatentRelationLearner;
use crate::induction::{InducedRelation, InductiveReasoner};
use crate::knowledge_governor::{KnowledgeAssessment, KnowledgeGovernor};
use crate::meta_rules::MetaRuleCompressor;
use crate::metacognition::{CognitiveAssessment, MetacognitiveController};
use crate::open_reasoning::{OpenAnswer, SemanticReasoner};
use crate::recursive_deliberation::{RecursiveDeliberator, RecursiveResult};
use crate::self_directed_compute::{ComputeRoute, SelfDirectedCompute};
use crate::skill_transfer::CrossDomainTransfer;
use crate::rules::RuleSynthesizer;
use crate::semantic::{concept_id, SemanticScene};
use crate::semantic_compression::SemanticCompressor;
use crate::semantic_consolidation::{ConsolidationReport, SemanticConsolidator};
use crate::types::RelationKind;
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
    latent_memory: LatentMemory,
    latent_relations: LatentRelationLearner,
    hybrid: HybridSemanticReasoner,
    semantic_compressor: SemanticCompressor,
    continual: ContinualSemanticLearner,
    composer: ConceptComposer,
    symbol_bridge: LatentSymbolBridge,
    consolidator: SemanticConsolidator,
    generator: GenerativeCognition,
    cognitive_loop: AutonomousCognitiveLoop,
    idle_scheduler: IdleCognitionScheduler,
}

impl OpenIntelligence {
    pub fn learn(&mut self, world: &mut WorldGraph, text: &str, timestamp: u64) -> SemanticScene {
        let _ = self.abstraction.learn_from_text(text);
        let canonical = self.abstraction.canonicalize_text(text);
        let scene = self.semantic.ingest(world, &canonical, timestamp);
        if scene.clauses.is_empty() {
            if let Some(inference) = self.hybrid.infer_clause(&self.latent_relations, &canonical) {
                self.hybrid.apply(world, &inference);
            }
        }
        let canonical_id = concept_id(&canonical);
        self.latent_memory.remember(canonical_id, &canonical, 0.85);
        self.continual.observe(canonical_id, &canonical);
        self.symbol_bridge.bind(canonical_id, &canonical, 0.85);
        let _ = self.semantic_compressor.observe(&canonical, 0.85);
        if scene.clauses.len() == 1 {
            let clause = &scene.clauses[0];
            self.latent_relations
                .observe(&canonical, clause.kind, clause.confidence);
        } else {
            for clause in &scene.clauses {
                let example = format!("{} -> {}", clause.subject.text, clause.object.text);
                self.latent_relations
                    .observe(&example, clause.kind, clause.confidence);
            }
        }
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

    pub fn observe_relation_example(
        &mut self,
        text: &str,
        kind: RelationKind,
        confidence: f32,
    ) {
        self.latent_relations.observe(text, kind, confidence);
        self.latent_memory.remember(concept_id(text), text, confidence);
        let _ = self.semantic_compressor.observe(text, confidence);
    }

    pub fn classify_latent_relation(&self, text: &str) -> Option<(RelationKind, f32)> {
        self.latent_relations.classify(text)
    }

    pub fn infer_latent_clause(&self, sentence: &str) -> Option<LatentInference> {
        self.hybrid.infer_clause(&self.latent_relations, sentence)
    }

    pub fn apply_latent_clause(&self, world: &mut WorldGraph, sentence: &str) -> bool {
        let Some(inference) = self.infer_latent_clause(sentence) else {
            return false;
        };
        self.hybrid.apply(world, &inference);
        true
    }

    pub fn latent_nearest(&self, text: &str, limit: usize) -> Vec<(crate::latent_memory::LatentItem, f32)> {
        self.latent_memory.nearest(text, limit)
    }

    pub fn semantic_cluster_count(&self) -> usize {
        self.semantic_compressor.len()
    }

    pub fn continual_similarity(&self, a: u64, b: u64) -> Option<f32> {
        self.continual.similarity(a, b)
    }

    pub fn compose_similarity(&self, parts: &[&str], target: &str) -> Option<f32> {
        self.composer.compositional_similarity(parts, target)
    }

    pub fn consolidate_semantics(&mut self, important_ids: &[u64]) -> ConsolidationReport {
        self.consolidator.consolidate(&mut self.continual, important_ids)
    }

    pub fn nearest_symbol(&self, text: &str) -> Option<(crate::latent_symbol_bridge::SemanticSymbol, f32)> {
        self.symbol_bridge.nearest_symbol(text)
    }

    pub fn generate_thought(&self, answer: &OpenAnswer, uncertainty: f32) -> GeneratedThought {
        self.generator.render(answer, uncertainty)
    }

    pub fn autonomous_cycle(
        &mut self,
        answer: &OpenAnswer,
        input: &CognitiveLoopInput,
    ) -> CognitiveLoopResult {
        let assessment = self.assess_cognition(
            input.support,
            input.opposition,
            input.path_len,
            input.evidence_count,
        );
        self.cognitive_loop.run(
            input.target,
            answer,
            &assessment,
            input.uncertainty,
            input.evidence_count,
        )
    }

    pub fn choose_idle_cognition(
        &mut self,
        device: DeviceState,
        pending_external_action: bool,
    ) -> IdleCognitiveTask {
        self.idle_scheduler.choose(
            device,
            pending_external_action,
            self.cognitive_loop.unresolved_questions(),
        )
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
