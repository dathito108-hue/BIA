# V131 integrated validation

Base: `fefb32ea19c6bd74abacb24e8b632cb303c490fc` (V130).

This is one integrated delivery across five bounded capability areas, not a claim
that any area is generally solved. Command syntax and practical limits are in
V131_GUIDE.md. All changes use the existing native BIA core; no teacher model,
pretrained model or external AI service is added.

## Evidence

The new integration tests cover causal paraphrases/direction, recognized negation,
inhibition parsing, named-source correction/retraction, hypothesis exclusion,
ambiguous pronouns, feedback-driven replanning, restart persistence, repeated-feedback
idempotence, forbidden outcomes, useful skills beyond the first eight declarations,
counterexample-aware transfer, duplicate examples, journal saturation, atomic bad
restore, pending-action preservation and document/governance isolation.

The native evaluator runs 32 fresh sessions varying entity IDs, four causal wording
forms and fallback plan length from two to four steps. Each session combines language,
source revision, planning, failure feedback, transfer and continuity. This tests
interactions between features rather than only separate components.

Commands:

```sh
cargo test --release --lib --tests
cargo clippy --all-targets -- -D warnings
cargo run --release --example v131_integrated
```

Android `/integrated` runs the same evaluator without touching the live user runtime.
CI additionally runs every earlier release evaluation and builds the ARM64 APK.
Desktop/CI elapsed times must not be reported as phone inference speed.

## Material limits

Managed sources are distinct from the legacy learned graph; corrections apply to the
named-source workspace and do not claim to erase every legacy-derived relation.
Negation handling is conservative exclusion for specified forms, not a full truth
logic. Skill feedback is user-reported, not a device execution receipt. Skills are
explicit precondition/add-effect descriptions; planning is bounded simulation only.
Transfer is shared-outcome induction over declared classes and named examples;
it never promotes the inferred result into a fact or executable capability.

Mutation state survives restart; transient current-plan/query context does not.
Journal capacity rejects further changes rather than forgetting retractions.

## Local measured results

- Release tests: 123 passed (87 existing library, 11 V129, 11 V130, 14 integrated).
- Clippy all targets, warnings denied: passed.
- Native evaluator: 32/32 language, revision, feedback, planning, transfer and continuity.
- Full combined evaluator took 6.203122 ms in this x86_64 workspace; this is not
  Android chat latency or a general intelligence benchmark.
