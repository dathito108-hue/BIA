use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn infers_omitted_object_role_only_when_unique() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v158a: mưa gây ra độ ẩm.");
    c.handle("Nguồn v158b: mưa gây ra bùn.");
    c.handle("Mưa có gây ra độ ẩm không?");

    let r = c.handle("Còn bùn?").expect("elliptic object");
    assert!(r.starts_with("Tiếp theo mạch về"), "{r}");
    assert!(r.contains("bun"), "{r}");
    assert!(r.contains("ủng hộ"), "{r}");
}

#[test]
fn infers_omitted_subject_role_only_when_unique() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v158a: mưa gây ra đường trơn.");
    c.handle("Nguồn v158b: gió gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");

    let r = c.handle("Còn gió thì sao?").expect("elliptic subject");
    assert!(r.starts_with("Vẫn với kết quả"), "{r}");
    assert!(r.contains("gio"), "{r}");
    assert!(r.contains("ủng hộ"), "{r}");
}

#[test]
fn refuses_ellipsis_when_entity_has_both_semantic_roles() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v158a: mưa gây ra bùn.");
    c.handle("Nguồn v158b: bùn gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");

    let r = c.handle("Thế còn bùn thì sao?").expect("clarify");
    assert!(r.contains("nguyên nhân hoặc kết quả"), "{r}");
    assert!(r.contains("bun"), "{r}");
}

#[test]
fn explicit_topic_shift_clears_active_relation_but_not_sources() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v158a: mưa gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");

    let shifted = c
        .handle("Chuyển chủ đề sang âm nhạc")
        .expect("topic shift");
    assert!(shifted.contains("am nhac"), "{shifted}");

    let why = c.handle("Tại sao?").expect("clarify after shift");
    assert!(why.contains("câu hỏi nào"), "{why}");

    let again = c
        .handle("Mưa có gây ra đường trơn không?")
        .expect("source remains");
    assert!(again.contains("ủng hộ"), "{again}");
}

#[test]
fn pragmatic_resolution_never_creates_execution_authority() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn v158a: ý định gây ra kế hoạch.");
    c.handle("Nguồn v158b: ý định gây ra hành động.");
    c.handle("Ý định có gây ra kế hoạch không?");
    let _ = c.handle("Còn hành động?");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
