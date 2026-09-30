use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn same_subject_continues_the_discourse_without_restarting() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v157a: mưa gây ra độ ẩm.");
    c.handle("Nguồn v157b: độ ẩm gây ra đường trơn.");
    c.handle("Nguồn v157c: mưa gây ra bùn.");

    let first = c.handle("Mưa có gây ra đường trơn không?").expect("first");
    assert!(first.contains("mưa") || first.contains("mua"), "{first}");

    let next = c.handle("Mưa có gây ra bùn không?").expect("next");
    assert!(next.starts_with("Tiếp theo mạch về"), "{next}");
    assert!(next.contains("bùn") || next.contains("bun"), "{next}");
}

#[test]
fn repeated_question_omits_details_already_given() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v157a: mưa gây ra độ ẩm.");
    c.handle("Nguồn v157b: độ ẩm gây ra đường trơn.");

    let first = c.handle("Mưa có gây ra đường trơn không?").expect("first");
    assert!(first.contains("Đường suy luận:"), "{first}");

    let repeated = c.handle("Mưa có gây ra đường trơn không?").expect("repeat");
    assert!(repeated.starts_with("Vẫn ở quan hệ này"), "{repeated}");
    assert!(repeated.contains("Mạch bằng chứng không đổi"), "{repeated}");
    assert!(!repeated.contains("Đường suy luận:"), "{repeated}");
    assert!(!repeated.contains("Đã xét các nguồn:"), "{repeated}");
}

#[test]
fn rerendering_does_not_pollute_previous_turn_resolution() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v157a: mưa gây ra đường ướt.");
    c.handle("Nguồn v157b: gió gây ra sóng.");
    c.handle("Mưa có gây ra đường ướt không?");
    c.handle("Gió có gây ra sóng không?");

    let brief = c.handle("Nói ngắn gọn").expect("brief");
    assert!(brief.contains("ủng hộ"), "{brief}");

    let previous = c.handle("Ý trước").expect("previous");
    assert!(previous.starts_with("Quay lại quan hệ giữa"), "{previous}");
    assert!(previous.contains("mua"), "{previous}");
    assert!(previous.contains("duong uot"), "{previous}");
}

#[test]
fn discourse_continuity_does_not_create_execution_authority() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v157a: ý định gây ra kế hoạch.");
    c.handle("Nguồn v157b: kế hoạch gây ra hành động.");
    let _ = c.handle("Ý định có gây ra hành động không?");
    let _ = c.handle("Ý định có gây ra hành động không?");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
