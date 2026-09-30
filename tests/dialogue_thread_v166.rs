use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn v166_free_follow_up_uses_existing_dialogue_state() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");

    let why = c.handle("Tại sao?").expect("why");
    let evidence = c.handle("Bạn dựa vào đâu?").expect("evidence");
    let next = c.handle("Tiếp theo?").expect("next");

    assert!(!why.is_empty());
    assert!(!evidence.is_empty());
    assert!(!next.is_empty());
}

#[test]
fn v166_unanchored_follow_up_keeps_normal_parser_available() {
    let mut c = IntegratedCognition::default();
    let response = c.handle("Tại sao?");
    assert!(response.is_some());
}
