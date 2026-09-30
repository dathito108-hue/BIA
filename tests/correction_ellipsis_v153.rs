use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn correction_updates_dialogue_context_not_sources() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra đường ướt.");
    c.handle("Nguồn b: mưa gây ra đường trơn.");
    let first = c.handle("Mưa có gây ra đường ướt không?").expect("first");
    assert!(first.contains("ủng hộ"), "{first}");

    let corrected = c
        .handle("Ý tôi là đường trơn chứ không phải đường ướt")
        .expect("corrected");
    assert!(corrected.contains("duong tron"), "{corrected}");

    let sources = c.handle("Bạn dựa vào đâu?").expect("sources");
    assert!(sources.contains("nguon b") || sources.contains("Nguồn b"), "{sources}");
}

#[test]
fn previous_case_resolves_boundedly() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra đường ướt.");
    c.handle("Nguồn b: gió gây ra sóng.");
    c.handle("Mưa có gây ra đường ướt không?");
    c.handle("Gió có gây ra sóng không?");
    let previous = c.handle("Trường hợp kia").expect("previous");
    assert!(previous.contains("mua") || previous.contains("duong uot"), "{previous}");
}

#[test]
fn ordinal_duyen_removal_does_not_guess() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra độ ẩm.");
    c.handle("Nguồn b: độ ẩm gây ra đường trơn.");
    c.handle("Mưa có gây ra đường trơn không?");
    let r = c.handle("Bỏ duyên thứ hai thì sao?").expect("clarify");
    assert!(r.contains("nêu tên duyên"), "{r}");
}
