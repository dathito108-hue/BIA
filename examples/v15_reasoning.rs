use bia_core::run_v15_reasoning_evaluation;

fn main() {
    let report = run_v15_reasoning_evaluation();
    println!(
        "V15_REASONING cases={} multi_hop={}/{} contradiction={}/{} revision={}/{} persistence={}/{} accuracy={:.3} elapsed_us={}",
        report.cases,
        report.multi_hop_passes,
        report.cases,
        report.contradiction_passes,
        report.cases,
        report.revision_passes,
        report.cases,
        report.causal_persistence_passes,
        report.cases,
        report.accuracy(),
        report.elapsed.as_micros()
    );
    assert!(report.passed(), "V15 reasoning quality failed: {report:?}");
}
