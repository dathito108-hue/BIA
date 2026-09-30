use bia_core::{DialogueSynthesisPlan, OpenDialogueMove, OpenDialoguePlan, ResponseDepth};
use bia_core::integrated_cognition::IntegratedCognition;

#[test]
fn v164_depth_is_selected_from_context() {
    let s = DialogueSynthesisPlan {
        moves: vec![bia_core::DialogueMove::Answer, bia_core::DialogueMove::Ground],
        uncertainty: 0.1, overlap: 0.0, depth: ResponseDepth::Standard,
    };
    assert_eq!(s.cap_for(bia_core::ExpressionStyle::Brief), ResponseDepth::Brief);
    assert_eq!(s.cap_for(bia_core::ExpressionStyle::Deep), ResponseDepth::Deep);
}

#[test]
fn v165_open_dialogue_can_continue_without_new_model() {
    let s = DialogueSynthesisPlan {
        moves: vec![bia_core::DialogueMove::Answer, bia_core::DialogueMove::Continue, bia_core::DialogueMove::Invite],
        uncertainty: 0.4, overlap: 0.7, depth: ResponseDepth::Deep,
    };
    let p = OpenDialoguePlan::from_synthesis(&s);
    assert!(p.has(OpenDialogueMove::Continue));
    assert!(p.has(OpenDialogueMove::Ask));
    let text = p.compose("Mạch hiện tại đã được nối.".into(), "mưa", "đường trơn");
    assert!(text.contains("mưa") && text.contains("đường trơn"));
}

#[test]
fn v163_v165_stays_in_one_dialogue_state_machine() {
    let mut c = IntegratedCognition::default();
    c.handle("Nguồn a: mưa gây ra đường trơn.");
    c.handle("Nguồn b: độ ẩm làm đường trơn.");
    let a = c.handle("Mưa có gây ra đường trơn không?").expect("answer");
    let b = c.handle("Tại sao lại như vậy?").expect("expand");
    let d = c.handle("Bạn dựa vào đâu?").expect("sources");
    assert!(!a.is_empty() && !b.is_empty() && !d.is_empty());
}
