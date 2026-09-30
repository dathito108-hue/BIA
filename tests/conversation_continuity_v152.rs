use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn resolves_previous_turn_and_ordinals_without_guessing_roles() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra đường ướt.");
    let first = c.handle("Mưa có gây ra đường ướt không?").expect("answer");
    assert!(first.contains("ủng hộ"), "{first}");

    let repeat = c.handle("Ý vừa rồi").expect("repeat");
    assert!(repeat.contains("ủng hộ"), "{repeat}");

    let second = c.handle("Cái thứ hai").expect("second");
    assert!(second.contains("duong uot"), "{second}");

    let ambiguous = c.handle("Cái thứ hai thì sao?").expect("ambiguous");
    assert!(ambiguous.contains("nguyên nhân hay kết quả"), "{ambiguous}");
}

#[test]
fn continuity_is_bounded_and_non_executable() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: ý định gây ra kế hoạch.");
    c.handle("Nguồn b: kế hoạch gây ra hành động.");
    let _ = c.handle("Ý định có gây ra hành động không?");
    let _ = c.handle("Ý vừa rồi");
    assert!(c.executable_plan("y dinh -> hanh dong").is_none());
}
