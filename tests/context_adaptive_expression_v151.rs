use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn default_style_deepens_for_overlapping_weave() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra độ ẩm.");
    c.handle("Nguồn b: độ ẩm gây ra đường trơn.");
    c.handle("Nguồn c: mưa gây ra bùn.");
    c.handle("Nguồn d: bùn gây ra đường trơn.");

    let answer = c.handle("Mưa có gây ra đường trơn không?").expect("answer");
    assert!(answer.contains("nhánh Duyên"), "{answer}");
    assert!(
        answer.contains("liên kết dùng chung") || answer.contains("mức chồng lấp"),
        "{answer}"
    );
}

#[test]
fn explicit_brief_overrides_adaptive_depth() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra độ ẩm.");
    c.handle("Nguồn b: độ ẩm gây ra đường trơn.");
    c.handle("Nguồn c: mưa gây ra bùn.");
    c.handle("Nguồn d: bùn gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");

    let brief = c.handle("Nói ngắn gọn").expect("brief");
    assert!(brief.contains("ủng hộ"), "{brief}");
    assert!(!brief.contains("liên kết dùng chung"), "{brief}");
    assert!(!brief.contains("Đường suy luận:"), "{brief}");
}

#[test]
fn adaptive_style_does_not_change_authority() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: ý định gây ra kế hoạch.");
    c.handle("Nguồn b: kế hoạch gây ra hành động.");
    let _ = c.handle("Ý định có gây ra hành động không?");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
