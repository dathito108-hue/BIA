use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn explicit_relation_repair_reanchors_the_conversation() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn gió: gió gây ra sóng.");
    c.handle("Nguồn mưa: mưa gây ra đường trơn.");

    let r=c.handle("Mưa có gây ra đường trơn không?").expect("initial");
    assert!(r.contains("ủng hộ"),"{r}");

    let repaired=c.handle("Không, ý tôi là gió gây ra sóng.").expect("repair");
    assert!(repaired.contains("Hiểu rồi"),"{repaired}");
    assert!(repaired.contains("gió"),"{repaired}");
    assert!(repaired.contains("sóng"),"{repaired}");

    let follow=c.handle("Có chắc không?").expect("follow");
    assert!(follow.contains("gió"),"{follow}");
    assert!(follow.contains("sóng"),"{follow}");
}

#[test]
fn subject_repair_preserves_object_and_goal_state() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn gió: gió gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");

    let repaired=c.handle("Không phải mưa mà gió.").expect("repair");
    assert!(repaired.contains("sửa nguyên nhân"),"{repaired}");
    assert!(repaired.contains("gió"),"{repaired}");
    assert!(repaired.contains("đường trơn"),"{repaired}");

    let conclusion=c.handle("Chốt lại").expect("conclusion");
    assert!(conclusion.contains("gió"),"{conclusion}");
    assert!(!conclusion.contains("Sau bước kiểm tra phản chứng"),"{conclusion}");
}

#[test]
fn object_repair_preserves_subject() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn mưa: mưa gây ra đường trơn.");
    c.handle("Nguồn mưa2: mưa gây ra đường ướt.");
    c.handle("Mưa có gây ra đường ướt không?");

    let repaired=c.handle("Không phải đường ướt, mà là đường trơn.").expect("repair");
    assert!(repaired.contains("sửa kết quả"),"{repaired}");
    assert!(repaired.contains("mưa"),"{repaired}");
    assert!(repaired.contains("đường trơn"),"{repaired}");
}

#[test]
fn ambiguous_grounding_repair_stops_and_asks_for_role() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn mưa: mưa gây ra mưa.");
    c.handle("Mưa có gây ra mưa không?");

    let r=c.handle("Không phải mưa mà gió.").expect("clarify");
    assert!(r.contains("cả hai vai"),"{r}");
}

#[test]
fn grounding_repair_never_creates_execution_authority() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn intent: ý định gây ra hành động.");
    c.handle("Ý định có gây ra hành động không?");
    let _=c.handle("Không phải ý định mà mong muốn.");
    let _=c.handle("Chốt lại");
    assert!(c.executable_plan("mong muon -> hanh dong").is_none());
}
