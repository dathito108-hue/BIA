use crate::answer_critic::{AnswerCritic, AnswerCritique};
use crate::internal_questions::{CognitiveAgenda, InternalQuestionKind};
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
        let _ = self.agenda.formulate(target, assessment, answer);
        let mut passes = 0usize;
        let mut resolved = 0usize;
        let mut confidence = assessment.certainty;
        let mut critique = self.critic.critique(answer, uncertainty, evidence_count);

        while passes < MAX_LOOP_PASSES {
            passes += 1;

            let need_more = critique.revise
                || assessment.decision == CognitiveDecision::SeekEvidence
                || assessment.decision == CognitiveDecision::Deepen;
            if !need_more {
                break;
            }

            let Some(question) = self.agenda.next().cloned() else {
                break;
            };
            confidence = refine_confidence(confidence, question.kind, &critique);
            if self.agenda.resolve_next() {
                resolved += 1;
            }

            critique = AnswerCritique {
                quality: (critique.quality + 0.07).min(1.0),
                evidence_sufficiency: (critique.evidence_sufficiency + 0.08).min(1.0),
                contradiction_risk: (critique.contradiction_risk - 0.05).max(0.0),
                overconfidence_risk: (critique.overconfidence_risk - 0.04).max(0.0),
                revise: false,
            };

            if confidence >= 0.78 && critique.quality >= 0.65 {
                break;
            }
        }

        let decision = if critique.contradiction_risk >= 0.45 {
            LoopDecision::GatherEvidence
        } else if confidence >= 0.72 && !critique.revise {
            LoopDecision::Answer
        } else if self.agenda.unresolved() > 0 {
            LoopDecision::GatherEvidence
        } else if assessment.complexity > 0.55 {
            LoopDecision::Deepen
        } else {
            LoopDecision::Hold
        };

        CognitiveLoopResult {
            decision,
            passes,
            internal_questions: self.agenda.len(),
            resolved_questions: resolved,
            critique,
            final_confidence: confidence.clamp(0.0, 1.0),
            stopped_bounded: passes <= MAX_LOOP_PASSES,
        }
    }

    pub fn unresolved_questions(&self) -> usize {
        self.agenda.unresolved()
    }
}

fn refine_confidence(current: f32, kind: InternalQuestionKind, critique: &AnswerCritique) -> f32 {
    let gain = match kind {
        InternalQuestionKind::MissingCause => 0.06,
        InternalQuestionKind::CounterEvidence => 0.04,
        InternalQuestionKind::AlternativeExplanation => 0.05,
        InternalQuestionKind::ClarifyGoal => 0.03,
    };
    (current + gain * critique.evidence_sufficiency.max(0.35)).clamp(0.0, 0.94)
}
