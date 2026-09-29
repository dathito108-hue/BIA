use crate::action::{ActionDecision, ActionProposal, CuTranPolicy};
use crate::budget::DeviceState;
use crate::capability::{infer_device_action, DeviceAction, DeviceActionKind};
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
    pending_action: Option<DeviceAction>,
}

impl OfflineMobileBia {
    pub fn new(bia: BiaDca) -> Self {
        Self {
            bia,
            language: VietnameseGate,
            policy: CuTranPolicy::default(),
            dialogue: DialogueContext::new(24),
            goals: GoalStack::new(16),
            pending_action: None,
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

        let action = infer_device_action(text, stable_id(text, timestamp));
        self.pending_action = action.clone();

        let mut reply = if let Some(lesson) = teaching {
            format!(
                "Tôi đã ghi nhận “{}” vào kinh nghiệm cục bộ và sẽ dùng nó như một duyên trong những lần quán sau.",
                lesson.trim()
            )
        } else if let Some(goal) = self.goals.active() {
            if text.to_lowercase().contains("mục tiêu") || text.to_lowercase().contains("muc tieu") {
                format!(
                    "Tôi đã nhận mục tiêu: “{}”. Tôi sẽ giữ nó làm duyên định hướng cho các bước tiếp theo.",
                    goal.description
                )
            } else {
                self.language.respond(text, &moment)
            }
        } else {
            self.language.respond(text, &moment)
        };

        if let Some(a) = &action {
            reply.push_str(&format!(
                " Tôi đã tạo hành động “{}”. Ứng dụng sẽ yêu cầu bạn xác nhận trước khi thực thi.",
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
            pending_action: action,
        })
    }

    pub fn pending_action(&self) -> Option<&DeviceAction> {
        self.pending_action.as_ref()
    }

    pub fn resolve_pending_action(&mut self, success: bool, timestamp: u64) {
        let Some(action) = self.pending_action.take() else {
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
            }
        }
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
