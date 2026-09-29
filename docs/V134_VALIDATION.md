# V134 game-loop validation

Base: `a28deb63d4594d261e6f5f4e2403206d394d2fb2`.

Native code segments connected components matching a calibrated RGB tolerance in a
bounded ROI, confirms a target across distinct fresh frames, and emits FPS/MOBA
reflex decisions. Android uses a user-consented foreground MediaProjection session
and a separately enabled AccessibilityService. Gesture scope is one exact package,
calibrated coordinates, 5 minutes / 1,200 attempts; no permission survives process death.
Projection callbacks, display geometry, foreground package, lock state, frame age,
thermal severity, dispatch return values and gesture callbacks gate execution.

## Tests

Nine native cases cover confirmation, simultaneous MOBA decisions, FPS range,
target loss, stale/nonincreasing timestamps, invalid dimensions/ROI, noise/large
background rejection, HUD exclusion and resetting tracking on a fresh grant.
The existing Rust suite and milestone evaluations also run.

Android instrumentation `GameLoopTest` is a required PR gate on an API 35 x86_64
emulator. It enables the service only inside that disposable test device, launches
the built-in arena, accepts the test's MediaProjection dialog, starts a five-minute
scope, and requires actual received multi-pointer MotionEvents plus arena hits.
It then leaves the target, requires the loop to pause, presses Stop, and requires
the capture service to end. Test output, logcat and a screenshot are retained as
`BIA-Game-evidence`; only passing output is evidence of this path working.

```sh
cargo clippy --all-targets -- -D warnings
cargo test --release
bash scripts/game-emulator-test.sh  # requires Android SDK, built JNI and emulator
```

## Evidence limits

The native cases use synthetic frames. The Android case uses a purpose-built local
arena with a known red marker and configured controls. Neither establishes skill
in an installed commercial FPS/MOBA, anti-cheat compatibility, general perception,
strategic reasoning, win rate, or latency/temperature on Samsung S21 FE / Android 16.
The product does not bypass protected capture or input restrictions.

## Android contracts consulted

- https://developer.android.com/media/grow/media-projection
- https://developer.android.com/reference/android/accessibilityservice/AccessibilityService
- https://github.com/ReactiveCircus/android-emulator-runner
