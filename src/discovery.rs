use crate::types::{Relation, RelationKind};
use crate::world::WorldGraph;

#[derive(Clone, Debug, PartialEq)]
pub struct DiscoveredSimilarity {
    pub relation: Relation,
    pub shared_contexts: usize,
}

#[derive(Clone, Debug)]
pub struct ContextDiscovery {
    min_shared: usize,
    max_candidates: usize,
}

impl Default for ContextDiscovery {
    fn default() -> Self {
        Self {
            min_shared: 2,
            max_candidates: 16,
        }
    }
}

impl ContextDiscovery {
    pub fn discover(&self, world: &WorldGraph) -> Vec<DiscoveredSimilarity> {
        let mut nodes: Vec<u64> = world.nodes().iter().map(|n| n.id).collect();
        nodes.sort_unstable();
        nodes.dedup();
        let mut out = Vec::new();

        for i in 0..nodes.len() {
            for j in (i + 1)..nodes.len() {
                let a = nodes[i];
                let b = nodes[j];
                if already_similar(world, a, b) {
                    continue;
                }
                let shared = shared_contexts(world, a, b);
                if shared < self.min_shared {
                    continue;
                }
                let confidence = (0.60 + 0.08 * shared.min(4) as f32).min(0.92);
                out.push(DiscoveredSimilarity {
                    relation: Relation {
                        from: a,
                        to: b,
                        kind: RelationKind::Similar,
                        strength: confidence,
                        confidence,
                    },
                    shared_contexts: shared,
                });
                if out.len() >= self.max_candidates {
                    return out;
                }
            }
        }
        out
    }

    pub fn apply(&self, world: &mut WorldGraph) -> usize {
        let xs = self.discover(world);
        let n = xs.len();
        for x in xs {
            world.relate(x.relation);
        }
        n
    }
}

fn already_similar(world: &WorldGraph, a: u64, b: u64) -> bool {
    world.edges().iter().any(|e| {
        e.kind == RelationKind::Similar
            && ((e.from == a && e.to == b) || (e.from == b && e.to == a))
    })
}

fn shared_contexts(world: &WorldGraph, a: u64, b: u64) -> usize {
    let mut shared = 0usize;
    for ea in world.edges().iter().filter(|e| structural(e.kind)) {
        let sig_a = signature(ea, a);
        let Some(sig_a) = sig_a else { continue };
        if world
            .edges()
            .iter()
            .filter(|e| structural(e.kind))
            .any(|eb| signature(eb, b).is_some_and(|sig_b| sig_b == sig_a))
        {
            shared += 1;
        }
    }
    shared.min(8)
}

fn signature(e: &Relation, node: u64) -> Option<(bool, u64, RelationKind)> {
    if e.from == node {
        Some((true, e.to, e.kind))
    } else if e.to == node {
        Some((false, e.from, e.kind))
    } else {
        None
    }
}

fn structural(kind: RelationKind) -> bool {
    matches!(kind, RelationKind::Causes | RelationKind::Enables | RelationKind::Inhibits)
}
