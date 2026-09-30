use crate::duyen_weave::DuyenWeave;
use crate::expression_style::ExpressionStyle;
use crate::open_reasoning::OpenAnswer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StyleDecision {
    pub style: ExpressionStyle,
    pub reason: &'static str,
}

#[derive(Clone, Debug, Default)]
pub struct AdaptiveExpressionSelector;

impl AdaptiveExpressionSelector {
    pub fn choose(
        &self,
        answer: &OpenAnswer,
        weave: &DuyenWeave,
        uncertainty: f32,
    ) -> StyleDecision {
        let uncertainty = uncertainty.clamp(0.0, 1.0);
        let conflict = matches!(answer, OpenAnswer::Contradicted { .. })
            || weave.opposing_paths > 0;
        let complex = weave.is_overlapping()
            || weave.max_depth >= 3
            || weave.supporting_paths + weave.opposing_paths >= 3;
        let sparse = matches!(answer, OpenAnswer::Unknown)
            || (weave.supporting_paths + weave.opposing_paths <= 1
                && weave.max_depth <= 1);

        if conflict || uncertainty >= 0.55 {
            return StyleDecision {
                style: ExpressionStyle::Deep,
                reason: "conflict_or_high_uncertainty",
            };
        }
        if complex {
            return StyleDecision {
                style: ExpressionStyle::Deep,
                reason: "overlapping_or_deep_weave",
            };
        }
        if sparse && uncertainty <= 0.25 {
            return StyleDecision {
                style: ExpressionStyle::Brief,
                reason: "simple_low_uncertainty",
            };
        }
        StyleDecision {
            style: ExpressionStyle::Standard,
            reason: "balanced_default",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chooses_deep_for_conflict() {
        let answer = OpenAnswer::Contradicted {
            support: 0.7,
            opposition: 0.5,
        };
        let weave = DuyenWeave {
            supporting_paths: 2,
            opposing_paths: 1,
            convergence_nodes: vec![2],
            shared_links: 0,
            max_depth: 2,
            overlap_score: 0.5,
        };
        assert_eq!(
            AdaptiveExpressionSelector.choose(&answer, &weave, 0.2).style,
            ExpressionStyle::Deep
        );
    }

    #[test]
    fn chooses_brief_for_simple_low_uncertainty() {
        let answer = OpenAnswer::Supported {
            confidence: 0.95,
            path: vec![1, 2],
        };
        assert_eq!(
            AdaptiveExpressionSelector
                .choose(&answer, &DuyenWeave::default(), 0.1)
                .style,
            ExpressionStyle::Brief
        );
    }
}
