#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CognitiveDecision {
    Answer,
    SeekEvidence,
    Deepen,
    Hold,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CognitiveAssessment {
    pub certainty: f32,
    pub conflict: f32,
    pub complexity: f32,
    pub evidence_gap: f32,
    pub decision: CognitiveDecision,
}

#[derive(Clone, Debug, Default)]
pub struct MetacognitiveController;

impl MetacognitiveController {
    pub fn assess(
        &self,
        support: f32,
        opposition: f32,
        path_len: usize,
        evidence_count: usize,
    ) -> CognitiveAssessment {
        let support = support.clamp(0.0, 1.0);
        let opposition = opposition.clamp(0.0, 1.0);
        let conflict = support.min(opposition);
        let separation = (support - opposition).abs();
        let evidence_strength = (evidence_count.min(8) as f32 / 8.0).sqrt();
        let certainty = (
            separation * 0.65
                + support.max(opposition) * 0.20
                + evidence_strength * 0.15
        )
            .clamp(0.0, 1.0);
        let complexity = (
            path_len.saturating_sub(1).min(8) as f32 / 8.0
                + conflict * 0.5
        )
            .clamp(0.0, 1.0);
        let evidence_gap = (1.0 - evidence_strength).clamp(0.0, 1.0);

        let decision = if conflict >= 0.45 || (certainty < 0.35 && evidence_count < 2) {
            CognitiveDecision::SeekEvidence
        } else if complexity > 0.55 && certainty < 0.80 {
            CognitiveDecision::Deepen
        } else if certainty >= 0.72 {
            CognitiveDecision::Answer
        } else {
            CognitiveDecision::Hold
        };

        CognitiveAssessment {
            certainty,
            conflict,
            complexity,
            evidence_gap,
            decision,
        }
    }
}
