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
# Explicit test classes avoid legacy runner scanning every dependency DEX at startup.
# Discover from test sources so new suites are not silently omitted.
test_classes=$(python3 - <<'PYTEST'
from pathlib import Path
import re
classes=[]
for p in sorted(Path('app/src/androidTest/java').rglob('*Test.java')):
    source=p.read_text()
    package=re.search(r'package\s+([\w.]+)\s*;',source).group(1)
    name=re.search(r'public\s+(?:final\s+)?class\s+(\w+)',source).group(1)
    classes.append(package+'.'+name)
assert classes
print(','.join(classes))
PYTEST
)
set +e
adb shell am instrument -w -r -e class "$test_classes" com.bia.mobile.test/android.test.InstrumentationTestRunner | tee game-evidence/instrumentation.txt
instrument_status=${PIPESTATUS[0]}
adb logcat -b crash -d > game-evidence/crash.txt
adb logcat -d -t 3000 > game-evidence/logcat.txt
set -e
test "$instrument_status" -eq 0
adb pull /sdcard/Android/data/com.bia.mobile/files/game-proof.png game-evidence/game-proof.png || true
adb pull /sdcard/Android/data/com.bia.mobile/files/game-fps-proof.png game-evidence/game-fps-proof.png || true
adb pull /sdcard/Android/data/com.bia.mobile/files/dex-evidence.txt game-evidence/dex-evidence.txt || true
adb pull /sdcard/Android/data/com.bia.mobile/files/market-evidence.txt game-evidence/market-evidence.txt || true
adb pull /sdcard/Android/data/com.bia.mobile/files/solana-evidence.txt game-evidence/solana-evidence.txt || true
python3 - <<'PY'
from pathlib import Path
s=Path('game-evidence/instrumentation.txt').read_text()
assert 'OK (13 tests)' in s and 'FAILURES' not in s and 'INSTRUMENTATION_FAILED' not in s, s
PY
