use bia_core::run_v14_stress;

fn main() {
    let report = run_v14_stress(20_000);
    println!(
        "V14_STRESS iterations={} bounded={}/{} state={}/{} noise={}/{} deterministic={}/{} elapsed_ms={} ns_per_iteration={}",
        report.iterations,
        report.bounded_token_passes,
        report.iterations,
        report.finite_state_passes,
        report.iterations,
        report.noise_passes,
        report.iterations,
        report.deterministic_replay_passes,
        report.iterations,
        report.elapsed.as_millis(),
        report.ns_per_iteration()
    );
    assert!(report.passed(), "V14 stress failed: {report:?}");
    assert!(report.elapsed.as_secs_f32() < 10.0, "V14 stress regression: {:?}", report.elapsed);
}
