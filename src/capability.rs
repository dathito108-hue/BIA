use crate::action::Authority;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceActionKind {
    OpenUrl,
    SearchWeb,
    OpenSettings,
    LaunchPackage,
    ClipboardWrite,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeviceAction {
    pub id: u64,
    pub kind: DeviceActionKind,
    pub payload: String,
    pub authority: Authority,
    pub confidence: f32,
    pub label: String,
}

pub fn infer_device_action(input: &str, id: u64) -> Option<DeviceAction> {
    let trimmed = input.trim();
    let n = normalize(trimmed);

    if n == "mo cai dat" || n == "mo settings" || n.contains("mo cai dat dien thoai") {
        return Some(DeviceAction {
            id,
            kind: DeviceActionKind::OpenSettings,
            payload: String::new(),
            authority: Authority::Reversible,
            confidence: 0.98,
            label: "Mở Cài đặt hệ thống".to_string(),
        });
    }

    if let Some(rest) = strip_prefix_ci(trimmed, "sao chép ")
        .or_else(|| strip_prefix_ci(trimmed, "copy "))
    {
        if !rest.trim().is_empty() {
            return Some(DeviceAction {
                id,
                kind: DeviceActionKind::ClipboardWrite,
                payload: rest.trim().to_string(),
                authority: Authority::ExternalWrite,
                confidence: 0.96,
                label: "Ghi vào clipboard".to_string(),
            });
        }
    }

    if let Some(rest) = strip_prefix_ci(trimmed, "tìm web ")
        .or_else(|| strip_prefix_ci(trimmed, "tim web "))
        .or_else(|| strip_prefix_ci(trimmed, "tìm trên web "))
    {
        if !rest.trim().is_empty() {
            return Some(DeviceAction {
                id,
                kind: DeviceActionKind::SearchWeb,
                payload: rest.trim().to_string(),
                authority: Authority::Reversible,
                confidence: 0.93,
                label: format!("Tìm web: {}", rest.trim()),
            });
        }
    }

    if n.contains("mo youtube") {
        return Some(DeviceAction {
            id,
            kind: DeviceActionKind::OpenUrl,
            payload: "https://www.youtube.com".to_string(),
            authority: Authority::Reversible,
            confidence: 0.95,
            label: "Mở YouTube".to_string(),
        });
    }

    if let Some(rest) = strip_prefix_ci(trimmed, "mở ứng dụng ")
        .or_else(|| strip_prefix_ci(trimmed, "mo ung dung "))
        .or_else(|| strip_prefix_ci(trimmed, "mở app "))
    {
        let package = rest.trim();
        if package.contains('.') && !package.contains(char::is_whitespace) {
            return Some(DeviceAction {
                id,
                kind: DeviceActionKind::LaunchPackage,
                payload: package.to_string(),
                authority: Authority::Reversible,
                confidence: 0.90,
                label: format!("Mở ứng dụng {package}"),
            });
        }
    }

    if let Some(url) = extract_http_url(trimmed) {
        if n.starts_with("mo ") || n.starts_with("truy cap ") {
            return Some(DeviceAction {
                id,
                kind: DeviceActionKind::OpenUrl,
                payload: url.to_string(),
                authority: Authority::Reversible,
                confidence: 0.97,
                label: format!("Mở {url}"),
            });
        }
    }

    None
}

pub fn encode_action(action: &DeviceAction) -> String {
    let kind = match action.kind {
        DeviceActionKind::OpenUrl => "OPEN_URL",
        DeviceActionKind::SearchWeb => "SEARCH_WEB",
        DeviceActionKind::OpenSettings => "OPEN_SETTINGS",
        DeviceActionKind::LaunchPackage => "LAUNCH_PACKAGE",
        DeviceActionKind::ClipboardWrite => "CLIPBOARD_WRITE",
    };
    format!(
        "{}\t{}\t{}\t{}",
        kind,
        escape(&action.label),
        escape(&action.payload),
        action.id
    )
}

fn escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
}

fn strip_prefix_ci<'a>(value: &'a str, prefix: &str) -> Option<&'a str> {
    let n_value = normalize(value);
    let n_prefix = normalize(prefix);
    if n_value.starts_with(&n_prefix) {
        value.get(prefix.chars().count()..)
            .or_else(|| {
                let split = value
                    .char_indices()
                    .nth(n_prefix.chars().count())
                    .map(|(i, _)| i)
                    .unwrap_or(value.len());
                value.get(split..)
            })
    } else {
        None
    }
}

fn extract_http_url(value: &str) -> Option<&str> {
    value
        .split_whitespace()
        .find(|w| w.starts_with("https://") || w.starts_with("http://"))
}

fn normalize(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| match c {
            'à'|'á'|'ạ'|'ả'|'ã'|'â'|'ầ'|'ấ'|'ậ'|'ẩ'|'ẫ'|'ă'|'ằ'|'ắ'|'ặ'|'ẳ'|'ẵ' => 'a',
            'è'|'é'|'ẹ'|'ẻ'|'ẽ'|'ê'|'ề'|'ế'|'ệ'|'ể'|'ễ' => 'e',
            'ì'|'í'|'ị'|'ỉ'|'ĩ' => 'i',
            'ò'|'ó'|'ọ'|'ỏ'|'õ'|'ô'|'ồ'|'ố'|'ộ'|'ổ'|'ỗ'|'ơ'|'ờ'|'ớ'|'ợ'|'ở'|'ỡ' => 'o',
            'ù'|'ú'|'ụ'|'ủ'|'ũ'|'ư'|'ừ'|'ứ'|'ự'|'ử'|'ữ' => 'u',
            'ỳ'|'ý'|'ỵ'|'ỷ'|'ỹ' => 'y',
            'đ' => 'd',
            c => c,
        })
        .collect()
}
