use bia_core::run_v16_generalization_evaluation;

fn main() {
    let report = run_v16_generalization_evaluation();
    println!(
        "V16_GENERALIZATION cases={} long_chain={}/{} distractor={}/{} counterfactual={}/{} reversal={}/{} persistence={}/{} accuracy={:.3} elapsed_us={}",
        report.cases,
        report.long_chain_passes,
        report.cases,
        report.distractor_passes,
        report.cases,
        report.counterfactual_passes,
        report.cases,
        report.reversal_passes,
        report.cases,
        report.persistence_passes,
        report.cases,
        report.accuracy(),
        report.elapsed.as_micros()
    );
    assert!(report.passed(), "V16 generalization failed: {report:?}");
}
