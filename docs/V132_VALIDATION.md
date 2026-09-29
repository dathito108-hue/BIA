# V132 execution validation

Base: `9a4d427b21d7d6617629f8fb5de36a2571aed7fa` (V131).

The native skill planner now prepares real Android adapter requests. Java displays
and approves each exact payload, claims its unique ID, commits and reads back the
claim before an OS effect, then records an ID-bound adapter receipt. Unknown claims
remain blocked after restart. Failed adapters stop the chain and persist a skill
block separately from the bounded user journal. Direct legacy actions also receive
monotonic IDs. OS dispatch receipts do not advance semantic goal completion.

## Native checks

`tests/skill_execution.rs` covers eleven scenarios:

- Exact Unicode, case, URL punctuation and literal backslash payload preservation;
  command-like payload text stays data, unsupported bindings are rejected.
- Planning/declaration does not execute; claims and receipts require the current ID.
- Duplicate/stale approval and completion cannot consume another queued step.
- Crash recovery holds an unknown claim and explicit restart never reuses its ID.
- Adapter failure stops the chain and blocks the skill across restart.
- Cancellation is not learned as failure; editing a binding does not mutate a queued payload.
- Unbound skills and commands in imported documents cannot execute.
- Invalid execution snapshots are rejected atomically, including counter rollback
  relative to retained IDs and duplicate pending IDs.
- Queue overflow cannot evict a claimed action.
- Direct actions also retain unique execution IDs across restart.
- Dispatch acceptance does not advance a semantic goal; adapter failure still
  persists when the 128-entry user journal is full.

```sh
cargo clippy --all-targets -- -D warnings
cargo test --release
```

The CI Rust job also runs the prior release evaluators. The Android job compiles
JNI for ARM64 and the Java app, then builds the debug APK. Native tests simulate
adapter acknowledgments; they do not run Android intents or clipboard APIs.

## Scope

The planner reasons over declared conditions/effects and is capped at four steps.
Each Android effect needs foreground approval. Clipboard uses immediate read-back;
other adapters only report OS dispatch acceptance. These are not end-to-end task
success proofs. This delivery does not include an on-device/emulator runtime test.
AtomicFile plus read-back is a process-interruption guard, not an exactly-once
transaction with an external application or protection against a hostile file rollback.
