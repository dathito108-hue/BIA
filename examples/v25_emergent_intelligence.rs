use bia_core::run_v25_emergent_intelligence_evaluation;

fn main() {
    let report = run_v25_emergent_intelligence_evaluation();
    println!(
        "V25_EMERGENT cases={} episodic={}/{} discovery={}/{} rules={}/{} competition={}/{} multidomain={}/{} accuracy={:.3} elapsed_us={}",
        report.cases,
        report.episodic_passes,
        report.cases,
        report.discovery_passes,
        report.cases,
        report.rule_passes,
        report.cases,
        report.competition_passes,
        report.cases,
        report.multidomain_passes,
        report.cases,
        report.accuracy(),
        report.elapsed.as_micros()
    );
    assert!(report.passed(), "V25 emergent intelligence failed: {report:?}");
}
