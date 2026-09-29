use crate::memory::{Seed, SeedMemory};
use crate::types::{Phenomenon, Relation, RelationKind, SenseGate, WorldLevel};
use crate::world::WorldGraph;

const MAGIC: &[u8; 8] = b"BIADCA01";
const VERSION: u16 = 1;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PersistenceError {
    Truncated,
    BadMagic,
    UnsupportedVersion,
    ChecksumMismatch,
    InvalidEnum,
    LimitExceeded,
}

#[derive(Clone, Debug)]
pub struct DharmaSnapshot {
    pub world: WorldGraph,
    pub memory: SeedMemory,
}

pub fn encode(snapshot: &DharmaSnapshot) -> Vec<u8> {
    let mut body = Vec::new();

    put_u32(&mut body, snapshot.world.max_nodes() as u32);
    put_u32(&mut body, snapshot.world.max_edges() as u32);
    put_u32(&mut body, snapshot.memory.capacity() as u32);
    put_u32(&mut body, snapshot.world.nodes().len() as u32);
    put_u32(&mut body, snapshot.world.edges().len() as u32);
    put_u32(&mut body, snapshot.memory.seeds().len() as u32);

    for p in snapshot.world.nodes() {
        put_u64(&mut body, p.id);
        body.push(world_level_to_u8(p.level));
        body.push(sense_gate_to_u8(p.gate));
        put_u32(&mut body, p.kind);
        put_f32(&mut body, p.confidence);
        put_f32(&mut body, p.salience);
        put_u64(&mut body, p.timestamp);
        put_f32_vec(&mut body, &p.features);
    }

    for r in snapshot.world.edges() {
        put_u64(&mut body, r.from);
        put_u64(&mut body, r.to);
        body.push(relation_kind_to_u8(r.kind));
        put_f32(&mut body, r.strength);
        put_f32(&mut body, r.confidence);
    }

    for s in snapshot.memory.seeds() {
        put_u32(&mut body, s.meaning);
        put_f32(&mut body, s.strength);
        put_f32(&mut body, s.utility);
        put_u32(&mut body, s.repetitions);
        put_u64(&mut body, s.last_seen);
        put_f32_vec(&mut body, &s.signature);
    }

    let checksum = fnv1a64(&body);
    let mut out = Vec::with_capacity(8 + 2 + 8 + body.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&VERSION.to_le_bytes());
    out.extend_from_slice(&checksum.to_le_bytes());
    out.extend_from_slice(&body);
    out
}

