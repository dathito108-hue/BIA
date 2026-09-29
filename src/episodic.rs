use crate::semantic::SemanticScene;
use crate::types::RelationKind;

const MAX_EPISODES: usize = 128;
const MAX_CLAUSES: usize = 12;

#[derive(Clone, Debug, PartialEq)]
pub struct EpisodeClause {
    pub from: u64,
    pub to: u64,
    pub kind: RelationKind,
    pub confidence: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Episode {
    pub timestamp: u64,
    pub clauses: Vec<EpisodeClause>,
    pub utility: f32,
}

#[derive(Clone, Debug, Default)]
pub struct EpisodicMemory {
    episodes: Vec<Episode>,
}

impl EpisodicMemory {
    pub fn observe(&mut self, scene: &SemanticScene, timestamp: u64, utility: f32) {
        if scene.clauses.is_empty() {
            return;
        }
        let clauses = scene
            .clauses
            .iter()
            .take(MAX_CLAUSES)
            .map(|c| EpisodeClause {
                from: c.subject.id,
                to: c.object.id,
                kind: c.kind,
                confidence: c.confidence,
            })
            .collect();
        if self.episodes.len() >= MAX_EPISODES {
            let idx = self
                .episodes
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    retention(a)
                        .partial_cmp(&retention(b))
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(i, _)| i)
                .unwrap_or(0);
            self.episodes.remove(idx);
        }
        self.episodes.push(Episode {
            timestamp,
            clauses,
            utility: utility.clamp(-1.0, 1.0),
        });
    }

    pub fn episodes(&self) -> &[Episode] {
        &self.episodes
    }

    pub fn len(&self) -> usize {
        self.episodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.episodes.is_empty()
    }
}

fn retention(e: &Episode) -> f32 {
    e.utility.abs() * 0.35 + e.clauses.len() as f32 * 0.05 + (e.timestamp as f32).ln_1p() * 0.02
}
