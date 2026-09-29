use std::time::Instant;

use bia_core::{run_v11_evaluation, DuyenTokenDecoder};

fn main() {
    let report = run_v11_evaluation();
    assert!(report.passed(), "V11 acceptance failed: {report:?}");

    let mut decoder = DuyenTokenDecoder::default();
    let start = Instant::now();
    let iterations = 20_000usize;
    let mut produced = 0usize;

    for i in 0..iterations {
        let input = match i % 5 {
            0 => "Mở YouTube",
            1 => "Tìm web Phật giáo Trúc Lâm",
            2 => "Tại sao cần provenance?",
            3 => "Suy luận logic trừu tượng",
            _ => "Không thực thi nếu chưa xác nhận",
        };
        produced += decoder.generate(input, 8).tokens.len();
    }

    let elapsed = start.elapsed();
    let ns_per_sequence = elapsed.as_nanos() / iterations as u128;

    println!(
        "V11_EVAL cases={} semantic={:.3} deterministic={:.3} realm={:.3} eval_us={}",
        report.cases,
        report.semantic_accuracy(),
        report.deterministic_rate(),
        report.realm_accuracy(),
        report.elapsed.as_micros()
    );
    println!(
        "V11_BENCH iterations={} produced_tokens={} elapsed_ms={} ns_per_sequence={}",
        iterations,
        produced,
        elapsed.as_millis(),
        ns_per_sequence
    );

    // Generous CI guardrail: catches accidental complexity explosions, not a phone claim.
    assert!(elapsed.as_secs_f32() < 5.0, "V11 benchmark regression: {elapsed:?}");
}
