#![forbid(unsafe_code)]

pub mod budget;
pub mod core;
pub mod memory;
pub mod types;
pub mod world;

pub use budget::{middle_way, Budget, DeviceState};
pub use core::{BiaDca, BiaDcaConfig};
pub use memory::{Seed, SeedMemory};
pub use types::*;
pub use world::WorldGraph;

#[cfg(test)]
mod tests {
    use super::*;

    fn device()->DeviceState{
        DeviceState{battery:0.8,thermal:0.2,load:0.2,available_memory_mb:512}
    }

    #[test]
    fn bounded_world_and_seed_memory() {
        let mut bia=BiaDca::new(BiaDcaConfig{world_nodes:3,world_edges:4,seeds:2,active_causes:2});
        for i in 0..8 {
            bia.observe(Phenomenon::new(i,WorldLevel::TieuThien,SenseGate::Mind,i as u32,vec![i as f32,1.0],0.8,0.5,i));
        }
        assert!(bia.world.len()<=3);
        for i in 0..5 {
            let p=Phenomenon::new(100+i,WorldLevel::TrungThien,SenseGate::System,i as u32,vec![1.0,i as f32+0.1],0.9,0.8,i);
            bia.experience(&p,i as u32,0.8,0.1);
        }
        assert!(bia.memory.len()<=2);
    }

    #[test]
    fn dependent_causes_form_hypotheses() {
        let mut bia=BiaDca::new(BiaDcaConfig::default());
        let rain=Phenomenon::new(1,WorldLevel::TrungThien,SenseGate::Sight,10,vec![1.0,0.2],0.95,0.8,1);
        let wet=Phenomenon::new(2,WorldLevel::TieuThien,SenseGate::Touch,11,vec![0.9,0.2],0.9,0.9,2);
        bia.observe(rain);
        bia.observe(wet.clone());
        bia.relate(Relation{from:1,to:2,kind:RelationKind::Causes,strength:0.9,confidence:0.95});
        let moment=bia.contemplate(wet,device(),0.8);
        assert!(!moment.hypotheses.is_empty());
        assert_eq!(moment.hypotheses[0].source,1);
        assert_eq!(bia.cycle(),1);
    }

    #[test]
    fn middle_way_enters_stillness_under_pressure() {
        let b=middle_way(DeviceState{battery:0.05,thermal:0.98,load:0.95,available_memory_mb:64},1.0,1.0);
        assert_eq!(b.mode,ComputeMode::Tinh);
        assert_eq!(b.contemplation_cycles,0);
    }

    #[test]
    fn experience_changes_recall_without_model_weights() {
        let mut bia=BiaDca::new(BiaDcaConfig::default());
        let p=Phenomenon::new(7,WorldLevel::TieuThien,SenseGate::Mind,77,vec![0.3,0.8,0.1],0.9,0.7,1);
        bia.experience(&p,900,0.9,0.0);
        let recalled=bia.memory.recall(&p,4);
        assert_eq!(recalled[0].meaning,900);
    }
}
