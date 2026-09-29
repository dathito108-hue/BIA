use crate::answer_critic::{AnswerCritic, AnswerCritique};
use crate::internal_questions::CognitiveAgenda;
use crate::metacognition::{CognitiveAssessment, CognitiveDecision};
use crate::open_reasoning::OpenAnswer;

const MAX_LOOP_PASSES: usize = 4;

#[derive(Clone, Debug, PartialEq)]
pub struct CognitiveLoopInput {
    pub target: u64,
    pub support: f32,
    pub opposition: f32,
    pub path_len: usize,
    pub evidence_count: usize,
    pub uncertainty: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoopDecision {
    Answer,
    GatherEvidence,
    Deepen,
    Hold,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CognitiveLoopResult {
    pub decision: LoopDecision,
    pub passes: usize,
    pub internal_questions: usize,
    pub resolved_questions: usize,
    pub critique: AnswerCritique,
    pub final_confidence: f32,
    pub stopped_bounded: bool,
}

#[derive(Clone, Debug, Default)]
pub struct AutonomousCognitiveLoop {
    agenda: CognitiveAgenda,
    critic: AnswerCritic,
}

impl AutonomousCognitiveLoop {
    pub fn run(
        &mut self,
        target: u64,
        answer: &OpenAnswer,
        assessment: &CognitiveAssessment,
        uncertainty: f32,
        evidence_count: usize,
    ) -> CognitiveLoopResult {
        // This agenda belongs to one query snapshot. A prior target must not
        // consume its capacity or have its questions silently resolved here.
        self.agenda = CognitiveAgenda::default();
        let _ = self.agenda.formulate(target, assessment, answer);
        let critique = self.critic.critique(answer, uncertainty, evidence_count);
        let answer_confidence = match answer {
            OpenAnswer::Supported { confidence, .. } | OpenAnswer::Opposed { confidence, .. } => {
                *confidence
            }
            OpenAnswer::Contradicted {
                support,
                opposition,
            } => (support - opposition).abs(),
            OpenAnswer::Counterfactual {
                factual_support,
                counterfactual_support,
                ..
            } => factual_support.max(*counterfactual_support),
            OpenAnswer::Unknown => 0.0,
        };
        let confidence = finite_unit(assessment.certainty)
            .min(finite_unit(answer_confidence))
            .min(if uncertainty.is_finite() {
                1.0 - finite_unit(uncertainty)
            } else {
                0.0
            });

        // No evidence provider is attached to this review. Repeating the same
        // snapshot cannot improve its evidence, resolve questions or remove
        // conflict. Stop immediately; the caller can retrieve/observe and retry.
        let needs_evidence = matches!(
            answer,
            OpenAnswer::Unknown | OpenAnswer::Contradicted { .. }
        ) || evidence_count == 0
            || assessment.decision == CognitiveDecision::SeekEvidence
            || critique.contradiction_risk >= 0.45;
        let decision = if needs_evidence {
            LoopDecision::GatherEvidence
        } else if confidence >= 0.72 && !critique.revise {
            LoopDecision::Answer
        } else if assessment.complexity > 0.55 {
            LoopDecision::Deepen
        } else if self.agenda.unresolved() > 0 || critique.revise {
            LoopDecision::GatherEvidence
        } else {
            LoopDecision::Hold
        };

        CognitiveLoopResult {
            decision,
            passes: 1,
            internal_questions: self.agenda.len(),
            resolved_questions: 0,
            critique,
            final_confidence: confidence,
            stopped_bounded: 1 <= MAX_LOOP_PASSES,
        }
    }

    pub fn unresolved_questions(&self) -> usize {
        self.agenda.unresolved()
    }
}

fn finite_unit(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        0.0
    }
}
