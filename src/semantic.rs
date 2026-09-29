use crate::types::{Phenomenon, Relation, RelationKind, SenseGate, WorldLevel};
use crate::world::WorldGraph;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticEntity {
    pub id: u64,
    pub text: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SemanticClause {
    pub subject: SemanticEntity,
    pub object: SemanticEntity,
    pub kind: RelationKind,
    pub confidence: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryKind {
    Causal,
    CounterfactualWithout,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SemanticQuery {
    pub kind: QueryKind,
    pub subject: SemanticEntity,
    pub object: SemanticEntity,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SemanticScene {
    pub clauses: Vec<SemanticClause>,
    pub query: Option<SemanticQuery>,
}

#[derive(Clone, Debug, Default)]
pub struct VietnameseSemanticParser;

impl VietnameseSemanticParser {
    pub fn parse(&self, text: &str) -> SemanticScene {
        let normalized = normalize(text);
        let mut clauses = Vec::new();
        let mut query = None;

        for sentence in split_sentences(&normalized) {
            let s = sentence.trim();
            if s.is_empty() {
                continue;
            }

            if let Some(q) = parse_counterfactual_query(s) {
                query = Some(q);
                continue;
            }
            if let Some(q) = parse_causal_query(s) {
                query = Some(q);
                continue;
            }
            if let Some(c) = parse_clause(s) {
                clauses.push(c);
            }
        }

        SemanticScene { clauses, query }
    }

    pub fn ingest(&self, world: &mut WorldGraph, scene: &SemanticScene, timestamp: u64) {
        for (index, clause) in scene.clauses.iter().enumerate() {
            let ts = timestamp.saturating_add(index as u64 * 2);
            upsert_entity(world, &clause.subject, ts);
            upsert_entity(world, &clause.object, ts.saturating_add(1));
            world.relate(Relation {
                from: clause.subject.id,
                to: clause.object.id,
                kind: clause.kind,
                strength: clause.confidence,
                confidence: clause.confidence,
            });
        }
        if let Some(q) = &scene.query {
            upsert_entity(world, &q.subject, timestamp.saturating_add(10_000));
            upsert_entity(world, &q.object, timestamp.saturating_add(10_001));
        }
    }
}

fn parse_clause(s: &str) -> Option<SemanticClause> {
    // Absence of a cause is not an inhibiting mechanism. Keep unsupported
    // negation out of the positive graph rather than inventing a causal edge.
    if unsupported_assertion(s) { return None; }
    if let Some((effect, cause)) = split_once_nonempty(s, " la do ") {
        return clause(cause, effect, RelationKind::Causes, 0.90);
    }
    if let Some((a, b)) = split_once_nonempty(s, " neu ") {
        if let Some((condition, consequence)) = split_once_nonempty(&format!("{a} neu {b}"), " thi ") {
            return clause(condition, consequence, RelationKind::Enables, 0.88);
        }
    }

    if let Some(rest) = s.strip_prefix("neu ") {
        if let Some((a, b)) = split_once_nonempty(rest, " thi ") {
            return clause(a, b, RelationKind::Enables, 0.92);
        }
    }

    if let Some(rest) = s.strip_prefix("vi ") {
        if let Some((a, b)) = split_once_nonempty(rest, " nen ") {
            return clause(a, b, RelationKind::Causes, 0.95);
        }
    }

    for marker in [" gay ra ", " dan den ", " dan toi ", " keo theo ", " la nguyen nhan cua ", " lam cho ", " khien "] {
        if let Some((a, b)) = split_once_nonempty(s, marker) {
            return clause(a, b, RelationKind::Causes, 0.93);
        }
    }

    for marker in [" ngan can ", " ngan ", " can tro ", " vo hieu hoa "] {
        if let Some((a, b)) = split_once_nonempty(s, marker) {
            return clause(a, b, RelationKind::Inhibits, 0.92);
        }
    }

    for marker in [" cho phep ", " tao dieu kien cho ", " ho tro "] {
        if let Some((a, b)) = split_once_nonempty(s, marker) {
            return clause(a, b, RelationKind::Enables, 0.88);
        }
    }

    for marker in [" giong ", " tuong tu ", " gan giong "] {
        if let Some((a, b)) = split_once_nonempty(s, marker) {
            return clause(a, b, RelationKind::Similar, 0.90);
        }
    }

    None
}

fn parse_causal_query(s: &str) -> Option<SemanticQuery> {
    let question = s.trim_end_matches('?').trim();

    for marker in [" co gay ra ", " co dan den ", " co dan toi ", " co keo theo ", " co la nguyen nhan cua ", " co lam cho ", " co khien "] {
        if let Some((a, right)) = split_once_nonempty(question, marker) {
            let b = right
                .strip_suffix(" khong")
                .unwrap_or(right)
                .trim();
            if !a.is_empty() && !b.is_empty() {
                return Some(SemanticQuery {
                    kind: QueryKind::Causal,
                    subject: entity(a),
                    object: entity(b),
                });
            }
        }
    }

    if let Some(rest) = question.strip_prefix("tai sao ") {
        return Some(SemanticQuery {
            kind: QueryKind::Causal,
            subject: entity("nguyen nhan"),
            object: entity(rest),
        });
    }

    None
}

fn parse_counterfactual_query(s: &str) -> Option<SemanticQuery> {
    let question = s.trim_end_matches('?').trim();
    for prefix in ["neu bo ", "neu khong co ", "neu loai bo "] {
        if let Some(rest) = question.strip_prefix(prefix) {
            if let Some((removed, target)) = split_once_nonempty(rest, " thi ") {
                return Some(SemanticQuery {
                    kind: QueryKind::CounterfactualWithout,
                    subject: entity(removed),
                    object: entity(target),
                });
            }
        }
    }
    None
}

fn clause(
    subject: &str,
    object: &str,
    kind: RelationKind,
    confidence: f32,
) -> Option<SemanticClause> {
    let subject = clean_phrase(subject);
    let object = clean_phrase(object);
    if subject.is_empty() || object.is_empty() {
        return None;
    }
    Some(SemanticClause {
        subject: entity(&subject),
        object: entity(&object),
        kind,
        confidence,
    })
}

fn entity(text: &str) -> SemanticEntity {
    let cleaned = clean_phrase(text);
    SemanticEntity {
        id: hash64(&cleaned),
        text: cleaned,
    }
}

fn upsert_entity(world: &mut WorldGraph, entity: &SemanticEntity, timestamp: u64) {
    world.upsert(Phenomenon::new(
        entity.id,
        WorldLevel::TrungThien,
        SenseGate::Mind,
        stable_kind(&entity.text),
        phrase_features(&entity.text),
        0.92,
        0.72,
        timestamp,
    ));
}

fn clean_phrase(s: &str) -> String {
    s.trim()
        .trim_matches(|c: char| matches!(c, ',' | ';' | ':' | '.' | '?' | '!'))
        .split_whitespace()
        .take(12)
        .collect::<Vec<_>>()
        .join(" ")
}

fn split_once_nonempty<'a>(s: &'a str, marker: &str) -> Option<(&'a str, &'a str)> {
    let (a, b) = s.split_once(marker)?;
    let a = a.trim();
    let b = b.trim();
    (!a.is_empty() && !b.is_empty()).then_some((a, b))
}

fn split_sentences(s: &str) -> impl Iterator<Item = &str> {
    s.split(['.', '?', '!', ';', '\n'])
}

pub fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| match c {
            'à'|'á'|'ạ'|'ả'|'ã'|'â'|'ầ'|'ấ'|'ậ'|'ẩ'|'ẫ'|'ă'|'ằ'|'ắ'|'ặ'|'ẳ'|'ẵ' => 'a',
            'è'|'é'|'ẹ'|'ẻ'|'ẽ'|'ê'|'ề'|'ế'|'ệ'|'ể'|'ễ' => 'e',
            'ì'|'í'|'ị'|'ỉ'|'ĩ' => 'i',
            'ò'|'ó'|'ọ'|'ỏ'|'õ'|'ô'|'ồ'|'ố'|'ộ'|'ổ'|'ỗ'|'ơ'|'ờ'|'ớ'|'ợ'|'ở'|'ỡ' => 'o',
            'ù'|'ú'|'ụ'|'ủ'|'ũ'|'ư'|'ừ'|'ứ'|'ự'|'ử'|'ữ' => 'u',
            'ỳ'|'ý'|'ỵ'|'ỷ'|'ỹ' => 'y',
            'đ' => 'd',
            c if c.is_alphanumeric() || c.is_whitespace() || matches!(c, '.'|'?'|'!'|';'|','|':') => c,
            _ => ' ',
        })
        .collect()
}

pub fn concept_id(text: &str) -> u64 {
    hash64(&clean_phrase(&normalize(text)))
}

fn stable_kind(s: &str) -> u32 {
    let h = hash64(s);
    (h ^ (h >> 32)) as u32
}

fn hash64(s: &str) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for b in s.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn phrase_features(s: &str) -> Vec<f32> {
    let mut v = vec![0.0; 8];
    for (i, b) in s.bytes().take(128).enumerate() {
        v[i & 7] += f32::from(b) / 255.0;
    }
    let scale = (s.len().max(1) as f32).sqrt();
    for x in &mut v {
        *x /= scale;
    }
    v
}

/// These expressions require truth/possibility operators not represented by a
/// positive causal edge. Callers must not send them through latent fallback.
pub fn unsupported_assertion(text: &str) -> bool {
    let text = normalize(text);
    ["khong gay", "khong dan", "khong keo", "khong lam", "khong khien",
     "khong phai", "chua chac", "co le", "gia thuyet:", "khong ngan", "khong can tro"]
        .iter().any(|marker| text.contains(marker))
}
