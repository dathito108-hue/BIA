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

#[derive(Clone, Debug, PartialEq)]
pub struct CounterfactualVerdict {
    pub factual: ReasoningVerdict,
    pub counterfactual: ReasoningVerdict,
    pub removed_node: u64,
    pub support_delta: f32,
    pub opposition_delta: f32,
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
        self.infer_filtered(world, target, None, None)
    }

    /// Query-scoped search: only paths starting at `source` can support or
    /// oppose this claim. A stronger cause elsewhere is irrelevant to A -> B.
    /// Work is capped by depth * beam * stored edges; no unbounded frontier.
    pub fn infer_between(&self, world: &WorldGraph, source: u64, target: u64) -> ReasoningVerdict {
        self.infer_between_with_paths(world, source, target).0
    }

    pub fn infer_between_with_paths(
        &self,
        world: &WorldGraph,
        source: u64,
        target: u64,
    ) -> (ReasoningVerdict, Vec<CausalPath>) {
        let mut frontier = vec![CausalPath {
            nodes: vec![source],
            score: 1.0,
            inhibited: false,
        }];
        let mut support = 0.0_f32;
        let mut opposition = 0.0_f32;
        let mut best_path: Option<CausalPath> = None;
        let mut paths: Vec<CausalPath> = Vec::new();
        for _ in 0..self.max_depth {
            let mut next = Vec::new();
            for path in &frontier {
                let head = *path.nodes.last().expect("nonempty causal path");
                for edge in world
                    .edges()
                    .iter()
                    .filter(|r| r.from == head && causal(r.kind))
                {
                    if path.nodes.contains(&edge.to) {
                        continue;
                    }
                    let edge_strength = edge_score(edge);
                    if !edge_strength.is_finite() || edge_strength <= 0.0 {
                        continue;
                    }
                    let score = path.score.min(edge_strength);
                    if score <= 0.0 {
                        continue;
                    }
                    let mut candidate = path.clone();
                    candidate.nodes.push(edge.to);
                    candidate.score = score;
                    candidate.inhibited ^= matches!(edge.kind, RelationKind::Inhibits);
                    if edge.to == target {
                        // Paths often share learned edges. Without independence
                        // provenance, noisy-OR would double-count the evidence.
                        if candidate.inhibited {
                            opposition = opposition.max(score);
                        } else {
                            support = support.max(score);
                        }
                        if best_path.as_ref().is_none_or(|old| score > old.score) {
                            best_path = Some(candidate.clone());
                        }
                        paths.push(candidate);
                        paths.sort_by(score_desc);
                        paths.truncate(self.beam);
                    } else {
                        next.push(candidate);
                        next.sort_by(score_desc);
                        next.truncate(self.beam);
                    }
                }
            }
            if next.is_empty() {
                break;
            }
            frontier = next;
        }
        let contradicted = support > 0.15 && opposition > 0.15;
        let confidence = if contradicted {
            (support - opposition).abs() * 0.65
        } else {
            support.max(opposition)
        };
        (
            ReasoningVerdict {
                target,
                support,
                opposition,
                confidence,
                contradicted,
                best_path,
            },
            paths,
        )
    }

    pub fn infer_without_node(
        &self,
        world: &WorldGraph,
        target: u64,
        removed_node: u64,
    ) -> ReasoningVerdict {
        let roots = causal_roots(world);
        self.infer_filtered(world, target, Some(removed_node), Some(&roots))
    }

    pub fn counterfactual_without(
        &self,
        world: &WorldGraph,
        target: u64,
        removed_node: u64,
    ) -> CounterfactualVerdict {
        let roots = causal_roots(world);
        let factual = self.infer_filtered(world, target, None, Some(&roots));
        let counterfactual = self.infer_filtered(world, target, Some(removed_node), Some(&roots));
        CounterfactualVerdict {
            support_delta: factual.support - counterfactual.support,
            opposition_delta: factual.opposition - counterfactual.opposition,
            factual,
            counterfactual,
            removed_node,
        }
    }

    fn infer_filtered(
        &self,
        world: &WorldGraph,
        target: u64,
        removed_node: Option<u64>,
        fixed_roots: Option<&[u64]>,
    ) -> ReasoningVerdict {
        let mut frontier: Vec<CausalPath> = world
            .edges()
            .iter()
            .filter(|r| {
                r.to == target
                    && causal(r.kind)
                    && removed_node.is_none_or(|x| r.from != x && r.to != x)
            })
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
                for r in world.edges().iter().filter(|r| {
                    r.to == head
                        && causal(r.kind)
                        && removed_node.is_none_or(|x| r.from != x && r.to != x)
                }) {
                    if path.nodes.contains(&r.from) {
                        continue;
                    }
                    let mut nodes = Vec::with_capacity(path.nodes.len() + 1);
                    nodes.push(r.from);
                    nodes.extend_from_slice(&path.nodes);
                    next.push(CausalPath {
                        nodes,
                        score: path.score.min(edge_score(r)),
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

        let completed: Vec<CausalPath> = all
            .iter()
            .filter(|path| {
                let Some(&head) = path.nodes.first() else {
                    return false;
                };
                if let Some(roots) = fixed_roots {
                    return roots.contains(&head);
                }
                let has_parent = world.edges().iter().any(|r| {
                    r.to == head
                        && causal(r.kind)
                        && removed_node.is_none_or(|x| r.from != x && r.to != x)
                });
                !has_parent || path.nodes.len() > self.max_depth
            })
            .cloned()
            .collect();
        let enforce_roots = fixed_roots.is_some();
        let scored_paths = if completed.is_empty() && !enforce_roots {
            &all
        } else {
            &completed
        };

        let mut support = 0.0f32;
        let mut opposition = 0.0f32;
        for path in scored_paths {
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

        let best_path = scored_paths.iter().cloned().max_by(|a, b| {
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

fn causal_roots(world: &WorldGraph) -> Vec<u64> {
    let mut roots = Vec::new();
    for edge in world.edges().iter().filter(|r| causal(r.kind)) {
        let candidate = edge.from;
        let has_parent = world
            .edges()
            .iter()
            .any(|r| r.to == candidate && causal(r.kind));
        if !has_parent && !roots.contains(&candidate) {
            roots.push(candidate);
        }
    }
    roots
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
