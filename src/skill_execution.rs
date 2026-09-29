//! Typed bindings and an identity-bound claim/receipt state. No OS calls here.
use crate::capability::{DeviceAction, DeviceActionKind};
use crate::integrated_cognition::{hex, unhex};
use crate::semantic::normalize;

#[derive(Clone, Debug)]
pub struct SkillExecution {
    pub in_flight: Option<u64>,
    pub skills: Vec<(u64, String)>,
    pub receipts: Vec<(u64, bool)>,
    next_id: u64,
}
impl Default for SkillExecution {
    fn default() -> Self {
        Self {
            in_flight: None,
            skills: Vec::new(),
            receipts: Vec::new(),
            next_id: 1,
        }
    }
}
impl SkillExecution {
    pub fn identify(&mut self, mut action: DeviceAction) -> Option<DeviceAction> {
        if self.next_id >= u32::MAX as u64 {
            return None;
        }
        action.id = 0x4249410000000000 | self.next_id;
        self.next_id += 1;
        Some(action)
    }
    pub fn prepare(&mut self, steps: Vec<(String, DeviceAction)>) -> Option<Vec<DeviceAction>> {
        if steps.is_empty()
            || steps.len() > 4
            || self.in_flight.is_some()
            || !self.skills.is_empty()
            || self.next_id + steps.len() as u64 > u32::MAX as u64
        {
            return None;
        }
        let mut actions = Vec::new();
        for (name, mut action) in steps {
            action.id = 0x4249410000000000 | self.next_id;
            self.next_id += 1;
            action.label = format!("Kỹ năng {name}: {}", action.label);
            self.skills.push((action.id, name));
            actions.push(action);
        }
        Some(actions)
    }
    pub fn claim(&mut self, id: u64, front: Option<u64>) -> bool {
        if self.in_flight.is_some() || front != Some(id) {
            return false;
        }
        self.in_flight = Some(id);
        true
    }
    pub fn complete(&mut self, id: u64, success: bool, front: Option<u64>) -> bool {
        if self.in_flight != Some(id) || front != Some(id) {
            return false;
        }
        self.in_flight = None;
        if self.receipts.len() >= 32 {
            self.receipts.remove(0);
        }
        self.receipts.push((id, success));
        true
    }
    pub fn cancel(&mut self) {
        self.in_flight = None;
        self.skills.clear();
    }
    pub fn export(&self) -> String {
        let mut lines = vec![format!(
            "E132\t{}\t{}",
            self.next_id,
            self.in_flight
                .map(|i| i.to_string())
                .unwrap_or_else(|| "-".into())
        )];
        lines.extend(
            self.skills
                .iter()
                .map(|(id, name)| format!("S132\t{id}\t{}", hex(name.as_bytes()))),
        );
        lines.extend(
            self.receipts
                .iter()
                .map(|(id, ok)| format!("R132\t{id}\t{}", u8::from(*ok))),
        );
        lines.join("\n")
    }
    pub fn restore(text: &str, pending: &[DeviceAction]) -> Option<Self> {
        let mut state = Self::default();
        let mut header = false;
        for line in text.lines() {
            if line.starts_with("E132\t") {
                if header {
                    return None;
                }
                header = true;
                let p: Vec<_> = line.split('\t').collect();
                if p.len() != 3 {
                    return None;
                }
                state.next_id = p[1].parse().ok()?;
                if state.next_id == 0 || state.next_id > u32::MAX as u64 {
                    return None;
                }
                state.in_flight = if p[2] == "-" {
                    None
                } else {
                    Some(p[2].parse().ok()?)
                };
            } else if line.starts_with("S132\t") {
                let p: Vec<_> = line.split('\t').collect();
                if p.len() != 3 || state.skills.len() >= 4 {
                    return None;
                }
                let id = p[1].parse().ok()?;
                let name = unhex(p[2])?;
                if !pending.iter().any(|a| a.id == id)
                    || state.skills.iter().any(|s| s.0 == id)
                    || name.len() > 320
                {
                    return None;
                }
                state.skills.push((id, name));
            } else if line.starts_with("R132\t") {
                let p: Vec<_> = line.split('\t').collect();
                if p.len() != 3 || state.receipts.len() >= 32 {
                    return None;
                }
                state.receipts.push((
                    p[1].parse().ok()?,
                    match p[2] {
                        "0" => false,
                        "1" => true,
                        _ => return None,
                    },
                ));
            }
        }
        if (!state.skills.is_empty() || !state.receipts.is_empty()) && !header {
            return None;
        }
        if let Some(id) = state.in_flight {
            if pending.first().map(|a| a.id) != Some(id) {
                return None;
            }
        }
        if pending.len() > 12
            || pending
                .iter()
                .enumerate()
                .any(|(i, a)| pending[..i].iter().any(|b| b.id == a.id))
        {
            return None;
        }
        for id in pending
            .iter()
            .map(|a| a.id)
            .chain(state.receipts.iter().map(|r| r.0))
        {
            if id & 0xffffffff00000000 == 0x4249410000000000
                && (!header || id & 0xffffffff >= state.next_id)
            {
                return None;
            }
        }
        if state.receipts.iter().enumerate().any(|(i, r)| {
            pending.iter().any(|a| a.id == r.0) || state.receipts[..i].iter().any(|p| p.0 == r.0)
        }) {
            return None;
        }
        Some(state)
    }
}

