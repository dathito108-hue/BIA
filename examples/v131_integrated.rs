fn main() {
    let report = bia_core::integrated_evaluation::run_integrated_evaluation();
    println!("V131_INTEGRATED {report:?}");
    assert!(report.passed());
}
