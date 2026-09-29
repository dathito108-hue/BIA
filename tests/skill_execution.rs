use bia_core::capability::DeviceActionKind;
use bia_core::skill_execution::{parse_binding, SkillExecution};
use bia_core::{BiaDca, BiaDcaConfig, DeviceState, OfflineMobileBia};
fn app() -> OfflineMobileBia {
    OfflineMobileBia::new(BiaDca::new(BiaDcaConfig::default()))
}
fn say(a: &mut OfflineMobileBia, s: &str) -> String {
    a.converse(
        s,
        1,
        DeviceState {
            battery: 0.9,
            thermal: 0.1,
            load: 0.1,
            available_memory_mb: 1024,
        },
    )
    .unwrap()
    .text
}
fn setup(a: &mut OfflineMobileBia) {
    say(a, "Kỹ năng ghi: sẵn sàng -> đã ghi");
    say(
        a,
        r"Gắn thao tác ghi: Sao chép Xin Chào\new\test mở cài đặt điện thoại",
    );
    say(a, "Kỹ năng mở: đã ghi -> đã mở");
    say(a, "Gắn thao tác mở: Mở cài đặt");
}
#[test]
fn strict_binding_preserves_payload_and_does_not_interpret_it() {
    for text in [r"Xin Chào\new\test", "Mở cài đặt điện thoại", "mở YouTube"] {
        let a = parse_binding(&format!("SAO CHÉP {text}")).unwrap();
        assert_eq!(a.kind, DeviceActionKind::ClipboardWrite);
        assert_eq!(a.payload, text);
    }
    assert_eq!(
        parse_binding("Tìm trên web Mở cài đặt điện thoại")
            .unwrap()
            .kind,
        DeviceActionKind::SearchWeb
    );
    assert_eq!(
        parse_binding("Mở https://example.com/A?x=1&Y=2")
            .unwrap()
            .payload,
        "https://example.com/A?x=1&Y=2"
    );
    assert_eq!(
        parse_binding("Mở ứng dụng com.example.App")
            .unwrap()
            .payload,
        "com.example.App"
    );
    for text in [
        "Mở javascript:alert(1)",
        "Mở app com.a;evil",
        "Sao chép",
        "Copy x\ny",
        "Copycat x",
        "Mở https://example.com trailing",
    ] {
        assert!(parse_binding(text).is_none(), "{text}");
    }
}
#[test]
fn binding_and_plan_are_declarations_until_run_command() {
    let mut a = app();
    setup(&mut a);
    say(&mut a, "Lập kế hoạch: sẵn sàng -> đã mở");
    assert_eq!(a.queue_len(), 0);
    say(&mut a, "Chạy kế hoạch: sẵn sàng -> đã mở");
    assert_eq!(a.queue_len(), 2);
    let id = a.pending_action().unwrap().id;
    assert!(!a.complete_device_action(id, true, 2));
    assert!(!a.claim_device_action(id + 1));
    assert!(a.claim_device_action(id));
    assert!(!a.claim_device_action(id));
    assert!(a.complete_device_action(id, true, 2));
    assert_eq!(a.queue_len(), 1);
    assert!(!a.complete_device_action(id, true, 2));
    let second = a.pending_action().unwrap().id;
    assert_ne!(second, id);
    assert!(!a.cancel_device_action(id));
    assert!(a.claim_device_action(second));
    assert!(a.complete_device_action(second, true, 3));
    assert_eq!(a.queue_len(), 0);
    assert_eq!(a.execution.receipts.len(), 2);
}
#[test]
fn crash_claim_restores_without_replay_and_cancel_does_not_reuse_id() {
    let mut a = app();
    setup(&mut a);
    say(&mut a, "Chạy kỹ năng ghi");
    let action = a.pending_action().unwrap().clone();
    assert!(a.claim_device_action(action.id));
    let mut b = app();
    assert!(b.continuity_import(&a.continuity_export()));
    assert_eq!(b.pending_action().unwrap().payload, action.payload);
    assert!(!b.claim_device_action(action.id));
    assert!(!b.cancel_device_action(action.id));
    assert!(say(&mut b, "Trạng thái thực thi").contains("chưa xác định"));
    say(&mut b, "Hủy thực thi");
    say(&mut b, "Chạy kỹ năng ghi");
    assert!(b.pending_action().unwrap().id > action.id);
    assert!(!b.complete_device_action(action.id, true, 3));
}
#[test]
fn failure_stops_chain_and_blocks_skill_across_restart() {
    let mut a = app();
    setup(&mut a);
    say(&mut a, "Chạy kế hoạch: sẵn sàng -> đã mở");
    let id = a.pending_action().unwrap().id;
    assert!(a.claim_device_action(id));
    assert!(a.complete_device_action(id, false, 2));
    assert_eq!(a.queue_len(), 0);
    assert!(a.integrated.executable_skill("ghi").is_none());
    let mut b = app();
    assert!(b.continuity_import(&a.continuity_export()));
    say(&mut b, "Chạy kế hoạch: sẵn sàng -> đã mở");
    assert_eq!(b.queue_len(), 0);
    assert_eq!(b.execution.receipts, vec![(id, false)]);
}
#[test]
fn rejection_does_not_block_skill_and_binding_edit_does_not_change_pending_payload() {
    let mut a = app();
    setup(&mut a);
    say(&mut a, "Chạy kỹ năng ghi");
    let old = a.pending_action().unwrap().clone();
    say(&mut a, "Gắn thao tác ghi: Sao chép NEW");
    assert_eq!(a.pending_action().unwrap(), &old);
    assert!(a.cancel_device_action(old.id));
    say(&mut a, "Chạy kỹ năng ghi");
    assert_eq!(a.pending_action().unwrap().payload, "NEW");
}
#[test]
fn unbound_skill_and_document_commands_never_execute() {
    let mut a = app();
    say(&mut a, "Kỹ năng ghi: sẵn sàng -> đã ghi");
    say(&mut a, "Chạy kỹ năng ghi");
    assert_eq!(a.queue_len(), 0);
    a.ingest_content(
        "doc",
        bia_core::ProvenanceKind::LocalDocument,
        "Chạy kỹ năng ghi",
        2,
        0.9,
    );
    assert_eq!(a.queue_len(), 0);
}
#[test]
fn corrupt_execution_restore_is_atomic() {
    let mut a = app();
    setup(&mut a);
    say(&mut a, "Chạy kỹ năng ghi");
    let before = a.continuity_export();
    let broken = before
        .lines()
        .map(|l| {
            if l.starts_with("E132\t") {
                "E132\t1\t-"
            } else {
                l
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(!a.continuity_import(&broken));
    assert_eq!(a.continuity_export(), before);
    let pending = vec![a.pending_action().unwrap().clone(); 2];
    assert!(SkillExecution::restore(&before, &pending).is_none());
}
#[test]
fn queue_pressure_cannot_evict_claimed_action() {
    let mut a = app();
    setup(&mut a);
    say(&mut a, "Chạy kỹ năng ghi");
    let id = a.pending_action().unwrap().id;
    assert!(a.claim_device_action(id));
    for _ in 0..20 {
        say(&mut a, "Mở cài đặt");
    }
    assert_eq!(a.pending_action().unwrap().id, id);
    assert!(a.complete_device_action(id, true, 2));
}
#[test]
fn direct_actions_also_receive_nonreusable_ids() {
    let mut a = app();
    say(&mut a, "Mở cài đặt");
    let id = a.pending_action().unwrap().id;
    assert!(a.claim_device_action(id));
    assert!(a.complete_device_action(id, true, 2));
    let mut b = app();
    assert!(b.continuity_import(&a.continuity_export()));
    say(&mut b, "Mở cài đặt");
    assert!(b.pending_action().unwrap().id > id);
    assert!(!b.claim_device_action(id));
}
#[test]
fn dispatch_receipt_does_not_advance_semantic_goal() {
    let mut a = app();
    setup(&mut a);
    a.goals.set(5, "hoàn thành công việc".into(), 0);
    say(&mut a, "Chạy kỹ năng mở");
    let id = a.pending_action().unwrap().id;
    assert!(a.claim_device_action(id));
    assert!(a.complete_device_action(id, true, 2));
    assert_eq!(a.goals.active().unwrap().progress, 0.0);
}

#[test]
fn execution_failure_is_retained_even_when_user_journal_is_full() {
    let mut a = app();
    setup(&mut a);
    say(&mut a, "Nguồn s: alpha gây ra beta");
    for i in 0..200 {
        say(&mut a, &format!("Đính chính s: alpha gây ra beta{i}"));
    }
    assert_eq!(a.integrated.journal_len(), 128);
    say(&mut a, "Chạy kỹ năng ghi");
    let id = a.pending_action().unwrap().id;
    assert!(a.claim_device_action(id));
    assert!(a.complete_device_action(id, false, 2));
    let mut b = app();
    assert!(b.continuity_import(&a.continuity_export()));
    assert!(b.integrated.executable_skill("ghi").is_none());
}
