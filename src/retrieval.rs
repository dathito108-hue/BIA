use crate::knowledge::{KnowledgeLedger, KnowledgeRecord};

#[derive(Clone, Debug, PartialEq)]
pub struct KnowledgeHit {
    pub record: KnowledgeRecord,
    pub score: f32,
}

#[derive(Clone, Debug, Default)]
pub struct SemanticRetriever;

impl SemanticRetriever {
    pub fn recall(
        &self,
        ledger: &KnowledgeLedger,
        query: &str,
        limit: usize,
    ) -> Vec<KnowledgeHit> {
        let query_tokens = tokens(query);
        if query_tokens.is_empty() || limit == 0 {
            return Vec::new();
        }

        let mut hits: Vec<KnowledgeHit> = ledger
            .records()
            .filter_map(|record| {
                let record_tokens = tokens(&record.excerpt);
                let overlap = overlap_score(&query_tokens, &record_tokens);
                if overlap <= 0.0 {
                    return None;
                }
                let confidence = record.confidence.clamp(0.0, 1.0);
                Some(KnowledgeHit {
                    record: record.clone(),
                    score: (overlap * 0.8 + confidence * 0.2).clamp(0.0, 1.0),
                })
            })
            .collect();

        hits.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        hits.truncate(limit.min(8));
        hits
    }
}

fn tokens(text: &str) -> Vec<String> {
    normalize(text)
        .split_whitespace()
        .filter(|t| t.chars().count() >= 2)
        .take(64)
        .map(ToString::to_string)
        .collect()
}

fn overlap_score(query: &[String], record: &[String]) -> f32 {
    if query.is_empty() || record.is_empty() {
        return 0.0;
    }
    let mut matched = 0usize;
    for q in query {
        if record.iter().any(|r| r == q) {
            matched += 1;
        }
    }
    let coverage = matched as f32 / query.len() as f32;
    let precision = matched as f32 / record.len().min(32).max(1) as f32;
    (coverage * 0.75 + precision * 0.25).clamp(0.0, 1.0)
}

fn normalize(s: &str) -> String {
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
            c if c.is_alphanumeric() || c.is_whitespace() => c,
            _ => ' ',
        })
        .collect()
}
