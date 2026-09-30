use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn discourse_generator_orders_overlap_before_limit() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra độ ẩm.");
    c.handle("Nguồn b: độ ẩm gây ra đường trơn.");
    c.handle("Nguồn c: mưa gây ra bùn.");
    c.handle("Nguồn d: bùn gây ra đường trơn.");
    let answer = c.handle("Mưa có gây ra đường trơn không?").expect("answer");
    let overlap = answer.find("nhánh Duyên").expect("overlap");
    let confidence = answer.find("Mức chắc chắn").expect("confidence");
    assert!(overlap < confidence, "{answer}");
}

#[test]
fn discourse_generator_names_no_single_cause_when_opposition_exists() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra độ ẩm.");
    c.handle("Nguồn b: độ ẩm gây ra đường trơn.");
    c.handle("Nguồn c: mưa gây ra thoát nước.");
    c.handle("Nguồn d: thoát nước ngăn đường trơn.");
    let answer = c.handle("Mưa có gây ra đường trơn không?").expect("answer");
    assert!(answer.contains("Duyên") || answer.contains("xung đột"), "{answer}");
}

#[test]
fn generated_discourse_remains_non_executable() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: ý định gây ra kế hoạch.");
    c.handle("Nguồn b: kế hoạch gây ra hành động.");
    let _ = c.handle("Ý định có gây ra hành động không?");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
