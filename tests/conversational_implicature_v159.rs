use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn acknowledgement_keeps_context_without_repeating_reasoning() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v159a: mưa gây ra độ ẩm.");
    c.handle("Nguồn v159b: độ ẩm gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");

    let ack=c.handle("Hiểu rồi").expect("ack");
    assert!(ack.contains("không lặp lại"),"{ack}");
    assert!(!ack.contains("Đường suy luận:"),"{ack}");

    let why=c.handle("Tại sao?").expect("why");
    assert!(why.contains("Đường suy luận:"),"{why}");
}

#[test]
fn confirmation_and_doubt_do_not_create_fake_semantic_turns() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v159a: mưa gây ra đường ướt.");
    c.handle("Nguồn v159b: gió gây ra sóng.");
    c.handle("Mưa có gây ra đường ướt không?");
    c.handle("Gió có gây ra sóng không?");

    let confirm=c.handle("Đúng chứ?").expect("confirm");
    assert!(confirm.contains("xác nhận kết luận"),"{confirm}");

    let doubt=c.handle("Có chắc không?").expect("doubt");
    assert!(doubt.contains("không tăng độ chắc"),"{doubt}");
    assert!(doubt.contains("Đường suy luận:"),"{doubt}");

    let previous=c.handle("Ý trước").expect("previous");
    assert!(previous.contains("mua") || previous.contains("duong uot"),"{previous}");
}

#[test]
fn new_only_reports_no_delta_when_relation_evidence_is_unchanged() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v159a: mưa gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");

    let r=c.handle("Có gì mới?").expect("delta");
    assert!(r.contains("Chưa có bằng chứng mới"),"{r}");
    assert!(!r.contains("Đường suy luận:"),"{r}");
}

#[test]
fn new_only_reports_relation_scoped_added_source_not_unrelated_noise() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v159a: mưa gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");
    c.handle("Nguồn noise: nhiệt gây ra giãn nở.");

    let none=c.handle("Chỉ nói phần mới").expect("no relevant delta");
    assert!(none.contains("Chưa có bằng chứng mới"),"{none}");
    assert!(!none.contains("noise"),"{none}");

    c.handle("Nguồn v159b: mưa gây ra độ ẩm.");
    c.handle("Nguồn v159c: độ ẩm gây ra đường trơn.");
    let delta=c.handle("Có gì mới?").expect("relevant delta");
    assert!(delta.contains("nguồn thêm"),"{delta}");
    assert!(delta.contains("v159b") || delta.contains("v159c"),"{delta}");
    assert!(delta.contains("Sau khi cập nhật"),"{delta}");
}

#[test]
fn implicature_layer_never_creates_execution_authority() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v159a: ý định gây ra hành động.");
    c.handle("Ý định có gây ra hành động không?");
    let _=c.handle("Đúng chứ?");
    let _=c.handle("Có chắc không?");
    let _=c.handle("Có gì mới?");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
