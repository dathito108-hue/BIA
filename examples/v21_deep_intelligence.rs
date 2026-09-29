use bia_core::run_v21_deep_intelligence_evaluation;

fn main() {
    let report = run_v21_deep_intelligence_evaluation();
    println!(
        "V21_DEEP cases={} abstraction={}/{} analogy={}/{} induction={}/{} composition={}/{} accuracy={:.3} elapsed_us={}",
        report.cases,
        report.abstraction_passes,
        report.cases,
        report.analogy_passes,
        report.cases,
        report.induction_passes,
        report.cases,
        report.compositional_passes,
        report.cases,
        report.accuracy(),
        report.elapsed.as_micros()
    );
    assert!(report.passed(), "V21 deep intelligence failed: {report:?}");
}
