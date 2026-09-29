use crate::types::{RelationKind, SenseGate, WorldLevel};
use crate::world::WorldGraph;

const MAX_ABSTRACT_CONCEPTS: usize = 32;
const MAX_MEMBERS: usize = 16;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RoleSignature {
    pub out_causes: u8,
    pub out_enables: u8,
    pub out_inhibits: u8,
    pub in_causes: u8,
    pub in_enables: u8,
    pub in_inhibits: u8,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AbstractConcept {
    pub id: u64,
    pub signature: RoleSignature,
    pub members: Vec<u64>,
    pub confidence: f32,
}

#[derive(Clone, Debug, Default)]
pub struct HierarchicalAbstraction {
    concepts: Vec<AbstractConcept>,
}

impl HierarchicalAbstraction {
    pub fn discover(&mut self, world: &WorldGraph) -> usize {
        let mut groups: Vec<(RoleSignature, Vec<u64>)> = Vec::new();
        for node in world.nodes() {
            let sig = signature(world, node.id);
            if signal_mass(&sig) < 2 {
                continue;
            }
            if let Some((_, members)) = groups.iter_mut().find(|(s, _)| *s == sig) {
                if members.len() < MAX_MEMBERS {
                    members.push(node.id);
                }
            } else {
                groups.push((sig, vec![node.id]));
            }
        }

        let mut added = 0usize;
        for (sig, mut members) in groups {
            members.sort_unstable();
            members.dedup();
            if members.len() < 2 {
                continue;
            }
            let id = abstract_id(&sig);
            let confidence = (0.58 + members.len().min(8) as f32 * 0.05).min(0.94);
            if let Some(existing) = self.concepts.iter_mut().find(|c| c.id == id) {
                existing.members = members;
                existing.confidence = confidence;
            } else if self.concepts.len() < MAX_ABSTRACT_CONCEPTS {
                self.concepts.push(AbstractConcept {
                    id,
                    signature: sig,
                    members,
                    confidence,
                });
                added += 1;
            }
        }
        added
    }

    pub fn concept_for(&self, node: u64) -> Option<&AbstractConcept> {
        self.concepts.iter().find(|c| c.members.contains(&node))
    }

    pub fn concepts(&self) -> &[AbstractConcept] {
        &self.concepts
    }

    pub fn len(&self) -> usize {
        self.concepts.len()
    }

    pub fn is_empty(&self) -> bool {
        self.concepts.is_empty()
    }
}

fn signature(world: &WorldGraph, node: u64) -> RoleSignature {
    let mut s = RoleSignature {
        out_causes: 0,
        out_enables: 0,
        out_inhibits: 0,
        in_causes: 0,
        in_enables: 0,
        in_inhibits: 0,
    };
    for e in world.edges() {
        let (out, kind) = if e.from == node {
            (true, e.kind)
        } else if e.to == node {
            (false, e.kind)
        } else {
            continue;
        };
        let slot = match (out, kind) {
            (true, RelationKind::Causes) => &mut s.out_causes,
            (true, RelationKind::Enables) => &mut s.out_enables,
            (true, RelationKind::Inhibits) => &mut s.out_inhibits,
            (false, RelationKind::Causes) => &mut s.in_causes,
            (false, RelationKind::Enables) => &mut s.in_enables,
            (false, RelationKind::Inhibits) => &mut s.in_inhibits,
            _ => continue,
        };
        *slot = (*slot).saturating_add(1).min(4);
    }
    s
}

fn signal_mass(s: &RoleSignature) -> u16 {
    u16::from(s.out_causes)
        + u16::from(s.out_enables)
        + u16::from(s.out_inhibits)
        + u16::from(s.in_causes)
        + u16::from(s.in_enables)
        + u16::from(s.in_inhibits)
}

fn abstract_id(s: &RoleSignature) -> u64 {
    let bytes = [
        s.out_causes,
        s.out_enables,
        s.out_inhibits,
        s.in_causes,
        s.in_enables,
        s.in_inhibits,
        WorldLevel::DaiThien as u8,
        SenseGate::Mind as u8,
    ];
    let mut h = 0xcbf29ce484222325u64;
    for b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}
