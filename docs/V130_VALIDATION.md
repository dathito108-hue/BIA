# V130: local multi-document evidence retrieval

Base: `3cd8a28adfaf5a4a8b71122d6149726f43ee6b81` (V129).

## Problem and implementation

The previous causal chat fallback retrieved only three lexically similar records,
then retried reasoning. Intermediate causal documents could be missed, and an
already-supported claim never retrieved local counterevidence.

V130 parses bounded excerpts once, indexes clauses by source concept, and follows
reachable concepts over up to six passes. Existing world edges can bridge into
local records. New clauses are discounted by source confidence and imported only
into a temporary graph with bounded extra capacity. Already-known edges are not
reinforced. A resulting contradiction remains a contradiction in normal mobile
review. Retrieved documents never enter device-action parsing or the learning loop.

## Evaluation design

`run_evidence_evaluation` runs 128 generated cases with source-to-target chains of
2–6 edges, varying identities and source quality, reversed document order and
24 unrelated parents of the target. It compares against an isolated top-three
lexical retrieval policy using the same parser and semantic reasoner. This is
not an end-to-end comparison with the entire V129 application.

Each case must recover the exact chain length, detect a newly supplied opposing
relation and leave the durable graph/ledger unchanged. `cargo run --release
--example v130_evidence` prints the measured counts and total duration. Android
`/evidence` runs the same evaluator on-device without touching the user's runtime.

Regression coverage also tests missing bridges, untrusted/non-finite source
confidence, duplicates, existing-world bridging, low-memory behavior, oversized
Unicode excerpts, counterevidence to an already-known claim, pending-action
preservation, record/depth limits and dense-import limits.

## Scope and limitations

The structured Vietnamese parser must recognize the source text. No broad-language
model or external AI service is introduced. At most 96 records are considered,
large excerpts are skipped, and source/frontier/search limits can omit valid paths.
The UI discloses retrieval truncation. Records/clauses are not independent evidence
counts. Counterfactual retrieval and durable learned-edge provenance are unchanged.
Confidence is a bounded engineering score, not a calibrated causal probability.

## Measured local results

- Release: 87 library tests + 11 V129 regressions + 11 V130 regressions = 109 passed.
- Clippy all targets with warnings denied: passed.
- 128/128 chain recovery, 128/128 conflict detection, 128/128 durable-memory isolation.
- Top-three lexical baseline: 0/128 chain recovery on these deliberately distracting
  multi-document fixtures; this is not a general benchmark score.
- Full 128-case evaluator: 17.943385 ms on this x86_64 workspace. This includes both
  retrieval policies and conflict/isolation checks; it is not phone chat latency.
- Android compilation and packaged APK are checked by the pull-request workflow.
