# V133 continuous execution validation

Base: `c6d7c046febeed77fde34debfe96f24601b852fe` (V132).

## Changes

- Explicit step, snapshot-bound batch and exact-effect session grants in native Rust.
- The Android JNI claim path now requires an unexpired grant before an ID can be claimed.
- Session grants expire after 30 minutes or 100 claims; attempts consume quota before
  an OS effect. Batch grants cannot authorize fresh IDs with identical payloads.
- Grants include adapter kind, exact payload and authority. They are not serialized;
  a successful continuity import revokes existing in-memory permission.
- Explicit/repeated queues support 12 steps; inference-based planning stays at four.
- One foreground scheduler runs one step per callback, with a 150 ms UI yield between
  steps. There is no busy polling. External intents wait for return to BIA.
- Existing write/read-back claim persistence remains before every OS effect. Failures,
  cancellation and storage uncertainty revoke automatic authority. Stop preserves an
  unresolved in-flight claim rather than pretending its side effect was undone.

## Verification

`tests/continuous_execution.rs` exercises fourteen scenarios: twelve-step execution
with one batch approval; one-step scope; stale review rejection; session budget;
exact payload/kind/authority scope; expiry and clock rollback; renewed consent after
restart; interrupted claims; failure revocation across different skills; stop;
binding replacement; atomic bounded sequence parsing; document isolation; and
permission revocation on import. The acknowledgments in these native tests are
simulated, not Android intent or clipboard execution evidence.

```sh
cargo clippy --all-targets -- -D warnings
cargo test --release
```

The CI runs prior release evaluations and compiles ARM64 JNI plus Android Java into
an APK. No emulator/device runtime measurement is included. The 150 ms delay is a
scheduler setting, not a measured device latency or speed-up claim. Session grants
allow only the already reviewed effects and do not establish that declared skill
preconditions or semantic goals are true.
