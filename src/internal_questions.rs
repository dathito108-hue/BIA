use crate::metacognition::{CognitiveAssessment, CognitiveDecision};
use crate::open_reasoning::OpenAnswer;

const MAX_INTERNAL_QUESTIONS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InternalQuestionKind {
    MissingCause,
    CounterEvidence,
    AlternativeExplanation,
    ClarifyGoal,
}

#[derive(Clone, Debug, PartialEq)]
pub struct InternalQuestion {
    pub target: u64,
    pub kind: InternalQuestionKind,
    pub priority: f32,
    pub resolved: bool,
}

#[derive(Clone, Debug, Default)]
pub struct CognitiveAgenda {
    questions: Vec<InternalQuestion>,
}

impl CognitiveAgenda {
    pub fn formulate(
        &mut self,
        target: u64,
        assessment: &CognitiveAssessment,
        answer: &OpenAnswer,
    ) -> usize {
        let mut proposed = Vec::new();

        if matches!(answer, OpenAnswer::Unknown) {
            proposed.push((InternalQuestionKind::MissingCause, 0.92));
        }
        if assessment.conflict >= 0.40 {
            proposed.push((InternalQuestionKind::CounterEvidence, 0.88));
            proposed.push((InternalQuestionKind::AlternativeExplanation, 0.76));
        }
        if assessment.decision == CognitiveDecision::Deepen && assessment.complexity > 0.55 {
            proposed.push((InternalQuestionKind::AlternativeExplanation, 0.72));
        }
        if assessment.evidence_gap > 0.70 {
            proposed.push((InternalQuestionKind::ClarifyGoal, 0.60));
        }

        let mut added = 0usize;
        for (kind, base) in proposed {
            if self.questions.iter().any(|q| q.target == target && q.kind == kind && !q.resolved) {
                continue;
            }
            if self.questions.len() >= MAX_INTERNAL_QUESTIONS {
                if let Some(idx) = self.questions.iter().position(|q| q.resolved) {
                    self.questions.remove(idx);
                } else {
                    break;
                }
            }
            let priority = (base
                + assessment.conflict * 0.08
                + assessment.evidence_gap * 0.06)
                .clamp(0.0, 1.0);
            self.questions.push(InternalQuestion {
                target,
                kind,
                priority,
                resolved: false,
            });
            added += 1;
        }
        self.questions.sort_by(|a, b| {
            b.priority
                .partial_cmp(&a.priority)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        added
    }

    pub fn next(&self) -> Option<&InternalQuestion> {
        self.questions.iter().find(|q| !q.resolved)
    }

    pub fn resolve_next(&mut self) -> bool {
        if let Some(q) = self.questions.iter_mut().find(|q| !q.resolved) {
            q.resolved = true;
            true
        } else {
            false
        }
    }

    pub fn unresolved(&self) -> usize {
        self.questions.iter().filter(|q| !q.resolved).count()
    }

    pub fn len(&self) -> usize { self.questions.len() }
    pub fn is_empty(&self) -> bool { self.questions.is_empty() }
}
