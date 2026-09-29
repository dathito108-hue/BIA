use bia_core::run_v18_open_reasoning_evaluation;

fn main() {
    let report = run_v18_open_reasoning_evaluation();
    println!(
        "V18_OPEN cases={} parse={}/{} composition={}/{} counterfactual={}/{} contradiction={}/{} paraphrase={}/{} accuracy={:.3} elapsed_us={}",
        report.cases,
        report.parse_passes,
        report.cases,
        report.composition_passes,
        report.cases,
        report.counterfactual_passes,
        report.cases,
        report.contradiction_passes,
        report.cases,
        report.paraphrase_passes,
        report.cases,
        report.accuracy(),
        report.elapsed.as_micros()
    );
    assert!(report.passed(), "V18 open reasoning failed: {report:?}");
}
