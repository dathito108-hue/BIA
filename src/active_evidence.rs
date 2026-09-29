use crate::metacognition::{CognitiveAssessment, CognitiveDecision};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EvidenceRequestKind {
    Supporting,
    Opposing,
    MissingLink,
    FreshObservation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvidenceRequest {
    pub target: u64,
    pub kind: EvidenceRequestKind,
    pub priority: f32,
}

#[derive(Clone, Debug, Default)]
pub struct ActiveEvidenceSeeker;

impl ActiveEvidenceSeeker {
    pub fn request(
        &self,
        target: u64,
        assessment: &CognitiveAssessment,
        missing_link: bool,
    ) -> Option<EvidenceRequest> {
        if assessment.decision != CognitiveDecision::SeekEvidence
            && assessment.decision != CognitiveDecision::Deepen
        {
            return None;
        }

        let kind = if missing_link {
            EvidenceRequestKind::MissingLink
        } else if assessment.conflict >= 0.45 {
            EvidenceRequestKind::Opposing
        } else if assessment.evidence_gap > 0.6 {
            EvidenceRequestKind::Supporting
        } else {
            EvidenceRequestKind::FreshObservation
        };

        Some(EvidenceRequest {
            target,
            kind,
            priority: (
                assessment.evidence_gap * 0.55
                    + assessment.conflict * 0.30
                    + assessment.complexity * 0.15
            )
                .clamp(0.0, 1.0),
        })
    }
}
