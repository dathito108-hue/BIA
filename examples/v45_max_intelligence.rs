use bia_core::run_v45_max_intelligence_evaluation;

fn main() {
    let report = run_v45_max_intelligence_evaluation();
    println!(
        "V45_MAX cases={} meta={}/{} calibration={}/{} evidence={}/{} recursive={}/{} routing={}/{} transfer={}/{} accuracy={:.3} elapsed_us={}",
        report.cases,
        report.metacognition_passes, report.cases,
        report.calibration_passes, report.cases,
        report.evidence_passes, report.cases,
        report.recursive_passes, report.cases,
        report.compute_routing_passes, report.cases,
        report.transfer_passes, report.cases,
        report.accuracy(),
        report.elapsed.as_micros()
    );
    assert!(report.passed(), "V45 max intelligence failed: {report:?}");
}
