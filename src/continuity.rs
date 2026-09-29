use crate::action::Authority;
use crate::capability::{DeviceAction, DeviceActionKind};
use crate::goals::{Goal, GoalStatus};
use crate::knowledge::{decode_record, encode_record, KnowledgeRecord};

#[derive(Clone, Debug, PartialEq)]
pub struct ContinuityState {
    pub active_goal: Option<Goal>,
    pub queued_actions: Vec<DeviceAction>,
    pub knowledge: Vec<KnowledgeRecord>,
}

pub fn encode_continuity(state: &ContinuityState) -> String {
    let mut lines = vec!["BIACONT2".to_string()];
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
    for record in &state.knowledge {
        lines.push(format!("K\t{}", encode_record(record)));
    }
    lines.join("\n")
}

pub fn decode_continuity(text: &str) -> Option<ContinuityState> {
    let mut lines = text.lines();
    let version = lines.next()?;
    if version != "BIACONT1" && version != "BIACONT2" {
        return None;
    }
    let mut goal = None;
    let mut actions = Vec::new();
    let mut knowledge = Vec::new();

    for line in lines {
        if let Some(rest) = line.strip_prefix("K\t") {
            if version == "BIACONT2" {
                if let Some(record) = decode_record(rest) {
                    knowledge.push(record);
                }
            }
            continue;
        }

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
        knowledge,
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
    s.replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
}

fn unescape(s: &str) -> String {
    let mut chars = s.chars();
    let mut out = String::new();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('\\') => out.push('\\'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    out
}
