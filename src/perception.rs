use crate::types::{Phenomenon, SenseGate, WorldLevel};

#[derive(Clone, Debug, PartialEq)]
pub struct PerceptPacket {
    pub gate: SenseGate,
    pub source: u32,
    pub values: Vec<f32>,
    pub confidence: f32,
    pub timestamp: u64,
}

#[derive(Clone, Debug, Default)]
pub struct MultiCanh;

impl MultiCanh {
    pub fn bind(&self, packets:&[PerceptPacket], base_id:u64)->Phenomenon {
        let confidence = if packets.is_empty() { 0.0 } else {
            packets.iter().map(|p|p.confidence).sum::<f32>() / packets.len() as f32
        };
        let timestamp = packets.iter().map(|p|p.timestamp).max().unwrap_or(0);
        let mut features=vec![0.0;16];
        for p in packets {
            for (i,x) in p.values.iter().enumerate(){ features[(i+p.source as usize)%16]+=*x; }
        }
        if !packets.is_empty(){ for x in &mut features{*x/=packets.len() as f32;} }
        Phenomenon::new(
            base_id, WorldLevel::TrungThien, SenseGate::System,
            packets.iter().fold(0u32,|a,p|a^p.source.rotate_left(5)),
            features, confidence, confidence, timestamp
        )
    }
}
