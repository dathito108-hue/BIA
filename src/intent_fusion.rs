use crate::semantic::normalize;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResponseSection {
    Explain,
    ComparePrevious,
    Summary,
    ConclusionFromPrevious,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IntentFusionPlan {
    pub sections: Vec<ResponseSection>,
    pub focus: Option<String>,
    pub excluded: Vec<ResponseSection>,
}

impl IntentFusionPlan {
    pub fn parse(input: &str) -> Option<Self> {
        let s = normalize(input);
        let mut sections = Vec::new();
        let mut excluded = Vec::new();

        if s.contains("giai thich") || s.contains("phan tich") || s.contains("noi ro") {
            sections.push(ResponseSection::Explain);
        }
        if s.contains("so sanh") && (s.contains("truong hop truoc") || s.contains("y truoc") || s.contains("truong hop kia")) {
            sections.push(ResponseSection::ComparePrevious);
        }
        if s.contains("tom tat") || s.contains("rut gon") || s.contains("noi ngan gon") {
            sections.push(ResponseSection::Summary);
        }
        if s.contains("ket luan theo truong hop truoc") || s.contains("ket luan theo y truoc") {
            sections.push(ResponseSection::ConclusionFromPrevious);
        }

        if s.contains("bo phan so sanh") || s.contains("khong can so sanh") {
            excluded.push(ResponseSection::ComparePrevious);
        }
        if s.contains("bo phan giai thich") || s.contains("khong can giai thich") {
            excluded.push(ResponseSection::Explain);
        }
        if s.contains("bo phan tom tat") || s.contains("khong can tom tat") {
            excluded.push(ResponseSection::Summary);
        }

        let focus = extract_focus(&s);
        sections.retain(|section| !excluded.contains(section));
        sections.dedup();
        excluded.dedup();

        let has_fusion = sections.len() >= 2 || (focus.is_some() && !sections.is_empty()) || !excluded.is_empty();
        has_fusion.then_some(Self {
            sections,
            focus,
            excluded,
        })
    }

    pub fn contains(&self, section: ResponseSection) -> bool {
        self.sections.contains(&section)
    }
}

fn extract_focus(s: &str) -> Option<String> {
    for marker in ["tap trung vao ", "nhan manh vao "] {
        if let Some(rest) = s.split_once(marker).map(|(_, rest)| rest) {
            let end = [",", " roi ", " va ", " nhung ", " bo phan ", " ket luan "]
                .iter()
                .filter_map(|stop| rest.find(stop))
                .min()
                .unwrap_or(rest.len());
            let focus = rest[..end].trim();
            if !focus.is_empty() && focus.chars().count() <= 80 {
                return Some(focus.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuses_focus_exclusion_and_previous_conclusion() {
        let p = IntentFusionPlan::parse(
            "Giải thích nhưng tập trung vào đường trơn, bỏ phần so sánh và kết luận theo trường hợp trước",
        )
        .expect("plan");
        assert!(p.contains(ResponseSection::Explain));
        assert!(p.contains(ResponseSection::ConclusionFromPrevious));
        assert!(!p.contains(ResponseSection::ComparePrevious));
        assert_eq!(p.focus.as_deref(), Some("duong tron"));
    }
}
