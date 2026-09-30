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
adb pull /sdcard/Android/data/com.bia.mobile/files/product-suite.zip game-evidence/product-suite.zip
adb pull /sdcard/Android/data/com.bia.mobile/files/product-demo.zip game-evidence/product-demo.zip
for name in creative-mandala.zip creative-landscape.zip creative-vase.zip creative-proof.png creative-scene-v141.zip creative-image-v141.zip motion-v142.glb motion-v142.zip image-trial-v142.zip; do
    adb pull "/sdcard/Android/data/com.bia.mobile/files/$name" "game-evidence/$name"
done
python3 - <<'PY'
from pathlib import Path
s=Path('game-evidence/instrumentation.txt').read_text()
assert 'OK (34 tests)' in s and 'FAILURES' not in s and 'INSTRUMENTATION_FAILED' not in s, s
PY

npm install --prefix /tmp/bia-gltf-validation --ignore-scripts --no-audit --no-fund gltf-validator@2.0.0-dev.3.10
NODE_PATH=/tmp/bia-gltf-validation/node_modules node scripts/validate-motion-glb.cjs game-evidence/motion-v142.glb game-evidence/glb-validation.json
