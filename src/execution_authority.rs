//! Explicit foreground grants. Never serialized: process restart requires renewed consent.
use crate::capability::{DeviceAction, DeviceActionKind};

pub const MAX_BATCH: usize = 12;
pub const SESSION_STEPS: u32 = 100;
pub const GRANT_DURATION_MS: u64 = 30 * 60 * 1000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApprovalMode {
    Step,
    Batch,
    Session,
}
#[derive(Clone, Debug, Default)]
pub struct ExecutionAuthority {
    allowed: Vec<DeviceAction>,
    mode: Option<ApprovalMode>,
    remaining: u32,
    issued_at: u64,
    expires_at: u64,
}
impl ExecutionAuthority {
    pub fn grant(&mut self, actions: &[DeviceAction], mode: ApprovalMode, now: u64) -> bool {
        if actions.is_empty() || actions.len() > MAX_BATCH {
            return false;
        }
        self.allowed = if mode == ApprovalMode::Step {
            actions[..1].to_vec()
        } else {
            actions.to_vec()
        };
        self.remaining = if mode == ApprovalMode::Session {
            SESSION_STEPS
        } else {
            self.allowed.len() as u32
        };
        self.mode = Some(mode);
        self.issued_at = now;
        self.expires_at = now.saturating_add(GRANT_DURATION_MS);
        true
    }
    pub fn permits(&self, action: &DeviceAction, now: u64) -> bool {
        self.remaining > 0
            && now >= self.issued_at
            && now < self.expires_at
            && self.allowed.iter().any(|a| {
                same_effect(a, action)
                    && (self.mode == Some(ApprovalMode::Session) || a.id == action.id)
            })
    }
    pub fn consume(&mut self, action: &DeviceAction, now: u64) -> bool {
        if !self.permits(action, now) {
            return false;
        }
        self.remaining -= 1;
        if self.mode != Some(ApprovalMode::Session) {
            self.allowed.retain(|a| a.id != action.id);
        }
        true
    }
    pub fn revoke(&mut self) {
        *self = Self::default();
    }
    pub fn status(&self, now: u64) -> String {
        if self.remaining == 0 || now < self.issued_at || now >= self.expires_at {
            return "Tự duyệt: tắt hoặc hết hạn".into();
        }
        format!(
            "{}: còn {} lượt, {} giây",
            match self.mode {
                Some(ApprovalMode::Session) => "Tự duyệt đúng thao tác đã cấp",
                Some(ApprovalMode::Batch) => "Đã duyệt chuỗi hiện tại",
                _ => "Đã duyệt một bước",
            },
            self.remaining,
            (self.expires_at - now) / 1000
        )
    }
}
fn same_effect(a: &DeviceAction, b: &DeviceAction) -> bool {
    // Include authority as well as exact adapter and payload, never a wildcard/domain match.
    a.kind == b.kind
        && a.payload == b.payload
        && a.authority == b.authority
        && matches!(
            b.kind,
            DeviceActionKind::OpenSettings
                | DeviceActionKind::OpenUrl
                | DeviceActionKind::SearchWeb
                | DeviceActionKind::LaunchPackage
                | DeviceActionKind::ClipboardWrite
        )
}
