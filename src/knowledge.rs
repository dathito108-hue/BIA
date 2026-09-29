#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProvenanceKind {
    User,
    SharedText,
    LocalDocument,
    WebExcerpt,
    System,
}

#[derive(Clone, Debug, PartialEq)]
pub struct KnowledgeRecord {
    pub id: u64,
    pub source: String,
    pub kind: ProvenanceKind,
    pub excerpt: String,
    pub timestamp: u64,
    pub confidence: f32,
}

#[derive(Clone, Debug)]
pub struct KnowledgeLedger {
    capacity: usize,
    records: Vec<KnowledgeRecord>,
}

impl KnowledgeLedger {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self { capacity, records: Vec::new() }
    }

    pub fn add(&mut self, record: KnowledgeRecord) {
        if self.records.len() >= self.capacity {
            self.records.remove(0);
        }
        self.records.push(record);
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn recent(&self) -> Option<&KnowledgeRecord> {
        self.records.last()
    }

    pub fn records(&self) -> impl Iterator<Item = &KnowledgeRecord> {
        self.records.iter()
    }
}

pub fn encode_record(record: &KnowledgeRecord) -> String {
    format!(
        "{}\t{}\t{}\t{}\t{}\t{}",
        record.id,
        kind_code(&record.kind),
        record.timestamp,
        record.confidence,
        esc(&record.source),
        esc(&record.excerpt)
    )
}

pub fn decode_record(line: &str) -> Option<KnowledgeRecord> {
    let parts: Vec<&str> = line.split('\t').collect();
    if parts.len() < 6 {
        return None;
    }
    Some(KnowledgeRecord {
        id: parts[0].parse().ok()?,
        kind: parse_kind(parts[1])?,
        timestamp: parts[2].parse().ok()?,
        confidence: parts[3].parse::<f32>().ok()?.clamp(0.0, 1.0),
        source: unesc(parts[4]),
        excerpt: unesc(parts[5]),
    })
}

fn kind_code(kind: &ProvenanceKind) -> &'static str {
    match kind {
        ProvenanceKind::User => "user",
        ProvenanceKind::SharedText => "share",
        ProvenanceKind::LocalDocument => "doc",
        ProvenanceKind::WebExcerpt => "web",
        ProvenanceKind::System => "system",
    }
}

fn parse_kind(code: &str) -> Option<ProvenanceKind> {
    match code {
        "user" => Some(ProvenanceKind::User),
        "share" => Some(ProvenanceKind::SharedText),
        "doc" => Some(ProvenanceKind::LocalDocument),
        "web" => Some(ProvenanceKind::WebExcerpt),
        "system" => Some(ProvenanceKind::System),
        _ => None,
    }
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\t', "\\t").replace('\n', "\\n")
}

fn unesc(s: &str) -> String {
    s.replace("\\n", "\n").replace("\\t", "\t").replace("\\\\", "\\")
}
