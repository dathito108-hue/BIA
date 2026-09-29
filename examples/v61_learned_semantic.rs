use bia_core::run_v61_learned_semantic_evaluation;

fn main() {
    let report = run_v61_learned_semantic_evaluation();
    println!(
        "V61_SEMANTIC cases={} embedding={}/{} latent_memory={}/{} relation={}/{} retrieval={}/{} compression={}/{} hybrid={}/{} accuracy={:.3} elapsed_us={}",
        report.cases,
        report.embedding_passes, report.cases,
        report.latent_memory_passes, report.cases,
        report.relation_passes, report.cases,
        report.vector_retrieval_passes, report.cases,
        report.compression_passes, report.cases,
        report.hybrid_passes, report.cases,
        report.accuracy(),
        report.elapsed.as_micros()
    );
    assert!(report.passed(), "V61 learned semantic failed: {report:?}");
}
