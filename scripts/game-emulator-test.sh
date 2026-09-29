#!/usr/bin/env bash
set -euo pipefail
mkdir -p game-evidence
gradle :app:installDebug :app:installDebugAndroidTest --console=plain
adb shell input keyevent KEYCODE_WAKEUP
adb shell wm dismiss-keyguard
# Fresh emulator images perform package indexing after boot_completed.
# Let that one-time work settle; production gesture deadlines stay unchanged.
sleep 30
adb logcat -c
set +e
adb shell am instrument -w -r com.bia.mobile.test/android.test.InstrumentationTestRunner | tee game-evidence/instrumentation.txt
instrument_status=${PIPESTATUS[0]}
adb logcat -b crash -d > game-evidence/crash.txt
adb logcat -d -t 3000 > game-evidence/logcat.txt
set -e
test "$instrument_status" -eq 0
adb pull /sdcard/Android/data/com.bia.mobile/files/game-proof.png game-evidence/game-proof.png || true
adb pull /sdcard/Android/data/com.bia.mobile/files/game-fps-proof.png game-evidence/game-fps-proof.png || true
adb pull /sdcard/Android/data/com.bia.mobile/files/dex-evidence.txt game-evidence/dex-evidence.txt || true
adb pull /sdcard/Android/data/com.bia.mobile/files/market-evidence.txt game-evidence/market-evidence.txt || true
python3 - <<'PY'
from pathlib import Path
s=Path('game-evidence/instrumentation.txt').read_text()
assert 'OK (5 tests)' in s and 'FAILURES' not in s and 'INSTRUMENTATION_FAILED' not in s, s
PY
