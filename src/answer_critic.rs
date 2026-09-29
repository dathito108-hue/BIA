use crate::open_reasoning::OpenAnswer;

#[derive(Clone, Debug, PartialEq)]
pub struct AnswerCritique {
    pub quality: f32,
    pub evidence_sufficiency: f32,
    pub contradiction_risk: f32,
    pub overconfidence_risk: f32,
    pub revise: bool,
}

#[derive(Clone, Debug, Default)]
pub struct AnswerCritic;

impl AnswerCritic {
    pub fn critique(&self, answer: &OpenAnswer, uncertainty: f32, evidence_count: usize) -> AnswerCritique {
        let evidence = (evidence_count.min(8) as f32 / 8.0).sqrt();
        let uncertainty = uncertainty.clamp(0.0, 1.0);

        let (base, contradiction, stated_confidence) = match answer {
            OpenAnswer::Supported { confidence, path } => (
                *confidence * (0.6 + path.len().min(6) as f32 * 0.06),
                0.0,
                *confidence,
            ),
            OpenAnswer::Opposed { confidence, path } => (
                *confidence * (0.6 + path.len().min(6) as f32 * 0.06),
                0.15,
                *confidence,
            ),
            OpenAnswer::Contradicted { support, opposition } => (
                (support - opposition).abs(),
                support.min(*opposition),
                support.max(*opposition),
            ),
            OpenAnswer::Counterfactual {
                factual_support,
                counterfactual_support,
                ..
            } => (
                factual_support.max(*counterfactual_support) * 0.78,
                0.05,
                factual_support.max(*counterfactual_support),
            ),
            OpenAnswer::Unknown => (0.15, 0.0, 0.1),
        };

        let overconfidence = (stated_confidence - (1.0 - uncertainty) * (0.55 + evidence * 0.45))
            .max(0.0)
            .clamp(0.0, 1.0);
        let evidence_sufficiency = (evidence * (1.0 - uncertainty * 0.35)).clamp(0.0, 1.0);
        let quality = (
            base * 0.55
                + evidence_sufficiency * 0.30
                + (1.0 - contradiction) * 0.10
                + (1.0 - overconfidence) * 0.05
        )
            .clamp(0.0, 1.0);
        let revise = matches!(answer, OpenAnswer::Unknown)
            || contradiction >= 0.45
            || overconfidence > 0.25
            || quality < 0.58;

        AnswerCritique {
            quality,
            evidence_sufficiency,
            contradiction_risk: contradiction,
            overconfidence_risk: overconfidence,
            revise,
        }
    }
}
