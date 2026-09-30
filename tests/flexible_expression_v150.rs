use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn brief_and_deep_keep_same_grounded_verdict() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra độ ẩm.");
    c.handle("Nguồn b: độ ẩm gây ra đường trơn.");
    c.handle("Nguồn c: mưa gây ra bùn.");
    c.handle("Nguồn d: bùn gây ra đường trơn.");

    let standard = c.handle("Mưa có gây ra đường trơn không?").expect("standard");
    assert!(standard.contains("ủng hộ"), "{standard}");

    let brief = c.handle("Nói ngắn gọn").expect("brief");
    assert!(brief.contains("ủng hộ"), "{brief}");
    assert!(!brief.contains("Đường suy luận:"), "{brief}");

    let deep = c.handle("Giải thích kỹ hơn").expect("deep");
    assert!(deep.contains("ủng hộ"), "{deep}");
    assert!(deep.contains("liên kết dùng chung") || deep.contains("mức chồng lấp"), "{deep}");
}

#[test]
fn style_changes_expression_not_authority() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: ý định gây ra kế hoạch.");
    c.handle("Nguồn b: kế hoạch gây ra hành động.");
    c.handle("Ý định có gây ra hành động không?");
    let _ = c.handle("Giải thích kỹ hơn");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
