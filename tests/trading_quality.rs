use bia_core::trading_quality::evaluate;
fn points() -> Vec<(u64, f64)> {
    (0..60)
        .map(|i| (100000 + i * 15000, 100.0 * (1.001_f64).powi(i as i32)))
        .collect()
}
#[test]
fn no_lookahead_in_pending_outcomes() {
    let p = points();
    let a = evaluate(&p[..56], p[55].0, 10.0).unwrap();
    let mut changed = p[..56].to_vec();
    changed[55].1 = 1.0;
    let b = evaluate(&changed, p[55].0, 10.0).unwrap();
    assert_eq!(a.samples, b.samples);
    assert_eq!(a.successes, b.successes + 1);
    assert!(a.lower_bound < 1.0);
}
#[test]
fn reject_stale_gap_reordered_and_nan() {
    let mut p = points();
    let now = p[59].0;
    assert!(evaluate(&p, now + 45001, 1.0).is_err());
    assert!(evaluate(&p, now, f64::NAN).is_err());
    p[4].0 = p[3].0;
    assert!(evaluate(&p, now, 1.0).is_err());
    p = points();
    p[4].1 = f64::INFINITY;
    assert!(evaluate(&p, now, 1.0).is_err());
}
#[test]
fn fees_can_remove_all_signals() {
    let p = points();
    let now = p[59].0;
    let q = evaluate(&p, now, 1000.0).unwrap();
    assert_eq!(q.direction, 0);
    assert_eq!(q.samples, 0);
    assert_eq!(q.lower_bound, 0.0);
}
