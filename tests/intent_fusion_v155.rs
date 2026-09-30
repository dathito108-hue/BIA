use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn fuses_focus_exclusion_and_previous_conclusion() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra đường trơn.");
    c.handle("Nguồn b: gió gây ra sóng.");
    c.handle("Mưa có gây ra đường trơn không?");
    c.handle("Gió có gây ra sóng không?");

    let r = c
        .handle("Giải thích nhưng tập trung vào sóng, bỏ phần so sánh và kết luận theo trường hợp trước")
        .expect("fused");
    assert!(r.contains("giữ trọng tâm"), "{r}");
    assert!(r.contains("Trước hết"), "{r}");
    assert!(r.contains("mốc tham chiếu"), "{r}");
    assert!(!r.contains("Đặt cạnh trường hợp trước"), "{r}");
    assert!(!r.contains("Giải thích:"), "{r}");
}

#[test]
fn rejects_focus_outside_current_semantic_roles() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");
    let r = c
        .handle("Giải thích và tập trung vào bão rồi tóm tắt")
        .expect("clarify");
    assert!(r.contains("chưa khớp rõ"), "{r}");
}

#[test]
fn intent_fusion_does_not_create_authority() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: ý định gây ra kế hoạch.");
    c.handle("Nguồn b: kế hoạch gây ra hành động.");
    c.handle("Ý định có gây ra hành động không?");
    let _ = c.handle("Giải thích và tóm tắt, tập trung vào hành động");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
