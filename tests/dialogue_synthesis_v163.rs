use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn natural_answer_keeps_cause_effect_and_evidence_in_one_turn() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra đường trơn.");
    c.handle("Nguồn b: độ ẩm làm đường trơn.");
    let r = c.handle("Mưa có gây ra đường trơn không?").expect("answer");
    let lower = r.to_lowercase();
    assert!(lower.contains("mưa") || lower.contains("mua"), "{r}");
    assert!(lower.contains("đường trơn") || lower.contains("duong tron"), "{r}");
    assert!(lower.contains("nguồn") || lower.contains("nguon") || r.contains("Đã xét"), "{r}");
}

#[test]
fn overlapping_relation_gets_qualified_without_changing_the_answer() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra đường trơn.");
    c.handle("Nguồn b: độ ẩm làm đường trơn.");
    let first = c.handle("Mưa có gây ra đường trơn không?").expect("answer");
    let second = c.handle("Có chắc không?").expect("review");
    assert!(!first.is_empty() && !second.is_empty());
    assert!(second.contains("chắc") || second.contains("bằng chứng") || second.contains("kiểm tra"), "{second}");
}
