use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn composes_explain_compare_and_summary_from_grounded_turns() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra đường ướt.");
    c.handle("Nguồn b: gió gây ra sóng.");
    c.handle("Mưa có gây ra đường ướt không?");
    c.handle("Gió có gây ra sóng không?");

    let r = c
        .handle("Giải thích rõ, so sánh với trường hợp trước rồi tóm tắt ngắn gọn")
        .expect("composite");
    assert!(r.contains("Giải thích hiện tại:"), "{r}");
    assert!(r.contains("Đối chiếu:"), "{r}");
    assert!(r.contains("Tóm tắt:"), "{r}");
    assert!(r.contains("ủng hộ"), "{r}");
}

#[test]
fn composite_request_requires_previous_case_for_comparison() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra đường ướt.");
    c.handle("Mưa có gây ra đường ướt không?");
    let r = c
        .handle("Giải thích và so sánh với trường hợp trước")
        .expect("clarify");
    assert!(r.contains("Chưa có trường hợp trước"), "{r}");
}

#[test]
fn compositional_dialogue_does_not_create_execution_authority() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: ý định gây ra kế hoạch.");
    c.handle("Nguồn b: kế hoạch gây ra hành động.");
    c.handle("Ý định có gây ra hành động không?");
    let _ = c.handle("Giải thích rồi tóm tắt ngắn gọn");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
