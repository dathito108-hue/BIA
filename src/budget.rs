use crate::types::ComputeMode;

#[derive(Clone, Copy, Debug)]
pub struct DeviceState { pub battery:f32, pub thermal:f32, pub load:f32, pub available_memory_mb:u32 }
#[derive(Clone, Copy, Debug)]
pub struct Budget { pub mode:ComputeMode, pub world_limit:usize, pub memory_limit:usize, pub contemplation_cycles:usize }

pub fn middle_way(d:DeviceState,importance:f32,uncertainty:f32)->Budget{
    let pressure=(0.35*(1.0-d.battery.clamp(0.0,1.0))+0.35*d.thermal.clamp(0.0,1.0)+0.30*d.load.clamp(0.0,1.0)).clamp(0.0,1.0);
    let need=(0.55*importance.clamp(0.0,1.0)+0.45*uncertainty.clamp(0.0,1.0)).clamp(0.0,1.0);
    if d.available_memory_mb<96 || pressure>0.88 { Budget{mode:ComputeMode::Tinh,world_limit:8,memory_limit:2,contemplation_cycles:0} }
    else if pressure>0.68 || need<0.30 { Budget{mode:ComputeMode::Nhanh,world_limit:24,memory_limit:4,contemplation_cycles:1} }
    else if need<0.72 { Budget{mode:ComputeMode::Thuong,world_limit:64,memory_limit:8,contemplation_cycles:3} }
    else { Budget{mode:ComputeMode::Sau,world_limit:128,memory_limit:16,contemplation_cycles:8} }
}
