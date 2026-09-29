use bia_core::run_v31_autonomous_knowledge_evaluation;

fn main() {
    let report = run_v31_autonomous_knowledge_evaluation();
    println!(
        "V31_AUTONOMOUS cases={} hierarchy={}/{} hypothesis={}/{} falsification={}/{} meta_rule={}/{} governance={}/{} accuracy={:.3} elapsed_us={}",
        report.cases,
        report.hierarchy_passes,
        report.cases,
        report.hypothesis_passes,
        report.cases,
        report.falsification_passes,
        report.cases,
        report.meta_rule_passes,
        report.cases,
        report.governance_passes,
        report.cases,
        report.accuracy(),
        report.elapsed.as_micros()
    );
    assert!(report.passed(), "V31 autonomous knowledge failed: {report:?}");
}
