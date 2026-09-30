use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn causal_answer_uses_duyen_geometry_for_natural_opening() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v156a: mưa gây ra độ ẩm.");
    c.handle("Nguồn v156b: độ ẩm gây ra đường trơn.");
    c.handle("Nguồn v156c: mưa gây ra bùn.");
    c.handle("Nguồn v156d: bùn gây ra đường trơn.");

    let answer = c.handle("Mưa có gây ra đường trơn không?").expect("answer");
    assert!(
        answer.starts_with("Nếu nhìn theo các Duyên đang chồng lên nhau"),
        "{answer}"
    );
    assert!(answer.contains("nhánh Duyên"), "{answer}");
    assert!(!answer.starts_with("Về quan hệ giữa"), "{answer}");
}

#[test]
fn fused_request_becomes_flowing_grounded_prose() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v156a: mưa gây ra đường trơn.");
    c.handle("Nguồn v156b: gió gây ra sóng.");
    c.handle("Mưa có gây ra đường trơn không?");
    c.handle("Gió có gây ra sóng không?");

    let answer = c
        .handle("Giải thích nhưng tập trung vào sóng, bỏ phần so sánh và kết luận theo trường hợp trước")
        .expect("fused");
    assert!(answer.contains("giữ trọng tâm"), "{answer}");
    assert!(answer.contains("Trước hết"), "{answer}");
    assert!(answer.contains("mốc tham chiếu"), "{answer}");
    assert!(!answer.contains("Giải thích:"), "{answer}");
    assert!(!answer.contains("Đối chiếu:"), "{answer}");
}

#[test]
fn brief_followup_stays_direct_and_grounded() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v156a: mưa gây ra độ ẩm.");
    c.handle("Nguồn v156b: độ ẩm gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");

    let brief = c.handle("Nói ngắn gọn").expect("brief");
    assert!(brief.contains("ủng hộ"), "{brief}");
    assert!(!brief.contains("Về quan hệ giữa"), "{brief}");
    assert!(!brief.contains("Đường suy luận:"), "{brief}");
}

#[test]
fn natural_surface_never_creates_execution_authority() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v156a: ý định gây ra kế hoạch.");
    c.handle("Nguồn v156b: kế hoạch gây ra hành động.");
    let _ = c.handle("Ý định có gây ra hành động không?");
    let _ = c.handle("Giải thích rồi tóm tắt ngắn gọn");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
