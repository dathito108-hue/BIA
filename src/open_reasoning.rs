use crate::duyen_weave::DuyenWeave;
use crate::reasoning::{CausalReasoner, CounterfactualVerdict, ReasoningVerdict};
use crate::semantic::{QueryKind, SemanticQuery, SemanticScene, VietnameseSemanticParser};
use crate::world::WorldGraph;

#[derive(Clone, Debug, PartialEq)]
pub enum OpenAnswer {
    Supported {
        confidence: f32,
        path: Vec<u64>,
    },
    Opposed {
        confidence: f32,
        path: Vec<u64>,
    },
    Contradicted {
        support: f32,
        opposition: f32,
    },
    Counterfactual {
        support_delta: f32,
        factual_support: f32,
        counterfactual_support: f32,
    },
    Unknown,
}

#[derive(Clone, Debug)]
pub struct SemanticReasoner {
    parser: VietnameseSemanticParser,
    causal: CausalReasoner,
}

impl Default for SemanticReasoner {
    fn default() -> Self {
        Self {
            parser: VietnameseSemanticParser,
            causal: CausalReasoner::new(6, 16),
        }
    }
}

impl SemanticReasoner {
    pub fn parse(&self, text: &str) -> SemanticScene {
        self.parser.parse(text)
    }

    pub fn ingest(&self, world: &mut WorldGraph, text: &str, timestamp: u64) -> SemanticScene {
        let scene = self.parser.parse(text);
        self.parser.ingest(world, &scene, timestamp);
        scene
    }

    pub fn answer_scene(&self, world: &WorldGraph, scene: &SemanticScene) -> OpenAnswer {
        self.answer_scene_with_weave(world, scene).0
    }

    pub fn answer_scene_with_weave(
        &self,
        world: &WorldGraph,
        scene: &SemanticScene,
    ) -> (OpenAnswer, DuyenWeave) {
        let Some(query) = &scene.query else {
            return (OpenAnswer::Unknown, DuyenWeave::default());
        };
        self.answer_query_with_weave(world, query)
    }

    pub fn answer_query(&self, world: &WorldGraph, query: &SemanticQuery) -> OpenAnswer {
        self.answer_query_with_weave(world, query).0
    }

    pub fn answer_query_with_weave(
        &self,
        world: &WorldGraph,
        query: &SemanticQuery,
    ) -> (OpenAnswer, DuyenWeave) {
        match query.kind {
            QueryKind::Causal => {
                let verdict = self
                    .causal
                    .infer_between(world, query.subject.id, query.object.id);
                answer_causal(verdict, query.subject.id)
            }
            QueryKind::CounterfactualWithout => {
                let verdict =
                    self.causal
                        .counterfactual_without(world, query.object.id, query.subject.id);
                (answer_counterfactual(verdict), DuyenWeave::default())
            }
        }
    }

    pub fn ingest_and_answer(
        &self,
        world: &mut WorldGraph,
        text: &str,
        timestamp: u64,
    ) -> OpenAnswer {
        let scene = self.ingest(world, text, timestamp);
        self.answer_scene(world, &scene)
    }
}

fn answer_causal(verdict: ReasoningVerdict, source: u64) -> (OpenAnswer, DuyenWeave) {
    let weave = DuyenWeave::from_paths(&verdict.paths, verdict.target);
    if verdict.contradicted {
        return (
            OpenAnswer::Contradicted {
                support: verdict.support,
                opposition: verdict.opposition,
            },
            weave,
        );
    }

    let Some(path) = verdict.best_path else {
        return (OpenAnswer::Unknown, weave);
    };
    if !path.nodes.contains(&source) {
        return (OpenAnswer::Unknown, weave);
    }

    let answer = if path.inhibited || verdict.opposition > verdict.support {
        OpenAnswer::Opposed {
            confidence: verdict.confidence,
            path: path.nodes,
        }
    } else {
        OpenAnswer::Supported {
            confidence: verdict.confidence,
            path: path.nodes,
        }
    };
    (answer, weave)
}

fn answer_counterfactual(verdict: CounterfactualVerdict) -> OpenAnswer {
    OpenAnswer::Counterfactual {
        support_delta: verdict.support_delta,
        factual_support: verdict.factual.support,
        counterfactual_support: verdict.counterfactual.support,
    }
}