pub fn decode(bytes: &[u8]) -> Result<DharmaSnapshot, PersistenceError> {
    if bytes.len() < 18 {
        return Err(PersistenceError::Truncated);
    }
    if &bytes[0..8] != MAGIC {
        return Err(PersistenceError::BadMagic);
    }

    let version = u16::from_le_bytes([bytes[8], bytes[9]]);
    if version != VERSION {
        return Err(PersistenceError::UnsupportedVersion);
    }

    let stored_checksum = u64::from_le_bytes(
        bytes[10..18]
            .try_into()
            .map_err(|_| PersistenceError::Truncated)?,
    );
    let body = &bytes[18..];
    if fnv1a64(body) != stored_checksum {
        return Err(PersistenceError::ChecksumMismatch);
    }

    let mut rd = Reader::new(body);
    let max_nodes = rd.u32()? as usize;
    let max_edges = rd.u32()? as usize;
    let seed_capacity = rd.u32()? as usize;
    let node_count = rd.u32()? as usize;
    let edge_count = rd.u32()? as usize;
    let seed_count = rd.u32()? as usize;

    if max_nodes == 0
        || max_edges == 0
        || seed_capacity == 0
        || node_count > max_nodes
        || edge_count > max_edges
        || seed_count > seed_capacity
    {
        return Err(PersistenceError::LimitExceeded);
    }

    let mut nodes = Vec::with_capacity(node_count);
    for _ in 0..node_count {
        let id = rd.u64()?;
        let level = u8_to_world_level(rd.byte()?)?;
        let gate = u8_to_sense_gate(rd.byte()?)?;
        let kind = rd.u32()?;
        let confidence = rd.f32()?;
        let salience = rd.f32()?;
        let timestamp = rd.u64()?;
        let features = rd.f32_vec(65_536)?;
        nodes.push(Phenomenon::new(
            id,
            level,
            gate,
            kind,
            features,
            confidence,
            salience,
            timestamp,
        ));
    }

    let mut edges = Vec::with_capacity(edge_count);
    for _ in 0..edge_count {
        edges.push(Relation {
            from: rd.u64()?,
            to: rd.u64()?,
            kind: u8_to_relation_kind(rd.byte()?)?,
            strength: rd.f32()?.clamp(0.0, 1.0),
            confidence: rd.f32()?.clamp(0.0, 1.0),
        });
    }

    let mut seeds = Vec::with_capacity(seed_count);
    for _ in 0..seed_count {
        seeds.push(Seed {
            meaning: rd.u32()?,
            strength: rd.f32()?.clamp(0.0, 1.0),
            utility: rd.f32()?.clamp(-1.0, 1.0),
            repetitions: rd.u32()?,
            last_seen: rd.u64()?,
            signature: rd.f32_vec(65_536)?,
        });
    }

    if !rd.finished() {
        return Err(PersistenceError::LimitExceeded);
    }

    Ok(DharmaSnapshot {
        world: WorldGraph::from_parts(max_nodes, max_edges, nodes, edges),
        memory: SeedMemory::from_seeds(seed_capacity, seeds),
    })
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn put_u32(out: &mut Vec<u8>, x: u32) {
    out.extend_from_slice(&x.to_le_bytes());
}
fn put_u64(out: &mut Vec<u8>, x: u64) {
    out.extend_from_slice(&x.to_le_bytes());
}
fn put_f32(out: &mut Vec<u8>, x: f32) {
    out.extend_from_slice(&x.to_le_bytes());
}
fn put_f32_vec(out: &mut Vec<u8>, xs: &[f32]) {
    put_u32(out, xs.len() as u32);
    for &x in xs {
        put_f32(out, x);
    }
}

struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}
impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }
    fn take<const N: usize>(&mut self) -> Result<[u8; N], PersistenceError> {
        let end = self.pos.checked_add(N).ok_or(PersistenceError::Truncated)?;
        let slice = self
            .bytes
            .get(self.pos..end)
            .ok_or(PersistenceError::Truncated)?;
        self.pos = end;
        slice.try_into().map_err(|_| PersistenceError::Truncated)
    }
    fn byte(&mut self) -> Result<u8, PersistenceError> {
        Ok(self.take::<1>()?[0])
    }
    fn u32(&mut self) -> Result<u32, PersistenceError> {
        Ok(u32::from_le_bytes(self.take()?))
    }
    fn u64(&mut self) -> Result<u64, PersistenceError> {
        Ok(u64::from_le_bytes(self.take()?))
    }
    fn f32(&mut self) -> Result<f32, PersistenceError> {
        Ok(f32::from_le_bytes(self.take()?))
    }
    fn f32_vec(&mut self, hard_limit: usize) -> Result<Vec<f32>, PersistenceError> {
        let len = self.u32()? as usize;
        if len > hard_limit {
            return Err(PersistenceError::LimitExceeded);
        }
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            out.push(self.f32()?);
        }
        Ok(out)
    }
    fn finished(&self) -> bool {
        self.pos == self.bytes.len()
    }
}

fn world_level_to_u8(x: WorldLevel) -> u8 {
    match x {
        WorldLevel::TieuThien => 0,
        WorldLevel::TrungThien => 1,
        WorldLevel::DaiThien => 2,
    }
}
fn u8_to_world_level(x: u8) -> Result<WorldLevel, PersistenceError> {
    match x {
        0 => Ok(WorldLevel::TieuThien),
        1 => Ok(WorldLevel::TrungThien),
        2 => Ok(WorldLevel::DaiThien),
        _ => Err(PersistenceError::InvalidEnum),
    }
}
fn sense_gate_to_u8(x: SenseGate) -> u8 {
    match x {
        SenseGate::Sight => 0,
        SenseGate::Sound => 1,
        SenseGate::Smell => 2,
        SenseGate::Taste => 3,
        SenseGate::Touch => 4,
        SenseGate::Mind => 5,
        SenseGate::System => 6,
    }
}
fn u8_to_sense_gate(x: u8) -> Result<SenseGate, PersistenceError> {
    match x {
        0 => Ok(SenseGate::Sight),
        1 => Ok(SenseGate::Sound),
        2 => Ok(SenseGate::Smell),
        3 => Ok(SenseGate::Taste),
        4 => Ok(SenseGate::Touch),
        5 => Ok(SenseGate::Mind),
        6 => Ok(SenseGate::System),
        _ => Err(PersistenceError::InvalidEnum),
    }
}
fn relation_kind_to_u8(x: RelationKind) -> u8 {
    match x {
        RelationKind::Causes => 0,
        RelationKind::Enables => 1,
        RelationKind::Inhibits => 2,
        RelationKind::Contains => 3,
        RelationKind::Similar => 4,
        RelationKind::Follows => 5,
        RelationKind::GoalRelevant => 6,
    }
}
fn u8_to_relation_kind(x: u8) -> Result<RelationKind, PersistenceError> {
    match x {
        0 => Ok(RelationKind::Causes),
        1 => Ok(RelationKind::Enables),
        2 => Ok(RelationKind::Inhibits),
        3 => Ok(RelationKind::Contains),
        4 => Ok(RelationKind::Similar),
        5 => Ok(RelationKind::Follows),
        6 => Ok(RelationKind::GoalRelevant),
        _ => Err(PersistenceError::InvalidEnum),
    }
}
