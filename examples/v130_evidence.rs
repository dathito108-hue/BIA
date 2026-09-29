fn main() {
    let report = bia_core::evidence_evaluation::run_evidence_evaluation();
    println!("V130_EVIDENCE {report:?}");
    assert!(report.passed());
}
