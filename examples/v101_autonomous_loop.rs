use bia_core::run_v101_autonomous_loop_evaluation;

fn main() {
    let report = run_v101_autonomous_loop_evaluation();
    println!(
        "V101_LOOP cases={} questions={}/{} critic={}/{} loop={}/{} self_review={}/{} idle={}/{} authority={}/{} accuracy={:.3} elapsed_us={}",
        report.cases,
        report.question_passes, report.cases,
        report.critic_passes, report.cases,
        report.loop_passes, report.cases,
        report.self_review_passes, report.cases,
        report.idle_passes, report.cases,
        report.authority_passes, report.cases,
        report.accuracy(),
        report.elapsed.as_micros()
    );
    assert!(report.passed(), "V101 autonomous loop failed: {report:?}");
}
