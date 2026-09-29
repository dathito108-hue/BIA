use bia_core::game_agent::{GameAgent, GameProfile};
fn profile(moba: bool) -> GameProfile {
    GameProfile {
        roi: [0.1, 0.1, 0.9, 0.9],
        rgb: [255, 48, 48],
        tolerance: 30,
        moba,
    }
}
fn frame(x: usize, y: usize) -> Vec<u32> {
    let mut p = vec![0xff101820; 10000];
    for yy in y..y + 4 {
        for xx in x..x + 4 {
            p[yy * 100 + xx] = 0xffff3030;
        }
    }
    p
}
#[test]
fn two_distinct_frames_are_required_before_action() {
    let mut a = GameAgent::default();
    let p = frame(60, 40);
    let first = a.observe(&p, 100, 100, &profile(false), 1);
    assert!(!first.confirmed);
    let d = a.observe(&p, 100, 100, &profile(false), 101);
    assert!(d.confirmed);
    assert!(d.aim[0] > 0.0 && d.aim[1] < 0.0);
    assert!(!d.fire);
}
#[test]
fn moba_can_move_attack_and_skill_together() {
    let mut a = GameAgent::default();
    let p = frame(61, 37);
    a.observe(&p, 100, 100, &profile(true), 1);
    let d = a.observe(&p, 100, 100, &profile(true), 101);
    assert!(d.movement[0] > 0.0 && d.movement[1] < 0.0);
    assert!(d.fire && d.skill);
    let next = a.observe(&p, 100, 100, &profile(true), 201);
    assert!(next.fire && !next.skill);
}
#[test]
fn fps_only_fires_near_center() {
    let mut a = GameAgent::default();
    let p = frame(49, 49);
    a.observe(&p, 100, 100, &profile(false), 1);
    assert!(a.observe(&p, 100, 100, &profile(false), 101).fire);
}
#[test]
fn lost_target_stops_actions_and_requires_reconfirmation() {
    let mut a = GameAgent::default();
    let p = frame(49, 49);
    a.observe(&p, 100, 100, &profile(false), 1);
    a.observe(&p, 100, 100, &profile(false), 101);
    let d = a.observe(&vec![0; 10000], 100, 100, &profile(false), 201);
    assert!(d.target.is_none() && !d.fire);
    assert!(!a.observe(&p, 100, 100, &profile(false), 301).confirmed);
}
#[test]
fn stale_or_nonincreasing_frame_timestamps_do_not_act() {
    let mut a = GameAgent::default();
    let p = frame(49, 49);
    a.observe(&p, 100, 100, &profile(false), 100);
    assert!(!a.observe(&p, 100, 100, &profile(false), 100).confirmed);
    a.observe(&p, 100, 100, &profile(false), 101);
    assert!(!a.observe(&p, 100, 100, &profile(false), 900).confirmed);
}
#[test]
fn invalid_dimensions_and_nonfinite_roi_are_rejected() {
    let mut a = GameAgent::default();
    let mut p = profile(false);
    p.roi[0] = f32::NAN;
    assert!(a.observe(&frame(49, 49), 100, 100, &p, 1).target.is_none());
    assert!(a
        .observe(&[0], 50000, 50000, &profile(false), 2)
        .target
        .is_none());
}
#[test]
fn blank_background_large_color_regions_and_single_pixel_noise_are_not_targets() {
    let mut a = GameAgent::default();
    assert!(a
        .observe(&vec![0xffff3030; 10000], 100, 100, &profile(false), 1)
        .target
        .is_none());
    let mut p = vec![0; 10000];
    p[5050] = 0xffff3030;
    assert!(a
        .observe(&p, 100, 100, &profile(false), 101)
        .target
        .is_none());
}
#[test]
fn roi_excludes_hud_and_tracking_prefers_nearby_component() {
    let mut a = GameAgent::default();
    let mut p = frame(49, 49);
    for y in 1..5 {
        for x in 1..5 {
            p[y * 100 + x] = 0xffff3030;
        }
    }
    a.observe(&p, 100, 100, &profile(false), 1);
    let d = a.observe(&p, 100, 100, &profile(false), 101);
    assert!(d.target.unwrap()[0] > 0.45);
}
#[test]
fn reset_for_new_permission_drops_tracking() {
    let mut a = GameAgent::default();
    let p = frame(49, 49);
    a.observe(&p, 100, 100, &profile(false), 1);
    a.observe(&p, 100, 100, &profile(false), 101);
    a.reset();
    assert!(!a.observe(&p, 100, 100, &profile(false), 201).confirmed);
}
