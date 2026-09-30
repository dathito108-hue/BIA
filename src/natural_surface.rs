use crate::conversation_continuity::RelationContinuity;
use crate::duyen_weave::DuyenWeave;
use crate::expression_style::ExpressionStyle;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceSection {
    Explain,
    ComparePrevious,
    Summary,
    ConclusionFromPrevious,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SurfacePart {
    pub kind: SurfaceSection,
    pub text: String,
}

impl SurfacePart {
    pub fn new(kind: SurfaceSection, text: String) -> Self {
        Self { kind, text }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NaturalSurfaceRealizer;

impl NaturalSurfaceRealizer {
    /// Realize a grounded causal answer without changing its reasoning content.
    ///
    /// The wording is selected only from bounded structural signals already
    /// produced by BIA (style + Duyen geometry). It does not add facts.
    pub fn relation(
        &self,
        subject: &str,
        object: &str,
        body: &str,
        style: ExpressionStyle,
        weave: &DuyenWeave,
    ) -> String {
        self.relation_contextual(
            subject,
            object,
            body,
            style,
            weave,
            RelationContinuity::New,
        )
    }

    pub fn relation_contextual(
        &self,
        subject: &str,
        object: &str,
        body: &str,
        style: ExpressionStyle,
        weave: &DuyenWeave,
        continuity: RelationContinuity,
    ) -> String {
        let body = body.trim();
        if style == ExpressionStyle::Brief {
            return body.to_string();
        }

        let lead = match continuity {
            RelationContinuity::Repeat => "Vẫn ở quan hệ này,".to_string(),
            RelationContinuity::SameSubject => {
                format!("Tiếp theo mạch về “{subject}”, khi xét đến “{object}”,")
            }
            RelationContinuity::SameObject => {
                format!("Vẫn với kết quả “{object}”, nhưng khi xét từ “{subject}”,")
            }
            RelationContinuity::Return => {
                format!("Quay lại quan hệ giữa “{subject}” và “{object}”,")
            }
            RelationContinuity::New => match (style, weave.is_overlapping()) {
                (ExpressionStyle::Deep, true) => format!(
                    "Nếu nhìn theo các Duyên đang chồng lên nhau giữa “{subject}” và “{object}”,"
                ),
                (ExpressionStyle::Deep, false) => {
                    format!("Xét kỹ quan hệ giữa “{subject}” và “{object}”,")
                }
                (ExpressionStyle::Standard, true) => format!(
                    "Trong quan hệ giữa “{subject}” và “{object}”, có nhiều Duyên cùng tham gia;"
                ),
                (ExpressionStyle::Standard, false) => {
                    format!("Với quan hệ giữa “{subject}” và “{object}”,")
                }
                (ExpressionStyle::Brief, _) => unreachable!(),
            },
        };

        format!("{lead} {}", lower_initial_when_safe(body))
    }

    /// Join independently grounded response sections into conversational prose.
    /// At most four sections are realized, keeping generation bounded.
    pub fn compose(&self, focus: Option<&str>, parts: &[SurfacePart]) -> String {
        let mut out = String::new();

        if let Some(focus) = focus.filter(|f| !f.trim().is_empty()) {
            out.push_str(&format!("Tôi sẽ giữ trọng tâm ở “{}”.", focus.trim()));
        }

        for (index, part) in parts.iter().take(4).enumerate() {
            if !out.is_empty() {
                out.push(' ');
            }

            let prefix = match part.kind {
                SurfaceSection::Explain => {
                    if index == 0 && focus.is_none() {
                        ""
                    } else {
                        "Trước hết, "
                    }
                }
                SurfaceSection::ComparePrevious => "Đặt cạnh trường hợp trước, ",
                SurfaceSection::Summary => "Tóm lại, ",
                SurfaceSection::ConclusionFromPrevious => {
                    "Nếu lấy trường hợp trước làm mốc tham chiếu, "
                }
            };

            out.push_str(prefix);
            if prefix.is_empty() && out.is_empty() {
                out.push_str(part.text.trim());
            } else {
                out.push_str(&lower_initial_when_safe(part.text.trim()));
            }
            ensure_sentence_end(&mut out);
        }

        out
    }
}

fn lower_initial_when_safe(text: &str) -> String {
    let mut chars = text.chars();
    let Some(first) = chars.next() else {
        return String::new();
    };

    // Vietnamese casing for a single leading scalar is deterministic for the
    // ASCII initials used by the generated sentence openings today. Keep
    // non-ASCII initials untouched to avoid lossy transformations.
    if first.is_ascii_uppercase() {
        let mut out = first.to_ascii_lowercase().to_string();
        out.push_str(chars.as_str());
        out
    } else {
        text.to_string()
    }
}

fn ensure_sentence_end(out: &mut String) {
    if !(out.ends_with('.') || out.ends_with('!') || out.ends_with('?')) {
        out.push('.');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn realizes_relation_from_style_and_duyen_geometry() {
        let weave = DuyenWeave {
            supporting_paths: 2,
            opposing_paths: 0,
            convergence_nodes: vec![7],
            shared_links: 0,
            max_depth: 2,
            overlap_score: 0.5,
        };
        let text = NaturalSurfaceRealizer.relation(
            "mưa",
            "đường trơn",
            "Các Duyên hiện tại nghiêng về phía ủng hộ kết luận này.",
            ExpressionStyle::Deep,
            &weave,
        );
        assert!(text.starts_with("Nếu nhìn theo các Duyên đang chồng lên nhau"));
        assert!(text.contains("mưa"));
        assert!(text.contains("đường trơn"));
    }

    #[test]
    fn continuity_changes_opening_without_changing_grounded_body() {
        let weave = DuyenWeave::default();
        let body = "Các Duyên hiện tại nghiêng về phía ủng hộ kết luận này.";
        let same_subject = NaturalSurfaceRealizer.relation_contextual(
            "mưa",
            "bùn",
            body,
            ExpressionStyle::Standard,
            &weave,
            RelationContinuity::SameSubject,
        );
        let repeat = NaturalSurfaceRealizer.relation_contextual(
            "mưa",
            "bùn",
            body,
            ExpressionStyle::Standard,
            &weave,
            RelationContinuity::Repeat,
        );
        assert!(same_subject.starts_with("Tiếp theo mạch về"));
        assert!(repeat.starts_with("Vẫn ở quan hệ này"));
        assert!(same_subject.contains("ủng hộ kết luận này"));
        assert!(repeat.contains("ủng hộ kết luận này"));
    }

    #[test]
    fn composes_grounded_sections_as_flowing_prose() {
        let parts = vec![
            SurfacePart::new(
                SurfaceSection::Explain,
                "Các Duyên hiện tại ủng hộ kết luận này.".into(),
            ),
            SurfacePart::new(
                SurfaceSection::Summary,
                "Mức chắc chắn khoảng 80%.".into(),
            ),
        ];
        let text = NaturalSurfaceRealizer.compose(Some("đường trơn"), &parts);
        assert!(text.contains("giữ trọng tâm"));
        assert!(text.contains("Trước hết"));
        assert!(text.contains("Tóm lại"));
        assert!(!text.contains("Giải thích:"));
    }
}
