use crate::types::{Relation, RelationKind};
use crate::world::WorldGraph;

#[derive(Clone, Debug, PartialEq)]
pub struct CausalPath {
    pub nodes: Vec<u64>,
    pub score: f32,
    pub inhibited: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReasoningVerdict {
    pub target: u64,
    pub support: f32,
    pub opposition: f32,
    pub confidence: f32,
    pub contradicted: bool,
    pub best_path: Option<CausalPath>,
}

#[derive(Clone, Debug)]
pub struct CausalReasoner {
    max_depth: usize,
    beam: usize,
}

impl Default for CausalReasoner {
    fn default() -> Self {
        Self {
            max_depth: 4,
            beam: 12,
        }
    }
}

impl CausalReasoner {
    pub fn new(max_depth: usize, beam: usize) -> Self {
        Self {
            max_depth: max_depth.clamp(1, 8),
            beam: beam.clamp(1, 32),
        }
    }

    pub fn infer(&self, world: &WorldGraph, target: u64) -> ReasoningVerdict {
        let mut frontier: Vec<CausalPath> = world
            .edges()
            .iter()
            .filter(|r| r.to == target && causal(r.kind))
            .map(|r| CausalPath {
                nodes: vec![r.from, r.to],
                score: edge_score(r),
                inhibited: matches!(r.kind, RelationKind::Inhibits),
            })
            .collect();

        frontier.sort_by(score_desc);
        frontier.truncate(self.beam);

        let mut all = frontier.clone();

        for _ in 1..self.max_depth {
            let mut next = Vec::new();
            for path in &frontier {
                let Some(&head) = path.nodes.first() else {
                    continue;
                };
                for r in world.edges().iter().filter(|r| r.to == head && causal(r.kind)) {
                    if path.nodes.contains(&r.from) {
                        continue;
                    }
                    let mut nodes = Vec::with_capacity(path.nodes.len() + 1);
                    nodes.push(r.from);
                    nodes.extend_from_slice(&path.nodes);
                    next.push(CausalPath {
                        nodes,
                        score: (path.score * edge_score(r)).clamp(0.0, 1.0),
                        inhibited: path.inhibited ^ matches!(r.kind, RelationKind::Inhibits),
                    });
                }
            }
            if next.is_empty() {
                break;
            }
            next.sort_by(score_desc);
            next.truncate(self.beam);
            all.extend(next.clone());
            frontier = next;
        }

        let mut support = 0.0f32;
        let mut opposition = 0.0f32;
        for path in &all {
            if path.inhibited {
                opposition = noisy_or(opposition, path.score);
            } else {
                support = noisy_or(support, path.score);
            }
        }

        let contradicted = support > 0.15 && opposition > 0.15;
        let separation = (support - opposition).abs();
        let confidence = if contradicted {
            (separation * 0.65).clamp(0.0, 1.0)
        } else {
            support.max(opposition).clamp(0.0, 1.0)
        };

        let best_path = all.into_iter().max_by(|a, b| {
            explanatory_rank(a)
                .partial_cmp(&explanatory_rank(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        ReasoningVerdict {
            target,
            support,
            opposition,
            confidence,
            contradicted,
            best_path,
        }
    }

    pub fn revise_with_relation(
        &self,
        world: &mut WorldGraph,
        relation: Relation,
        target: u64,
    ) -> ReasoningVerdict {
        world.relate(relation);
        self.infer(world, target)
    }
}

fn causal(kind: RelationKind) -> bool {
    matches!(
        kind,
        RelationKind::Causes
            | RelationKind::Enables
            | RelationKind::Inhibits
            | RelationKind::Follows
    )
}

fn edge_score(r: &Relation) -> f32 {
    (r.strength * r.confidence).clamp(0.0, 1.0)
}

fn noisy_or(current: f32, evidence: f32) -> f32 {
    (1.0 - (1.0 - current) * (1.0 - evidence)).clamp(0.0, 1.0)
}

fn explanatory_rank(path: &CausalPath) -> f32 {
    let depth_bonus = 1.0 + 0.15 * path.nodes.len().saturating_sub(2) as f32;
    path.score * depth_bonus
}

fn score_desc(a: &CausalPath, b: &CausalPath) -> std::cmp::Ordering {
    b.score
        .partial_cmp(&a.score)
        .unwrap_or(std::cmp::Ordering::Equal)
}
