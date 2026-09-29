use std::time::Instant;

use bia_core::{run_v12_evaluation, DuyenTokenDecoder};

fn main() {
    let report = run_v12_evaluation();
    assert!(report.passed(), "V12 acceptance failed: {report:?}");

    let mut decoder = DuyenTokenDecoder::default();
    let learned = decoder.learn_text(
        "lotusgraph duyencore tructam provenancefield causallattice mobilekernel",
    );
    assert!(learned >= 3);

    let iterations = 20_000usize;
    let start = Instant::now();
    let mut produced = 0usize;

    for i in 0..iterations {
        let input = match i % 5 {
            0 => "Hãy mở phần cài đặt",
            1 => "Tại sao cần trạng thái tái diễn",
            2 => "Phân tích kiến trúc mới",
            3 => "Đọc tệp trên thiết bị",
            _ => "Không mở ứng dụng đó",
        };
        produced += decoder.generate(input, 8).tokens.len();
    }

    let elapsed = start.elapsed();
    let ns_per_token = elapsed.as_nanos() / produced.max(1) as u128;
    let tokens_per_sec = if elapsed.as_nanos() == 0 {
        0
    } else {
        (produced as u128 * 1_000_000_000u128) / elapsed.as_nanos()
    };

    println!(
        "V12_EVAL held_out={} semantic={:.3} deterministic={:.3} learned={} emitted={} eval_us={}",
        report.held_out_cases,
        report.semantic_accuracy(),
        report.deterministic_rate(),
        report.dynamic_vocab_learned,
        report.dynamic_vocab_emitted,
        report.elapsed.as_micros()
    );
    println!(
        "V12_BENCH iterations={} produced_tokens={} elapsed_ms={} ns_per_token={} tokens_per_sec={} vocab={}",
        iterations,
        produced,
        elapsed.as_millis(),
        ns_per_token,
        tokens_per_sec,
        decoder.learned_vocab_len()
    );

    assert!(elapsed.as_secs_f32() < 5.0, "V12 benchmark regression: {elapsed:?}");
}
