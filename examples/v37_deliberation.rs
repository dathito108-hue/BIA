use bia_core::run_v37_deliberation_evaluation;

fn main() {
    let report = run_v37_deliberation_evaluation();
    println!(
        "V37_DELIBERATION cases={} prediction={}/{} planning={}/{} avoidance={}/{} replan={}/{} bounded={}/{} accuracy={:.3} elapsed_us={}",
        report.cases,
        report.prediction_passes, report.cases,
        report.planning_passes, report.cases,
        report.avoidance_passes, report.cases,
        report.replan_passes, report.cases,
        report.bounded_passes, report.cases,
        report.accuracy(),
        report.elapsed.as_micros()
    );
    assert!(report.passed(), "V37 deliberation failed: {report:?}");
}
