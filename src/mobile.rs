use crate::action::{ActionDecision, ActionProposal, CuTranPolicy};
use crate::action_queue::ActionQueue;
use crate::budget::DeviceState;
use crate::capability::{
    action_for_goal, infer_device_action, DeviceAction, DeviceActionKind,
};
use crate::continuity::{decode_continuity, encode_continuity, ContinuityState};
use crate::core::BiaDca;
use crate::dialogue::{DialogueContext, DialogueTurn, Speaker};
use crate::goals::{GoalStack, GoalStatus};
use crate::language::VietnameseGate;
use crate::types::{CognitiveMoment, Phenomenon, SenseGate, WorldLevel};

#[derive(Clone, Debug, PartialEq)]
pub struct MobileReply {
    pub text: String,
    pub moment: CognitiveMoment,
    pub pending_action: Option<DeviceAction>,
}

pub struct OfflineMobileBia {
    pub bia: BiaDca,
    pub language: VietnameseGate,
    pub policy: CuTranPolicy,
    pub dialogue: DialogueContext,
    pub goals: GoalStack,
    queue: ActionQueue,
}

impl OfflineMobileBia {
    pub fn new(bia: BiaDca) -> Self {
        Self {
            bia,
            language: VietnameseGate,
            policy: CuTranPolicy::default(),
            dialogue: DialogueContext::new(24),
            goals: GoalStack::new(16),
            queue: ActionQueue::new(8),
        }
    }

    pub fn converse(
        &mut self,
        text: &str,
        timestamp: u64,
        device: DeviceState,
    ) -> Option<MobileReply> {
        self.dialogue.push(DialogueTurn {
            speaker: Speaker::User,
            text: text.to_string(),
            timestamp,
        });

        if let Some(goal) = extract_goal(text) {
            let id = stable_id(goal, timestamp);
            self.goals.set(id, goal.to_string(), timestamp);
        }

        let teaching = extract_teaching(text);
        let mut ps = self.language.perceive(teaching.unwrap_or(text), timestamp);
        let focus = ps.pop()?;

        for p in &ps {
            let meaning = self.bia.observe(p.clone());
            if teaching.is_some() {
                self.bia.experience(p, meaning, 0.85, 0.0);
            }
        }

        let moment = self.bia.contemplate(focus.clone(), device, 0.7);

        if teaching.is_some() {
            if let Some(meaning) = moment.recognition.first().copied() {
                self.bia.experience(&focus, meaning, 0.9, 0.0);
            }
        }

        let direct = infer_device_action(text, stable_id(text, timestamp));
        if let Some(action) = direct {
            self.queue.push(action);
        } else if is_continue(text) && self.queue.is_empty() {
            if let Some(goal) = self.goals.active() {
                if let Some(next) = action_for_goal(
                    &goal.description,
                    stable_id(&goal.description, timestamp),
                ) {
                    self.queue.push(next);
                }
            }
        }

        let pending = self.queue.front().cloned();

        let mut reply = if let Some(lesson) = teaching {
            format!(
                "Tôi đã ghi nhận “{}” vào kinh nghiệm cục bộ và sẽ dùng nó như một duyên trong những lần quán sau.",
                lesson.trim()
            )
        } else if let Some(goal) = self.goals.active() {
            if is_goal_command(text) {
                format!(
                    "Tôi đã nhận mục tiêu: “{}”. Tôi sẽ giữ nó làm duyên định hướng và có thể tiếp tục bằng nhiều bước.",
                    goal.description
                )
            } else {
                self.language.respond(text, &moment)
            }
        } else {
            self.language.respond(text, &moment)
        };

        if let Some(a) = &pending {
            reply.push_str(&format!(
                " Hàng đợi hiện có {} hành động; bước kế tiếp là “{}”. Ứng dụng sẽ yêu cầu xác nhận.",
                self.queue.len(),
                a.label
            ));
        }

        self.dialogue.push(DialogueTurn {
            speaker: Speaker::Bia,
            text: reply.clone(),
            timestamp: timestamp.saturating_add(1),
        });

        Some(MobileReply {
            text: reply,
            moment,
            pending_action: pending,
        })
    }

