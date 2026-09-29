use crate::capability::{DeviceAction, DeviceActionKind};
use crate::action::Authority;
use crate::goals::{Goal, GoalStatus};

#[derive(Clone, Debug, PartialEq)]
pub struct ContinuityState {
    pub active_goal: Option<Goal>,
    pub queued_actions: Vec<DeviceAction>,
}

pub fn encode_continuity(state: &ContinuityState) -> String {
    let mut lines = vec!["BIACONT1".to_string()];
    if let Some(goal) = &state.active_goal {
        lines.push(format!(
            "G\t{}\t{}\t{}\t{}",
            goal.id,
            goal.created_at,
            goal.progress,
            escape(&goal.description)
        ));
    }
    for a in &state.queued_actions {
        lines.push(format!(
            "A\t{}\t{}\t{}\t{}\t{}",
            a.id,
            action_kind(a.kind),
            a.confidence,
            authority(a.authority),
            escape(&a.payload)
        ));
    }
    lines.join("\n")
}

pub fn decode_continuity(text: &str) -> Option<ContinuityState> {
    let mut lines = text.lines();
    if lines.next()? != "BIACONT1" {
        return None;
    }
    let mut goal = None;
    let mut actions = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split('\t').collect();
        match parts.first().copied() {
            Some("G") if parts.len() >= 5 => {
                goal = Some(Goal {
                    id: parts[1].parse().ok()?,
                    created_at: parts[2].parse().ok()?,
                    progress: parts[3].parse::<f32>().ok()?.clamp(0.0, 1.0),
                    description: unescape(parts[4]),
                    status: GoalStatus::Active,
                });
            }
            Some("A") if parts.len() >= 6 => {
                let kind = parse_kind(parts[2])?;
                let auth = parse_authority(parts[4])?;
                let payload = unescape(parts[5]);
                actions.push(DeviceAction {
                    id: parts[1].parse().ok()?,
                    kind,
                    payload: payload.clone(),
                    authority: auth,
                    confidence: parts[3].parse::<f32>().ok()?.clamp(0.0, 1.0),
                    label: default_label(kind, &payload),
                });
            }
            _ => {}
        }
    }
    Some(ContinuityState {
        active_goal: goal,
        queued_actions: actions,
    })
}

fn action_kind(kind: DeviceActionKind) -> &'static str {
    match kind {
        DeviceActionKind::OpenUrl => "url",
        DeviceActionKind::SearchWeb => "search",
        DeviceActionKind::OpenSettings => "settings",
        DeviceActionKind::LaunchPackage => "package",
        DeviceActionKind::ClipboardWrite => "clipboard",
    }
}

fn parse_kind(s: &str) -> Option<DeviceActionKind> {
    match s {
        "url" => Some(DeviceActionKind::OpenUrl),
        "search" => Some(DeviceActionKind::SearchWeb),
        "settings" => Some(DeviceActionKind::OpenSettings),
        "package" => Some(DeviceActionKind::LaunchPackage),
        "clipboard" => Some(DeviceActionKind::ClipboardWrite),
        _ => None,
    }
}

fn authority(a: Authority) -> &'static str {
    match a {
        Authority::ObserveOnly => "observe",
        Authority::Reversible => "reversible",
        Authority::ExternalWrite => "external",
        Authority::Irreversible => "irreversible",
    }
}

fn parse_authority(s: &str) -> Option<Authority> {
    match s {
        "observe" => Some(Authority::ObserveOnly),
        "reversible" => Some(Authority::Reversible),
        "external" => Some(Authority::ExternalWrite),
        "irreversible" => Some(Authority::Irreversible),
        _ => None,
    }
}

fn default_label(kind: DeviceActionKind, payload: &str) -> String {
    match kind {
        DeviceActionKind::OpenUrl => format!("Mở {payload}"),
        DeviceActionKind::SearchWeb => format!("Tìm web: {payload}"),
        DeviceActionKind::OpenSettings => "Mở Cài đặt hệ thống".to_string(),
        DeviceActionKind::LaunchPackage => format!("Mở ứng dụng {payload}"),
        DeviceActionKind::ClipboardWrite => "Ghi vào clipboard".to_string(),
    }
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\t', "\\t").replace('\n', "\\n")
}

fn unescape(s: &str) -> String {
    s.replace("\\n", "\n").replace("\\t", "\t").replace("\\\\", "\\")
}
