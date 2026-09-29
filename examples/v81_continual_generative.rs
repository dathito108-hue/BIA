use bia_core::run_v81_continual_generative_evaluation;

fn main() {
    let report = run_v81_continual_generative_evaluation();
    println!(
        "V81_CONTINUAL cases={} continual={}/{} anchor={}/{} composition={}/{} symbol={}/{} consolidation={}/{} generation={}/{} accuracy={:.3} elapsed_us={}",
        report.cases,
        report.continual_passes, report.cases,
        report.anchor_passes, report.cases,
        report.composition_passes, report.cases,
        report.symbol_passes, report.cases,
        report.consolidation_passes, report.cases,
        report.generation_passes, report.cases,
        report.accuracy(),
        report.elapsed.as_micros()
    );
    assert!(report.passed(), "V81 continual/generative failed: {report:?}");
}
