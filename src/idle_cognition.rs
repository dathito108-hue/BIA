use crate::budget::DeviceState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleCognitiveTask {
    None,
    ConsolidateMemory,
    RehearseAnchors,
    ReevaluateHypotheses,
}

#[derive(Clone, Debug, Default)]
pub struct IdleCognitionScheduler {
    ticks: u32,
}

impl IdleCognitionScheduler {
    pub fn choose(
        &mut self,
        device: DeviceState,
        pending_external_action: bool,
        unresolved_questions: usize,
    ) -> IdleCognitiveTask {
        self.ticks = self.ticks.saturating_add(1);
        if pending_external_action
            || device.battery < 0.25
            || device.thermal > 0.65
            || device.load > 0.70
            || device.available_memory_mb < 192
        {
            return IdleCognitiveTask::None;
        }

        if unresolved_questions > 0 {
            IdleCognitiveTask::ReevaluateHypotheses
        } else if self.ticks.is_multiple_of(3) {
            IdleCognitiveTask::RehearseAnchors
        } else {
            IdleCognitiveTask::ConsolidateMemory
        }
    }

    pub fn ticks(&self) -> u32 { self.ticks }
}
