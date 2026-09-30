use crate::open_reasoning::OpenAnswer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResponseStance {
    Certain,
    Cautious,
    Contradictory,
    Counterfactual,
    Unknown,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GeneratedThought {
    pub text: String,
    pub stance: ResponseStance,
    pub confidence: f32,
    pub evidence_depth: usize,
}

#[derive(Clone, Debug, Default)]
pub struct GenerativeCognition;

impl GenerativeCognition {
    pub fn render(&self, answer: &OpenAnswer, uncertainty: f32) -> GeneratedThought {
        match answer {
            OpenAnswer::Supported { confidence, path, weave } => {
                let confidence = calibrated(*confidence, uncertainty);
                GeneratedThought {
                    text: compose_supported(confidence, path.len().saturating_sub(1), weave),
                    stance: if confidence >= 0.75 { ResponseStance::Certain } else { ResponseStance::Cautious },
                    confidence,
                    evidence_depth: path.len().saturating_sub(1),
                }
            }
            OpenAnswer::Opposed { confidence, path, weave } => {
                let confidence = calibrated(*confidence, uncertainty);
                GeneratedThought {
                    text: compose_opposed(confidence, path.len().saturating_sub(1), weave),
                    stance: ResponseStance::Cautious,
                    confidence,
                    evidence_depth: path.len().saturating_sub(1),
                }
            }
            OpenAnswer::Contradicted { support, opposition, weave } => {
                let conflict = support.min(*opposition);
                GeneratedThought {
                    text: format!(
                        "Các Duyên đang xung đột: {} nhánh ủng hộ và {} nhánh phản đối cùng hội tụ vào kết quả; sức ủng hộ {:.0}% và phản đối {:.0}%. Tôi giữ cả hai hướng và cần thêm bằng chứng trước khi kết luận.",
                        weave.supporting_paths,
                        weave.opposing_paths,
                        support * 100.0,
                        opposition * 100.0
                    ),
                    stance: ResponseStance::Contradictory,
                    confidence: (1.0 - conflict).clamp(0.0,1.0),
                    evidence_depth: 0,
                }
            }
            OpenAnswer::Counterfactual { support_delta, factual_support, counterfactual_support } => GeneratedThought {
                text: format!(
                    "Khi loại điều kiện đang xét, sức ủng hộ thay đổi {:.0} điểm phần trăm, từ {:.0}% còn {:.0}%. Điều này cho thấy điều kiện đó có ảnh hưởng {}.",
                    support_delta * 100.0,
                    factual_support * 100.0,
                    counterfactual_support * 100.0,
                    if *support_delta > 0.35 { "mạnh" } else { "đáng kể nhưng chưa tuyệt đối" }
                ),
                stance: ResponseStance::Counterfactual,
                confidence: factual_support.max(*counterfactual_support).clamp(0.0,1.0),
                evidence_depth: 1,
            },
            OpenAnswer::Unknown => GeneratedThought {
                text: if uncertainty > 0.65 {
                    "Tôi chưa có đủ Duyên để kết luận. Bước hợp lý là thu thêm bằng chứng hoặc truy hồi ký ức liên quan.".to_string()
                } else {
                    "Tri thức hiện có chưa tạo thành một chuỗi nhân–quả đủ mạnh để trả lời chắc chắn.".to_string()
                },
                stance: ResponseStance::Unknown,
                confidence: (1.0 - uncertainty).clamp(0.0,0.5),
                evidence_depth: 0,
            },
        }
    }
}

fn calibrated(confidence: f32, uncertainty: f32) -> f32 {
    (confidence * (1.0 - uncertainty.clamp(0.0,1.0) * 0.35)).clamp(0.0,1.0)
}

fn compose_supported(confidence: f32, depth: usize, weave: &crate::duyen_weave::DuyenWeave) -> String {
    let opening = if confidence >= 0.85 {
        "Các Duyên hiện tại ủng hộ mạnh kết luận này."
    } else if confidence >= 0.65 {
        "Các Duyên hiện tại nghiêng về phía ủng hộ kết luận này."
    } else {
        "Có tín hiệu ủng hộ, nhưng mức chắc chắn vẫn còn giới hạn."
    };
    if weave.is_overlapping() {
        format!(
            "{} Có {} nhánh Duyên cùng tham gia, với {} điểm hội tụ; nhánh rõ nhất sâu {} mắt xích. Độ tin cậy khoảng {:.0}%.",
            opening,
            weave.supporting_paths + weave.opposing_paths,
            weave.convergence_nodes.len(),
            weave.max_depth.max(depth),
            confidence * 100.0
        )
    } else {
        format!(
            "{} Tôi tìm được chuỗi suy luận gồm {} mắt xích với độ tin cậy khoảng {:.0}%.",
            opening,
            depth,
            confidence * 100.0
        )
    }
}

fn compose_opposed(confidence: f32, depth: usize, weave: &crate::duyen_weave::DuyenWeave) -> String {
    let opening = if confidence >= 0.80 {
        "Bằng chứng phản đối đang chiếm ưu thế."
    } else {
        "Hiện có nhiều Duyên phản đối hơn Duyên ủng hộ."
    };
    if weave.is_overlapping() {
        format!(
            "{} Có {} nhánh Duyên chồng lấp, trong đó {} nhánh mang tác dụng ức chế/phản đối; độ sâu lớn nhất {} mắt xích, độ tin cậy khoảng {:.0}%.",
            opening,
            weave.supporting_paths + weave.opposing_paths,
            weave.opposing_paths,
            weave.max_depth.max(depth),
            confidence * 100.0
        )
    } else {
        format!(
            "{} Chuỗi phản chứng có {} mắt xích, độ tin cậy khoảng {:.0}%.",
            opening,
            depth,
            confidence * 100.0
        )
    }
}
