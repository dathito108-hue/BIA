use crate::types::{Phenomenon, Relation, RelationKind, WorldLevel};

#[derive(Clone, Debug)]
pub struct WorldGraph {
    max_nodes: usize,
    max_edges: usize,
    nodes: Vec<Phenomenon>,
    edges: Vec<Relation>,
}

impl WorldGraph {
    pub fn new(max_nodes: usize, max_edges: usize) -> Self {
        assert!(max_nodes > 0 && max_edges > 0);
        Self {
            max_nodes,
            max_edges,
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn from_parts(
        max_nodes: usize,
        max_edges: usize,
        mut nodes: Vec<Phenomenon>,
        mut edges: Vec<Relation>,
    ) -> Self {
        assert!(max_nodes > 0 && max_edges > 0);
        nodes.truncate(max_nodes);
        edges.retain(|e| {
            nodes.iter().any(|n| n.id == e.from) && nodes.iter().any(|n| n.id == e.to)
        });
        edges.truncate(max_edges);
        Self {
            max_nodes,
            max_edges,
            nodes,
            edges,
        }
    }

    pub fn upsert(&mut self, p: Phenomenon) {
        if let Some(x) = self.nodes.iter_mut().find(|x| x.id == p.id) {
            *x = p;
            return;
        }
        if self.nodes.len() >= self.max_nodes {
            let i = self
                .nodes
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    a.salience
                        .partial_cmp(&b.salience)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(i, _)| i)
                .unwrap_or(0);
            let removed = self.nodes.remove(i).id;
            self.edges.retain(|e| e.from != removed && e.to != removed);
        }
        self.nodes.push(p);
    }

    pub fn relate(&mut self, r: Relation) {
        if let Some(existing) = self
            .edges
            .iter_mut()
            .find(|e| e.from == r.from && e.to == r.to && e.kind == r.kind)
        {
            existing.strength = (existing.strength * 0.8 + r.strength * 0.2).clamp(0.0, 1.0);
            existing.confidence =
                (existing.confidence * 0.8 + r.confidence * 0.2).clamp(0.0, 1.0);
            return;
        }
        if self.edges.len() >= self.max_edges {
            let i = self
                .edges
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    a.strength
                        .partial_cmp(&b.strength)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.edges.remove(i);
        }
        self.edges.push(r);
    }

    pub fn activate(&self, focus: &Phenomenon, limit: usize) -> Vec<Phenomenon> {
        let mut scored: Vec<(f32, &Phenomenon)> = self
            .nodes
            .iter()
            .map(|p| {
                let level = match (focus.level, p.level) {
                    (a, b) if a == b => 1.0,
                    (_, WorldLevel::TrungThien) => 0.85,
                    (_, WorldLevel::DaiThien) => 0.7,
                    _ => 0.75,
                };
                let kind = if focus.kind == p.kind { 1.0 } else { 0.5 };
                let linked = self
                    .edges
                    .iter()
                    .filter(|e| {
                        (e.from == focus.id && e.to == p.id)
                            || (e.to == focus.id && e.from == p.id)
                    })
                    .map(|e| e.strength * e.confidence)
                    .fold(0.0_f32, f32::max);
                (
                    0.35 * p.salience
                        + 0.25 * p.confidence
                        + 0.15 * level
                        + 0.15 * kind
                        + 0.10 * linked,
                    p,
                )
            })
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored
            .into_iter()
            .take(limit)
            .map(|(_, p)| p.clone())
            .collect()
    }

    pub fn causes_for(&self, id: u64, limit: usize) -> Vec<Relation> {
        let mut xs: Vec<Relation> = self
            .edges
            .iter()
            .filter(|e| {
                e.to == id
                    && matches!(
                        e.kind,
                        RelationKind::Causes | RelationKind::Enables | RelationKind::Inhibits
                    )
            })
            .cloned()
            .collect();
        xs.sort_by(|a, b| {
            (b.strength * b.confidence)
                .partial_cmp(&(a.strength * a.confidence))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        xs.truncate(limit);
        xs
    }

    pub fn nodes(&self) -> &[Phenomenon] {
        &self.nodes
    }

    pub fn edges(&self) -> &[Relation] {
        &self.edges
    }

    pub fn max_nodes(&self) -> usize {
        self.max_nodes
    }

    pub fn max_edges(&self) -> usize {
        self.max_edges
    }

    pub fn node(&self, id: u64) -> Option<&Phenomenon> {
        self.nodes.iter().find(|p| p.id == id)
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
