use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn challenge_finds_only_grounded_inhibiting_source_on_current_relation() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v161a: mưa gây ra đường trơn.");
    c.handle("Nguồn v161b: mưa gây ra bùn.");
    c.handle("Nguồn v161c: bùn ngăn đường trơn.");
    c.handle("Nguồn noise: nhiệt gây ra giãn nở.");
    c.handle("Mưa có gây ra đường trơn không?");

    let r=c.handle("Có phản chứng không?").expect("challenge");
    assert!(r.contains("Có phản chứng"),"{r}");
    assert!(r.contains("v161c"),"{r}");
    assert!(!r.contains("noise"),"{r}");
}

#[test]
fn goal_progression_survives_nonsemantic_turns_and_concludes_current_relation() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v161a: mưa gây ra đường trơn.");
    c.handle("Nguồn v161b: mưa gây ra bùn.");
    c.handle("Nguồn v161c: bùn ngăn đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");

    let verify=c.handle("Có chắc không?").expect("verify");
    assert!(verify.contains("không tăng độ chắc"),"{verify}");

    let challenge=c.handle("Tìm phản chứng").expect("challenge");
    assert!(challenge.contains("v161c"),"{challenge}");

    c.handle("Nguồn v161d: mưa gây ra độ ẩm.");
    c.handle("Nguồn v161e: độ ẩm gây ra đường trơn.");

    let conclusion=c.handle("Vậy kết luận thế nào?").expect("conclusion");
    assert!(conclusion.contains("Sau bước kiểm tra phản chứng"),"{conclusion}");
    assert!(
        conclusion.contains("xung đột")
            || conclusion.contains("ủng hộ")
            || conclusion.contains("phản đối"),
        "{conclusion}"
    );
}

#[test]
fn new_semantic_relation_reanchors_implicit_goal_state() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v161a: mưa gây ra đường trơn.");
    c.handle("Nguồn v161b: mưa gây ra bùn.");
    c.handle("Nguồn v161c: bùn ngăn đường trơn.");
    c.handle("Nguồn v161d: gió gây ra sóng.");

    c.handle("Mưa có gây ra đường trơn không?");
    let _=c.handle("Có phản chứng không?");

    c.handle("Gió có gây ra sóng không?");
    let conclusion=c.handle("Chốt lại").expect("conclusion");
    assert!(conclusion.starts_with("Theo bằng chứng hiện có"),"{conclusion}");
    assert!(!conclusion.contains("Sau bước kiểm tra phản chứng"),"{conclusion}");
}

#[test]
fn explicit_topic_shift_clears_active_implicit_goal() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v161a: mưa gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");
    let _=c.handle("Có chắc không?");
    c.handle("Chuyển chủ đề sang âm nhạc");

    let r=c.handle("Chốt lại").expect("no active relation");
    assert!(r.contains("Chưa có quan hệ hiện tại"),"{r}");
}

#[test]
fn dialogue_goal_tracking_never_creates_execution_authority() {
    let mut c=IntegratedCognition::default();
    c.handle("Nguồn v161a: ý định gây ra hành động.");
    c.handle("Ý định có gây ra hành động không?");
    let _=c.handle("Có chắc không?");
    let _=c.handle("Có phản chứng không?");
    let _=c.handle("Chốt lại");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
