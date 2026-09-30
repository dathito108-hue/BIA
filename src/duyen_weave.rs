use crate::reasoning::CausalPath;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DuyenWeave {
    pub supporting_paths: usize,
    pub opposing_paths: usize,
    pub convergence_nodes: Vec<u64>,
    pub shared_links: usize,
    pub max_depth: usize,
    pub overlap_score: f32,
}

impl DuyenWeave {
    pub fn from_paths(paths: &[CausalPath], target: u64) -> Self {
        let mut supporting_paths = 0usize;
        let mut opposing_paths = 0usize;
        let mut max_depth = 0usize;
        let mut convergence_nodes = Vec::new();
        let mut shared_links = 0usize;

        for path in paths.iter().take(32) {
            if path.inhibited { opposing_paths += 1; } else { supporting_paths += 1; }
            max_depth = max_depth.max(path.nodes.len().saturating_sub(1));
        }

        let mut unique_nodes = Vec::new();
        for path in paths.iter().take(32) {
            for &node in &path.nodes {
                if node != target && !unique_nodes.contains(&node) {
                    unique_nodes.push(node);
                }
            }
        }
        for node in unique_nodes {
            let count = paths.iter().take(32).filter(|p| p.nodes.contains(&node)).count();
            if count >= 2 {
                convergence_nodes.push(node);
            }
        }
        convergence_nodes.truncate(16);

        for (i, a) in paths.iter().take(16).enumerate() {
            for b in paths.iter().take(16).skip(i + 1) {
                if a.nodes.windows(2).any(|aw| b.nodes.windows(2).any(|bw| aw == bw)) {
                    shared_links += 1;
                }
            }
        }

        let total = supporting_paths + opposing_paths;
        let overlap_score = if total < 2 {
            0.0
        } else {
            ((convergence_nodes.len() as f32 + shared_links as f32 * 0.5)
                / total as f32)
                .clamp(0.0, 1.0)
        };

        Self {
            supporting_paths,
            opposing_paths,
            convergence_nodes,
            shared_links,
            max_depth,
            overlap_score,
        }
    }

    pub fn is_overlapping(&self) -> bool {
        self.supporting_paths + self.opposing_paths > 1
            && (!self.convergence_nodes.is_empty() || self.shared_links > 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detects_overlap_and_opposition() {
        let paths = vec![
            CausalPath { nodes: vec![1,2,4], score: 0.8, inhibited: false },
            CausalPath { nodes: vec![1,3,4], score: 0.7, inhibited: false },
            CausalPath { nodes: vec![5,3,4], score: 0.6, inhibited: true },
        ];
        let w = DuyenWeave::from_paths(&paths, 4);
        assert_eq!(w.supporting_paths, 2);
        assert_eq!(w.opposing_paths, 1);
        assert!(w.convergence_nodes.contains(&1) || w.convergence_nodes.contains(&3));
        assert!(w.is_overlapping());
    }
}
