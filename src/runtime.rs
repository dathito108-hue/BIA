#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuntimeTarget { AndroidArm64, LinuxArm64, LinuxX64, WindowsX64, Wasm }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CapacityTier { Tiny, Mobile, Extended }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeProfile {
    pub target:RuntimeTarget,
    pub tier:CapacityTier,
    pub max_world_nodes:usize,
    pub max_relations:usize,
    pub max_seeds:usize,
    pub max_concepts:usize,
}

impl RuntimeProfile {
    pub fn for_memory_mb(target:RuntimeTarget, memory_mb:u32)->Self {
        if memory_mb<192 {
            Self{target,tier:CapacityTier::Tiny,max_world_nodes:512,max_relations:1024,max_seeds:512,max_concepts:256}
        } else if memory_mb<2048 {
            Self{target,tier:CapacityTier::Mobile,max_world_nodes:4096,max_relations:16384,max_seeds:4096,max_concepts:2048}
        } else {
            Self{target,tier:CapacityTier::Extended,max_world_nodes:32768,max_relations:131072,max_seeds:32768,max_concepts:16384}
        }
    }
}
