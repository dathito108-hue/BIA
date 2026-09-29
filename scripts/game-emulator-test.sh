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
adb shell am instrument -w -r com.bia.mobile.test/android.test.InstrumentationTestRunner | tee game-evidence/instrumentation.txt
adb logcat -d > game-evidence/logcat.txt
adb pull /sdcard/Android/data/com.bia.mobile/files/game-proof.png game-evidence/game-proof.png || true
adb pull /sdcard/Android/data/com.bia.mobile/files/game-fps-proof.png game-evidence/game-fps-proof.png || true
python3 - <<'PY'
from pathlib import Path
s=Path('game-evidence/instrumentation.txt').read_text()
assert 'OK (1 test)' in s and 'FAILURES' not in s and 'INSTRUMENTATION_FAILED' not in s, s
PY
