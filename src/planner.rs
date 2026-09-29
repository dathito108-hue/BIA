use crate::capability::{infer_device_action, DeviceAction};

#[derive(Clone, Debug, PartialEq)]
pub struct Plan {
    pub goal: String,
    pub steps: Vec<DeviceAction>,
}

pub fn decompose_goal(goal: &str, seed: u64) -> Plan {
    let mut steps = Vec::new();
    for (index, part) in split_steps(goal).into_iter().enumerate() {
        if let Some(action) = infer_device_action(part, seed.wrapping_add(index as u64 + 1)) {
            steps.push(action);
        }
    }
    if steps.is_empty() {
        if let Some(action) = crate::capability::action_for_goal(goal, seed) {
            steps.push(action);
        }
    }
    Plan {
        goal: goal.trim().to_string(),
        steps,
    }
}

fn split_steps(goal: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut start = 0usize;
    for marker in [" rồi ", " sau đó ", ";"] {
        if goal[start..].contains(marker) {
            let parts: Vec<&str> = goal.split(marker).collect();
            if parts.len() > 1 {
                return parts.into_iter().map(str::trim).filter(|s| !s.is_empty()).collect();
            }
        }
        start = 0;
    }
    if !goal.trim().is_empty() {
        out.push(goal.trim());
    }
    out
}
