use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn composed_ack_doubt_new_only_preserves_delta_until_output_filter() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v160a: mưa gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");
    c.handle("Nguồn v160b: mưa gây ra độ ẩm.");
    c.handle("Nguồn v160c: độ ẩm gây ra đường trơn.");

    let r=c
        .handle("Tôi hiểu rồi, nhưng có chắc không, nếu có gì mới thì chỉ nói phần mới thôi")
        .expect("composed");
    assert!(r.contains("Phần mới"),"{r}");
    assert!(r.contains("nguồn thêm"),"{r}");
    assert!(r.contains("v160b") || r.contains("v160c"),"{r}");
    assert!(!r.contains("Đường suy luận:"),"{r}");
    assert!(!r.contains("Đã hiểu"),"{r}");
}

#[test]
fn composed_doubt_new_only_reports_no_delta_without_repeating_answer() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v160a: mưa gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");

    let r=c
        .handle("Có chắc không, nếu không có gì mới thì chỉ nói phần mới")
        .expect("composed");
    assert!(r.contains("đã kiểm tra lại") || r.contains("Đã kiểm tra lại"),"{r}");
    assert!(r.contains("Chưa có bằng chứng mới"),"{r}");
    assert!(!r.contains("Đường suy luận:"),"{r}");
}

#[test]
fn confirm_plus_expand_emits_one_deep_grounded_response() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v160a: mưa gây ra độ ẩm.");
    c.handle("Nguồn v160b: độ ẩm gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");

    let r=c.handle("Đúng chứ, nói thêm đi").expect("composed");
    assert!(r.contains("Mở rộng thêm"),"{r}");
    assert!(r.contains("Đường suy luận:"),"{r}");
    assert!(!r.contains("xác nhận kết luận"),"{r}");
}

#[test]
fn composed_intents_do_not_pollute_semantic_turn_history() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v160a: mưa gây ra đường ướt.");
    c.handle("Nguồn v160b: gió gây ra sóng.");
    c.handle("Mưa có gây ra đường ướt không?");
    c.handle("Gió có gây ra sóng không?");

    let _=c.handle("Hiểu rồi nhưng có chắc không và nói thêm");
    let previous=c.handle("Ý trước").expect("previous");
    assert!(previous.contains("mua") || previous.contains("duong uot"),"{previous}");
}

#[test]
fn dynamic_composition_never_creates_execution_authority() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v160a: ý định gây ra hành động.");
    c.handle("Ý định có gây ra hành động không?");
    let _=c.handle("Hiểu rồi, nhưng có chắc không, nói thêm và chỉ nói phần mới");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