    pub fn pending_action(&self) -> Option<&DeviceAction> {
        self.queue.front()
    }

    pub fn resolve_pending_action(&mut self, success: bool, timestamp: u64) {
        let Some(action) = self.queue.pop_front() else {
            return;
        };

        let kind = action_kind_code(action.kind);
        let event = Phenomenon::new(
            action.id,
            WorldLevel::TrungThien,
            SenseGate::System,
            kind,
            action_features(&action),
            if success { 0.95 } else { 0.8 },
            0.85,
            timestamp,
        );
        self.bia.observe(event.clone());
        self.bia.experience(
            &event,
            kind,
            if success { 0.95 } else { 0.0 },
            if success { 0.0 } else { 0.85 },
        );

        if let Some(goal) = self.goals.active() {
            if success {
                let next = (goal.progress + 0.25).min(1.0);
                let status = if next >= 1.0 {
                    GoalStatus::Completed
                } else {
                    GoalStatus::Active
                };
                self.goals.update_active(next, status);
            } else {
                self.goals.update_active(goal.progress, GoalStatus::Blocked);
                self.queue.clear();
            }
        }
    }

    pub fn continuity_export(&self) -> String {
        encode_continuity(&ContinuityState {
            active_goal: self.goals.active().cloned(),
            queued_actions: self.queue.items().cloned().collect(),
        })
    }

    pub fn continuity_import(&mut self, text: &str) -> bool {
        let Some(state) = decode_continuity(text) else {
            return false;
        };
        if let Some(goal) = state.active_goal {
            self.goals.restore_active(goal);
        }
        self.queue.clear();
        for action in state.queued_actions {
            self.queue.push(action);
        }
        true
    }

    pub fn queue_len(&self) -> usize {
        self.queue.len()
    }

    pub fn authorize(&self, proposal: ActionProposal) -> ActionDecision {
        self.policy.evaluate(proposal)
    }
}

fn extract_teaching(input: &str) -> Option<&str> {
    let lower = input.to_lowercase();
    for prefix in ["nhớ rằng ", "ghi nhớ rằng ", "ghi nhớ ", "nho rang ", "ghi nho "] {
        if lower.starts_with(prefix) {
            let n = prefix.chars().count();
            return input
                .char_indices()
                .nth(n)
                .map(|(i, _)| &input[i..])
                .or(Some(""));
        }
    }
    None
}

fn extract_goal(input: &str) -> Option<&str> {
    let lower = input.to_lowercase();
    for prefix in ["mục tiêu: ", "mục tiêu ", "muc tieu: ", "muc tieu "] {
        if lower.starts_with(prefix) {
            let n = prefix.chars().count();
            return input
                .char_indices()
                .nth(n)
                .map(|(i, _)| &input[i..])
                .or(Some(""));
        }
    }
    None
}

fn is_goal_command(input: &str) -> bool {
    let lower = input.to_lowercase();
    lower.starts_with("mục tiêu") || lower.starts_with("muc tieu")
}

fn is_continue(input: &str) -> bool {
    matches!(
        input.trim().to_lowercase().as_str(),
        "tiếp tục" | "tiep tuc" | "làm tiếp" | "lam tiep" | "kế tiếp" | "ke tiep"
    )
}

fn stable_id(text: &str, timestamp: u64) -> u64 {
    let mut h = 0xcbf29ce484222325_u64 ^ timestamp;
    for b in text.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn action_kind_code(kind: DeviceActionKind) -> u32 {
    match kind {
        DeviceActionKind::OpenUrl => 0xA001,
        DeviceActionKind::SearchWeb => 0xA002,
        DeviceActionKind::OpenSettings => 0xA003,
        DeviceActionKind::LaunchPackage => 0xA004,
        DeviceActionKind::ClipboardWrite => 0xA005,
    }
}

fn action_features(action: &DeviceAction) -> Vec<f32> {
    let mut out = vec![0.0; 8];
    for (i, b) in action.payload.bytes().enumerate() {
        out[i % 8] += f32::from(b) / 255.0;
    }
    out[0] += action.confidence;
    out
}
