#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WorldLevel { TieuThien, TrungThien, DaiThien }

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SenseGate { Sight, Sound, Smell, Taste, Touch, Mind, System }

#[derive(Clone, Debug, PartialEq)]
pub struct Phenomenon {
    pub id: u64,
    pub level: WorldLevel,
    pub gate: SenseGate,
    pub kind: u32,
    pub features: Vec<f32>,
    pub confidence: f32,
    pub salience: f32,
    pub timestamp: u64,
}

impl Phenomenon {
    pub fn new(id:u64, level:WorldLevel, gate:SenseGate, kind:u32, features:Vec<f32>, confidence:f32, salience:f32, timestamp:u64)->Self{
        Self{id,level,gate,kind,features,confidence:confidence.clamp(0.0,1.0),salience:salience.clamp(0.0,1.0),timestamp}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelationKind { Causes, Enables, Inhibits, Contains, Similar, Follows, GoalRelevant }

#[derive(Clone, Debug)]
pub struct Relation {
    pub from:u64, pub to:u64, pub kind:RelationKind, pub strength:f32, pub confidence:f32,
}

#[derive(Clone, Debug)]
pub struct Hypothesis {
    pub source:u64, pub target:u64, pub score:f32, pub support:Vec<u64>,
}

#[derive(Clone, Debug)]
pub struct Intention {
    pub action:u32,
    pub target:Option<u64>,
    pub expected_benefit:f32,
    pub expected_harm:f32,
    pub reversibility:f32,
    pub confidence:f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComputeMode { Tinh, Nhanh, Thuong, Sau }

#[derive(Clone, Debug)]
pub struct CognitiveMoment {
    pub observed:Vec<u64>,
    pub active_causes:Vec<Relation>,
    pub feeling:f32,
    pub recognition:Vec<u32>,
    pub hypotheses:Vec<Hypothesis>,
    pub chosen:Option<Intention>,
    pub mode:ComputeMode,
    pub uncertainty:f32,
}
