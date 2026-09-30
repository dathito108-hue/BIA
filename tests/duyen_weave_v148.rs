use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn overlapping_duyen_paths_shape_generation() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra độ ẩm.");
    c.handle("Nguồn b: độ ẩm gây ra đường trơn.");
    c.handle("Nguồn c: mưa gây ra bùn.");
    c.handle("Nguồn d: bùn gây ra đường trơn.");
    let answer = c.handle("Mưa có gây ra đường trơn không?").expect("answer");
    assert!(answer.contains("nhánh Duyên"), "{answer}");
    assert!(answer.contains("2"), "{answer}");
}

#[test]
fn overlapping_support_and_inhibition_remains_explicit() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra độ ẩm.");
    c.handle("Nguồn b: độ ẩm gây ra đường trơn.");
    c.handle("Nguồn c: mưa gây ra thoát nước.");
    c.handle("Nguồn d: thoát nước ngăn đường trơn.");
    let answer = c.handle("Mưa có gây ra đường trơn không?").expect("answer");
    assert!(answer.contains("Duyên"), "{answer}");
    assert!(answer.contains("phản đối") || answer.contains("xung đột"), "{answer}");
}

#[test]
fn weave_does_not_create_execution_authority() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: ý định gây ra kế hoạch.");
    c.handle("Nguồn b: kế hoạch gây ra hành động.");
    let _ = c.handle("Ý định có gây ra hành động không?");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
