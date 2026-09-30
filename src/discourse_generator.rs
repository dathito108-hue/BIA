use crate::duyen_weave::DuyenWeave;
use crate::open_reasoning::OpenAnswer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiscourseMove {
    Verdict,
    Convergence,
    Opposition,
    Limitation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DiscoursePlan {
    pub moves: Vec<DiscourseMove>,
    pub branch_count: usize,
    pub convergence_count: usize,
    pub has_opposition: bool,
}

impl DiscoursePlan {
    pub fn build(answer: &OpenAnswer, weave: &DuyenWeave) -> Self {
        let mut moves = vec![DiscourseMove::Verdict];
        if weave.is_overlapping() {
            moves.push(DiscourseMove::Convergence);
        }
        let has_opposition = weave.opposing_paths > 0
            || matches!(answer, OpenAnswer::Opposed { .. } | OpenAnswer::Contradicted { .. });
        if has_opposition {
            moves.push(DiscourseMove::Opposition);
        }
        if !matches!(answer, OpenAnswer::Supported { confidence, .. } if *confidence >= 0.9)
            || has_opposition
        {
            moves.push(DiscourseMove::Limitation);
        }
        moves.truncate(4);
        Self {
            moves,
            branch_count: weave.supporting_paths + weave.opposing_paths,
            convergence_count: weave.convergence_nodes.len(),
            has_opposition,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn overlapping_conflict_gets_ordered_discourse_moves() {
        let answer = OpenAnswer::Contradicted { support: 0.7, opposition: 0.6 };
        let weave = DuyenWeave {
            supporting_paths: 2,
            opposing_paths: 1,
            convergence_nodes: vec![2,3],
            shared_links: 1,
            max_depth: 3,
            overlap_score: 0.8,
        };
        let p = DiscoursePlan::build(&answer, &weave);
        assert_eq!(p.moves[0], DiscourseMove::Verdict);
        assert!(p.moves.contains(&DiscourseMove::Convergence));
        assert!(p.moves.contains(&DiscourseMove::Opposition));
        assert!(p.moves.contains(&DiscourseMove::Limitation));
    }
}