pub fn parse_binding(input: &str) -> Option<DeviceAction> {
    if input.chars().count() > 1024 || input.contains(['\n', '\r', '\t']) {
        return None;
    }
    let input = input.trim();
    let n = normalize(input);
    let (kind, payload) = if n == "mo cai dat" || n == "mo settings" {
        (DeviceActionKind::OpenSettings, String::new())
    } else if n == "mo youtube" {
        (DeviceActionKind::OpenUrl, "https://www.youtube.com".into())
    } else {
        let mut parsed = None;
        for (prefix, kind) in [
            ("sao chep", DeviceActionKind::ClipboardWrite),
            ("copy", DeviceActionKind::ClipboardWrite),
            ("tim web", DeviceActionKind::SearchWeb),
            ("tim tren web", DeviceActionKind::SearchWeb),
            ("mo ung dung", DeviceActionKind::LaunchPackage),
            ("mo app", DeviceActionKind::LaunchPackage),
            ("mo", DeviceActionKind::OpenUrl),
            ("truy cap", DeviceActionKind::OpenUrl),
        ] {
            let count = prefix.split_whitespace().count();
            let words: Vec<_> = input.split_whitespace().collect();
            if words.len() <= count || normalize(&words[..count].join(" ")) != prefix {
                continue;
            }
            // Derive the payload from the original string, never its folded form.
            let offset = words[count].as_ptr() as usize - input.as_ptr() as usize;
            parsed = Some((kind, input[offset..].trim().to_string()));
            break;
        }
        parsed?
    };
    if kind == DeviceActionKind::LaunchPackage
        && (!payload.contains('.')
            || !payload
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'_'))
    {
        return None;
    }
    if kind == DeviceActionKind::OpenUrl
        && (!(payload.starts_with("https://") || payload.starts_with("http://"))
            || payload.contains(char::is_whitespace))
    {
        return None;
    }
    Some(DeviceAction {
        id: 0,
        kind,
        payload,
        authority: if kind == DeviceActionKind::ClipboardWrite {
            crate::action::Authority::ExternalWrite
        } else {
            crate::action::Authority::Reversible
        },
        confidence: 1.0,
        label: match kind {
            DeviceActionKind::OpenSettings => "Mở cài đặt",
            DeviceActionKind::OpenUrl => "Mở URL",
            DeviceActionKind::SearchWeb => "Tìm web",
            DeviceActionKind::LaunchPackage => "Mở ứng dụng",
            DeviceActionKind::ClipboardWrite => "Ghi clipboard",
        }
        .into(),
    })
}
