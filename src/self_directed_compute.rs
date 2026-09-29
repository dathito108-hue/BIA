use crate::budget::DeviceState;
use crate::metacognition::{CognitiveAssessment, CognitiveDecision};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReasoningTier {
    Instant,
    Normal,
    Deep,
    EvidenceFirst,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComputeRoute {
    pub tier: ReasoningTier,
    pub reasoning_passes: u8,
    pub evidence_budget: u8,
}

#[derive(Clone, Debug, Default)]
pub struct SelfDirectedCompute;

impl SelfDirectedCompute {
    pub fn route(
        &self,
        assessment: &CognitiveAssessment,
        device: DeviceState,
    ) -> ComputeRoute {
        let pressure = (
            (1.0 - device.battery).clamp(0.0, 1.0) * 0.25
                + device.thermal.clamp(0.0, 1.0) * 0.35
                + device.load.clamp(0.0, 1.0) * 0.25
                + if device.available_memory_mb < 192 { 0.20 } else { 0.0 }
        )
            .clamp(0.0, 1.0);

        if pressure > 0.78 {
            return ComputeRoute {
                tier: ReasoningTier::Instant,
                reasoning_passes: 1,
                evidence_budget: 0,
            };
        }

        match assessment.decision {
            CognitiveDecision::SeekEvidence => ComputeRoute {
                tier: ReasoningTier::EvidenceFirst,
                reasoning_passes: 1,
                evidence_budget: if pressure < 0.4 { 3 } else { 1 },
            },
            CognitiveDecision::Deepen => ComputeRoute {
                tier: ReasoningTier::Deep,
                reasoning_passes: if pressure < 0.35 { 4 } else { 2 },
                evidence_budget: 1,
            },
            CognitiveDecision::Answer if assessment.certainty > 0.88 => ComputeRoute {
                tier: ReasoningTier::Instant,
                reasoning_passes: 1,
                evidence_budget: 0,
            },
            _ => ComputeRoute {
                tier: ReasoningTier::Normal,
                reasoning_passes: 2,
                evidence_budget: 1,
            },
        }
    }
}
