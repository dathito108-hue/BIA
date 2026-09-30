use bia_core::integrated_cognition::IntegratedCognition;
use bia_core::semantic::{QueryKind, VietnameseSemanticParser};

#[test]
fn natural_causal_forms_preserve_roles() {
    let parser = VietnameseSemanticParser;
    let a = parser.parse("Mưa có phải là nguyên nhân của đường ướt không?");
    let q = a.query.expect("direct natural query");
    assert_eq!(q.kind, QueryKind::Causal);
    assert_eq!(q.subject.text, "mua");
    assert_eq!(q.object.text, "duong uot");

    let b = parser.parse("Đường ướt có phải là do mưa không?");
    let q = b.query.expect("inverse natural query");
    assert_eq!(q.subject.text, "mua");
    assert_eq!(q.object.text, "duong uot");
}

#[test]
fn polite_free_form_question_reaches_grounded_reasoning() {
    let mut c = IntegratedCognition::default();
    c.handle("Ghi nhớ rằng mưa gây ra đường ướt.");
    let answer = c
        .handle("BIA ơi, cho mình hỏi, mưa có phải là nguyên nhân của đường ướt không nhỉ?")
        .expect("dialogue answer");
    assert!(answer.contains("ủng hộ"), "{answer}");
}

#[test]
fn discourse_freedom_does_not_guess_ambiguous_roles_or_create_actions() {
    let mut c = IntegratedCognition::default();
    c.handle("Ghi nhớ rằng gió gây ra sóng.");
    c.handle("Gió có gây ra sóng không?");
    let ambiguous = c.handle("Thế còn bão thì sao?").expect("clarification");
    assert!(ambiguous.contains("nguyên nhân hay kết quả"), "{ambiguous}");
    assert!(c.executable_plan("gio -> song").is_none());
}
