use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn repairs_explicit_misunderstanding_without_inventing_a_new_fact() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v162a: mưa gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");
    let r=c.handle("Bạn hiểu sai rồi").expect("repair");
    assert!(r.contains("điểm lệch"),"{r}");
    let follow=c.handle("Ý tôi là hỏi nguyên nhân của đường trơn").expect("clarify");
    assert!(follow.contains("sửa mạch hiểu"),"{follow}");
}

#[test]
fn repair_signal_does_not_execute_or_create_authority() {
    let mut c=IntegratedCognition::default();
    c.handle("Ý định gây ra hành động.");
    c.handle("Ý định có gây ra hành động không?");
    let r=c.handle("Không phải hành động").expect("repair");
    assert!(r.contains("sửa mạch hiểu"),"{r}");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}

#[test]
fn repair_is_bounded_and_reanchored_by_next_semantic_question() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v162a: gió gây ra sóng.");
    c.handle("Gió có gây ra sóng không?");
    let r=c.handle("Không đúng ý tôi").expect("repair");
    assert!(r.contains("điểm lệch"),"{r}");
    let q=c.handle("Mưa có gây ra đường trơn không?").expect("new question");
    assert!(q.contains("chưa") || q.contains("không") || q.contains("bằng chứng") || q.contains("đường trơn"),"{q}");
}
